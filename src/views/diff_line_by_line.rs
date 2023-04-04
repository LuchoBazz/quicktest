/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use crate::constants::IDENTATION;
use colored::*;
use difference::{Changeset, Difference};

pub fn diff_line_by_line<WriteType: std::io::Write>(
    tout: &mut WriteType,
    expected: &str,
    output: &str,
) {
    let Changeset { diffs, .. } = Changeset::new(expected, output, "\n");

    // + Expected
    // - Output

    const LINE: &str = "──────────────────────";
    writeln!(tout, "\n{}  {}", IDENTATION, LINE.bold().white()).ok();

    for diff in &diffs {
        match diff {
            Difference::Same(ref text) => {
                let text = text.trim().bold();
                writeln!(tout, "{}  {}", IDENTATION, text).ok();
            }
            Difference::Add(ref text) => {
                let text = format!("- {}", text.trim()).bold().bright_red();
                writeln!(tout, "{}{}", IDENTATION, text).ok();
            }
            Difference::Rem(ref text) => {
                let text = format!("+ {}", text.trim()).bold().bright_green();
                writeln!(tout, "{}{}", IDENTATION, text).ok();
            }
        }
    }
    writeln!(tout, "{}  {}\n", IDENTATION, LINE.bold().white()).ok();

    tout.flush().unwrap();
}
