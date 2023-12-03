/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use exitfailure::ExitFailure;

use crate::{
    cli::model::traits::AdapterCommand,
    constants::{
        QTEST_CHECKER_FILE, QTEST_ERROR_FILE, QTEST_EXPECTED_FILE, QTEST_INPUT_FILE,
        QTEST_OUTPUT_FILE,
    },
    error::handle_error::throw_compiler_error_msg,
    file_handler::file::{
        can_run_language_or_error, file_exists_or_error, is_extension_supported_or_error,
    },
    runner::types::Language,
};

use super::language_handler::{get_generator_handler, get_language_handler, LanguageHandler};

pub fn get_executor_generator(
    command: &dyn AdapterCommand,
) -> Result<Box<LanguageHandler>, ExitFailure> {
    // if --prefix was set, the generator will not be executed
    if !command.get_prefix().is_empty() {
        return Ok(Box::default());
    }

    // verify that the generator file exists
    file_exists_or_error(command.get_generator_file().to_str().unwrap(), "<gen-file>")?;

    // verify that the generator file extension is supported
    is_extension_supported_or_error(command.get_generator_file().to_str().unwrap())?;

    let generator_file_lang = *get_generator_handler(
        &command
            .get_generator_file()
            .into_os_string()
            .into_string()
            .unwrap()[..],
        "<gen-file>",
        QTEST_INPUT_FILE,
    )?;

    // verify that the program to run the generator file is installed
    can_run_language_or_error(&generator_file_lang)?;

    // TODO: Verify Error
    generator_file_lang.init();

    let can_compile_gen = generator_file_lang.build();
    if !can_compile_gen {
        return Err(throw_compiler_error_msg("generator", "<gen-file>").unwrap_err());
    }

    Ok(Box::new(generator_file_lang))
}

pub fn get_executor_target(
    command: &dyn AdapterCommand,
) -> Result<Box<LanguageHandler>, ExitFailure> {
    // verify that the target file exists
    file_exists_or_error(command.get_target_file().to_str().unwrap(), "<target-file>")?;

    // verify that the target file extension is supported
    is_extension_supported_or_error(command.get_target_file().to_str().unwrap())?;

    // Get the language depending on the extension of the target_file
    let target_file_lang = *get_language_handler(
        &command
            .get_target_file()
            .into_os_string()
            .into_string()
            .unwrap()[..],
        "<target-file>",
        QTEST_INPUT_FILE,
        QTEST_OUTPUT_FILE,
        QTEST_ERROR_FILE,
    )?;

    // verify that the program to run the generator file is installed
    can_run_language_or_error(&target_file_lang)?;

    // TODO: Verify Error
    target_file_lang.init();

    let can_compile_target = target_file_lang.build();
    if !can_compile_target {
        return Err(throw_compiler_error_msg("target", "<target-file>").unwrap_err());
    }

    Ok(Box::new(target_file_lang))
}

pub fn get_executor_checker(
    command: &dyn AdapterCommand,
) -> Result<Box<LanguageHandler>, ExitFailure> {
    // verify that the checker_file file exists
    file_exists_or_error(
        command.get_checker_file().to_str().unwrap(),
        "<checker-file>",
    )?;

    // verify that the checker file extension is supported
    is_extension_supported_or_error(command.get_checker_file().to_str().unwrap())?;

    // Get the language depending on the extension of the checker_file_lang
    let checker_file_lang = *get_language_handler(
        &command
            .get_checker_file()
            .into_os_string()
            .into_string()
            .unwrap()[..],
        "<checker-file>",
        QTEST_OUTPUT_FILE,
        QTEST_CHECKER_FILE,
        QTEST_ERROR_FILE,
    )?;

    // verify that the program to run the checker file is installed
    can_run_language_or_error(&checker_file_lang)?;

    // TODO: Verify Error
    checker_file_lang.init();

    let can_compile_checker = checker_file_lang.build();
    if !can_compile_checker {
        return Err(throw_compiler_error_msg("checker", "<checker-file>").unwrap_err());
    }
    Ok(Box::new(checker_file_lang))
}

pub fn get_executor_correct(
    command: &dyn AdapterCommand,
) -> Result<Box<LanguageHandler>, ExitFailure> {
    // verify that the checker_file file exists
    file_exists_or_error(
        command.get_correct_file().to_str().unwrap(),
        "<correct-file>",
    )?;

    // verify that the checker file extension is supported
    is_extension_supported_or_error(command.get_correct_file().to_str().unwrap())?;

    // Get the language depending on the extension of the checker_file_lang
    let correct_file_lang = *get_language_handler(
        &command
            .get_correct_file()
            .into_os_string()
            .into_string()
            .unwrap()[..],
        "<correct-file>",
        QTEST_INPUT_FILE,
        QTEST_EXPECTED_FILE,
        QTEST_ERROR_FILE,
    )?;

    // verify that the program to run the checker file is installed
    can_run_language_or_error(&correct_file_lang)?;

    // TODO: Verify Error
    correct_file_lang.init();

    let can_compile_correct = correct_file_lang.build();
    if !can_compile_correct {
        return Err(throw_compiler_error_msg("correct", "<correct-file>").unwrap_err());
    }
    Ok(Box::new(correct_file_lang))
}
