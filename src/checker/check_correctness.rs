/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021  Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::path::PathBuf;

use exitfailure::ExitFailure;

pub fn run(_target_file: PathBuf, _slow_file: PathBuf,
        _gen_file: PathBuf, _timeout: u32, _test_cases: u32) -> Result<(), ExitFailure>  {
    
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

            _                                                
 _   _  ___| |_                                              
| | | |/ _ \ __|                                             
| |_| |  __/ |_                                              
 \__, |\___|\__|                                             
 |___/
               
   "#);
    unimplemented!();
}
