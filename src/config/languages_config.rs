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
                "h",
                "hh",
                "hpp",
                "hxx",
                "h++",
                "cc",
                "cpp",
                "cxx",
                "c++"
            ],
            "description":"Compilador GNU C++ version 17",
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
            "id":"Language::Rust",
            "name":"Rust Lang",
            "extensions":[
                "rs"
            ],
            "description":"Rust Programming Language",
            "config_files":[
                {
                    "path":"~/.quicktest/rust/Cargo.toml",
                    "content":"[package]\nname = \"rust\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[dependencies]\nproconio = \"0.4.3\"\nnum = \"0.4.0\""
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
                "unix":"cp ${FILE_NAME}.rs ~/.quicktest/rust/src/main.rs && cargo build --release --quiet --manifest-path ~/.quicktest/rust/Cargo.toml && cp ~/.quicktest/rust/target/release/rust .qtest/${FILE_NAME_BINARY}.o",
                "windows":"cp ${FILE_NAME}.rs ~/.quicktest/rust/src/main.rs && cargo build --release --quiet --manifest-path ~/.quicktest/rust/Cargo.toml && cp ~/.quicktest/rust/target/release/rust .qtest/${FILE_NAME_BINARY}.exe"
            },
            "execute":{
                "unix":".qtest/${FILE_NAME_BINARY}.o",
                "windows":".qtest/${FILE_NAME_BINARY}.exe"
            },
            "check_installed":"${PROGRAM} --help"
        }
    ]
}
"#;
