/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use colored::*;
use pad::*;
use separator::Separatable;

pub fn show_accepted(test_number: u32, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        test_number.to_string().bold().white(),
        "OK".bold().green(),
        "Finished in".bold().green(),
        time
    );
}

pub fn show_accepted_case(case: &str, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        case.to_string().bold().white(),
        "OK".bold().green(),
        "Finished in".bold().green(),
        time
    );
}

pub fn show_wrong_answer(test_number: u32, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        test_number.to_string().bold().white(),
        "WA".bold().red(),
        "Finished in".bold().red(),
        time
    );
}

pub fn show_time_limit_exceeded(test_number: u32, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        test_number.to_string().bold().white(),
        "TLE".bold().red(),
        "Time Limit Exceeded".bold().red(),
        time
    );
}

pub fn show_time_limit_exceeded_case(case: &str, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        case.to_string().bold().white(),
        "TLE".bold().red(),
        "Time Limit Exceeded".bold().red(),
        time
    );
}
// TODO: modularize these functions
pub fn show_time_limit_exceeded_generator(test_number: u32, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        test_number.to_string().bold().white(),
        "TLE".bold().red(),
        "Generator Time Limit Exceeded".bold().red(),
        time
    );
}

pub fn show_time_limit_exceeded_checker(test_number: u32, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        test_number.to_string().bold().white(),
        "TLE".bold().red(),
        "Checker Time Limit Exceeded".bold().red(),
        time
    );
}

pub fn show_time_limit_exceeded_correct(test_number: u32, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        test_number.to_string().bold().white(),
        "TLE".bold().red(),
        "Correct File Time Limit Exceeded".bold().red(),
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

pub fn show_memory_limit_exceeded_error(test_number: u32, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        test_number.to_string().bold().white(),
        "MLE".bold().red(),
        "Memory Limit Exceeded :".bold().red(),
        time
    );
}

pub fn show_ran_successfully(test_number: u32, time: u32) {
    println!(
        "  {} [{}] {} {}ms",
        test_number.to_string().bold().white(),
        "OK".bold().green(),
        "Ran Successfully".bold().green(),
        time
    );
}

// Report example:
// ╭────────────────────────────────╮
// │           λ Report             │
// ├────────────────────────────────┤
// │          9/10          AC      │
// │          1/10          WA      │
// │          0/10          TLE     │
// │          0/10          RTE     │
// ├────────────────────────────────┤
// │        Final Status:   WA      │
// ╰────────────────────────────────╯
pub fn show_stats(ac: u32, wa: u32, tle: u32, rte: u32, mle: u32) {
    let total = ac + wa + tle + rte + mle;

    let final_status = if ac == total {
        "AC".with_exact_width(3).bold().green()
    } else if rte > 0 {
        "RTE".with_exact_width(3).bold().red()
    } else if wa > 0 {
        "WA".with_exact_width(3).bold().red()
    } else if tle > 0 {
        "TLE".with_exact_width(3).bold().red()
    } else if mle > 0 {
        "MLE".with_exact_width(3).bold().red()
    } else {
        "WA".with_exact_width(3).bold().red()
    };

    println!(
        r#"
╭────────────────────────────────╮
│           {}             │
├────────────────────────────────┤
│ {:>10}/{:10}  {}     │
│ {:>10}/{:10}  {}     │
│ {:>10}/{:10}  {}     │
│ {:>10}/{:10}  {}     │
│ {:>10}/{:10}  {}     │
├────────────────────────────────┤
│        {}   {}     │
╰────────────────────────────────╯
"#,
        "λ Report".italic().bold().blue(),
        ac.separated_string(),
        total.separated_string(),
        "AC".with_exact_width(3).bold().green(),
        wa.separated_string(),
        total.separated_string(),
        "WA".with_exact_width(3).bold().red(),
        tle.separated_string(),
        total.separated_string(),
        "TLE".with_exact_width(3).bold().red(),
        rte.separated_string(),
        total.separated_string(),
        "RTE".with_exact_width(3).bold().red(),
        mle.separated_string(),
        total.separated_string(),
        "MLE".with_exact_width(3).bold().red(),
        "Final Status:".bold(),
        final_status
    );
}

pub fn show_input_test_case<WriteType: std::io::Write>(tout: &mut WriteType, input: &str) {
    let input = input.split('\n');

    writeln!(tout).ok();

    for line in input {
        writeln!(tout, "{}", line.trim().dimmed().bold().black().on_white()).ok();
    }
}

pub fn show_config_file_path(file_name: &str) {
    println!(
        "\n{} Your configuration file has been written in '{}'\n",
        "»".bold().white(),
        file_name.bold().white()
    );
}

pub fn show_config_file_deleted_path(file_name: &str) {
    println!(
        "\n{} Your configuration file '{}' has been deleted\n",
        "»".bold().white(),
        file_name.bold().white()
    );
}

pub fn show_installing_dependencies(name_lang: &str) {
    println!("Installing {} dependencies ...\n", name_lang.bold().white());
}
