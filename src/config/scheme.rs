use yaml_rust::YamlLoader;

use crate::runner::lang::{
    cpp::{CppConfig, DEFAULT_CPP_CONFIG},
    python::{PythonConfig, DEFAULT_PYTHON_CONFIG},
};

#[derive(Debug, PartialEq)]
pub struct DefaultConfig {
    cpp_config: CppConfig,
    python_config: PythonConfig,
}

pub fn load_default_config() -> Option<DefaultConfig> {
    let s = "
cpp-config:
    program: g++
    standard: -std=c++17
python-config:
    program: python3
";
    let docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &docs[0];

    let cpp_config_program = if let Some(program) = doc["cpp-config"]["program"].as_str() {
        program.to_string()
    } else {
        DEFAULT_CPP_CONFIG.program.clone()
    };

    let cpp_config_standard = if let Some(standard) = doc["cpp-config"]["standard"].as_str() {
        standard.to_string()
    } else {
        DEFAULT_CPP_CONFIG.standard.clone()
    };

    let python_config_program = if let Some(program) = doc["python-config"]["program"].as_str() {
        program.to_string()
    } else {
        DEFAULT_PYTHON_CONFIG.program.clone()
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

    println!("{:?}", tree);

    Some(tree)
}
