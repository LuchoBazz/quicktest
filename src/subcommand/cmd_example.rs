/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021-present / Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use exitfailure::ExitFailure;

use crate::{
    cli::structures::ExampleCommand,
    views::example::{show_examples_check_cmd, show_examples_cmp_cmd, show_examples_stress_cmd},
};

pub fn run(command: &ExampleCommand) -> Result<(), ExitFailure> {
    if command.cmp {
        show_examples_cmp_cmd();
    } else if command.stress {
        show_examples_stress_cmd();
    } else if command.check {
        show_examples_check_cmd();
    }
    Ok(())
}
