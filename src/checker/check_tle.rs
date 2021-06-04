use std::path::PathBuf;
use std::env;
use std::time::Duration;

use colored::*;
use failure::ResultExt;
use exitfailure::ExitFailure;

use crate::runner::config::default_gnucpp17;
use crate::runner::config::default_set_output_gnucpp17;
use crate::runner::types::Compiler;

pub fn run(target_file: PathBuf, gen_file: PathBuf,
        test_cases: u32, timeout: u32) -> Result<(), ExitFailure> {
  
    let root = match env::current_dir() {
        Ok(it) => it,
        _ => unreachable!(),
    };

    let root = match root.to_str() {
        Some(s) => s ,
        _ => unreachable!(),
    };

    let target_file_cpp = default_gnucpp17(
        root,
        target_file.to_str().unwrap(),
        &"main.o",
        &"input.txt",
        &"output.txt",
        "error.txt"
    );

    let generator_file_cpp = default_set_output_gnucpp17(
        root,
        gen_file.to_str().unwrap(),
        &"gen.o",
        &"input.txt",
    );

    target_file_cpp.compile();

    generator_file_cpp.compile();

    for test_number in 0..test_cases {
        let time_gen: Duration = generator_file_cpp.execute(timeout as u32);
        let time_target: Duration = target_file_cpp.execute(timeout as u32);
        
        let mills_gen = time_target.as_millis();
        let mills_target = time_target.as_millis();

        
        if time_gen >= Duration::from_millis(timeout as u64) {
            // TLE Generator
            println!("{} {}ms", "⛔ Generator Time Limit Exceeded :".to_string().red(), mills_gen);
            let error = Err(failure::err_msg("very slow generator"));
            return Ok(error.context("Generator TLE".to_string())?);
        } 

        if time_target >= Duration::from_millis(timeout as u64) {
            // TLE Target file
            // let error = Err(failure::err_msg("root cause failure"));
            // return Ok(error.context("this is some context".to_string())?);
            println!("{} {}ms", "⛔ Time Limit Exceeded :".bold().red(), mills_target);
        } else {
            println!(
                "  {} [{}] {} {}ms",
                test_number.to_string().bold().white(),
                "OK".bold().green(),
                "Finished in".bold().green(), mills_target
            );
        }

        // remove files
    }

    Ok(())
}