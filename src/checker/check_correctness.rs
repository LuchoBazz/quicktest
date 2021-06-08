/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021  Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::collections::VecDeque;
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

pub fn run(target_file: PathBuf, correct_file: PathBuf,
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

    // verify that the correct file exists
    match fs::File::open(correct_file.to_str().unwrap()) {
        Err(_) => {
            let error: Result<(), failure::Error> =
                Err(failure::err_msg(format!("Can't open the file {}", target_file.to_str().unwrap())));
            return Ok(error.context("<correct-file> Not found".to_string())?);
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

    let correct_file_cpp: Cpp = default_gnucpp17(
        root,
        correct_file.to_str().unwrap(),
        &"correct.o",
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

    correct_file_cpp.compile();

    target_file_cpp.compile();

    generator_file_cpp.compile();

    // TODO: if wa is true, remove testcase_tle_*.txt

    let mut tle_count: u32 = 0;
    let mut wa_count: u32 = 0;

    for test_number in 1..=test_cases {
        let time_gen: Duration = generator_file_cpp.execute(timeout as u32);
        let time_correct: Duration = correct_file_cpp.execute(timeout as u32);
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

        if time_correct >= Duration::from_millis(timeout as u64) {
            // TLE Correct file
            println!(
                "  {} [{}] {} {}ms",
                test_number,
                "TLE".bold().red(),
                "Correct File give Time Limit Exceeded :".bold().red(),
                timeout
            );

            let error: Result<(), failure::Error> = Err(failure::err_msg("Correct file very slow"));
            return Ok(error.context("Correct File TLE".to_string())?);
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
        
            // Verify that the folder test_cases exists, in case it does not exist create it
            if save_cases && !Path::new("test_cases").exists() {
                match fs::create_dir("test_cases") {
                    Err(_) => {
                        let error = Err(failure::err_msg("Could not create folder test_cases"));
                        return Ok(error.context("test_cases folder".to_string())?);
                    }
                    _ => (),
                }
            }

            // Save the input of the test case that gave status tle
            if save_cases {
                let filename: String = format!("test_cases/testcase_tle_{}.txt", tle_count);
                let mut file = fs::File::create(filename)
                    .expect("Error creating file test_cases/testcase(i).txt");
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
                fs::remove_file(&"correct.o")?;
                return Ok(());
            }
        } else {
            // The time is in the allowed range

            // Check WA Status
            if compare_file(&format!("{}/quicktest_output.txt", root), &format!("{}/expected_testcase.txt", root), true) {
                // is OK
                println!(
                    "  {} [{}] {} {}ms",
                    test_number.to_string().bold().white(),
                    "OK".bold().green(),
                    "Finished in".bold().green(), mills_target
                );
            } else {
                // WA found
                wa_count += 1;
                println!(
                    "  {} [{}] {} {}ms",
                    test_number.to_string().bold().white(),
                    "WA".bold().red(),
                    "Finished in".bold().red(), mills_target
                );

                // Verify that the folder test_cases exists, in case it does not exist create it
                if save_cases && !Path::new("test_cases").exists() {
                    match fs::create_dir("test_cases") {
                        Err(_) => {
                            let error = Err(failure::err_msg("Could not create folder test_cases"));
                            return Ok(error.context("test_cases folder".to_string())?);
                        }
                        _ => (),
                    }
                }

                // Save the input of the test case that gave status tle
                if save_cases {
                    let filename: String = format!("test_cases/testcase_wa_{}.txt", wa_count);
                    let mut file = fs::File::create(filename)
                        .expect("Error creating file test_cases/testcase(i).txt");
                    file.write_all(fs::read_to_string("quicktest_input.txt").unwrap().as_bytes()).unwrap();
                }

                if wa_break {
                    let error = Err(failure::err_msg(format!("Wrong answer on test {}", test_number)));
                    return Ok(error.context("WA Status".to_string())?);
                }
            }
        }

        // remove input, output and error files
        fs::remove_file(&"quicktest_input.txt")?;
        fs::remove_file(&"quicktest_output.txt")?;
        fs::remove_file(&"quicktest_error.txt")?;
        fs::remove_file(&"expected_testcase.txt")?;
        
    }

    fs::remove_file(&"main.o")?;
    fs::remove_file(&"gen.o")?;
    fs::remove_file(&"correct.o")?;

    Ok(())
}

pub fn compare_file(target_file: &String, correct_file: &String, ignore_space: bool) -> bool {
    let mut is_good = true;
    let target_content = match std::fs::read_to_string(target_file) {
        Ok(content) => Some(content),
        Err(_) => {
            is_good = false;
            None
        },
    };

    let correct_content = match std::fs::read_to_string(correct_file) {
        Ok(content) => Some(content),
        Err(_) => {
            is_good = false;
            None
        },
    };

    if !is_good { return false; }

    let mut target_content_vec: VecDeque<char> = VecDeque::new();
    let mut correct_content_vec: VecDeque<char> = VecDeque::new();
    
    is_good = match (target_content, correct_content) {
        (Some(_), None) => false,
        (None, Some(_)) => false,
        (None, None) => false,
        (Some(s1), Some(s2)) => {
            target_content_vec = s1.chars().collect::<VecDeque<char>>();
            correct_content_vec = s2.chars().collect::<VecDeque<char>>();
            true
        },
    };

    if !is_good { return false; }

    if ignore_space {
        // Remove spaces at the beginning and end of the file
        // for target_content_vec

        // TODO: check when the whole vector has blanks, because a Run Time Error is generated
        while !target_content_vec.is_empty()
            && *target_content_vec.back().unwrap()==' '||*target_content_vec.back().unwrap()=='\n' {
            target_content_vec.pop_back();
        }
        while !target_content_vec.is_empty()
            && *target_content_vec.front().unwrap()==' '||*target_content_vec.front().unwrap()=='\n' {
            target_content_vec.pop_front();
        }
        // for correct_content_vec
        while !correct_content_vec.is_empty()
            && *correct_content_vec.back().unwrap()==' '||*correct_content_vec.back().unwrap()=='\n' {
            correct_content_vec.pop_back();
        }
        while !correct_content_vec.is_empty()
            && *correct_content_vec.front().unwrap()==' '||*correct_content_vec.front().unwrap()=='\n' {
            correct_content_vec.pop_front();
        }
    }

    target_content_vec == correct_content_vec
}