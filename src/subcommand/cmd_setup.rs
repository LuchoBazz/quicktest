/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use exitfailure::ExitFailure;

use crate::{
    config::load_config::{read_language_configuration, write_language_configuration},
    error::handle_error::throw_setup_label_is_not_correct_msg,
    language::json::language_scheme::Languages, painter::setup::show_argument_was_updated_success,
};

/*
use crate::{
    config::scheme::{write_config_yaml, write_default_config_yaml, DefaultConfig},
    constants::CONFIG_FILE,
    file_handler::file::read_file,
    painter::setup::show_argument_was_updated_success,
};

pub fn setup_cpp(program: &str, standard: &str, flags: &str) -> Result<(), ExitFailure> {
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

    if !flags.is_empty() {
        deserializer.cpp_config.flags = flags
            .split(';')
            .collect::<Vec<_>>()
            .iter()
            .map(|&flag| flag.to_string())
            .collect::<Vec<_>>();
        show_argument_was_updated_success(
            "C++",
            "flags",
            &format!("{:?}", deserializer.cpp_config.flags)[..],
        );
    }

    let serializer = serde_yaml::to_string(&deserializer).unwrap();

    write_config_yaml(&serializer[..]);

    Ok(())
}

pub fn setup_python(program: &str, flags: &str) -> Result<(), ExitFailure> {
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

    if !flags.is_empty() {
        deserializer.python_config.flags = flags
            .split(';')
            .collect::<Vec<_>>()
            .iter()
            .map(|&flag| flag.to_string())
            .collect::<Vec<_>>();
        show_argument_was_updated_success(
            "Python",
            "flags",
            &format!("{:?}", deserializer.python_config.flags)[..],
        );
    }

    let serializer = serde_yaml::to_string(&deserializer).unwrap();

    write_config_yaml(&serializer[..]);
    Ok(())
}
*/

pub fn run(label: &str, value: &str) -> Result<(), ExitFailure> {
    let text = read_language_configuration();

    let mut langs: serde_json::Result<Languages> = serde_json::from_str(&text[..]);

    let cmds: Vec<&str> = label.split('.').collect();

    println!("{:?}", cmds);

    if cmds.len() != 2 {
        return throw_setup_label_is_not_correct_msg(label);
    }

    if let Ok(lg) = &mut langs {
        let lang_label = cmds[0].clone();
        let label = cmds[1].clone();

        println!("{:?}", lang_label);
        println!("{:?}", label);


        for idx in 0..lg.languages.len() {
            if lg.languages[idx].id == lang_label && lg.languages[idx].env.contains_key(label) {
                lg.languages[idx]
                    .env
                    .insert(label.to_string(), value.to_string());
                write_language_configuration(&langs.unwrap())?;

                show_argument_was_updated_success(
                    &lang_label,
                    &label,
                    &value
                );

                return Ok(());
            }
        }
    }

    throw_setup_label_is_not_correct_msg(label)
}
