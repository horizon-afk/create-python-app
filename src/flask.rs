use std::fs;



fn mkdir(folder_name : String) {
    fs::create_dir(folder_name).expect("Error creating folder");
}

pub fn create_flask_env() {
    
    
    mkdir(String::from("templates"));
    mkdir(String::from("static"));
}