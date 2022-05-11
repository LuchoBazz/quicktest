/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use crate::cli::opt::Opt;
use cli::structures::{
    CheckCommand, CmpCommand, ExampleCommand, RunCommand, SetupCommand, StressCommand,
};

pub mod cli;
pub mod config;
pub mod constants;
pub mod diff;
pub mod error;
pub mod file_handler;
pub mod generator;
pub mod language;
pub mod runner;
pub mod subcommand;
pub mod util;
pub mod views;

use exitfailure::ExitFailure;
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    match opt {
        Opt::Stress {
            target_file,
            gen_file,
            test_cases,
            timeout,
            memory_limit,
            break_bad,
            save_bad,
            save_all,
            run_all,
            run_ac,
            run_wa,
            run_tle,
            run_rte,
            run_mle,
        } => subcommand::cmd_stress::run(&StressCommand::new(
            target_file,
            gen_file,
            test_cases,
            timeout,
            memory_limit,
            break_bad,
            save_bad,
            save_all,
            run_all,
            run_ac,
            run_wa,
            run_tle,
            run_rte,
            run_mle,
        )),
        Opt::Cmp {
            target_file,
            correct_file,
            gen_file,
            timeout,
            memory_limit,
            test_cases,
            break_bad,
            save_bad,
            save_all,
            run_all,
            run_ac,
            run_wa,
            run_tle,
            run_rte,
            run_mle,
            diff,
        } => subcommand::cmd_cmp::run(&CmpCommand::new(
            target_file,
            correct_file,
            gen_file,
            timeout,
            memory_limit,
            test_cases,
            break_bad,
            save_bad,
            save_all,
            run_all,
            run_ac,
            run_wa,
            run_tle,
            run_rte,
            run_mle,
            diff,
        )),
        Opt::Check {
            target_file,
            checker_file,
            gen_file,
            test_cases,
            timeout,
            memory_limit,
            break_bad,
            save_bad,
            save_all,
            run_all,
            run_ac,
            run_wa,
            run_tle,
            run_rte,
            run_mle,
        } => subcommand::cmd_check::run(&CheckCommand::new(
            target_file,
            checker_file,
            gen_file,
            timeout,
            memory_limit,
            test_cases,
            break_bad,
            save_bad,
            save_all,
            run_all,
            run_ac,
            run_wa,
            run_tle,
            run_rte,
            run_mle,
        )),
        Opt::Run {
            target_file,
            prefix,
            timeout,
            memory_limit,
            break_bad,
            save_out,
        } => subcommand::cmd_run::run(&RunCommand::new(
            target_file,
            prefix,
            timeout,
            memory_limit,
            break_bad,
            save_out,
        )),
        Opt::Setup { subcommand } => match subcommand {
            cli::opt::SetUp::Config { label, value } => {
                subcommand::cmd_setup::run(&SetupCommand::new(label, value))
            }
        },
        Opt::Example {
            cmp,
            stress,
            check,
            run,
            setup,
        } => subcommand::cmd_example::run(&ExampleCommand::new(cmp, stress, check, run, setup)),
    }
}
