# Releases

Binary releases can be downloaded manually at:
https://github.com/LuisMBaezCo/quicktest/releases

### 1.0.0-rc.2 / 2022.05.12

feat: subcommand folder was renamed to controllers [2d5f859]
feat: prefix option to run test cases matching that prefix was added to the cmp, stress and check  subcommands [545213c]
refactor: cli module structures was updated to model and Adapter Command  trait was added [48ee06d]
refactor: check command was refactored using adapter pattern [b99c8e9]
refactor: cmp command was refactored using adapter pattern [cbae9bb]
refactor: stress command was refactored using adapter pattern [f7beebc]
refactor: output command was refactored using adapter pattern [e4cfefc]

### 1.0.0-rc.1 / 2022.05.11

- feat: documentation website using docusaurus was initialized [3c240b9]
- feat: report table was added using ascii style [4923d80]
- feat: short code examples were added for cmp, stress and check subcommands [08f4cff]
- feat: examples were added for run and setup subcommands #55 [062c3f7]
- feat: Alias of timeout and test-cases options were updated [4d2b6a8, 3559c54]
- feat: show test case input was added in diff [bd366a2, 561590e]
- feat: mle support was added to the cmp subcommand [2854cc1]
- feat: mle support was added to the stress subcommand [c7ccecd]
- feat: mle support was added to the check subcommand [3c74a6c]
- feat: mle support was added to the run subcommand [84bd676]
- feat: mle status added to report table [2c9065a]
- feat: flag memory_limit was added to cmp, stress, check and run subcommands [0553b9f]
- feat: label reporting the location of the configuration file was added [878302f]
- feat: library attribute was removed from configuration json [d2954b3]
- feat: status code for cmp subcommand were added [696e7c5]
- feat: status code for stress subcommand were added [c765f9c]
- feat: status code for check subcommand were added [ecf8b79]
- feat: `run` subcommand was renamed to `output` [a73b75e]
- fix: text view was fixed [eba2bcb]
- fix: issue about run_mle was fixed in cmp subcommand [79582c8]
- docs: initial documentation was added [61dc8e5]
- docs: cmp examples were updated [1bf9f0f]

### 0.10.0 / 2022.04.27

- feat: rust version was updated to 1.58.0 and process_controll version was updated [1e7dc52]
- refactor: painter was renamed to views [b3c9bf8]
- feat: command was updated to #45 [e04998f]

### 0.9.2 / 2022.04.21

- fixed: issue about No such file or directory v2 [8697c01]

### 0.9.1 / 2022.04.21

- fixed: issue about No such file or directory [2b4ddda]

### 0.9.0 / 2022.04.20

- feat: Added build functionality via a config file [5131600]
- feat: tle, cmp, check and run subcommands were updated to use the LanguageHandler trait [fa3c6b1, 2030aea, 52984ac, 77d10c3]
- feat: show help setup after help dynamically [683b0e8]
- feat: Setup subcommand was updated for new languages.config.json structure [496e1dc, 0f3aa8f]
- refactory: old cpp and python structures were removed [13aa497]
- refactory: is_extension_supported was updated [4e1bf15]
- refactory: yaml configuration was deleted [d2a39eb]
- refactory: The parameters of the run functions of the subcommands were refactored. [f53f1d0]
- docs: comments about help setup after help was added [e9cb0ca]
- docs: errors mapping was initialized [44dd66d]
- docs: docs about QTEST_COMPILER_ERROR and QTEST_RUNTIME_ERROR were  added [8c82da7, 0de6b85]
- docs: mapping error was updated [a9ca52d, b90c8af, 93e866e]
- docs: docs about cmp --diff and setup config subcommand were updated [22e7df0]
- issue: issue about binary path was fixed [316079c]
- issue: issue about python3 was fixed [ae55054]
- issue: issue about RTE was fixed [f046f4d]
- fix: issue about circleci config file was fixed [65855ca]

### 0.8.0 / 2022.04.11

- feat: difference feature was added to cmp subcommand [a3c1612]
    - `--diff` flag was added to `quicktest cmp`
- feat: line by line difference function was added [6540719]
- feat: diff colors was change to bright_ [c33190f]

### 0.7.0 / 2022.01.06

- feat: config schema and load_default_config were added [ae89f0b]
- feat: read and configure settings from file [5928071]
- feat: default configuration was updated from lazy_static to Default trait [58c7616]
- feat: setup subcommand for cpp and python was added #51 [5752eb0]
- test: test about setup command was added #51 [9ef7b9b]
- issue: issue about setup command test 'cmd_setup_python_program' was fixed [ede3f94]
- feat: default configuration for cpp and python was updated dinamically using ~/.quicktest/config.yaml file #51 [c049c1b]
docs: docs about setup command was added #51 [e4a51ae]
- feat: rust version was updated to 1.57.0 and cargo clippy was fixed [607168f]
- feat: support of setup subcommand for c++ and python flags was added [abc68e5]
- docs: examples about setup command were added [5410b02]
- ci: set python3 program was added to .circleci/config.yml [b9a359a]
- feat: cpp and python flags were support by setup command #51 [3cb99c3]
- test: tests about setup command for flags were added [ffe9821]
- fix: clippy errors were fixed [d1da0a6]
- docs: examples about flags in setup command were added [3565fbc]

### 0.6.0 / 2021.12.16

- feat: run command was added [9868f9c]
- feat: run command structopt:enum was added [9a5357c]
- feat: Check if the CACHE_FOLDER folder is already created was added to run command [97ab226]
- fix: load_testcases_from_prefix was updated for fixed prefix error [0bee037]
- docs: examples for run command was added [272807e]
- feat: save output of run command was added [4af28f3]
- feat: run command flag was updated from --save to --save-out [e79a1f5]
- docs: run command docs was updated [a2983a3]
- fix: issue about clippy was fixed [2fcb90c]
- test: test about run command was added [08942c3]
- test: circleci config was updated: run_subcommand test was added [f80ad33]
- test: cpp.rs file was renamed to common.rs [62c9a28]
- fix: circleci config was fixed [bab6856, 31f6596]
- docs: docs about run command was added to main README [74bccd9]
- docs: RELEASES.md file was added [edf4711, e31954c]

### 0.5.3 / 2021.12.03

- feat: handle errors about not extension sopported was added [b66222d]
- feat: error handle that verify that a lang programs it's installed was added [58d3d51]
- issue: issue about emoji in powershell and windows cmd was fixed #44 [67535fa, c962962]
- feat: version of dependencies was updated [a25e802, 7c01b59]

### 0.5.2 / 2021.10.30

- Logo was added [bfdac4d]
- issue: colors in the windows terminal was fixed #39 [09c3a46]
- feat: logo's reference was added #41 [0fa662a]
- issue: Warnings generated by were fixed #40 [e758c98]
- docs: CODE_OF_CONDUCT.md was added [b90c4af]

### 0.5.1 / 2021.06.22

- refactor: test about tle subcommand was modularized [7d5062b]
- refactor: test about cmp subcommand was modularized [c252289]
- refactor: header was added to tle test files [3361df4]
- refactor: test about check subcommand was modularized [6d473bf]
- feat: seed and the number of the test case was passed as an argument … …[5752547]
- docs: command description was added[f9e4d0d]
- docs: command description was added II [c044901]
- docs: command description was added III [35eb29f]
- docs: examples folder with examples about cmp, tle and check subcomma… … [577bac7]
- docs: examples about tle subcommand using cpp and python were added [3acb53a]
- style: the code of the whole project was formatted using fmt style [d070f05]
- docs: examples about cmp subcommand in cpp and python were added [bb37320]
- style: command style were updated [52e4e38]
- docs: example about check subcommand were added [8c844b9]
- release: v0.5.1 [9c58559]

### 0.5.0 / 2021.06.20

- feat: feature about run cases in /test_cases folder was added for tle subcommand [4201511]
- feat: flag for run test_cases about cmp and check subcommands were implemented [1b509a0]
- feat: example subcommand was implemented [c0032e5]