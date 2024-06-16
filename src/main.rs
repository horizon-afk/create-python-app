use std::{fs, env};
use std::process::Command;
use clap::{command, Parser};

mod flask;

#[derive(Parser)]
#[command(version,about)]
struct  Interface {
    
    /// Flask app layout with a basic flask app files with a app.py and templates
    #[arg(long, default_value_t = false)]
    flask: bool,

    /// Name of the file
    directory: String
}

fn mkdir(folder_name : String) {
    fs::create_dir(folder_name).expect("Error creating folder");
}

fn mkfile(file_name: String) {
    fs::File::create_new(file_name).expect("File already exists");
}


fn main() {

    let interface : Interface = Interface::parse();
    let dir_name = interface.directory;

    // creating the app directory
    mkdir(dir_name.clone());

    // changing current working directory to the wrokspace
    env::set_current_dir(&dir_name).expect("Error! Can't change current directory");

    // creating app.py, .gitignore, requirements.txt
    mkfile(String::from("app.py"));
    mkfile(String::from(".gitignore"));
    mkfile(String::from("requirements.txt"));

    // creating the virtual environment
    let mut venv = Command::new("python");
    venv.args(["-m", "virtualenv", ".venv"]).spawn()
        .expect("Could not create venv. Download virtualenv and try again.");

    // initialize git
    let mut git = Command::new("git");
    git.args(["init"]).spawn()
        .expect("Could not git init. Download git and try again.");

    // flask workspace
    if interface.flask {
        flask::create_flask_env();
    }
    
    

    
}
