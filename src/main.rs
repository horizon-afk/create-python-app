use clap::{command, Parser};
use std::{io::Write, process::Command};

mod flask;
mod os;

#[derive(Parser)]
#[command(version, about)]
struct Interface {
    /// Flask app layout with a basic flask app files with a app.py and templates
    #[arg(long, default_value_t = false)]
    flask: bool,

    /// Name of the file
    directory: String,
}

fn main() {
    let interface: Interface = Interface::parse();
    let dir_name = interface.directory;

    // creating the app directory
    os::mkdir(dir_name.clone());

    // changing current working directory to the wrokspace
    os::cwd(dir_name.clone());

    // creating app.py, .gitignore, requirements.txt
    os::mkfile(String::from("app.py"));
    os::mkfile(String::from(".gitignore"));
    os::mkfile(String::from("requirements.txt"));

    // creating the virtual environment
    let mut venv = Command::new("python");
    venv.args(["-m", "virtualenv", ".venv"])
        .spawn()
        .expect("Could not create venv. Download virtualenv and try again.");

    // creating app.py
    let mut py_file = os::mkfile(String::from("app.py"));
    py_file
        .write_all("print (\"Hello World\")".as_bytes())
        .expect("Failed to write app.py");

    // initialize git
    let mut git = Command::new("git");
    git.args(["init"])
        .spawn()
        .expect("Could not git init. Download git and try again.");

    // flask workspace
    if interface.flask {
        flask::create_flask_env();
    }
}
