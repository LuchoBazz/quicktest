use std::path::PathBuf;
use std::env;
use crate::runner::config::default_gnucpp17;
use crate::runner::config::default_set_output_gnucpp17;
use crate::runner::types::Compiler;

pub fn run(target_file: PathBuf, gen_file: PathBuf,
        _test_cases: i32, _timeout: i32) {
    
    println!("{}", r#"
             _
 _ __   ___ | |_
| '_ \ / _ \| __|
| | | | (_) | |_
|_| |_|\___/ \__|

  _                 _                           _           _ 
 (_)_ __ ___  _ __ | | ___ _ __ ___   ___ _ __ | |_ ___  __| |
 | | '_ ` _ \| '_ \| |/ _ \ '_ ` _ \ / _ \ '_ \| __/ _ \/ _` |
 | | | | | | | |_) | |  __/ | | | | |  __/ | | | ||  __/ (_| |
 |_|_| |_| |_| .__/|_|\___|_| |_| |_|\___|_| |_|\__\___|\__,_|
             |_|      
    "#);
  
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

    generator_file_cpp.execute();
    target_file_cpp.execute();

}