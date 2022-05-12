/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021 - Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

pub struct ExampleCommand {
    pub stress: bool,
    pub cmp: bool,
    pub check: bool,
    pub output: bool,
    pub setup: bool,
}

impl ExampleCommand {
    pub fn new(cmp: bool, stress: bool, check: bool, output: bool, setup: bool) -> ExampleCommand {
        ExampleCommand {
            stress,
            cmp,
            check,
            output,
            setup,
        }
    }
}
