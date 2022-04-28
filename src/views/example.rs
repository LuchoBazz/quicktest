/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use colored::*;

fn show_command_examples_text() {
    #[cfg(unix)]
    println!("   📕 {}\n", "Command Examples".bold().blue());

    #[cfg(windows)]
    println!("    {}\n", "Command Examples".bold().blue());
}

pub fn show_examples_cmp_cmd() {
    show_command_examples_text();
    println!(
        "   $ {} {} {}",
        "quicktest".bold().green(),
        "cmp".bold().blue(),
        "--target-file=main.cpp --correct-file=correct.cpp --gen-file=gen.cpp --timeout=2000 --test-cases=1000".bold().white()
    );
}

pub fn show_examples_stress_cmd() {
    show_command_examples_text();
    println!(
        "   $ {} {} {}",
        "quicktest".bold().green(),
        "stress".bold().blue(),
        "--target-file=main.cpp --gen-file=gen.cpp --timeout=2000 --test-cases=1000"
            .bold()
            .white()
    );
}

pub fn show_examples_check_cmd() {
    show_command_examples_text();
    println!(
        "   $ {} {} {}",
        "quicktest".bold().green(),
        "check".bold().blue(),
        "--target-file=main.cpp --checker-file=check.cpp --gen-file=gen.cpp --timeout=2000 --test-cases=1000".bold().white()
    );
}
