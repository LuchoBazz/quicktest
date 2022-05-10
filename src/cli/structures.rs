use std::path::PathBuf;

/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

pub struct StressCommand {
    pub target_file: PathBuf,
    pub gen_file: PathBuf,
    pub timeout: u32,
    pub test_cases: u32,
    pub break_bad: bool,
    pub save_bad: bool,
    pub save_all: bool,
    pub run_all: bool,
    pub run_ac: bool,
    pub run_wa: bool,
    pub run_tle: bool,
    pub run_rte: bool,
    pub run_mle: bool,
}

#[allow(clippy::too_many_arguments)]
impl StressCommand {
    pub fn new(
        target_file: PathBuf,
        gen_file: PathBuf,
        test_cases: u32,
        timeout: u32,
        break_bad: bool,
        save_bad: bool,
        save_all: bool,
        run_all: bool,
        run_ac: bool,
        run_wa: bool,
        run_tle: bool,
        run_rte: bool,
        run_mle: bool,
    ) -> StressCommand {
        StressCommand {
            target_file,
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
            run_mle,
        }
    }
}

pub struct CmpCommand {
    pub target_file: PathBuf,
    pub correct_file: PathBuf,
    pub gen_file: PathBuf,
    pub timeout: u32,
    pub test_cases: u32,
    pub break_bad: bool,
    pub save_bad: bool,
    pub save_all: bool,
    pub run_all: bool,
    pub run_ac: bool,
    pub run_wa: bool,
    pub run_tle: bool,
    pub run_rte: bool,
    pub run_mle: bool,
    pub diff: bool,
}

#[allow(clippy::too_many_arguments)]
impl CmpCommand {
    pub fn new(
        target_file: PathBuf,
        correct_file: PathBuf,
        gen_file: PathBuf,
        timeout: u32,
        test_cases: u32,
        break_bad: bool,
        save_bad: bool,
        save_all: bool,
        run_all: bool,
        run_ac: bool,
        run_wa: bool,
        run_tle: bool,
        run_rte: bool,
        run_mle: bool,
        diff: bool,
    ) -> CmpCommand {
        CmpCommand {
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
            run_mle,
            diff,
        }
    }
}

pub struct CheckCommand {
    pub target_file: PathBuf,
    pub checker_file: PathBuf,
    pub gen_file: PathBuf,
    pub timeout: u32,
    pub test_cases: u32,
    pub break_bad: bool,
    pub save_bad: bool,
    pub save_all: bool,
    pub run_all: bool,
    pub run_ac: bool,
    pub run_wa: bool,
    pub run_tle: bool,
    pub run_rte: bool,
}

#[allow(clippy::too_many_arguments)]
impl CheckCommand {
    pub fn new(
        target_file: PathBuf,
        checker_file: PathBuf,
        gen_file: PathBuf,
        timeout: u32,
        test_cases: u32,
        break_bad: bool,
        save_bad: bool,
        save_all: bool,
        run_all: bool,
        run_ac: bool,
        run_wa: bool,
        run_tle: bool,
        run_rte: bool,
    ) -> CheckCommand {
        CheckCommand {
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
        }
    }
}

pub struct RunCommand {
    pub target_file: PathBuf,
    pub prefix: String,
    pub timeout: u32,
    pub break_bad: bool,
    pub save_out: bool,
}

impl RunCommand {
    pub fn new(
        target_file: PathBuf,
        prefix: String,
        timeout: u32,
        break_bad: bool,
        save_out: bool,
    ) -> RunCommand {
        RunCommand {
            target_file,
            prefix,
            timeout,
            break_bad,
            save_out,
        }
    }
}

pub struct ExampleCommand {
    pub stress: bool,
    pub cmp: bool,
    pub check: bool,
    pub run: bool,
    pub setup: bool,
}

impl ExampleCommand {
    pub fn new(cmp: bool, stress: bool, check: bool, run: bool, setup: bool) -> ExampleCommand {
        ExampleCommand {
            stress,
            cmp,
            check,
            run,
            setup,
        }
    }
}

pub struct SetupCommand {
    pub label: String,
    pub value: String,
}

impl SetupCommand {
    pub fn new(label: String, value: String) -> SetupCommand {
        SetupCommand { label, value }
    }
}
