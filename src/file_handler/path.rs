/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

pub fn get_root_path() -> String {
    let root = match std::env::current_dir() {
        Ok(it) => it,
        _ => unreachable!(),
    };

    let root = match root.to_str() {
        Some(s) => s.to_string(),
        _ => unreachable!(),
    };
    return root;
}