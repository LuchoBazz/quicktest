use structopt::StructOpt;

pub mod cli;
pub mod checker;
pub mod runner;

use crate::cli::Opt;

use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    // TLE
    // $ quicktest tle --target-file "main.cpp" --gen-file "gen.cpp"
    
    // Compare
    // $ quicktest compare --target-file "main.cpp" --slow-file "slow.cpp" --gen-file "gen.cpp"

    let opt = Opt::from_args();

    println!("{:?}", opt);

    let response = match opt {
        Opt::TLE { target_file, gen_file, test_cases, timeout} => {
            checker::check_tle::run(target_file, gen_file, test_cases, timeout)
        },
        Opt::Compare { target_file, slow_file, gen_file, timeout, test_cases } => {
            checker::check_correctness::run(target_file, slow_file, gen_file, timeout, test_cases)
        }
    };

    response
}