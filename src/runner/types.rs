/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::time::Duration;

#[derive(PartialEq)]
pub enum CPStatus {
    AC,
    WA,
    TLE,
    CE,
    RTE
}
pub trait Language {
    fn build(&self) -> bool;
    fn execute(&self, timeout: u32) -> StatusResponse;
    fn set_stdio(&mut self, stdin: &str);
}


pub struct StatusResponse {
    pub time: Duration,
    pub status: CPStatus
}

impl StatusResponse {
    pub fn new(time: Duration, status: CPStatus) -> StatusResponse {
        StatusResponse {
            time,
            status
        }
    }
}