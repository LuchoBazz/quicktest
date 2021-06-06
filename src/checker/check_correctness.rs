/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021  Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::env;
use std::time::Duration;
use std::io::Write;

use crate::runner::config::default_gnucpp17;
use crate::runner::config::default_set_output_gnucpp17;
use crate::runner::lang::cpp::Cpp;
use crate::runner::types::Compiler;

use failure::ResultExt;
use exitfailure::ExitFailure;
use colored::*;

pub fn run(target_file: PathBuf, slow_file: PathBuf,
        gen_file: PathBuf, timeout: u32, test_cases: u32, wa_break: bool,
        save_cases: bool) -> Result<(), ExitFailure>  {
    
    // verify that the target file exists
    match fs::File::open(target_file.to_str().unwrap()) {
        Err(_) => {
            let error: Result<(), failure::Error> =
                Err(failure::err_msg(format!("Can't open the file {}", target_file.to_str().unwrap())));
            return Ok(error.context("<target-file> Not found".to_string())?);
        },
        _ => (),
    };

    // verify that the slow file exists
    match fs::File::open(slow_file.to_str().unwrap()) {
        Err(_) => {
            let error: Result<(), failure::Error> =
                Err(failure::err_msg(format!("Can't open the file {}", target_file.to_str().unwrap())));
            return Ok(error.context("<slow-file> Not found".to_string())?);
        },
        _ => (),
    };

    // verify that the generator file exists
    match fs::File::open(gen_file.to_str().unwrap()) {
        Err(_) => {
            let error: Result<(), failure::Error> =
                Err(failure::err_msg(format!("Can't open the file {}", gen_file.to_str().unwrap())));
            return Ok(error.context("<gen-file> Not found".to_string())?);
        },
        _ => (),
    };

    // get root path
    let root: PathBuf = match env::current_dir() {
        Ok(it) => it,
        _ => unreachable!(),
    };

    let root: &str = match root.to_str() {
        Some(s) => s ,
        _ => unreachable!(),
    };

    let slow_file_cpp: Cpp = default_gnucpp17(
        root,
        slow_file.to_str().unwrap(),
        &"slow.o",
        &"quicktest_input.txt",
        &"expected_testcase.txt",
        "quicktest_error.txt"
    );

    let target_file_cpp: Cpp = default_gnucpp17(
        root,
        target_file.to_str().unwrap(),
        &"main.o",
        &"quicktest_input.txt",
        &"quicktest_output.txt",
        "quicktest_error.txt"
    );


    let generator_file_cpp: Cpp = default_set_output_gnucpp17(
        root,
        gen_file.to_str().unwrap(),
        &"gen.o",
        &"quicktest_input.txt",
    );

    slow_file_cpp.compile();

    target_file_cpp.compile();

    generator_file_cpp.compile();

    let mut tle_count: u32 = 0;

    for test_number in 1..=test_cases {
        let time_gen: Duration = generator_file_cpp.execute(timeout as u32);
        let time_slow: Duration = slow_file_cpp.execute(timeout as u32);
        let time_target: Duration = target_file_cpp.execute(timeout as u32);

        let mills_target: u128 = time_target.as_millis();

        if time_gen >= Duration::from_millis(timeout as u64) {
            // TLE Generator

            println!(
                "  {} [{}] {} {}ms",
                test_number,
                "TLE".bold().red(),
                "Generator Time Limit Exceeded :".bold().red(),
                timeout
            );

            let error: Result<(), failure::Error> = Err(failure::err_msg("very slow generator"));
            return Ok(error.context("Generator TLE".to_string())?);
        }

        if time_slow >= Duration::from_millis(timeout as u64) {
            // TLE Slow file

            println!(
                "  {} [{}] {} {}ms",
                test_number,
                "TLE".bold().red(),
                "Slow File give Time Limit Exceeded :".bold().red(),
                timeout
            );

            let error: Result<(), failure::Error> = Err(failure::err_msg("Slow file give tle"));
            return Ok(error.context("Slow File TLE".to_string())?);
        }

        if time_target >= Duration::from_millis(timeout as u64) {
            // TLE Target file
            
            tle_count += 1;

            println!(
                "  {} [{}] {} {}ms",
                test_number,
                "TLE".bold().red(),
                "Time Limit Exceeded :".bold().red(),
                timeout
            );
        
            // Verify that the folder wa_cases exists, in case it does not exist create it
            if save_cases && !Path::new("wa_cases").exists() {
                match fs::create_dir("wa_cases") {
                    Err(_) => {
                        let error = Err(failure::err_msg("Could not create folder wa_cases"));
                        return Ok(error.context("wa_cases folder".to_string())?);
                    }
                    _ => (),
                }
            }

            // Save the input of the test case that gave status tle
            if save_cases {
                let filename: String = format!("wa_cases/testcase{}.txt", tle_count);
                let mut file = fs::File::create(filename)
                    .expect("Error creating file wa_cases/testcase(n).txt");
                file.write_all(fs::read_to_string("quicktest_input.txt").unwrap().as_bytes()).unwrap();
            }
            
            // check if the wa_break flag is high
            if wa_break {
                // remove input, output and error files
                fs::remove_file(&"quicktest_input.txt")?;
                fs::remove_file(&"quicktest_output.txt")?;
                fs::remove_file(&"quicktest_error.txt")?;
                fs::remove_file(&"expected_testcase.txt")?;
                fs::remove_file(&"main.o")?;
                fs::remove_file(&"gen.o")?;
                fs::remove_file(&"slow.o")?;
                return Ok(());
            }
        } else {
            // TODO: Verify WA status
            println!(
                "  {} [{}] {} {}ms",
                test_number.to_string().bold().white(),
                "OK".bold().green(),
                "Finished in".bold().green(), mills_target
            );
        }

        // remove input, output and error files
        fs::remove_file(&"quicktest_input.txt")?;
        fs::remove_file(&"quicktest_output.txt")?;
        fs::remove_file(&"quicktest_error.txt")?;
        fs::remove_file(&"expected_testcase.txt")?;
        
    }

    fs::remove_file(&"main.o")?;
    fs::remove_file(&"gen.o")?;
    fs::remove_file(&"slow.o")?;

    Ok(())
}
