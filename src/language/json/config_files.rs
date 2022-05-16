/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021-present / Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use serde::{Deserialize, Serialize};

use crate::file_handler::file::write_file;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigFile {
    pub path: String,
    pub content: String,
}

impl ConfigFile {
    pub fn new() -> Self {
        ConfigFile {
            path: String::new(),
            content: String::new(),
        }
    }

    pub fn create(&self) -> bool {
        // create the configuration files
        if write_file(
            &shellexpand::tilde(&self.path).to_string()[..],
            self.content.as_bytes(),
        )
        .is_ok()
        {
            return true;
        }
        false
    }
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self::new()
    }
}
