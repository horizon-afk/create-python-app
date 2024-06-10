use std::env;
use std::fs;
use clap::arg;
use clap::command;
use clap::{Command};



fn main() {
    let generate = command!()
    .arg(arg!(--dirname <DIRECTORY> "Name of the project"))
    .get_matches();


    
}
