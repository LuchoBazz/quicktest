/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021 - Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::Duration;

use crate::cli::model::traits::AdapterCommand;
use crate::constants::QTEST_INPUT_FILE;
use crate::error::handle_error::{
    throw_compiler_error_msg, throw_memory_limit_exceeded_msg, throw_runtime_error_msg,
    throw_time_limit_exceeded_msg,
};
use crate::file_handler::file::copy_file;
use crate::runner::types::{
    is_compiled_error, is_memory_limit_exceeded, is_runtime_error, is_time_limit_exceeded, Language,
};
use crate::views::style::show_time_limit_exceeded_generator;
use exitfailure::ExitFailure;

pub fn execute_generator(
    generator_file_lang: &dyn Language,
    command: &dyn AdapterCommand,
    cases: &mut VecDeque<PathBuf>,
    test_number: u32,
    can_continue: &mut bool,
) -> Result<(), ExitFailure> {
    if command.get_prefix().is_empty() {
        let response_gen = generator_file_lang.execute(
            command.get_timeout() as u32,
            command.get_memory_limit(),
            test_number,
        );
        let time_gen = response_gen.time;

        if is_runtime_error(&response_gen.status) {
            return throw_runtime_error_msg("generator", "<gen-file>");
        } else if is_compiled_error(&response_gen.status) {
            return throw_compiler_error_msg("generator", "<gen-file>");
        } else if is_memory_limit_exceeded(&response_gen.status) {
            return throw_memory_limit_exceeded_msg("generator", "<gen-file>");
        }

        if time_gen >= Duration::from_millis(command.get_timeout() as u64)
            || is_time_limit_exceeded(&response_gen.status)
        {
            // TLE Generator
            show_time_limit_exceeded_generator(test_number, command.get_timeout());
            return throw_time_limit_exceeded_msg("generator", "<gen-file>");
        }
        *can_continue = true;
        return Ok(());
    }

    if !cases.is_empty() {
        let case = cases.pop_front().unwrap();
        copy_file(case.to_str().unwrap(), QTEST_INPUT_FILE)?;
        *can_continue = true;
    } else {
        *can_continue = false;
    }

    Ok(())
}
