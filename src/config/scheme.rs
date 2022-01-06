use std::fs;

use serde::{Deserialize, Serialize};
use yaml_rust::YamlLoader;

use crate::{
    constants::{CONFIG_FILE, CONFIG_FOLDER},
    file_handler::file::{read_file, write_file},
    runner::lang::{cpp::CppConfig, python::PythonConfig},
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultConfig {
    pub cpp_config: CppConfig,
    pub python_config: PythonConfig,
}

pub const DEFAULT_CONFIG_YAML: &str = "
cpp_config:
    program: g++
    standard: \"-std=c++17\"
    flags:
        - \"-Wall\"
        - \"-DONLINE_JUDGE=1\"
python_config:
    program: python
    flags:
        - \"ONLINE_JUDGE=1\"
";

pub fn write_config_yaml(yaml_content: &str) -> String {
    let config_file = &shellexpand::tilde(CONFIG_FILE).to_string()[..];
    let config_folder = shellexpand::tilde(CONFIG_FOLDER).to_string();

    // create the folder ~/.quicktest and the file ~/.quicktest/config.yaml
    // with the default settings
    if fs::create_dir_all(config_folder).is_ok()
        && write_file(config_file, yaml_content.as_bytes()).is_ok()
    {}
    DEFAULT_CONFIG_YAML.to_string()
}

pub fn write_default_config_yaml() -> String {
    write_config_yaml(DEFAULT_CONFIG_YAML)
}

pub fn load_default_config() -> DefaultConfig {
    let mut config_text = String::new();

    let config_file = &shellexpand::tilde(CONFIG_FILE).to_string()[..];

    let cpp_default = CppConfig::default();
    let python_default = PythonConfig::default();

    if let Some(text) = read_file(config_file) {
        // if ~/.quicktest/config.yaml file exists, read the settings
        config_text.push_str(&text[..]);
    } else {
        // create the folder ~/.quicktest and the file ~/.quicktest/config.yaml
        // with the default settings
        config_text = write_default_config_yaml();
    }

    let docs = YamlLoader::load_from_str(&config_text[..]).unwrap();
    let doc = &docs[0];

    let cpp_config_program = if let Some(program) = doc["cpp_config"]["program"].as_str() {
        program.to_string()
    } else {
        cpp_default.program.clone()
    };

    let cpp_config_standard = if let Some(standard) = doc["cpp_config"]["standard"].as_str() {
        standard.to_string()
    } else {
        cpp_default.standard
    };

    let cpp_config_flags = if let Some(flags) = doc["cpp_config"]["flags"].as_vec() {
        flags
            .iter()
            .map(|item| item.as_str().unwrap().to_string())
            .collect::<Vec<String>>()
    } else {
        cpp_default.flags
    };

    let python_config_program = if let Some(program) = doc["python_config"]["program"].as_str() {
        program.to_string()
    } else {
        python_default.program
    };

    let python_config_flags = if let Some(flags) = doc["python_config"]["flags"].as_vec() {
        flags
            .iter()
            .map(|item| item.as_str().unwrap().to_string())
            .collect::<Vec<String>>()
    } else {
        python_default.flags
    };

    DefaultConfig {
        cpp_config: CppConfig {
            program: cpp_config_program,
            standard: cpp_config_standard,
            flags: cpp_config_flags,
        },
        python_config: PythonConfig {
            program: python_config_program,
            flags: python_config_flags,
        },
    }
}
