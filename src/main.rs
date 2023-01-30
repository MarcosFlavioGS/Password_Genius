use std::io;
use rand::Rng;

fn main() {
    greetings();
    loop { 
        println!("Press:\n1 - Generate new password.\n2 - See saved passwords\n3 - Finish Program");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let response: u8 = input.trim().parse().expect("Please type 1 or 2");
        if response == 1 {
            loop {
                let _password = new_password();
                println!("1 - Generate new password\n2 - Save password\n3 - Finish program");
                let mut response = String::new();
                io::stdin().read_line(&mut response).expect("Failed to read line");
                let response: u8 = response.trim().parse().expect("Please type 1, 2 or 3");
                match response {
                    1 => continue,
                    2 => panic!("Sorry, not yet implemented"),
                    3 => std::process::exit(0),
                    _ => panic!("Please type one of the options(1, 2 or 3)"),
                }
            }
        }
        else if response == 2 {
            panic!("Not yet implemented");
        }
        else if response == 3 {
            std::process::exit(0);
        }
        else {
            panic!("Please typ 1 ot 2");
        }
    }
}

fn greetings(){
    println!("Hello, this is your password app.\n");
}

fn new_password() -> String {
    let mut input = String::new();
    println!("Please type the lenght of the password:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let lenght: u8 = input.trim().parse().expect("Please type a number");
    let password = generate_password(lenght);
    println!("your new password is: {}", password);
    password
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
        3 => special_chr(&mut rng),
        _ => panic!("Something went wrong!"),
    }
}

fn special_chr(rng: &mut rand::rngs::ThreadRng) -> char {
    let special_chars = "*+-&%$@!><?~";
    let index = rng.gen_range(0..=special_chars.len());
    let random_char = special_chars.chars().nth(index).unwrap();
    random_char
}
