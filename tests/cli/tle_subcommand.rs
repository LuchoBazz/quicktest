/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

// reference: https://mattgathu.github.io/2017/10/01/testing-rust-cli-apps.html

static WITHOUT_ARGS_OUTPUT: &'static str =
r#"quicktest-tle 0.3.1
Check TLE

USAGE:
    quicktest tle [FLAGS] [OPTIONS] --gen-file <gen-file> --target-file <target-file>

FLAGS:
    -h, --help          Prints help information
    -s, --save-cases    Save test cases
    -b, --tle-break     Break if Time Limit Exceeded (TLE) occurs
    -V, --version       Prints version information

OPTIONS:
    -g, --gen-file <gen-file>          Generator File
    -t, --target-file <target-file>    Target File
    -n, --test-cases <test-cases>      Number of test cases [default: 1000]
    -o, --timeout <timeout>            Timeout TLE [default: 2000]
"#;

mod tle_subcommand {
    use std::process::Command;
    use crate::cli::tle_subcommand::WITHOUT_ARGS_OUTPUT;

    #[cfg(unix)]
    #[test]
    fn help() {
        let output = Command::new("./target/debug/quicktest")
            .arg("tle")
            .arg("--help")
            .output()
            .expect("help in tle subcommand failed");
        
        assert_eq!(String::from_utf8_lossy(&output.stdout), WITHOUT_ARGS_OUTPUT);
    }
}