/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use crate::cli::opt::Opt;
use cli::model::{
    check_command::CheckCommand, cmp_command::CmpCommand, example_command::ExampleCommand,
    output_command::OutputCommand, setup_command::SetupCommand, stress_command::StressCommand,
};

pub mod cli;
pub mod config;
pub mod constants;
pub mod controllers;
pub mod diff;
pub mod error;
pub mod file_handler;
pub mod generator;
pub mod language;
pub mod runner;
pub mod util;
pub mod views;

use exitfailure::ExitFailure;
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    let status = match opt {
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
        } => controllers::cmd_stress::run(&StressCommand::new(
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
        } => controllers::cmd_cmp::run(&CmpCommand::new(
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
        } => controllers::cmd_check::run(&CheckCommand::new(
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
        Opt::Output {
            target_file,
            prefix,
            timeout,
            memory_limit,
            break_bad,
            save_out,
        } => controllers::cmd_output::run(&OutputCommand::new(
            target_file,
            prefix,
            timeout,
            memory_limit,
            break_bad,
            save_out,
        )),
        Opt::Setup { subcommand } => match subcommand {
            cli::opt::SetUp::Config { label, value } => {
                controllers::cmd_setup::run(&SetupCommand::new(label, value))
            }
        },
        Opt::Example {
            cmp,
            stress,
            check,
            output,
            setup,
        } => controllers::cmd_example::run(&ExampleCommand::new(cmp, stress, check, output, setup)),
    };

    if let Err(err) = status {
        return Err(err);
    }
    std::process::exit(exitcode::OK);
}
