/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

pub const CACHE_FOLDER: &str = ".qtest/";

#[cfg(unix)]
pub const TARGET_BINARY_FILE: &str= ".qtest/main.o";
#[cfg(windows)]
pub const TARGET_BINARY_FILE: &str= ".qtest/main.exe";

#[cfg(unix)]
pub const CORRECT_BINARY_FILE: &str= ".qtest/correct.o";
#[cfg(windows)]
pub const CORRECT_BINARY_FILE: &str= ".qtest/correct.exe";

#[cfg(unix)]
pub const GEN_BINARY_FILE: &str= ".qtest/gen.o";
#[cfg(windows)]
pub const GEN_BINARY_FILE: &str= ".qtest/gen.exe";

pub const QTEST_INPUT_FILE: &str= ".qtest/quicktest_input.txt";
pub const QTEST_OUTPUT_FILE: &str= ".qtest/quicktest_output.txt";
pub const QTEST_ERROR_FILE: &str= ".qtest/quicktest_error.txt";

pub const QTEST_EXPECTED_FILE: &str= ".qtest/expected_testcase.txt";