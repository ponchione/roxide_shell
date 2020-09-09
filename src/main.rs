use commands::{command, util};
use utils::util;
use std::env;
use std::io::{Write, stdout, stdin};

mod commands;
mod utils;


fn main() {

    loop {
        util::print_logo();
        let mut full_command: String = String::new();
        stdin().read_line(&mut full_command).expect("Failed to read command!");
        util::push_to_history(&full_command);
        let cmd = util::tokenize_command(&full_command);


    }
}
