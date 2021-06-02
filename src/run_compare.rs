use std::path::PathBuf;

use crate::runner::cpp::CppRunner;

pub fn run(target_file: PathBuf, slow_file: PathBuf,
    gen_file: PathBuf, timeout: i32, test_cases: i32) {
        let cpp = CppRunner::new(target_file);
        cpp.compile();

}