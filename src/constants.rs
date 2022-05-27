/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

pub const CACHE_FOLDER: &str = ".qt/";

pub const TEST_CASES_FOLDER: &str = "test_cases";

pub const OUTPUT_FOLDER: &str = "output";

pub const CONFIG_FOLDER: &str = "~/.quicktest";

pub const LANGUAGE_CONFIG_FILE: &str = "~/.quicktest/languages.config.json";

#[cfg(unix)]
pub const TARGET_BINARY_FILE: &str = ".qt/main";
#[cfg(windows)]
pub const TARGET_BINARY_FILE: &str = ".qt/main.exe";

#[cfg(unix)]
pub const CORRECT_BINARY_FILE: &str = ".qt/correct";
#[cfg(windows)]
pub const CORRECT_BINARY_FILE: &str = ".qt/correct.exe";

#[cfg(unix)]
pub const CHECKER_BINARY_FILE: &str = ".qt/checker";
#[cfg(windows)]
pub const CHECKER_BINARY_FILE: &str = ".qt/checker.exe";

#[cfg(unix)]
pub const GEN_BINARY_FILE: &str = ".qt/gen";
#[cfg(windows)]
pub const GEN_BINARY_FILE: &str = ".qt/gen.exe";

pub const QTEST_INPUT_FILE: &str = ".qt/input.txt";
pub const QTEST_OUTPUT_FILE: &str = ".qt/output.txt";
pub const QTEST_ERROR_FILE: &str = ".qt/error.txt";

pub const QTEST_EXPECTED_FILE: &str = ".qt/expected.txt";
pub const QTEST_CHECKER_FILE: &str = ".qt/checker.txt";

pub const PREFIX_AC_FILES: &str = "testcase_ac";
pub const PREFIX_TLE_FILES: &str = "testcase_tle";
pub const PREFIX_WA_FILES: &str = "testcase_wa";
pub const PREFIX_RTE_FILES: &str = "testcase_rte";
pub const PREFIX_MLE_FILES: &str = "testcase_mle";

pub const IDENTATION: &str = "  ";

pub const GENERATOR_TIMEOUT: u32 = 5000;
