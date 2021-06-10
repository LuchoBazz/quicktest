/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::io;
use std::fs;
use std::path::PathBuf;

pub fn file_exists(file_name: &str) -> Result<bool, io::Error> {
    fs::File::open(file_name)?;
    Ok(true)
}

pub fn get_extension(path: &PathBuf) -> Option<&str> {
    let ext = path.extension()?.to_str()?;
    Some(ext)
}