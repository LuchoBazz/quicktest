/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021-present / Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use std::path::PathBuf;
pub trait AdapterCommand {
    fn get_target_file(&self) -> PathBuf {
        unreachable!("get_target_file method not implemented")
    }
    fn get_correct_file(&self) -> PathBuf {
        unreachable!("get_correct_file method not implemented")
    }
    fn get_checker_file(&self) -> PathBuf {
        unreachable!("get_checker_file method not implemented")
    }
    fn get_generator_file(&self) -> PathBuf {
        unreachable!("get_generator_file method not implemented")
    }
    fn get_timeout(&self) -> u32 {
        unreachable!("get_timeout method not implemented")
    }
    fn get_memory_limit(&self) -> u64 {
        unreachable!("get_memory_limit method not implemented")
    }
    fn get_test_cases(&self) -> u32 {
        unreachable!("get_test_cases method not implemented")
    }
    fn get_break_bad(&self) -> bool {
        unreachable!("get_break_bad method not implemented")
    }
    fn get_save_bad(&self) -> bool {
        unreachable!("get_save_bad method not implemented")
    }
    fn get_save_all(&self) -> bool {
        unreachable!("get_save_all method not implemented")
    }
    fn get_save_out(&self) -> bool {
        unreachable!("get_save_out method not implemented")
    }
    fn get_run_all(&self) -> bool {
        unreachable!("get_run_all method not implemented")
    }
    fn get_run_ac(&self) -> bool {
        unreachable!("get_run_ac method not implemented")
    }
    fn get_run_wa(&self) -> bool {
        unreachable!("get_run_wa method not implemented")
    }
    fn get_run_tle(&self) -> bool {
        unreachable!("get_run_tle method not implemented")
    }
    fn get_run_rte(&self) -> bool {
        unreachable!("get_run_rte method not implemented")
    }
    fn get_run_mle(&self) -> bool {
        unreachable!("get_run_mle method not implemented")
    }
    fn can_run_cases(&self) -> bool {
        unreachable!("can_run_cases method not implemented")
    }
    fn get_diff(&self) -> bool {
        unreachable!("get_diff method not implemented")
    }
    fn get_prefix(&self) -> String {
        unreachable!("get_prefix method not implemented")
    }
    fn has_test_cases(&self, _test_number: u32) -> bool {
        unreachable!("has_test_cases method not implemented")
    }
}
