use std::{collections::VecDeque, path::PathBuf, time::Duration};

use exitfailure::ExitFailure;

use crate::{
    constants::{
        CACHE_FOLDER, GEN_BINARY_FILE, QTEST_ERROR_FILE, QTEST_INPUT_FILE, QTEST_OUTPUT_FILE,
        TARGET_BINARY_FILE,
    },
    error::handle_error::throw_compiler_error_msg,
    file_handler::{
        file::{
            can_run_language_or_error, copy_file, create_folder_or_error, file_exists_or_error,
            get_filename_output, is_extension_supported_or_error, load_testcases_from_prefix,
            remove_files, save_test_case_output,
        },
        path::get_root_path,
    },
    painter::style::{show_ran_successfully, show_runtime_error, show_time_limit_exceeded},
    runner::types::{is_compiled_error, is_runtime_error, is_time_limit_exceeded, Language},
    util::lang::get_language_by_ext_default,
};

pub fn run(
    target_file: PathBuf,
    prefix: &str,
    timeout: u32,
    break_bad: bool,
    save_all: bool,
) -> Result<(), ExitFailure> {
    // Check if the CACHE_FOLDER folder is already created
    create_folder_or_error(CACHE_FOLDER)?;

    // verify that the target file exists
    file_exists_or_error(target_file.to_str().unwrap(), "<target-file>")?;

    // verify that the target file extension is supported
    is_extension_supported_or_error(target_file.to_str().unwrap())?;

    let root = &get_root_path()[..];

    // Get the language depending on the extension of the target_file
    let any_target: Option<Box<dyn Language>> = get_language_by_ext_default(
        root,
        target_file,
        &TARGET_BINARY_FILE,
        &QTEST_INPUT_FILE,
        &QTEST_OUTPUT_FILE,
        &QTEST_ERROR_FILE,
    );
    let any_target: Box<dyn Language> = any_target.unwrap();
    let target_file_lang: &dyn Language = any_target.as_ref();

    // verify that the program to run the target file is installed
    can_run_language_or_error(target_file_lang)?;

    let can_compile_target = target_file_lang.build();
    if !can_compile_target {
        return throw_compiler_error_msg("target", "<target-file>");
    }

    let mut cases: VecDeque<PathBuf> = VecDeque::new();
    load_testcases_from_prefix(&mut cases, prefix)?;

    let mut test_number: u32 = 0;

    let cases_len: usize = cases.len();

    while (test_number as usize) < cases_len {
        test_number += 1;

        // Load test case in stdin
        let case = cases.pop_front().unwrap();
        copy_file(case.to_str().unwrap(), QTEST_INPUT_FILE)?;

        let response_target = target_file_lang.execute(timeout as u32, test_number);
        let time_target: Duration = response_target.time;
        let mills_target = time_target.as_millis();

        if is_runtime_error(&response_target.status) {
            show_runtime_error(test_number, mills_target as u32);

            // check if the tle_breck flag is high
            if break_bad {
                // remove input, output and error files
                remove_files(vec![
                    QTEST_INPUT_FILE,
                    QTEST_OUTPUT_FILE,
                    QTEST_ERROR_FILE,
                    TARGET_BINARY_FILE,
                    GEN_BINARY_FILE,
                ]);

                return Ok(());
            }
            continue;
        } else if is_compiled_error(&response_target.status) {
            return throw_compiler_error_msg("target", "<target-file>");
        }

        if time_target >= Duration::from_millis(timeout as u64)
            || is_time_limit_exceeded(&response_target.status)
        {
            show_time_limit_exceeded(test_number, timeout);
            // check if the tle_breck flag is high
            if break_bad {
                // remove input, output and error files
                remove_files(vec![
                    QTEST_INPUT_FILE,
                    QTEST_OUTPUT_FILE,
                    QTEST_ERROR_FILE,
                    TARGET_BINARY_FILE,
                    GEN_BINARY_FILE,
                ]);

                return Ok(());
            }
        } else {
            if save_all {
                let file_name =
                    &get_filename_output(prefix, &case.file_name().unwrap().to_str().unwrap())[..];

                // save testcase
                save_test_case_output(file_name, QTEST_OUTPUT_FILE);
            }
            show_ran_successfully(test_number, mills_target as u32);
        }
    }

    Ok(())
}
