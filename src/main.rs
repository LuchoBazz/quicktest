/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use structopt::StructOpt;

pub mod cli;
pub mod constants;
pub mod error;
pub mod file_handler;
pub mod generator;
pub mod painter;
pub mod runner;
pub mod subcommand;
pub mod util;

use crate::cli::Opt;

use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    let response = match opt {
        Opt::TLE {
            target_file,
            gen_file,
            test_cases,
            timeout,
            break_bad,
            save_bad,
            save_all,
            run_all,
            run_ac,
            run_wa,
            run_tle,
            run_rte,
        } => subcommand::cmd_tle::run(
            target_file,
            gen_file,
            test_cases,
            timeout,
            break_bad,
            save_bad,
            save_all,
            run_all,
            run_ac,
            run_wa,
            run_tle,
            run_rte,
        ),
        Opt::Cmp {
            target_file,
            correct_file,
            gen_file,
            timeout,
            test_cases,
            break_bad,
            save_bad,
            save_all,
            run_all,
            run_ac,
            run_wa,
            run_tle,
            run_rte,
        } => subcommand::cmd_cmp::run(
            target_file,
            correct_file,
            gen_file,
            timeout,
            test_cases,
            break_bad,
            save_bad,
            save_all,
            run_all,
            run_ac,
            run_wa,
            run_tle,
            run_rte,
        ),
        Opt::Check {
            target_file,
            checker_file,
            gen_file,
            test_cases,
            timeout,
            break_bad,
            save_bad,
            save_all,
            run_all,
            run_ac,
            run_wa,
            run_tle,
            run_rte,
        } => subcommand::cmd_check::run(
            target_file,
            checker_file,
            gen_file,
            timeout,
            test_cases,
            break_bad,
            save_bad,
            save_all,
            run_all,
            run_ac,
            run_wa,
            run_tle,
            run_rte,
        ),
        Opt::Example { cmp, tle, check } => subcommand::cmd_example::run(cmp, tle, check),
    };

    response
}
