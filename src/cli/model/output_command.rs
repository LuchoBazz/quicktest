/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021 - Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use std::path::PathBuf;

use super::traits::AdapterCommand;

pub struct OutputCommand {
    target_file: PathBuf,
    prefix: String,
    timeout: u32,
    memory_limit: u64,
    break_bad: bool,
    save_out: bool,
}

impl OutputCommand {
    pub fn new(
        target_file: PathBuf,
        prefix: String,
        timeout: u32,
        memory_limit: u64,
        break_bad: bool,
        save_out: bool,
    ) -> OutputCommand {
        OutputCommand {
            target_file,
            prefix,
            timeout,
            memory_limit,
            break_bad,
            save_out,
        }
    }
}

impl AdapterCommand for OutputCommand {
    fn get_target_file(&self) -> PathBuf {
        self.target_file.clone()
    }
    fn get_timeout(&self) -> u32 {
        self.timeout
    }
    fn get_memory_limit(&self) -> u64 {
        self.memory_limit
    }
    fn get_break_bad(&self) -> bool {
        self.break_bad
    }
    fn get_prefix(&self) -> String {
        self.prefix.clone()
    }
    fn get_save_out(&self) -> bool {
        self.save_out
    }
}
