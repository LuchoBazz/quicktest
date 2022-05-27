/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{
    test_command_handler::execute_command_cmp_with_timeout,
    test_constants::{BINARY, CORRECT_FILE_GO, FOLDER_CMP, GEN_FILE_GO, TARGET_FILE_GO},
    test_utilities::create_files_cmp,
};

use super::codes::{CORRECT_GO_CMP, GEN_GO_CMP, TARGET_GO_CMP};

#[test]
fn cmp_target_go_correct_go_gen_go() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_GO,
        CORRECT_FILE_GO,
        GEN_FILE_GO,
        TARGET_GO_CMP,
        CORRECT_GO_CMP,
        GEN_GO_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp_with_timeout(
        &mut cmd,
        TARGET_FILE_GO,
        CORRECT_FILE_GO,
        GEN_FILE_GO,
        cases,
        2000_usize,
    );

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}
