use rand::Rng;
use std::io;

pub fn new_password() -> String {
    let mut input = String::new();
    println!("Please type the length of the password:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let length: u8 = input.trim().parse().expect("Please type a number");
    let password = generate_password(length);// New password
    println!("your new password is: {}", password);
    password
}

fn generate_password(length: u8) -> String {
    let mut password: String = String::new();
    for _ in 0..length {
        password.push(get_random_char());// Pushing characters to password string
    }
    let special = count_special(&password);// Checking how many special characters are in pass
    if special < (length * (20 / 100)) { // Making sure theres at liest 20% of special chars in password
        password = generate_password(length); // Recursion because I'm lazy
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
        3 => special_chr(&mut rng), // I'll explain below |
        _ => panic!("Something went wrong!"),           //v
    }
}

fn special_chr(rng: &mut rand::rngs::ThreadRng) -> char {
    let special_chars = "*+-_#=&%$@!><?~";           // Pulling the same way as normal chars was excluding many characters
    let index = rng.gen_range(0..special_chars.len());
    let random_char = special_chars.chars().nth(index).unwrap();
    random_char
}

fn count_special(s: &String) -> u8 {
    let mut _counter: u8 = 0;

    for c in s.chars() {
        if c.is_alphabetic() || c.is_numeric() {
            continue;
        } else {
            _counter += 1;
        }
    }
    _counter
}
