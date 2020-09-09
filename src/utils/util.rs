use std::{env, io};
use std::io::{Write, stdout, stdin};
use std::fs::OpenOptions;



pub struct Command {
    keyword : String,
    args : Vec<String>,
}

pub fn tokenize_command(c: &String) -> Command {
    let mut split: Vec<String> = c.split_whitespace().map(|s| s.to_string()).collect();
    println!("DEBUG: {:?}", split);

    match split.len() {
        0 => Command {keyword: "".to_owned(), args: Vec::new()},
        _ => Command {keyword: split.remove(0), args: split}
    }
}

pub fn process_command(cmd: &Command) {
    match command::from_str() {

    }
}

pub fn push_to_history(c: &String) {
    let history_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("roxide_history");

    match history_file {
        Ok(_) => {
            history_file.unwrap().write_all(c.as_bytes())
        },
        _ => Err(io::Error::from_raw_os_error(1))
    };

    //TODO better error handling on this match
}

pub fn print_current_path() {
    let current = env::current_dir().unwrap();
    let here = current.into_os_string().into_string().unwrap();
    print!("ROX {} ", here);
    stdout().flush().expect("Cannot flush terminal!");
}

pub fn print_logo() {
    println!("  ______  _____  _     _ _____ ______  _______");
    println!(" |_____/ |     |  \\___/    |   |     \\ |______");
    println!(" |    \\_ |_____| _/   \\_ __|__ |_____/ |______    v0.1.0\n");
}