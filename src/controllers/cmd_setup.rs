/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use exitfailure::ExitFailure;

use crate::{
    cli::model::setup_command::SetupCommand,
    config::load_config::{read_language_configuration, write_language_configuration},
    constants::LANGUAGE_CONFIG_FILE,
    error::handle_error::throw_setup_label_is_not_correct_msg,
    file_handler::file::remove_file,
    language::json::language_scheme::Languages,
    views::{setup::show_argument_was_updated_success, style::show_config_file_deleted_path},
};

pub fn run(command: &SetupCommand) -> Result<(), ExitFailure> {
    let text = read_language_configuration();
    let label: &str = &command.label[..];
    let value: &str = &command.value[..];

    let mut langs: serde_json::Result<Languages> = serde_json::from_str(&text[..]);

    let cmds: Vec<&str> = label.split('.').collect();

    if cmds.len() != 2 {
        return throw_setup_label_is_not_correct_msg(label);
    }

    if let Ok(lg) = &mut langs {
        let lang_label = cmds[0];
        let label = cmds[1];

        for idx in 0..lg.languages.len() {
            if lg.languages[idx].id == lang_label && lg.languages[idx].env.contains_key(label) {
                lg.languages[idx]
                    .env
                    .insert(label.to_string(), value.to_string());
                write_language_configuration(&langs.unwrap())?;

                show_argument_was_updated_success(lang_label, label, value);

                return Ok(());
            }
        }
    }

    throw_setup_label_is_not_correct_msg(label)
}

// Show Help setup after help: quicktest setup config --help
// EXAMPLES:
//     quicktest setup config --label="Language::Cpp.PROGRAM" --value="g++"
//     quicktest setup config --label="Language::Cpp.STANDARD" --value="-std=c++17"
//     quicktest setup config --label="Language::Python.PROGRAM" --value="python"

pub fn show_help_setup() -> &'static str {
    let text = read_language_configuration();

    let mut langs: serde_json::Result<Languages> = serde_json::from_str(&text[..]);

    let mut labels = vec!["EXAMPLES:".to_string()];

    if let Ok(lg) = &mut langs {
        for idx in 0..lg.languages.len() {
            for (key, value) in &lg.languages[idx].env {
                let label = format!(
                    "    quicktest setup config --label=\"{}.{}\" --value=\"{}\"",
                    lg.languages[idx].id, key, value
                );
                labels.push(label);
            }
        }
    }
    let labels: String = labels.join("\n");
    Box::leak(labels.into_boxed_str())
}

pub fn reset() -> Result<(), ExitFailure> {
    let config_file = &shellexpand::tilde(LANGUAGE_CONFIG_FILE).to_string()[..];
    if remove_file(config_file) {
        show_config_file_deleted_path(config_file);
    }
    Ok(())
}
