use std::fs::OpenOptions;
use std::io;
use std::io::Write;




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