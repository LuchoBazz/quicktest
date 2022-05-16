---
sidebar_position: 2
title: Configuration
sidebar_label: Configuration
---

## Configuration File Example

_Path:_ `~/.quicktest/languages.config.json`

### Scheme

```json
{
    "languages":[
        {
            "id":"string",
            "name":"string",
            "extensions":["string"],
            "description":"string",
            "config_files":[
                {
                    "path":"string",
                    "content":"string"
                }
            ],
            "env":{
                "PROGRAM":"string"
            },
            "initialize":{
                "unix":"string",
                "windows":"string"
            },
            "compile":{
                "unix":"string",
                "windows":"string"
            },
            "execute":{
                "unix":"string",
                "windows":"string"
            },
            "check_installed":"string"
        }
    ]
}
```

## Fields Description

### `languages: list<Languages>` Field

List of supported languages

## `languages[i]`

### `id: string` Field

Language Identifier

* _Required:_ `true`
* _Example:_ 
```json
"id":"Language::Cpp"
```

### `name: string` Field

Language Name

* _Required:_ `true`
* _Example:_ 
```json
"name":"GNU C++17 (64)"
```

### `extensions: list<string>` Field

List of extensions to run with the associated language

* _Required:_ `true`
* _Example:_ 
```json
"extensions":[
    "hh",
    "hpp",
    "hxx",
    "h++",
    "cc",
    "cpp",
    "cxx",
    "c++"
]
```

### `description: string` Field

Brief description of the language

* _Required:_ `true`
* _Example:_ 
```json
"description":"GNU C++ compiler"
```

### `config_files: Object` Field

Configuration files for the specific language, which will be created in the `path` with the `content`

* _Required:_ `false`
* _Example:_ 

```json
[
    {
        "path":"~/.quicktest/rust/Cargo.toml",
        "content":"[package]\nname = \"rust\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[dependencies]"
    }
]
```

### `env: Object` Field

Environment variables that can be used in command fields

* _Required:_ `true`
* _Example:_ 

```json
"env":{
    "PROGRAM":"g++",
    "STANDARD":"-std=c++17"
}
// "compile":{
//    "unix":"${PROGRAM} ${STANDARD} ${FILE_NAME}.cpp -o .qt/${FILE_NAME_BINARY}.o",
//    "windows":"${PROGRAM} ${STANDARD} ${FILE_NAME}.cpp -o .qt/${FILE_NAME_BINARY}.exe"
// }

// will be processed to

// "compile":{
//    "unix":"g++ -std=c++17 main.cpp -o main.o",
//    "windows":"g++ -std=c++17 main.cpp -o main.exe"
// }
```

### `initialize: Object` Field

Command to be executed before language compilation
* _Required:_ `false`
* _Example:_ 

```json
"initialize":{
    "unix":"cargo init ~/.quicktest/rust",
    "windows":"cargo init ~/.quicktest/rust"
}
```

### `compile: Object` Field

Language compile command

* _Required:_ `false`
* _Example:_ 

```json
"compile":{
    "unix":"${PROGRAM} ${STANDARD} ${FILE_NAME}.cpp -o .qt${FILE_NAME_BINARY}.o",
    "windows":"${PROGRAM} ${STANDARD} ${FILE_NAME}.cpp -o .qt/${FILE_NAME_BINARY}.exe"
}
```

### `execute: Object` Field

Language execution command

* _Required:_ `true`
* _Example:_ 

```json
"execute":{
    "unix":".qt/${FILE_NAME_BINARY}.o",
    "windows":".qt/${FILE_NAME_BINARY}.exe"
},
```

### `check_installed: String` Field

Command to verify that the program with which the programming language is going to be executed is installed locally

* _Required:_ `true`
* _Example:_ 

```json
"check_installed":"${PROGRAM} --help"
```

---

## `~/.quicktest/languages.config.json` File

```json
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
                "unix":"${PROGRAM} ${STANDARD} ${FILE_NAME}.cpp -o .qt/${FILE_NAME_BINARY}.o",
                "windows":"${PROGRAM} ${STANDARD} ${FILE_NAME}.cpp -o .qt/${FILE_NAME_BINARY}.exe"
            },
            "execute":{
                "unix":".qt/${FILE_NAME_BINARY}.o",
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
                "unix":"${PROGRAM} ${STANDARD} ${FILE_NAME}.c -o .qt/${FILE_NAME_BINARY}.o",
                "windows":"${PROGRAM} ${STANDARD} ${FILE_NAME}.c -o .qt/${FILE_NAME_BINARY}.exe"
            },
            "execute":{
                "unix":".qt/${FILE_NAME_BINARY}.o",
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
                "unix":"cp ${FILE_NAME}.rs ~/.quicktest/rust/src/main.rs && ${PROGRAM} build --release --quiet --manifest-path ~/.quicktest/rust/Cargo.toml && cp ~/.quicktest/rust/target/release/rust .qt/${FILE_NAME_BINARY}.o",
                "windows":"cp ${FILE_NAME}.rs ~/.quicktest/rust/src/main.rs && ${PROGRAM} build --release --quiet --manifest-path ~/.quicktest/rust/Cargo.toml && cp ~/.quicktest/rust/target/release/rust .qt/${FILE_NAME_BINARY}.exe"
            },
            "execute":{
                "unix":".qt/${FILE_NAME_BINARY}.o",
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
                "unix":"cp ${FILE_NAME}.go ~/.quicktest/go_mod/main.go && ${PROGRAM} build -buildmode=exe -o ./.qt/${FILE_NAME_BINARY}.o ~/.quicktest/go_mod/main.go",
                "windows":"cp ${FILE_NAME}.go ~/.quicktest/go_mod/main.go && ${PROGRAM} build -buildmode=exe -o ./.qt/${FILE_NAME_BINARY}.exe ~/.quicktest/go_mod/main.go"
            },
            "execute":{
                "unix":".qt/${FILE_NAME_BINARY}.o",
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
```