#![allow(warnings, unused)]

use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use exitfailure::ExitFailure;
use std::process::Command;

use crate::{
    config::load_config::get_root_path,
    language::extension,
    runner::{
        cmd::{execute_program, has_installed_controller},
        types::{CPStatus, Language, StatusResponse},
    },
    util::file::get_extension,
};

use super::{json::language_scheme::LanguageScheme, traits::BuildEnvVariables};

#[derive(Debug, Clone)]
pub struct LanguageHandler {
    check_installed: String,

    compile: Option<String>,

    execute: String,

    name: String,

    stderr: Option<PathBuf>,

    stdin: Option<PathBuf>,

    stdout: Option<PathBuf>,
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

        let mut lang = lang.clone();

        let mut env = lang.env.clone();

        lang.build_env_variables(&env);

        #[cfg(unix)]
        let execute = lang.execute.unix.clone();
        #[cfg(windows)]
        let execute = lang.execute.windows.clone();

        #[cfg(unix)]
        let mut compile = if lang.compile.is_some() {
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
            compile: compile,
            execute: execute,
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

        let mut lang = lang.clone();

        let mut env = lang.env.clone();

        lang.build_env_variables(&env);

        #[cfg(unix)]
        let execute = lang.execute.unix.clone();
        #[cfg(windows)]
        let execute = lang.execute.windows.clone();

        #[cfg(unix)]
        let mut compile = if lang.compile.is_some() {
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
            compile: compile,
            execute: execute,
            check_installed: lang.check_installed.clone(),
            name: lang.name.clone(),
            stdin: None,
            stdout,
            stderr: None,
        }
    }
}

impl Language for LanguageHandler {
    fn build(&self) -> bool {
        // println!("Compile {:#?}", self.compile);

        if self.compile.is_none() {
            return true;
        }

        let compile = self.compile.clone().unwrap();
        let commands_str = compile.split(" ").collect::<Vec<_>>();

        let mut cmd = Command::new(&commands_str[0]);

        if commands_str.len() > 1 {
            cmd.args(&commands_str[1..]);
        }

        let status = cmd.status().expect("Compiling C++ error");

        status.code() == Some(0)
    }

    fn execute(&self, timeout: u32, testcase: u32) -> StatusResponse {
        // println!("Execute {}", self.execute);
        let execute = self.execute.clone();
        let commands_str = execute.split(" ").collect::<Vec<_>>();

        execute_program(
            timeout,
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
        // println!("Is Installed {}", self.check_installed);

        let check_installed = self.check_installed.clone();
        let commands_str = check_installed.split(" ").collect::<Vec<_>>();

        has_installed_controller(
            &commands_str[0],
            if commands_str.len() > 1 {
                (&commands_str[1..]).to_vec()
            } else {
                Vec::new()
            },
        )
    }

    fn get_name(&self) -> String {
        // println!("Name {}", self.name);
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

    let ext: &str = get_extension(&path).unwrap();

    if let Err(err) = extension::map_extension(file_label, ext, &mut lang_out) {
        return Err(err);
    }

    let name = path.file_stem().unwrap().to_str().unwrap();

    let mut file_name = file_name.split(".").collect::<Vec<_>>();
    file_name.pop();
    let file_name = file_name.join(".");

    lang_out.add_env_variable(&String::from("FILE_NAME"), &file_name);
    let mut file_name = file_name.split("/").collect::<Vec<_>>();
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
