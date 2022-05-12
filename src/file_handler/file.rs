/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{
    collections::VecDeque,
    fs::{self, remove_dir_all},
    io::Write,
    path::{Path, PathBuf},
};

use exitfailure::ExitFailure;
use glob::glob;

use crate::{
    constants::{
        PREFIX_AC_FILES, PREFIX_MLE_FILES, PREFIX_RTE_FILES, PREFIX_TLE_FILES, PREFIX_WA_FILES,
        TEST_CASES_FOLDER,
    },
    error::handle_error::{
        throw_couldnt_create_folder_msg, throw_couldnt_open_file_msg,
        throw_couldnt_write_to_file_msg, throw_extension_not_supported_msg,
        throw_program_not_installed_msg,
    },
    language::extension::is_extension_supported,
    runner::types::Language,
    util::file::file_exists,
};

pub fn remove_files(files: Vec<&str>) {
    for file in files {
        if file_exists(file).is_ok() && fs::remove_file(&file).is_ok() {}
    }
}

pub fn remove_files_with_prefix(prefix: &str) {
    if let Ok(paths) = glob(prefix) {
        for path in paths.flatten() {
            if fs::remove_file(path.to_str().unwrap()).is_ok() {}
        }
    }
}

pub fn remove_folder(path: &str) {
    if remove_dir_all(path).is_ok() {}
}

pub fn write_file(file_name: &str, bytes: &[u8]) -> Result<(), ExitFailure> {
    if let Ok(mut file) = std::fs::File::create(file_name) {
        if file.write_all(bytes).is_err() {
            return throw_couldnt_write_to_file_msg(file_name);
        }
    }
    Ok(())
}

pub fn read_file(file_name: &str) -> Option<String> {
    if let Ok(data) = fs::read_to_string(file_name) {
        return Some(data);
    }
    None
}

pub fn file_exists_or_error(file_name: &str, label: &str) -> Result<(), ExitFailure> {
    if fs::File::open(file_name).is_err() {
        return throw_couldnt_open_file_msg(file_name, label);
    }
    Ok(())
}

pub fn create_folder_or_error(name: &str) -> Result<(), ExitFailure> {
    if fs::read_dir(name).is_err() && fs::create_dir(name).is_err() {
        // If not, create the folder
        return throw_couldnt_create_folder_msg(name);
    }
    Ok(())
}

pub fn format_filename_test_case(folder: &str, prefix: &str, number: u32) -> String {
    if number > 0 {
        let file_name = format!("{}/{}_{}.txt", folder, prefix, number);
        return file_name;
    }
    let file_name = format!("{}/{}_0{}.txt", folder, prefix, number);
    file_name
}

pub fn save_test_case(file_name: &str, from_path: &str) {
    // create test_cases folder
    if create_folder_or_error(TEST_CASES_FOLDER).is_ok() {}
    // Example: test_cases/testcase_tle_1.txt
    //let file_name: &str = &format!( "{}/{}_{}.txt", TEST_CASES_FOLDER, PREFIX_TLE_FILES, tle_count)[..];
    if write_file(file_name, fs::read_to_string(from_path).unwrap().as_bytes()).is_ok() {}
}

pub fn save_test_case_output(file_name: &str, from_path: &str) {
    if write_file(file_name, fs::read_to_string(from_path).unwrap().as_bytes()).is_ok() {}
}

pub fn copy_file(from: &str, to: &str) -> Result<(), ExitFailure> {
    if let Some(data) = read_file(from) {
        write_file(to, data.as_bytes())?;
    }
    Ok(())
}

pub fn is_extension_supported_or_error(file_name: &str) -> Result<(), ExitFailure> {
    let path = Path::new(&file_name);
    let ext = match path.extension() {
        Some(p) => p.to_str().unwrap_or(""),
        _ => "",
    };
    if !is_extension_supported(ext) {
        return throw_extension_not_supported_msg(file_name, ext);
    }
    Ok(())
}

pub fn can_run_language_or_error(lang: &dyn Language) -> Result<(), ExitFailure> {
    if !(*lang).is_installed() {
        let name = (*lang).get_name();
        return throw_program_not_installed_msg(&name[..]);
    }
    Ok(())
}

pub fn load_testcases_from_prefix_with_folder(
    queue: &mut VecDeque<PathBuf>,
    folder: &str,
    prefix: &str,
) -> Result<(), ExitFailure> {
    let paths = glob(&format!("{}/{}*", folder, prefix))?;
    for path in paths.flatten() {
        queue.push_back(path);
    }
    Ok(())
}

pub fn load_testcases_from_prefix(
    queue: &mut VecDeque<PathBuf>,
    prefix: &str,
) -> Result<(), ExitFailure> {
    let paths = glob(&format!("{}*", prefix))?;
    for path in paths.flatten() {
        queue.push_back(path);
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn load_testcases_from_states(
    queue: &mut VecDeque<PathBuf>,
    folder: &str,
    run_all: bool,
    run_ac: bool,
    run_wa: bool,
    run_tle: bool,
    run_rte: bool,
    run_mle: bool,
) -> Result<(), ExitFailure> {
    let mut run_ac = run_ac;
    let mut run_wa = run_wa;
    let mut run_tle = run_tle;
    let mut run_rte = run_rte;
    let mut run_mle = run_mle;

    if run_all {
        run_ac = true;
        run_wa = true;
        run_tle = true;
        run_rte = true;
        run_mle = true;
    }

    if run_ac {
        load_testcases_from_prefix_with_folder(queue, folder, PREFIX_AC_FILES)?;
    }

    if run_wa {
        load_testcases_from_prefix_with_folder(queue, folder, PREFIX_WA_FILES)?;
    }

    if run_tle {
        load_testcases_from_prefix_with_folder(queue, folder, PREFIX_TLE_FILES)?;
    }

    if run_rte {
        load_testcases_from_prefix_with_folder(queue, folder, PREFIX_RTE_FILES)?;
    }

    if run_mle {
        load_testcases_from_prefix_with_folder(queue, folder, PREFIX_MLE_FILES)?;
    }

    Ok(())
}

pub fn get_filename_output(prefix: &str, filename: &str) -> String {
    // prefix:   examples/run/test_cases/testcase_ac
    // filename: testcase_ac_01.txt
    // output: examples/run/test_cases/out_testcase_ac_01.txt

    let prefix_clone = prefix.to_string();
    let mut prefix_token = prefix_clone.split('/').collect::<Vec<&str>>();

    // Check what filename is empty
    // if prefix_token.is_empty() {
    //     return throw_filename_cannot_be_empty();
    // }

    let _ = prefix_token.pop();

    let mut path = String::new();

    for (i, &item) in prefix_token.iter().enumerate() {
        if i > 0 {
            path.push('/');
        }
        path.push_str(item);
    }

    format!("{}/out_{}", path, filename)
}
