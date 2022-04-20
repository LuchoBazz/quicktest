use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::language::{traits::BuildEnvVariables, util::replace_env_variable};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OSCommand {
    pub unix: String,
    pub windows: String,
}

impl OSCommand {
    pub fn new() -> Self {
        OSCommand {
            unix: String::new(),
            windows: String::new(),
        }
    }
}

impl Default for OSCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildEnvVariables for OSCommand {
    fn build_env_variables(&mut self, env: &BTreeMap<String, String>) {
        if cfg!(windows) {
            self.windows = replace_env_variable(&self.windows, env);
        } else {
            self.unix = replace_env_variable(&self.windows, env);
        }
    }
}
