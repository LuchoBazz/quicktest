mod cli;
mod check_tle;
mod compare;

use structopt::StructOpt;

use crate::cli::Opt;

fn main() {
    // TLE
    // $ quicktest tle --target-file "main.cpp" --gen-file "gen.cpp"
    
    // Compare
    // $ quicktest compare --target-file "main.cpp" --slow-file "slow.cpp" --gen-file "gen.cpp"

    let opt = Opt::from_args();
    println!("{:#?}", opt);

    match opt {
        Opt::TLE { target_file, gen_file, test_cases, timeout} => {
            check_tle::run(target_file, gen_file, test_cases, timeout);
        },
        Opt::Compare { target_file, slow_file, gen_file, timeout, test_cases } => {
            compare::run(target_file, slow_file, gen_file, timeout, test_cases);
        }
    }
}