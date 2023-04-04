/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021-present / Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use std::{
    path::{Path, PathBuf},
    process::Stdio,
};

use exitfailure::ExitFailure;
use std::process::Command;

use crate::{
    file_handler::{file::{configuration_commands, get_extension}, path::get_root_path},
    language::extension,
    runner::{
        cmd::{execute_program, has_installed_controller},
        types::{Language, StatusResponse},
    },
    views::style::show_installing_dependencies,
};

use super::{
    json::{config_files::ConfigFile, language_scheme::LanguageScheme},
    traits::BuildEnvVariables,
};

#[derive(Debug, Clone)]
pub struct LanguageHandler {
    check_installed: String,

    config_files: Option<Vec<ConfigFile>>,

    initialize: Option<String>,

    compile: Option<String>,

    execute: String,

    name: String,

    stderr: Option<PathBuf>,

    stdin: Option<PathBuf>,

    stdout: Option<PathBuf>,
}

#[allow(clippy::derivable_impls)]
impl Default for LanguageHandler {
    fn default() -> Self {
        Self {
            check_installed: String::new(),
            initialize: None,
            config_files: None,
            compile: None,
            execute: String::new(),
            name: String::new(),
            stderr: None,
            stdin: None,
            stdout: None,
        }
    }
}

impl LanguageHandler {
    pub fn get_language(
        lang: LanguageScheme,
        stdin: &str,
        stdout: &str,
        stderr: &str,
    ) -> LanguageHandler {
        // TODO: Revisar como refactorizar el codigo duplicado
        let root = &get_root_path()[..];

        // Agregar parametros
        let stdin = Some(PathBuf::from(format!("{}/{}", root, stdin)));
        let stdout = Some(PathBuf::from(format!("{}/{}", root, stdout)));
        let stderr = Some(PathBuf::from(format!("{}/{}", root, stderr)));

        let mut lang = lang;

        let env = lang.env.clone();

        lang.build_env_variables(&env);

        #[cfg(unix)]
        let initialize = if lang.initialize.is_some() {
            Some(lang.initialize.clone().unwrap().unix)
        } else {
            None
        };
        #[cfg(windows)]
        let initialize = if lang.initialize.is_some() {
            Some(lang.initialize.clone().unwrap().windows)
        } else {
            None
        };

        let config_files = if lang.config_files.is_some() {
            Some(lang.config_files.clone().unwrap())
        } else {
            None
        };

        #[cfg(unix)]
        let execute = lang.execute.unix.clone();
        #[cfg(windows)]
        let execute = lang.execute.windows.clone();

        #[cfg(unix)]
        let compile = if lang.compile.is_some() {
            Some(lang.compile.clone().unwrap().unix)
        } else {
            None
        };
        #[cfg(windows)]
        let compile = if lang.compile.is_some() {
            Some(lang.compile.clone().unwrap().windows)
        } else {
            None
        };

        LanguageHandler {
            initialize,
            config_files,
            compile,
            execute,
            check_installed: lang.check_installed.clone(),
            name: lang.name.clone(),
            stdin,
            stdout,
            stderr,
        }
    }

    pub fn get_generator(lang: LanguageScheme, stdout: &str) -> LanguageHandler {
        let root = &get_root_path()[..];

        let stdout = Some(PathBuf::from(format!("{}/{}", root, stdout)));

        let mut lang = lang;

        let env = lang.env.clone();

        lang.build_env_variables(&env);

        #[cfg(unix)]
        let initialize = if lang.initialize.is_some() {
            Some(lang.initialize.clone().unwrap().unix)
        } else {
            None
        };
        #[cfg(windows)]
        let initialize = if lang.initialize.is_some() {
            Some(lang.initialize.clone().unwrap().windows)
        } else {
            None
        };

        let config_files = if lang.config_files.is_some() {
            Some(lang.config_files.clone().unwrap())
        } else {
            None
        };

        #[cfg(unix)]
        let execute = lang.execute.unix.clone();
        #[cfg(windows)]
        let execute = lang.execute.windows.clone();

        #[cfg(unix)]
        let compile = if lang.compile.is_some() {
            Some(lang.compile.clone().unwrap().unix)
        } else {
            None
        };
        #[cfg(windows)]
        let compile = if lang.compile.is_some() {
            Some(lang.compile.clone().unwrap().windows)
        } else {
            None
        };
        LanguageHandler {
            initialize,
            config_files,
            compile,
            execute,
            check_installed: lang.check_installed.clone(),
            name: lang.name.clone(),
            stdin: None,
            stdout,
            stderr: None,
        }
    }
}

impl Language for LanguageHandler {
    fn init(&self) -> bool {
        if self.initialize.is_none() {
            return true;
        }

        show_installing_dependencies(&self.name[..]);

        // execute initialization command
        let initialize = self.initialize.clone().unwrap();
        let commands = initialize.split("&&").collect::<Vec<_>>();
        for &cmd in commands.iter() {
            let commands_str = cmd.split(' ').collect::<Vec<_>>();

            // remove whitespace from commands and expand ~ characters by $HOME
            let commands_str = commands_str
                .iter()
                .map(|val| shellexpand::tilde(&val.trim()).to_string())
                .filter(|val| !val.trim().is_empty())
                .collect::<Vec<_>>();

            // convert from Vec<String> to Vec<&str>
            let commands_str: Vec<&str> = commands_str.iter().map(|s| s as &str).collect();

            if configuration_commands(&commands_str) {
                continue;
            }

            // add main command
            let mut process_cmd = Command::new(&commands_str[0]);

            // Disable stdout and stderr
            process_cmd.stderr(Stdio::piped());
            process_cmd.stdout(Stdio::piped());

            // add the arguments
            if commands_str.len() > 1 {
                process_cmd.args(&commands_str[1..]);
            }
            let status = process_cmd.status().expect("Initialization Command");
            // check status
            if status.code() != Some(0) {
                return false;
            }
        }

        if self.config_files.is_some() {
            // create the configuration files
            let iter = self.config_files.clone().unwrap();
            for file in iter.iter() {
                file.create();
            }
        }

        true
    }

    fn build(&self) -> bool {
        if self.compile.is_none() {
            return true;
        }

        let compile = self.compile.clone().unwrap();
        let commands = compile.split("&&").collect::<Vec<_>>();

        for &cmd in commands.iter() {
            let commands_str = cmd.trim().split(' ').collect::<Vec<_>>();

            // remove whitespace from commands and expand ~ characters by $HOME
            let commands_str = commands_str
                .iter()
                .map(|val| shellexpand::tilde(&val.trim()).to_string())
                .filter(|val| !val.trim().is_empty())
                .collect::<Vec<_>>();

            // convert from Vec<String> to Vec<&str>
            let commands_str: Vec<&str> = commands_str.iter().map(|s| s as &str).collect();

            if configuration_commands(&commands_str) {
                continue;
            }

            // add main command
            let mut process_cmd = Command::new(&commands_str[0]);

            // add the arguments
            if commands_str.len() > 1 {
                process_cmd.args(&commands_str[1..]);
            }

            let status = process_cmd.status().expect("Compiling Command");

            // check status
            if status.code() != Some(0) {
                return false;
            }
        }

        true
    }

    fn execute(&self, timeout: u32, memory_limit: u64, testcase: u32) -> StatusResponse {
        let execute = self.execute.clone();
        let commands_str = execute.split(' ').collect::<Vec<_>>();

        execute_program(
            timeout,
            memory_limit,
            testcase,
            commands_str,
            self.stdin.clone(),
            self.stdout.clone(),
            self.stderr.clone(),
        )
    }

    fn set_stdio(&mut self, stdin: &str) {
        self.stdin = Some(PathBuf::from(stdin));
    }

    fn is_installed(&self) -> bool {
        let check_installed = self.check_installed.clone();
        let commands_str = check_installed.split(' ').collect::<Vec<_>>();

        has_installed_controller(
            commands_str[0],
            if commands_str.len() > 1 {
                commands_str[1..].to_vec()
            } else {
                Vec::new()
            },
        )
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub fn get_language_handler(
    file_name: &str,
    file_label: &str,
    input_file: &str,
    output_file: &str,
    error_file: &str,
) -> Result<Box<LanguageHandler>, ExitFailure> {
    let mut lang_out = LanguageScheme::new();

    let path = Path::new(&file_name);

    let ext: &str = get_extension(path).unwrap();

    extension::map_extension(file_label, ext, &mut lang_out)?;

    let mut file_name = file_name.split('.').collect::<Vec<_>>();
    file_name.pop();
    let file_name = file_name.join(".");

    lang_out.add_env_variable(&String::from("FILE_NAME"), &file_name);
    let mut file_name = file_name.split('/').collect::<Vec<_>>();
    file_name.reverse();
    while file_name.len() > 1 {
        file_name.pop();
    }
    let file_name = file_name.join(".");
    lang_out.add_env_variable(&String::from("FILE_NAME_BINARY"), &file_name);

    let handler =
        LanguageHandler::get_language(lang_out.clone(), input_file, output_file, error_file);

    Ok(Box::new(handler))
}

pub fn get_generator_handler(
    file_name: &str,
    file_label: &str,
    output_file: &str,
) -> Result<Box<LanguageHandler>, ExitFailure> {
    let mut lang_out = LanguageScheme::new();

    let path = Path::new(&file_name);

    let ext: &str = get_extension(path).unwrap();

    extension::map_extension(file_label, ext, &mut lang_out)?;

    let mut file_name = file_name.split('.').collect::<Vec<_>>();
    file_name.pop();
    let file_name = file_name.join(".");

    lang_out.add_env_variable(&String::from("FILE_NAME"), &file_name);
    let mut file_name = file_name.split('/').collect::<Vec<_>>();
    file_name.reverse();
    while file_name.len() > 1 {
        file_name.pop();
    }
    let file_name = file_name.join(".");
    lang_out.add_env_variable(&String::from("FILE_NAME_BINARY"), &file_name);

    let handler = LanguageHandler::get_generator(lang_out.clone(), output_file);

    Ok(Box::new(handler))
}
