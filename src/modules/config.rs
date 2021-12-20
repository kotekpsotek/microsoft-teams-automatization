use std::env;
use std::fs;

use crate::add_newline_characters;

pub struct Config {
    pub system_disk: String,
    pub program_files_folder: String,
    pub this_program_folder: String,
    pub driver_directory: String
}

impl Config {
    pub fn new() -> Config
    {
        let system_disk: String = env::var_os("windir").expect(add_newline_characters("Environment variable could not be read", 2, 2, "err").as_str()).to_str().unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
        let program_files_folder: String = format!("{}:/Programs", system_disk);
        let this_program_folder: String = format!("{}/teams-automatization", program_files_folder);
        let driver_directory: String = fs::canonicalize("./drivers").expect(add_newline_characters("Could not read the full location of the browser driver folder!!!", 2, 2, "err").as_str()).to_str().unwrap().to_string();
        Config { system_disk, program_files_folder, this_program_folder, driver_directory }
    }
}