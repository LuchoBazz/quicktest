#![allow(warnings, unused)]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::language::{traits::BuildEnvVariables, util::replace_env_variable};

use super::os_command::OSCommand;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Languages {
    pub languages: Vec<LanguageScheme>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LanguageScheme {
    pub id: String,
    pub name: String,
    pub extensions: Vec<String>,
    pub description: String,
    pub env: BTreeMap<String, String>,
    pub compile: Option<OSCommand>,
    pub execute: OSCommand,
    pub check_installed: String,
}

impl LanguageScheme {
    pub fn new() -> Self {
        LanguageScheme {
            id: String::new(),
            name: String::new(),
            extensions: Vec::new(),
            description: String::new(),
            env: BTreeMap::new(),
            compile: Some(OSCommand::new()),
            execute: OSCommand::new(),
            check_installed: String::new(),
        }
    }

    pub fn add_env_variable(&mut self, env: &String, value: &String) {
        self.env.insert(env.clone(), value.clone());
    }
}

impl BuildEnvVariables for LanguageScheme {
    fn build_env_variables(&mut self, env: &BTreeMap<String, String>) {
        // replace Environment Variable of self.execute
        self.execute.build_env_variables(&env);

        // replace Environment Variable of self.compile
        if self.compile.is_some() {
            let op = self.compile.as_ref().unwrap();
            let mut temporal_op = op.clone();
            temporal_op.build_env_variables(&env);
            self.compile = Some(temporal_op);
        }
        // replace Environment Variable of self.check_installed
        self.check_installed.build_env_variables(&env);
    }
}

impl BuildEnvVariables for String {
    fn build_env_variables(&mut self, env: &BTreeMap<String, String>) {
        let str_self = self.clone();
        let temporal = replace_env_variable(&str_self, &env);
        *self = temporal.clone();
    }
}
