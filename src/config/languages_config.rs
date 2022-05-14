/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

pub const LANGUAGES_CONFIG_JSON: &str = r#"
{
    "languages":[
        {
            "id":"Language::Cpp",
            "name":"GNU C++17 (64)",
            "extensions":[
                "hh",
                "hpp",
                "hxx",
                "h++",
                "cc",
                "cpp",
                "cxx",
                "c++"
            ],
            "description":"GNU C++ compiler",
            "env":{
                "PROGRAM":"g++",
                "STANDARD":"-std=c++17"
            },
            "compile":{
                "unix":"${PROGRAM} ${STANDARD} ${FILE_NAME}.cpp -o .qtest/${FILE_NAME_BINARY}.o",
                "windows":"${PROGRAM} ${STANDARD} ${FILE_NAME}.cpp -o .qtest/${FILE_NAME_BINARY}.exe"
            },
            "execute":{
                "unix":".qtest/${FILE_NAME_BINARY}.o",
                "windows":".qtest/${FILE_NAME_BINARY}.exe"
            },
            "check_installed":"${PROGRAM} --help"
        },
        {
            "id":"Language::Python",
            "name":"Python 3",
            "extensions":[
                "py"
            ],
            "description":"Python Language Interpreter",
            "env":{
                "PROGRAM":"python3"
            },
            "execute":{
                "unix":"${PROGRAM} ${FILE_NAME}.py",
                "windows":"${PROGRAM} ${FILE_NAME}.py"
            },
            "check_installed":"${PROGRAM} --help"
        },
        {
            "id":"Language::Java",
            "name":"Java",
            "extensions":[
                "java"
            ],
            "description":"Java Programming Language",
            "env":{
                "PROGRAM":"java",
                "COMPILER":"javac"
            },
            "compile":{
                "unix":"${COMPILER} -d .qtest/ ${FILE_NAME}.java",
                "windows":"${COMPILER} -d .qtest/ ${FILE_NAME}.java"
            },
            "execute":{
                "unix":"${PROGRAM} -cp .qtest/ ${FILE_NAME_BINARY}",
                "windows":"${PROGRAM} -cp .qtest/ ${FILE_NAME_BINARY}"
            },
            "check_installed":"${PROGRAM}"
        },
        {
            "id":"Language::Rust",
            "name":"Rust Lang",
            "extensions":[
                "rs"
            ],
            "description":"Rust Programming Language",
            "config_files":[
                {
                    "path":"~/.quicktest/rust/Cargo.toml",
                    "content":"[package]\nname = \"rust\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[dependencies]\nproconio = \"0.4.3\"\nnum = \"0.4.0\"\nrand = { version = \"0.8.5\", features = [\"small_rng\"]}\nregex = \"1.5.5\"\nnum-bigint = \"0.4.3\""
                }
            ],
            "env":{
                "PROGRAM":"cargo"
            },
            "initialize":{
                "unix":"${PROGRAM} init ~/.quicktest/rust",
                "windows":"${PROGRAM} init ~/.quicktest/rust"
            },
            "compile":{
                "unix":"cp ${FILE_NAME}.rs ~/.quicktest/rust/src/main.rs && ${PROGRAM} build --release --quiet --manifest-path ~/.quicktest/rust/Cargo.toml && cp ~/.quicktest/rust/target/release/rust .qtest/${FILE_NAME_BINARY}.o",
                "windows":"cp ${FILE_NAME}.rs ~/.quicktest/rust/src/main.rs && ${PROGRAM} build --release --quiet --manifest-path ~/.quicktest/rust/Cargo.toml && cp ~/.quicktest/rust/target/release/rust .qtest/${FILE_NAME_BINARY}.exe"
            },
            "execute":{
                "unix":".qtest/${FILE_NAME_BINARY}.o",
                "windows":".qtest/${FILE_NAME_BINARY}.exe"
            },
            "check_installed":"${PROGRAM} --help"
        },
        {
            "id":"Language::Go",
            "name":"Go Lang",
            "extensions":[
                "go"
            ],
            "description":"Go Programming Language",
            "config_files":[
                {
                    "path":"~/.quicktest/go_mod/go.mod",
                    "content":"module go_mod\n\ngo 1.17\n\ngithub.com/emirpasic/gods v1.18.1 // indirect\n\n"
                }
            ],
            "env":{
                "PROGRAM":"go"
            },
            "initialize":{
                "unix":"mkdir ~/.quicktest/go_mod && GO111MODULE=off ${PROGRAM} get github.com/emirpasic/gods && GO111MODULE=off ${PROGRAM} get https://github.com/gonum/gonum",
                "windows":"mkdir ~/.quicktest/go_mod && GO111MODULE=off ${PROGRAM} get github.com/emirpasic/gods && GO111MODULE=off ${PROGRAM} get https://github.com/gonum/gonum"
            },
            "compile":{
                "unix":"cp ${FILE_NAME}.go ~/.quicktest/go_mod/main.go && ${PROGRAM} build -buildmode=exe -o ./.qtest/${FILE_NAME_BINARY}.o ~/.quicktest/go_mod/main.go",
                "windows":"cp ${FILE_NAME}.go ~/.quicktest/go_mod/main.go && ${PROGRAM} build -buildmode=exe -o ./.qtest/${FILE_NAME_BINARY}.exe ~/.quicktest/go_mod/main.go"
            },
            "execute":{
                "unix":".qtest/${FILE_NAME_BINARY}.o",
                "windows":".qtest/${FILE_NAME_BINARY}.exe"
            },
            "check_installed":"${PROGRAM} --version"
        }
    ]
}
"#;
