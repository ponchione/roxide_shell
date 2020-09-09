use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::str::FromStr;


pub enum BuiltIns {
    Pwd,
    Ls,
    Cd
}

impl FromStr for BuiltIns {
    type Err = ();

    fn from_str(built_in: &str) -> Result<Self, Self::Err> {
        match built_in {
            "pwd" => Ok(BuiltIns::Pwd),
            "ls" => Ok(BuiltIns::Ls),
            "cd" => Ok(BuiltIns::Cd),
            _ => Err(())
        }
    }
}