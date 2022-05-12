/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use crate::config::languages_config::LANGUAGES_CONFIG_JSON;
use crate::constants::{CONFIG_FOLDER, LANGUAGE_CONFIG_FILE};
use crate::file_handler::file::{read_file, write_file};
use crate::language::json::language_scheme::Languages;
use crate::views::style::show_config_file_path;
use std::fs;

pub fn write_config_data(json: &str) {
    let config_file = &shellexpand::tilde(LANGUAGE_CONFIG_FILE).to_string()[..];
    let config_folder = shellexpand::tilde(CONFIG_FOLDER).to_string();

    // create the folder ~/.quicktest and the file ~/.quicktest/language.config.json
    // with the default settings
    if fs::create_dir_all(config_folder).is_ok() && write_file(config_file, json.as_bytes()).is_ok()
    {
        show_config_file_path(config_file);
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

        let data = LANGUAGES_CONFIG_JSON.as_bytes().to_vec();

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
