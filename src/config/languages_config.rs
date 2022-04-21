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
            "check_installed":"${PROGRAM} --help",
            "libraries":[
                {
                    "name":"ac-library",
                    "version":"1.4",
                    "source":"https://github.com/atcoder/ac-library"
                }
            ]
        },
        {
            "id":"Language::Python",
            "name":"Python 3",
            "extensions":[
                "py"
            ],
            "description":"Interprete del Lenguaje Python 3",
            "env":{
                "PROGRAM":"python3"
            },
            "execute":{
                "unix":"${PROGRAM} ${FILE_NAME}.py",
                "windows":"${PROGRAM} ${FILE_NAME}.py"
            },
            "check_installed":"${PROGRAM} --help",
            "libraries":[
                {
                    "name":"numba",
                    "version":"*"
                }
            ]
        }
    ]
}
"#;
