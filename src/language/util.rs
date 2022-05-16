/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021-present / Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use regex::Regex;
use std::collections::BTreeMap;

pub fn replace_env_variable(text: &str, env: &BTreeMap<String, String>) -> String {
    let mut text = text.to_string();
    for (key, value) in env.iter() {
        // Replace ${WORD} to env.WORD
        let regex_key = &format!(r"\$\{{{}\}}", key)[..];
        let re = Regex::new(regex_key).unwrap();
        let result = re.replace_all(&text[..], &value[..]);
        text = result.to_string();
    }
    text
}
