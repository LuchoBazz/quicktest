/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021 - Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use std::time::Duration;

use exitfailure::ExitFailure;
use crate::painter::style::show_time_limit_exceeded_generator;
use crate::error::handle_error::{
    throw_compiler_error_msg, throw_runtime_error_msg, throw_time_limit_exceeded_msg
};
use crate::runner::types::{
    Language, is_compiled_error, is_runtime_error, is_time_limit_exceeded
};

pub fn execute_generator(generator_file_lang: &dyn Language, timeout: u32, test_number: u32) -> Result<(), ExitFailure>{
    let response_gen = generator_file_lang.execute(timeout as u32);
    let time_gen = response_gen.time;

    if is_runtime_error(&response_gen.status) {
        return throw_runtime_error_msg("generator", "<gen-file>");
    } else if is_compiled_error(&response_gen.status) {
        return throw_compiler_error_msg("generator", "<gen-file>");
    }

    if time_gen >= Duration::from_millis(timeout as u64) || is_time_limit_exceeded(&response_gen.status) {
        // TLE Generator
        show_time_limit_exceeded_generator(test_number, timeout);
        return throw_time_limit_exceeded_msg("generator", "<gen-file>");
    }

    Ok(())
}