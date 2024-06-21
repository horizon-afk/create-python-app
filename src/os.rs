use std::env;
use std::fs;
use std::fs::File;

pub fn mkdir(folder_name: String) {
    fs::create_dir(folder_name).expect("Error creating folder");
}

pub fn mkfile(file_name: String) -> File {
    fs::File::create_new(file_name).expect("Error! File already exists")
}

pub fn cwd(dir_name: String) {
    env::set_current_dir(&dir_name).expect("Unable to change directory");
}
