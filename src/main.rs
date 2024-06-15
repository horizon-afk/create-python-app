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

    // creating the app directory
    let dir_name = interface.directory;
    fs::create_dir(dir_name.clone()).expect("Invalid filename");
    fs::File::create_new(dir_name.clone()+ "/app.py").expect("File already exists");
    env::set_current_dir(&dir_name).expect("Error! Can't change current directory");

    // creating the virtual environment
    let mut venv = Command::new("python");
    venv.args(["-m", "virtualenv", ".venv"]).spawn()
        .expect("Could not create venv. Download virtualenv and try again.");
    
}
