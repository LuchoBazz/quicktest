use std::fs;

use yaml_rust::YamlLoader;

use crate::{
    constants::{CONFIG_FILE, CONFIG_FOLDER},
    file_handler::file::{read_file, write_file},
    runner::lang::{cpp::CppConfig, python::PythonConfig},
};

#[derive(Debug, PartialEq)]
pub struct DefaultConfig {
    cpp_config: CppConfig,
    python_config: PythonConfig,
}

pub const DEFAULT_CONFIG_YAML: &str = "
cpp-config:
    program: g++
    standard: -std=c++17
python-config:
    program: python3
";

pub fn load_default_config() -> DefaultConfig {
    let mut config_text = String::new();

    let config_file = &shellexpand::tilde(CONFIG_FILE).to_string()[..];
    let config_folder = shellexpand::tilde(CONFIG_FOLDER).to_string();

    let cpp_default = CppConfig::default();
    let python_default = PythonConfig::default();

    if let Some(text) = read_file(config_file) {
        // if ~/.quicktest/config.yaml file exists, read the settings
        config_text.push_str(&text[..]);
    } else {
        // create the folder ~/.quicktest and the file ~/.quicktest/config.yaml
        // with the default settings
        if fs::create_dir_all(config_folder).is_ok()
            && write_file(config_file, DEFAULT_CONFIG_YAML.as_bytes()).is_ok()
        {}
        config_text = DEFAULT_CONFIG_YAML.to_string();
    }

    let docs = YamlLoader::load_from_str(&config_text[..]).unwrap();
    let doc = &docs[0];

    let cpp_config_program = if let Some(program) = doc["cpp-config"]["program"].as_str() {
        program.to_string()
    } else {
        cpp_default.program.clone()
    };

    let cpp_config_standard = if let Some(standard) = doc["cpp-config"]["standard"].as_str() {
        standard.to_string()
    } else {
        cpp_default.standard.clone()
    };

    let python_config_program = if let Some(program) = doc["python-config"]["program"].as_str() {
        program.to_string()
    } else {
        python_default.program.clone()
    };

    let tree = DefaultConfig {
        cpp_config: CppConfig {
            program: cpp_config_program,
            standard: cpp_config_standard,
        },
        python_config: PythonConfig {
            program: python_config_program,
        },
    };

    tree
}
