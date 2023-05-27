mod clipboarding;
mod new_pass;
mod insert_pass;
mod insert;
mod generate;

use generate::generate;
use insert::insert;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("TODO: Show filetree of all created passwords."),
        (2..=3) => {
            match &args[1][..] {
                "generate" => {
                    let filepath = &args[2][..];
                    let mut base_path = String::from("passwords/");
                    base_path.push_str(filepath);
                    base_path.push_str("/pass");
                    println!("{base_path}");
                    generate(&base_path);
                },
                "insert" => {
                    let filepath = &args[2][..];
                    let mut base_path = String::from("passwords/");
                    base_path.push_str(filepath);
                    base_path.push_str("/pass");
                    insert(filepath);
                },
                _ => println!("Do nothing !"),
            }
        },
        _ => panic!("Too many arguments !"),
    }
}
