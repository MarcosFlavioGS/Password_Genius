use rand::Rng;
use std::io;

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

fn generate_password(lenght: u8) -> String {
    let mut password: String = String::new();
    for _ in 0..length {
        password.push(get_random_char());
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
        3 => special_chr(&mut rng),
        _ => panic!("Something went wrong!"),
    }
}

fn special_chr(rng: &mut rand::rngs::ThreadRng) -> char {
    let special_chars = "*+-&%$@!><?~";
    let index = rng.gen_range(0..special_chars.len());
    let random_char = special_chars.chars().nth(index).unwrap();
    random_char
}
