use std::{fs, env};
use std::path::Path;
use std::process::Command;
use clap::{command, Parser};



#[derive(Parser)]
#[command(version,about)]
struct  Interface {
    
    /// Flask app layout with a basic flask app files with a app.py and templates
    #[arg(long, default_value_t = false)]
    flask: bool,

    /// Name of the file
    directory: String
}


fn main() {

    let interface : Interface = Interface::parse();

    let dir_name = interface.directory;
    fs::create_dir(dir_name.clone()).expect("Invalid filename");
    fs::File::create_new(dir_name.clone()+ "/app.py").expect("File already exists");

    
}
