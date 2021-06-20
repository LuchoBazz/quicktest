/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use structopt::StructOpt;

pub mod cli;
pub mod checker;
pub mod runner;
pub mod util;
pub mod constants;
pub mod painter;
pub mod error;
pub mod file_handler;

use crate::cli::Opt;

use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    let response = match opt {
        Opt::TLE { target_file, gen_file, test_cases,
            timeout, tle_break, save_bad, save_all} => {
            checker::check_tle::run(
                target_file,
                gen_file,
                test_cases,
                timeout,
                tle_break,
                save_bad,
                save_all
            )
        },
        Opt::Cmp { target_file, correct_file, gen_file,
            timeout, test_cases, break_bad, save_bad, save_all} => {
            checker::check_correctness::run(
                target_file,
                correct_file,
                gen_file,
                timeout,
                test_cases,
                break_bad,
                save_bad,
                save_all
            )
        },
        Opt::Check{target_file, checker_file, gen_file,
            test_cases, timeout, break_bad, save_bad, save_all} => {
            checker::cmd_checker::run(
                target_file,
                checker_file,
                gen_file,
                timeout,
                test_cases,
                break_bad,
                save_bad,
                save_all
            )
        },
        Opt::Run{target_file, timeout, all, wa, tle, rte} => {
    
            checker::run_cases::run(target_file, timeout, all, wa, tle, rte)
        },
    };

    response
}