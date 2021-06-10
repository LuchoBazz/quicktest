/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::time::Duration;

pub trait Language {
    fn build(&self);
    fn execute(&self, timeout: u32) -> Duration;
}
