/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021 - Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use exitfailure::ExitFailure;

use crate::painter::example::{
    show_examples_check_cmd, show_examples_cmp_cmd, show_examples_tle_cmd,
};

pub fn run(cmp: bool, tle: bool, check: bool) -> Result<(), ExitFailure> {
    if cmp {
        show_examples_cmp_cmd();
    } else if tle {
        show_examples_tle_cmd();
    } else if check {
        show_examples_check_cmd();
    }
    Ok(())
}
