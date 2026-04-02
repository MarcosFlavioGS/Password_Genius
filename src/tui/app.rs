//! Application state and keyboard handling for the Ratatui interface.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::widgets::ListState;

use crate::clipboard::clipboarder::clipboarder;
use crate::config::write::write_config;
use crate::config::{default_config, read::read_config_result, Config};
use crate::deleter::delete::delete;
use crate::directories::get::get_directories;
use crate::generator::gen::generate_stored;
use crate::inserter::insert::insert_password;
use crate::password::getter::getter;
use crate::path::config::get_path;
use crate::utils::get_path::get_base_path;

/// Which field is focused on the generate form.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GenFocus {
    Name,
    Length,
}

/// Which field is focused on the insert form.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InsertFocus {
    Name,
    Password,
}

/// Which field is focused on the config form.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConfigFocus {
    ShowPass,
    Key,
}

/// Active screen in the TUI.
#[derive(Clone)]
pub enum Screen {
    Main,
    Generate {
        name: String,
        length: String,
        focus: GenFocus,
    },
    Insert {
        name: String,
        password: String,
        focus: InsertFocus,
    },
    Get {
        name: String,
        plain: Option<String>,
        err: Option<String>,
    },
    DeleteConfirm {
        name: String,
    },
    Config {
        show_pass: bool,
        passgen_key: String,
        focus: ConfigFocus,
        is_new: bool,
    },
}

/// Last user-facing status line (success vs error styling in [`super::ui`]).
pub enum StatusKind {
    Ok,
    Err,
}

/// Full TUI state.
pub struct App {
    pub screen: Screen,
    pub config: Option<Config>,
    pub entries: Vec<String>,
    pub list_state: ListState,
    pub status: Option<(StatusKind, String)>,
}

impl App {
    /// Builds initial state: loads config or opens the config wizard.
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        match read_config_result() {
            Ok(config) => {
                let mut app = Self {
                    screen: Screen::Main,
                    config: Some(config),
                    entries: Vec::new(),
                    list_state,
                    status: None,
                };
                app.refresh_list();
                app
            }
            Err(e) => {
                let d = default_config();
                list_state.select(None);
                Self {
                    screen: Screen::Config {
                        show_pass: d.options.show_pass,
                        passgen_key: d.encryption.passgen_key.clone(),
                        focus: ConfigFocus::Key,
                        is_new: true,
                    },
                    config: None,
                    entries: Vec::new(),
                    list_state,
                    status: Some((
                        StatusKind::Err,
                        format!("No valid config ({e}). Create one below, then save (Ctrl+S)."),
                    )),
                }
            }
        }
    }

    /// Reloads password names from `~/passgen/`.
    pub fn refresh_list(&mut self) {
        let path = get_path();
        self.entries = get_directories(&path);
        self.entries.sort();
        if self.entries.is_empty() {
            self.list_state.select(None);
        } else {
            match self.list_state.selected() {
                Some(i) if i < self.entries.len() => {}
                _ => self.list_state.select(Some(0)),
            }
        }
    }

    fn set_status_ok(&mut self, msg: impl Into<String>) {
        self.status = Some((StatusKind::Ok, msg.into()));
    }

    fn set_status_err(&mut self, msg: impl Into<String>) {
        self.status = Some((StatusKind::Err, msg.into()));
    }

    fn selected_name(&self) -> Option<&str> {
        self.list_state
            .selected()
            .and_then(|i| self.entries.get(i).map(|s| s.as_str()))
    }

    /// Returns `true` when the app should exit.
    pub fn handle_key(&mut self, key: &KeyEvent) -> bool {
        match &mut self.screen {
            Screen::Main => self.handle_main(key),
            Screen::Generate { .. } => self.handle_generate(key),
            Screen::Insert { .. } => self.handle_insert(key),
            Screen::Get { .. } => self.handle_get(key),
            Screen::DeleteConfirm { .. } => self.handle_delete_confirm(key),
            Screen::Config { .. } => self.handle_config(key),
        }
    }

    fn handle_main(&mut self, key: &KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => return true,
            KeyCode::Char('g') | KeyCode::Char('G') => {
                self.screen = Screen::Generate {
                    name: String::new(),
                    length: String::from("16"),
                    focus: GenFocus::Name,
                };
                self.status = None;
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                self.screen = Screen::Insert {
                    name: String::new(),
                    password: String::new(),
                    focus: InsertFocus::Name,
                };
                self.status = None;
            }
            KeyCode::Char(c)
                if matches!(c, 's' | 'S') && !key.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                if let Some(ref cfg) = self.config {
                    self.screen = Screen::Config {
                        show_pass: cfg.options.show_pass,
                        passgen_key: cfg.encryption.passgen_key.clone(),
                        focus: ConfigFocus::ShowPass,
                        is_new: false,
                    };
                } else {
                    let d = default_config();
                    self.screen = Screen::Config {
                        show_pass: d.options.show_pass,
                        passgen_key: d.encryption.passgen_key.clone(),
                        focus: ConfigFocus::ShowPass,
                        is_new: true,
                    };
                }
                self.status = None;
            }
            KeyCode::Enter => {
                if let Some(name) = self.selected_name() {
                    let name = name.to_string();
                    self.open_get(name);
                } else {
                    self.set_status_err("No entry selected.");
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if let Some(name) = self.selected_name() {
                    self.screen = Screen::DeleteConfirm {
                        name: name.to_string(),
                    };
                    self.status = None;
                } else {
                    self.set_status_err("No entry selected.");
                }
            }
            KeyCode::Down | KeyCode::Char('j') => self.select_next(),
            KeyCode::Up | KeyCode::Char('k') => self.select_prev(),
            _ => {}
        }
        false
    }

    fn select_next(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        let i = self.list_state.selected().unwrap_or(0);
        let next = (i + 1).min(self.entries.len() - 1);
        self.list_state.select(Some(next));
    }

    fn select_prev(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        let i = self.list_state.selected().unwrap_or(0);
        let prev = i.saturating_sub(1);
        self.list_state.select(Some(prev));
    }

    fn open_get(&mut self, name: String) {
        let Some(ref config) = self.config else {
            self.set_status_err("Config missing.");
            return;
        };
        match getter(&name, config) {
            Ok(p) => {
                self.screen = Screen::Get {
                    name,
                    plain: Some(p),
                    err: None,
                };
                self.status = None;
            }
            Err(e) => {
                self.screen = Screen::Get {
                    name,
                    plain: None,
                    err: Some(e.to_string()),
                };
                self.status = None;
            }
        }
    }

    fn handle_generate(&mut self, key: &KeyEvent) -> bool {
        let Screen::Generate {
            name,
            length,
            focus,
        } = &mut self.screen
        else {
            return false;
        };

        match key.code {
            KeyCode::Esc => {
                self.screen = Screen::Main;
                self.status = None;
            }
            KeyCode::Tab => {
                *focus = match focus {
                    GenFocus::Name => GenFocus::Length,
                    GenFocus::Length => GenFocus::Name,
                };
            }
            KeyCode::Enter => {
                if *focus == GenFocus::Name {
                    *focus = GenFocus::Length;
                    return false;
                }
                let Some(cfg) = self.config.as_ref() else {
                    self.set_status_err("Config missing.");
                    return false;
                };
                if name.trim().is_empty() {
                    self.set_status_err("Name cannot be empty.");
                    return false;
                }
                let len: u8 = match length.trim().parse() {
                    Ok(v) => v,
                    Err(_) => {
                        self.set_status_err("Length must be a number (1–255).");
                        return false;
                    }
                };
                if len == 0 {
                    self.set_status_err("Length must be at least 1.");
                    return false;
                }
                let base = get_base_path(name.trim(), "passgen/");
                match generate_stored(&base, cfg, len) {
                    Ok(pass) => {
                        let msg = if cfg.options.show_pass {
                            format!("Saved and copied. Password: {pass}")
                        } else {
                            "Saved and copied to clipboard.".to_string()
                        };
                        self.set_status_ok(msg);
                        self.screen = Screen::Main;
                        self.refresh_list();
                    }
                    Err(e) => self.set_status_err(e),
                }
            }
            KeyCode::Backspace => match focus {
                GenFocus::Name => {
                    name.pop();
                }
                GenFocus::Length => {
                    length.pop();
                }
            },
            KeyCode::Char(c) => match focus {
                GenFocus::Name => {
                    if c.is_ascii() && !c.is_control() {
                        name.push(c);
                    }
                }
                GenFocus::Length => {
                    if c.is_ascii_digit() {
                        length.push(c);
                    }
                }
            },
            _ => {}
        }
        false
    }

    fn handle_insert(&mut self, key: &KeyEvent) -> bool {
        let Screen::Insert {
            name,
            password,
            focus,
        } = &mut self.screen
        else {
            return false;
        };

        match key.code {
            KeyCode::Esc => {
                self.screen = Screen::Main;
                self.status = None;
            }
            KeyCode::Tab => {
                *focus = match focus {
                    InsertFocus::Name => InsertFocus::Password,
                    InsertFocus::Password => InsertFocus::Name,
                };
            }
            KeyCode::Enter => {
                let Some(cfg) = self.config.as_ref() else {
                    self.set_status_err("Config missing.");
                    return false;
                };
                if name.trim().is_empty() {
                    self.set_status_err("Name cannot be empty.");
                    return false;
                }
                if password.is_empty() {
                    self.set_status_err("Password cannot be empty.");
                    return false;
                }
                let base = get_base_path(name.trim(), "passgen/");
                match insert_password(&base, cfg, password) {
                    Ok(()) => {
                        self.set_status_ok("Password saved.");
                        self.screen = Screen::Main;
                        self.refresh_list();
                    }
                    Err(e) => self.set_status_err(format!("{e}")),
                }
            }
            KeyCode::Backspace => match focus {
                InsertFocus::Name => {
                    name.pop();
                }
                InsertFocus::Password => {
                    password.pop();
                }
            },
            KeyCode::Char(c) => {
                if c.is_ascii() && !c.is_control() {
                    match focus {
                        InsertFocus::Name => name.push(c),
                        InsertFocus::Password => password.push(c),
                    }
                }
            }
            _ => {}
        }
        false
    }

    fn handle_get(&mut self, key: &KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                self.screen = Screen::Main;
                self.status = None;
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if let Screen::Get {
                    plain: Some(ref p),
                    name,
                    ..
                } = &self.screen
                {
                    match clipboarder(&p[..]) {
                        Ok(()) => self.set_status_ok(format!("Copied {name} to clipboard.")),
                        Err(e) => self.set_status_err(format!("Clipboard: {e}")),
                    }
                }
            }
            _ => {}
        }
        false
    }

    fn handle_delete_confirm(&mut self, key: &KeyEvent) -> bool {
        let Screen::DeleteConfirm { name } = &self.screen else {
            return false;
        };
        let name_owned = name.clone();

        match key.code {
            KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
                self.screen = Screen::Main;
                self.status = None;
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                let base = get_base_path(&name_owned, "passgen/");
                delete(&base);
                self.set_status_ok(format!("Deleted `{name_owned}`."));
                self.screen = Screen::Main;
                self.refresh_list();
            }
            _ => {}
        }
        false
    }

    fn handle_config(&mut self, ev: &KeyEvent) -> bool {
        let Screen::Config {
            show_pass,
            passgen_key,
            focus,
            is_new,
        } = &mut self.screen
        else {
            return false;
        };

        match ev.code {
            KeyCode::Esc => {
                if self.config.is_some() {
                    self.screen = Screen::Main;
                    self.status = None;
                } else if *is_new {
                    return true;
                }
            }
            KeyCode::Tab => {
                *focus = match focus {
                    ConfigFocus::ShowPass => ConfigFocus::Key,
                    ConfigFocus::Key => ConfigFocus::ShowPass,
                };
            }
            KeyCode::Char(' ') => {
                if *focus == ConfigFocus::ShowPass {
                    *show_pass = !*show_pass;
                }
            }
            KeyCode::Char('s') | KeyCode::Char('S')
                if ev.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                let new_cfg = Config {
                    options: crate::config::Options {
                        show_pass: *show_pass,
                    },
                    encryption: crate::config::Encryption {
                        passgen_key: passgen_key.trim().to_string(),
                    },
                };
                if new_cfg.encryption.passgen_key.is_empty() {
                    self.set_status_err("Encryption key cannot be empty.");
                    return false;
                }
                match write_config(&new_cfg) {
                    Ok(()) => {
                        self.config = Some(new_cfg);
                        self.set_status_ok("Configuration saved.");
                        self.screen = Screen::Main;
                        self.refresh_list();
                    }
                    Err(e) => self.set_status_err(format!("{e}")),
                }
            }
            KeyCode::Backspace => {
                if *focus == ConfigFocus::Key {
                    passgen_key.pop();
                }
            }
            KeyCode::Char(c) => {
                if *focus == ConfigFocus::Key && c.is_ascii() && !c.is_control() {
                    passgen_key.push(c);
                }
            }
            _ => {}
        }
        false
    }
}
