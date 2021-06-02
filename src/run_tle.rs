use std::path::PathBuf;

use crate::runner::cpp::CppRunner;

pub fn run(target_file: PathBuf, gen_file: PathBuf,
        test_cases: i32, timeout: i32) {

    let cpp = CppRunner::new(target_file);
    cpp.compile();
}