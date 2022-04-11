/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use colored::*;
use difference::{Changeset, Difference};

pub fn diff_line_by_line<WriteType: std::io::Write>(
    tout: &mut WriteType,
    expected: &str,
    output: &str,
) {
    let Changeset { diffs, .. } = Changeset::new(expected, output, "\n");

    writeln!(tout, "{}", "-Expected".bold().red()).ok();
    writeln!(tout, "{}\n", "+Output".bold().green()).ok();

    for diff in &diffs {
        match diff {
            Difference::Same(ref text) => {
                let text = text.trim().bold();
                writeln!(tout, " {}", text).ok();
            }
            Difference::Add(ref text) => {
                let text = format!("+{}", text.trim()).bold().green();
                writeln!(tout, "{}", text).ok();
            }
            Difference::Rem(ref text) => {
                let text = format!("-{}", text.trim()).bold().red();
                writeln!(tout, "{}", text).ok();
            }
        }
    }
    tout.flush().unwrap();
}
