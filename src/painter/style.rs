/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use colored::*;

pub fn show_accepted(test_number: u32, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        test_number.to_string().bold().white(),
        "OK".bold().green(),
        "Finished in".bold().green(), time
    );
}

pub fn show_accepted_case(case: &str, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        case.to_string().bold().white(),
        "OK".bold().green(),
        "Finished in".bold().green(), time
    );
}

pub fn show_wrong_answer(test_number: u32, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        test_number.to_string().bold().white(),
        "WA".bold().red(),
        "Finished in".bold().red(), time
    );
}

pub fn show_time_limit_exceeded(test_number: u32, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        test_number.to_string().bold().white(),
        "TLE".bold().red(),
        "Time Limit Exceeded :".bold().red(),
        time
    );
}

pub fn show_time_limit_exceeded_case(case: &str, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        case.to_string().bold().white(),
        "TLE".bold().red(),
        "Time Limit Exceeded :".bold().red(),
        time
    );
}
// TODO: modularize these functions
pub fn show_time_limit_exceeded_generator(test_number: u32, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        test_number.to_string().bold().white(),
        "TLE".bold().red(),
        "Generator Time Limit Exceeded :".bold().red(),
        time
    );
}

pub fn show_time_limit_exceeded_checker(test_number: u32, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        test_number.to_string().bold().white(),
        "TLE".bold().red(),
        "Checker Time Limit Exceeded :".bold().red(),
        time
    );
}

pub fn show_time_limit_exceeded_correct(test_number: u32, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        test_number.to_string().bold().white(),
        "TLE".bold().red(),
        "Correct File Time Limit Exceeded :".bold().red(),
        time
    );
}

pub fn show_runtime_error(test_number: u32, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        test_number.to_string().bold().white(),
        "RTE".bold().red(),
        "Runtime Error :".bold().red(),
        time
    );
}

pub fn show_stats(ac: u32, wa: u32, tle: u32, rte: u32) {

    println!("\n  📌{}\n", "-Stats-".bold().white());

    println!(
        "  {}={}, {}={}, {}={}, {}={}",
        "AC".bold().green(), ac.to_string().bold().white(),
        "WA".bold().red(), wa.to_string().bold().white(),
        "TLE".bold().red(), tle.to_string().bold().white(),
        "RTE".bold().red(), rte.to_string().bold().white()
    );
}