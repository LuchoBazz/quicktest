use std::io::Write;
use std::path::{PathBuf, Path};
use std::time::Duration;
use std::env;
use std::fs;

use colored::*;
use failure::ResultExt;
use exitfailure::ExitFailure;

use crate::runner::config::default_gnucpp17;
use crate::runner::config::default_set_output_gnucpp17;
use crate::runner::types::Compiler;

pub fn run(target_file: PathBuf, gen_file: PathBuf,
        test_cases: u32, timeout: u32, tle_break: bool) -> Result<(), ExitFailure> {
    
    // verify that the target file exists
    match fs::File::open(target_file.to_str().unwrap()) {
        Err(_) => {
            let error = Err(failure::err_msg(format!("Can't open the file {}", target_file.to_str().unwrap())));
            return Ok(error.context("<target-file> Not found".to_string())?);
        },
        _ => (),
    }

    // verify that the generator file exists
    match fs::File::open(gen_file.to_str().unwrap()) {
        Err(_) => {
            let error = Err(failure::err_msg(format!("Can't open the file {}", gen_file.to_str().unwrap())));
            return Ok(error.context("<gen-file> Not found".to_string())?);
        },
        _ => (),
    }

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
        &"quicktest_input.txt",
        &"quicktest_output.txt",
        "quicktest_error.txt"
    );

    let generator_file_cpp = default_set_output_gnucpp17(
        root,
        gen_file.to_str().unwrap(),
        &"gen.o",
        &"quicktest_input.txt",
    );

    target_file_cpp.compile();

    generator_file_cpp.compile();

    let mut tle_count: u32 = 0;

    for test_number in 1..=test_cases {
        let time_gen: Duration = generator_file_cpp.execute(timeout as u32);
        let time_target: Duration = target_file_cpp.execute(timeout as u32);
        
        let mills_target = time_target.as_millis();
        
        if time_gen >= Duration::from_millis(timeout as u64) {
            // TLE Generator

            println!(
                "  {} [{}] {} {}ms",
                test_number,
                "TLE".bold().red(),
                "Generator Time Limit Exceeded :".bold().red(),
                timeout
            );

            let error = Err(failure::err_msg("very slow generator"));
            return Ok(error.context("Generator TLE".to_string())?);
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
        
            // Verify that the folder tle_cases exists, in case it does not exist create it
            if !Path::new("tle_cases").exists() {
                match fs::create_dir("tle_cases") {
                    Err(_) => {
                        let error = Err(failure::err_msg("Could not create folder tle_cases"));
                        return Ok(error.context("tle_cases folder".to_string())?);
                    }
                    _ => (),
                }
            }

            // Save the input of the test case that gave status tle
            let filename = format!("tle_cases/testcase{}.txt", tle_count);

            let mut file = fs::File::create(filename)
                .expect("Error creating file tle_cases/testcase(n).txt");

            file.write_all(fs::read_to_string("quicktest_input.txt").unwrap().as_bytes()).unwrap();

            // check if the tle_breck flag is high
            if tle_break {
                // remove input, output and error files
                fs::remove_file(&"quicktest_input.txt")?;
                fs::remove_file(&"quicktest_output.txt")?;
                fs::remove_file(&"quicktest_error.txt")?;
                fs::remove_file(&"main.o")?;
                fs::remove_file(&"gen.o")?;
               return Ok(());
            }
        } else {
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
    }

    fs::remove_file(&"main.o")?;
    fs::remove_file(&"gen.o")?;

    Ok(())
}