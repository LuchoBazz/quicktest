/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use exitfailure::ExitFailure;

use crate::{
    config::load_config::{read_language_configuration, write_language_configuration},
    error::handle_error::throw_setup_label_is_not_correct_msg,
    language::json::language_scheme::Languages,
    painter::setup::show_argument_was_updated_success,
};

pub fn run(label: &str, value: &str) -> Result<(), ExitFailure> {
    let text = read_language_configuration();

    let mut langs: serde_json::Result<Languages> = serde_json::from_str(&text[..]);

    let cmds: Vec<&str> = label.split('.').collect();

    println!("{:?}", cmds);

    if cmds.len() != 2 {
        return throw_setup_label_is_not_correct_msg(label);
    }

    if let Ok(lg) = &mut langs {
        let lang_label = &(*cmds[0]);
        let label = &(*cmds[1]);

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
