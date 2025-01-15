<div align="center">
  <img width=100% src="https://github.com/MarcosFlavioGS/Password_Genius/assets/95108526/8939958b-8dd5-46e8-8a9c-12e175157604" alt="Capturar">
</div>

# PassGen

This is a password generator written in Rust. It generates secure passwords with a specified length and saves them in separate files for easy retrieval. The generated passwords can also be copied to the clipboard for convenient use.

## Features

The password generator provides the following features:

- Generate secure passwords with a specified length
- Save generated passwords in separate files
- Retrieve passwords from saved files
- Copy passwords to the clipboard for quick access

## Requirements

- Rust programming language

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/MarcosFlavioGS/Password_Genius.git
   ```
 2. Cd do project directory and run the **install.sh** scritp:

  ```bash
  ./install.sh
  ```

## Config

Once you have installed or built Passgen, you need to run "passgen config" so it can create the your initial config file. It will be saved at ~/.config/passgen/passgen.toml. This file is created automaticaly so you don't need to worry with it.

### Show pass

You can either request for the passwords to be displayed in the terminal once you generate a new one or get an existing one, or for it to be hidden from the terminal, only being copied to your clipboard.

### Encryption

PassGen encrypts your passwords for better safety. But for that it expects that you have a secret key in your passgen config file. If not key is given, it will default to "ramdomkey123".

## Usage

The password generator can be used with the following command-line arguments:

```bash
$ ./passgen [command] [dirname]
```
- [command] can be any of the following:
   - *Generate* | *g*:  Generates a new password and saves it in a file in the specified directory name.
   - *Insert* | *i*: Inserts a password into the specified dir file.
   - *get*: Retrieves the password from the specified file.
   - *config*: Creates the config file for Passgen.
   - *version*: Displays the current version.
- [dirname] is the path to the directory where the password will be stored or retrieved. Also, it is the identifier.

## Examples

Creates a new directory named *github* that will contain a file with the new password:
```bash
$ ./passgen generate github
```
Passgen will then create a new github folder inside a *password* folder in your Home directory.

You could create multiple passwords for the same source passing the entire path:
```bash
./passgen generate github/profile1
```
Then, to retrieve the password, just type get followed profile1:
```bash
./passgen get profile1
```
or
```bash
./passgen get github/profile1
```

## File Structure
The generated passwords are stored in separate files within the passwords directory. Each file represents a specific password source or category.

The file structure will look like:
```
passwords/
├── source1/
│ └── pass
├── source2/
│ └── pass
└── source3/
└── pass
```
