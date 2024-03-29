/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::time::Duration;

pub trait Language {
    fn init(&self) -> bool;
    fn build(&self) -> bool;
    fn execute(&self, timeout: u32, memory_limit: u64, testcase: u32) -> StatusResponse;
    fn set_stdio(&mut self, stdin: &str);
    fn is_installed(&self) -> bool;
    fn get_name(&self) -> String;
}

pub struct StatusResponse {
    pub time: Duration,
    pub status: CPStatus,
}

impl StatusResponse {
    pub fn new(time: Duration, status: CPStatus) -> StatusResponse {
        StatusResponse { time, status }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq, Eq)]
pub enum CPStatus {
    AC,
    WA,
    TLE,
    CE,
    RTE,
    MLE,
}

pub fn is_accepted(status: &CPStatus) -> bool {
    *status == CPStatus::AC
}

pub fn is_wrong_answer(status: &CPStatus) -> bool {
    *status == CPStatus::WA
}

pub fn is_time_limit_exceeded(status: &CPStatus) -> bool {
    *status == CPStatus::TLE
}

pub fn is_compiled_error(status: &CPStatus) -> bool {
    *status == CPStatus::CE
}

pub fn is_runtime_error(status: &CPStatus) -> bool {
    *status == CPStatus::RTE
}

pub fn is_memory_limit_exceeded(status: &CPStatus) -> bool {
    *status == CPStatus::MLE
}
