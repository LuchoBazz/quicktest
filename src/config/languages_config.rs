/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
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
                "unix":"${PROGRAM} ${STANDARD} -DONLINE_JUDGE=1 ${FILE_NAME}.cpp -o .qt/${FILE_NAME_BINARY}",
                "windows":"${PROGRAM} ${STANDARD} -DONLINE_JUDGE=1 ${FILE_NAME}.cpp -o .qt/${FILE_NAME_BINARY}.exe"
            },
            "execute":{
                "unix":".qt/${FILE_NAME_BINARY}",
                "windows":".qt/${FILE_NAME_BINARY}.exe"
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
                "PROGRAM":"python"
            },
            "execute":{
                "unix":"${PROGRAM} ${FILE_NAME}.py",
                "windows":"${PROGRAM} ${FILE_NAME}.py"
            },
            "check_installed":"${PROGRAM} -c \"print('check installed')\""
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
                "unix":"${COMPILER} -d .qt/ ${FILE_NAME}.java",
                "windows":"${COMPILER} -d .qt/ ${FILE_NAME}.java"
            },
            "execute":{
                "unix":"${PROGRAM} -cp .qt/ ${FILE_NAME_BINARY}",
                "windows":"${PROGRAM} -cp .qt/ ${FILE_NAME_BINARY}"
            },
            "check_installed":"${PROGRAM}"
        },
        {
            "id":"Language::C",
            "name":"GNU GCC C11",
            "extensions":[
                "h",
                "c"
            ],
            "description":"GNU C compiler",
            "env":{
                "PROGRAM":"gcc",
                "STANDARD":"-std=gnu11"
            },
            "compile":{
                "unix":"${PROGRAM} ${STANDARD} -lm ${FILE_NAME}.c -o .qt/${FILE_NAME_BINARY}",
                "windows":"${PROGRAM} ${STANDARD} -lm ${FILE_NAME}.c -o .qt/${FILE_NAME_BINARY}.exe"
            },
            "execute":{
                "unix":".qt/${FILE_NAME_BINARY}",
                "windows":".qt/${FILE_NAME_BINARY}.exe"
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
                "unix":"cp ${FILE_NAME}.rs ~/.quicktest/rust/src/main.rs && ${PROGRAM} build --release --quiet --manifest-path ~/.quicktest/rust/Cargo.toml && cp ~/.quicktest/rust/target/release/rust .qt/${FILE_NAME_BINARY} && rm ~/.quicktest/rust/target/release/rust && rm ~/.quicktest/rust/target/release/deps/rust",
                "windows":"cp ${FILE_NAME}.rs ~/.quicktest/rust/src/main.rs && ${PROGRAM} build --release --quiet --manifest-path ~/.quicktest/rust/Cargo.toml && cp ~/.quicktest/rust/target/release/rust.exe .qt/${FILE_NAME_BINARY}.exe && rm ~/.quicktest/rust/target/release/rust.exe && rm ~/.quicktest/rust/target/release/deps/rust.exe"
            },
            "execute":{
                "unix":".qt/${FILE_NAME_BINARY}",
                "windows":".qt/${FILE_NAME_BINARY}.exe"
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
            "env":{
                "PROGRAM":"go"
            },
            "initialize":{
                "unix":"mkdir ~/.quicktest/go_mod",
                "windows":"mkdir ~/.quicktest/go_mod"
            },
            "compile":{
                "unix":"cp ${FILE_NAME}.go ~/.quicktest/go_mod/main.go && ${PROGRAM} build -buildmode=exe -o ./.qt/${FILE_NAME_BINARY} ~/.quicktest/go_mod/main.go",
                "windows":"cp ${FILE_NAME}.go ~/.quicktest/go_mod/main.go && ${PROGRAM} build -buildmode=exe -o ./.qt/${FILE_NAME_BINARY}.exe ~/.quicktest/go_mod/main.go"
            },
            "execute":{
                "unix":".qt/${FILE_NAME_BINARY}",
                "windows":".qt/${FILE_NAME_BINARY}.exe"
            },
            "check_installed":"${PROGRAM} --version"
        },
        {
            "id":"Language::Kotlin",
            "name":"Kotlin",
            "extensions":[
                "kt"
            ],
            "description":"Kotlin Programming Language",
            "env":{
                "PROGRAM":"java",
                "COMPILER":"kotlinc"
            },
            "compile":{
                "unix":"${COMPILER} ${FILE_NAME}.kt -include-runtime -d .qt/${FILE_NAME_BINARY}.jar",
                "windows":"${COMPILER} ${FILE_NAME}.kt -include-runtime -d .qt/${FILE_NAME_BINARY}.jar"
            },
            "execute":{
                "unix":"${PROGRAM} -jar .qt/${FILE_NAME_BINARY}.jar",
                "windows":"${PROGRAM} -jar .qt/${FILE_NAME_BINARY}.jar"
            },
            "check_installed":"${PROGRAM}"
        }
    ]
}
"#;
