/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021 - Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

pub struct SetupCommand {
    pub label: String,
    pub value: String,
}

impl SetupCommand {
    pub fn new(label: String, value: String) -> SetupCommand {
        SetupCommand { label, value }
    }
}
