/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

pub const CACHE_FOLDER: &str = ".qtest/";

pub const TEST_CASES_FOLDER: &str = "test_cases";

pub const OUTPUT_FOLDER: &str = "output";

pub const CONFIG_FOLDER: &str = "~/.quicktest";

pub const CONFIG_FILE: &str = "~/.quicktest/config.yaml";

#[cfg(unix)]
pub const TARGET_BINARY_FILE: &str = ".qtest/main.o";
#[cfg(windows)]
pub const TARGET_BINARY_FILE: &str = ".qtest/main.exe";

#[cfg(unix)]
pub const CORRECT_BINARY_FILE: &str = ".qtest/correct.o";
#[cfg(windows)]
pub const CORRECT_BINARY_FILE: &str = ".qtest/correct.exe";

#[cfg(unix)]
pub const CHECKER_BINARY_FILE: &str = ".qtest/checker.o";
#[cfg(windows)]
pub const CHECKER_BINARY_FILE: &str = ".qtest/checker.exe";

#[cfg(unix)]
pub const GEN_BINARY_FILE: &str = ".qtest/gen.o";
#[cfg(windows)]
pub const GEN_BINARY_FILE: &str = ".qtest/gen.exe";

pub const QTEST_INPUT_FILE: &str = ".qtest/quicktest_input.txt";
pub const QTEST_OUTPUT_FILE: &str = ".qtest/quicktest_output.txt";
pub const QTEST_ERROR_FILE: &str = ".qtest/quicktest_error.txt";

pub const QTEST_EXPECTED_FILE: &str = ".qtest/expected_testcase.txt";
pub const QTEST_CHECKER_FILE: &str = ".qtest/checker_testcase.txt";

pub const PREFIX_AC_FILES: &str = "testcase_ac";
pub const PREFIX_TLE_FILES: &str = "testcase_tle";
pub const PREFIX_WA_FILES: &str = "testcase_wa";
pub const PREFIX_RTE_FILES: &str = "testcase_rte";

pub const IDENTATION: &str = "  ";