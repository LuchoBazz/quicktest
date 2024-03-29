/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{
    io::Write,
    path::{Path, PathBuf},
};

use crate::util::test_constants::FOLDER;

pub fn create_files_tle(
    target: &str,
    gen: &str,
    target_code: &str,
    gen_code: &str,
    new_folder: &str,
) -> Result<(), std::io::Error> {
    if !Path::new(FOLDER).exists() {
        std::fs::create_dir(FOLDER).expect(&format!("Unable to create {}", FOLDER));
    }

    let folder_path = &format!("{}/{}", FOLDER, new_folder)[..];

    if !Path::new(folder_path).exists() {
        std::fs::create_dir(folder_path).expect(&format!("Unable to create {}", folder_path));
    }

    let target_file = format!("{}/{}", folder_path, target);
    let gen_file = format!("{}/{}", folder_path, gen);

    let mut target_file = std::fs::File::create(PathBuf::from(target_file))?;
    target_file.write_all(target_code.as_bytes())?;

    let mut gen_file = std::fs::File::create(PathBuf::from(gen_file))?;
    gen_file.write_all(gen_code.as_bytes())?;
    Ok(())
}

pub fn create_files_cmp(
    target: &str,
    correct: &str,
    gen: &str,
    target_code: &str,
    correct_code: &str,
    gen_code: &str,
    new_folder: &str,
) -> Result<(), std::io::Error> {
    if !Path::new(FOLDER).exists() {
        std::fs::create_dir(FOLDER).expect(&format!("Unable to create {}", FOLDER));
    }

    let folder_path = &format!("{}/{}", FOLDER, new_folder)[..];

    if !Path::new(folder_path).exists() {
        std::fs::create_dir(folder_path).expect(&format!("Unable to create {}", folder_path));
    }

    let target_file = format!("{}/{}", folder_path, target);
    let correct_file = format!("{}/{}", folder_path, correct);
    let gen_file = format!("{}/{}", folder_path, gen);

    let mut target_file = std::fs::File::create(PathBuf::from(target_file))?;
    target_file.write_all(target_code.as_bytes())?;

    let mut correct_file = std::fs::File::create(PathBuf::from(correct_file))?;
    correct_file.write_all(correct_code.as_bytes())?;

    let mut gen_file = std::fs::File::create(PathBuf::from(gen_file))?;
    gen_file.write_all(gen_code.as_bytes())?;
    Ok(())
}

pub fn create_files_check(
    target: &str,
    check: &str,
    gen: &str,
    target_code: &str,
    check_code: &str,
    gen_code: &str,
    new_folder: &str,
) -> Result<(), std::io::Error> {
    if !Path::new(FOLDER).exists() {
        std::fs::create_dir(FOLDER).expect(&format!("Unable to create {}", FOLDER));
    }

    let folder_path = &format!("{}/{}", FOLDER, new_folder)[..];

    if !Path::new(folder_path).exists() {
        std::fs::create_dir(folder_path).expect(&format!("Unable to create {}", folder_path));
    }

    let target_file = format!("{}/{}", folder_path, target);
    let check_file = format!("{}/{}", folder_path, check);
    let gen_file = format!("{}/{}", folder_path, gen);

    let mut target_file = std::fs::File::create(PathBuf::from(target_file))?;
    target_file.write_all(target_code.as_bytes())?;

    let mut check_file = std::fs::File::create(PathBuf::from(check_file))?;
    check_file.write_all(check_code.as_bytes())?;

    let mut gen_file = std::fs::File::create(PathBuf::from(gen_file))?;
    gen_file.write_all(gen_code.as_bytes())?;
    Ok(())
}

pub fn create_files_output_cmd(
    files: Vec<(&str, &str)>,
    new_folder: &str,
) -> Result<(), std::io::Error> {
    if !Path::new(FOLDER).exists() {
        std::fs::create_dir(FOLDER).expect(&format!("Unable to create {}", FOLDER));
    }

    for &item in files.iter() {
        let folder_path = &format!("{}/{}", FOLDER, new_folder)[..];

        if !Path::new(folder_path).exists() {
            std::fs::create_dir(folder_path).expect(&format!("Unable to create {}", folder_path));
        }

        let filename = format!("{}/{}", folder_path, item.0);
        let mut file = std::fs::File::create(PathBuf::from(filename))?;
        file.write_all(item.1.as_bytes())?;
    }

    Ok(())
}
