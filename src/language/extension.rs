/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use crate::{
    config::load_config::read_language_configuration,
    error::handle_error::{throw_configuration_file_error_msg, throw_extension_not_supported_msg},
    language::json::language_scheme::{LanguageScheme, Languages},
};
use exitfailure::ExitFailure;

pub fn map_extension(
    file_label: &str,
    ext: &str,
    lang_out: &mut LanguageScheme,
) -> Result<(), ExitFailure> {
    let text = read_language_configuration();

    let langs: serde_json::Result<Languages> = serde_json::from_str(&text[..]);

    if let Ok(lg) = langs {
        let ext_str = ext.to_string();

        for idx in 0..lg.languages.len() {
            let pos = lg.languages[idx]
                .extensions
                .iter()
                .position(|item| *item == ext_str);

            if pos.is_some() {
                let temporal = lg.languages[idx].clone();
                *lang_out = temporal;
                return Ok(());
            }
        }
        return throw_extension_not_supported_msg(file_label, ext);
    }

    throw_configuration_file_error_msg("~/.quicktest/languages.config.json")
}

pub fn is_extension_supported(ext: &str) -> bool {
    let text = read_language_configuration();

    let langs: serde_json::Result<Languages> = serde_json::from_str(&text[..]);

    if let Ok(lg) = langs {
        let ext_str = ext.to_string();

        for idx in 0..lg.languages.len() {
            let pos = lg.languages[idx]
                .extensions
                .iter()
                .position(|item| *item == ext_str);

            if pos.is_some() {
                return true;
            }
        }
    }
    false
}
