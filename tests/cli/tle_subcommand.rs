/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

// reference I : https://mattgathu.github.io/2017/10/01/testing-rust-cli-apps.html
// reference II: https://www.duskborn.com/posts/rust-lit/

static GEN_CPP: &'static str = r#"
#include <bits/stdc++.h>
using namespace std;
template <typename T>
T random(const T from, const T to) {
    static random_device rdev;
    static default_random_engine re(rdev());

    using dist_type = typename conditional<
        is_floating_point<T>::value,
        uniform_real_distribution<T>,
        uniform_int_distribution<T>
    >::type;

    dist_type uni(from, to);
    return static_cast<T>(uni(re));
}
int main() {
    #define endl '\n'
    int n = random<int>(1e5, 2e5);
    cout << n << endl;
    for(int i=0;i<n;++i) cout << random<int>(1, 1e9) << " ";
    cout << endl;
    return 0;
}
"#;

static TARGET_FILE: &'static str = r#"
#include <bits/stdc++.h>
using namespace std;
int n;
vector<int> A;
int main() {
    cin >> n;
    A.resize(n);
    for(auto &ref: A) cin >> ref;
    sort(A.begin(), A.end());
    bool change = (A[0] % 1000 == 0);
    if(change) swap(A[0], A[1]);
    cout << n << endl;
    for(auto &a: A) cout << a << " ";
    cout << endl;
    return 0;
}
"#;

mod tle_gen_cpp_target_cpp {

    use std::{io::Write, path::PathBuf, process::Command};  // Run programs
    use assert_cmd::prelude::*; // Add methods on commands
    use predicates::prelude::*; // Used for writing assertion
    use std::error::Error;

    fn create_files() -> Result<(), std::io::Error>{
        use crate::cli::tle_subcommand::GEN_CPP;
        use crate::cli::tle_subcommand::TARGET_FILE;
        match std::fs::create_dir("target/code/") {_=>(),}
        let mut gen_file = std::fs::File::create(PathBuf::from("target/code/gen.cpp"))?;
        gen_file.write_all(GEN_CPP.as_bytes())?;
        let mut gen_file = std::fs::File::create(PathBuf::from("target/code/main.cpp"))?;
        gen_file.write_all(TARGET_FILE.as_bytes())?;
        Ok(())
    }

    
    #[test]
    fn run() -> Result<(), Box<dyn Error>> {
        create_files()?;

        #[cfg(unix)]
        let mut cmd = Command::new("./target/debug/quicktest");

        #[cfg(windows)]
        let mut cmd = Command::new("./target/debug/quicktest.exe");

        let cases: usize = 10;

        cmd.arg("tle")
            .arg("--target-file")
            .arg("target/code/main.cpp")
            .arg("--gen-file")
            .arg("target/code/gen.cpp")
            .arg("--timeout=1000")
            .arg(format!("--test-cases={}", cases));

        cmd.assert()
            .success()
            .stdout(predicate::str::contains("[OK]").count(cases));
        
        Ok(())
    }
}