/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use colored::*;

pub fn show_argument_was_updated_success(lang: &str, argument_name: &str, value: &str) {
    // Argumento [] en [] fué actualizado a [] correctamente
    // Argument [] in [] was updated to [] successfully

    println!(
        "  [{}] Argument {} in {} was updated to {} successfully",
        "INFO".bold().green(),
        argument_name.bold().blue(),
        lang.bold().blue(),
        value.bold().blue(),
    );
}
