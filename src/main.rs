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

use crate::cli::Opt;

use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    let response = match opt {
        Opt::TLE { target_file, gen_file, test_cases,
            timeout, tle_break, save_cases} => {
            checker::check_tle::run(
                target_file,
                gen_file,
                test_cases,
                timeout,
                tle_break,
                save_cases
            )
        },
        Opt::Cmp { target_file, correct_file, gen_file,
            timeout, test_cases, wa_break, save_cases} => {
            checker::check_correctness::run(
                target_file,
                correct_file,
                gen_file,
                timeout,
                test_cases,
                wa_break,
                save_cases
            )
        },
        Opt::Check{target_file, checker_file, gen_file,
            test_cases, timeout, wa_break, save_cases} => {
            checker::cmd_checker::run(
                target_file,
                checker_file,
                gen_file,
                timeout,
                test_cases,
                wa_break,
                save_cases
            )
        },
    };

    response
}