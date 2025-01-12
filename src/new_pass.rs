use rand::Rng;
use std::io;

///
/// Generates a new password based on users preferences.
///
pub fn new_password() -> String {
    let mut input = String::new();
    println!("Please type the length of the password:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let length: u8 = input.trim().parse().expect("Please type a number");
    let password = generate_password(length);
    println!("your new password is: {}", password);
    password
}

fn generate_password(length: u8) -> String {
    let mut password: String = String::new();
    for _ in 0..length {
        password.push(get_random_char());
    }
    // Making sure theres at least 20% of special chars in password
    if (count_special(&password) as f32) < (0.2 * length as f32) {
        password = generate_password(length);
    }
    password
}

fn get_random_char() -> char {
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(0..=3);
    match random_number {
        0 => rng.gen_range('a'..='z'),
        1 => rng.gen_range('A'..='Z'),
        2 => rng.gen_range('0'..='9'),
        3 => special_chr(&mut rng, "*+-_#=&%$@!><?~"),
        _ => panic!("Something went wrong!"),
    }
}

fn special_chr(rng: &mut rand::rngs::ThreadRng, special_chars: &str) -> char {
    special_chars
        .chars()
        .nth(rng.gen_range(0..special_chars.len()))
        .unwrap()
}

fn count_special(s: &String) -> u8 {
    s.chars()
        .filter(|c| !c.is_alphabetic() && !c.is_numeric())
        .count() as u8
}
