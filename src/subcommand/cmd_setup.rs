/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use exitfailure::ExitFailure;

use crate::{
    config::scheme::{write_config_yaml, write_default_config_yaml, DefaultConfig},
    constants::CONFIG_FILE,
    file_handler::file::read_file,
    painter::setup::show_argument_was_updated_success,
};

pub fn setup_cpp(program: &str, standard: &str) -> Result<(), ExitFailure> {
    let mut config_text = String::new();

    let config_file = &shellexpand::tilde(CONFIG_FILE).to_string()[..];

    if let Some(text) = read_file(config_file) {
        // if ~/.quicktest/config.yaml file exists, read the settings
        config_text.push_str(&text[..]);
    } else {
        // create the folder ~/.quicktest and the file ~/.quicktest/config.yaml
        // with the default settings
        config_text = write_default_config_yaml();
    }

    let mut deserializer: DefaultConfig = serde_yaml::from_str(&config_text[..])?;

    if !program.is_empty() {
        deserializer.cpp_config.program = program.to_string();
        show_argument_was_updated_success("C++", "program", &deserializer.cpp_config.program[..]);
    }
    if !standard.is_empty() {
        deserializer.cpp_config.standard = standard.to_string();
        show_argument_was_updated_success("C++", "standard", &deserializer.cpp_config.standard[..]);
    }

    let serializer = serde_yaml::to_string(&deserializer).unwrap();

    write_config_yaml(&serializer[..]);

    Ok(())
}

pub fn setup_python(program: &str) -> Result<(), ExitFailure> {
    let mut config_text = String::new();

    let config_file = &shellexpand::tilde(CONFIG_FILE).to_string()[..];

    if let Some(text) = read_file(config_file) {
        // if ~/.quicktest/config.yaml file exists, read the settings
        config_text.push_str(&text[..]);
    } else {
        // create the folder ~/.quicktest and the file ~/.quicktest/config.yaml
        // with the default settings
        config_text = write_default_config_yaml();
    }

    let mut deserializer: DefaultConfig = serde_yaml::from_str(&config_text[..])?;

    if !program.is_empty() {
        deserializer.python_config.program = program.to_string();
        show_argument_was_updated_success(
            "Python",
            "program",
            &deserializer.python_config.program[..],
        );
    }

    let serializer = serde_yaml::to_string(&deserializer).unwrap();

    write_config_yaml(&serializer[..]);
    Ok(())
}
