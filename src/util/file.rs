/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::io;
use std::fs;

pub fn file_exists(file_name: &str) -> Result<bool, io::Error> {
    fs::File::open(file_name)?;
    Ok(true)
}