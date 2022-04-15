use std::collections::BTreeMap;

// use crate::runner::types::StatusResponse;

/*
pub trait Language {
    fn build(&self) -> bool;
    fn execute(&self, timeout: u32, testcase: u32) -> StatusResponse;
    // fn execute(&self, timeout: u32, testcase: u32);
    fn set_stdio(&mut self, stdin: &str);
    fn is_installed(&self) -> bool;
    fn get_name(&self) -> String;
}*/

pub trait BuildEnvVariables {
    fn build_env_variables(&mut self, env: &BTreeMap<String, String>);
}
