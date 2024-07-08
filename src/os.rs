use std::env;
use std::fs;
use std::fs::File;

pub fn mkdir(folder_name: String) {
    fs::create_dir(&folder_name).expect(
        (String::from("Error ") + &folder_name + &String::from(" already exists")).as_str(),
    );
}

pub fn mkfile(file_name: String) -> File {
    fs::File::create_new(&file_name)
        .expect((String::from("Error! ") + &file_name + &String::from("already exists")).as_str())
}

pub fn cwd(dir_name: String) {
    env::set_current_dir(&dir_name)
        .expect((String::from("Unable to change directory to ") + &dir_name).as_str());
}
