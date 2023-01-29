use std::io;
use rand::Rng;

fn main() {
    greetings();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let lenght: u8 = input.trim().parse().expect("Please type a number!");
    let password = generate_password(lenght);
    println!("Your password is: {}", password);
}

fn greetings(){
    println!("Hello, this is your password generator.\nPlease infor the lenght: ");
}

fn generate_password(lenght: u8) -> String {
    let mut password: String = String::new();
    for _ in 0..lenght {
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
        3 => rng.gen_range('!'..='~'),
        _ => panic!("Something went wrong!"),
    }
}
