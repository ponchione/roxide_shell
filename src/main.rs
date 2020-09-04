use commands::command;
use std::env;
use std::io::{Write, stdout, stdin};

mod commands;


fn main() {

    loop {
        print_current_path();
        let mut full_command: String = String::new();
        stdin().read_line(&mut full_command).expect("Failed to read command!");

        //Create history file with entries
        command::push_to_history(&full_command);

        let cmd = command::tokenize_command(&full_command);

    }
}

fn print_current_path() {
    let current = env::current_dir().unwrap();
    let here = current.into_os_string().into_string().unwrap();
    print!("ROX {} ", here);
    stdout().flush().expect("Cannot flush terminal!");
}

fn print_logo() {
    println!("  ______  _____  _     _ _____ ______  _______");
    println!(" |_____/ |     |  \\___/    |   |     \\ |______");
    println!(" |    \\_ |_____| _/   \\_ __|__ |_____/ |______    v0.1.0\n");
}