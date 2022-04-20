/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use crate::constants::{CONFIG_FOLDER, LANGUAGE_CONFIG_FILE, LANGUAGE_CONFIG_FILE_DEFAULT};
use crate::file_handler::file::{read_file, write_file};
use crate::file_handler::path::get_root_path;
use crate::language::json::language_scheme::Languages;
use std::fs::File;
use std::{fs, io::Read};

pub fn write_config_data(json: &str) {
    let config_file = &shellexpand::tilde(LANGUAGE_CONFIG_FILE).to_string()[..];
    let config_folder = shellexpand::tilde(CONFIG_FOLDER).to_string();

    // create the folder ~/.quicktest and the file ~/.quicktest/language.config.json
    // with the default settings
    if fs::create_dir_all(config_folder).is_ok() && write_file(config_file, json.as_bytes()).is_ok()
    {
    }
}

pub fn read_language_configuration() -> String {
    let mut config_text = String::new();

    let config_file = &shellexpand::tilde(LANGUAGE_CONFIG_FILE).to_string()[..];

    if let Some(text) = read_file(config_file) {
        // if ~/.quicktest/language.config.json file exists, read the settings
        config_text.push_str(&text[..]);
    } else {
        // create the folder ~/.quicktest and the file ~/.quicktest/config.yaml
        // with the default settings
        let root = &get_root_path()[..];

        let config_file_default = &format!("{}/{}", root, LANGUAGE_CONFIG_FILE_DEFAULT)[..];
        println!("{}", config_file_default);
        let mut config_file_default = File::open(config_file_default).expect("Unable to open file");

        let mut data = Vec::new();
        config_file_default
            .read_to_end(&mut data)
            .expect("Unable to read data");

        config_text = data.iter().map(|ch| *ch as char).collect::<String>();

        write_config_data(&config_text[..]);
    }

    config_text
}

pub fn write_language_configuration(data: &Languages) -> serde_json::Result<()> {
    let lang_str = serde_json::to_string_pretty(&data)?;
    write_config_data(&lang_str[..]);
    Ok(())
}
