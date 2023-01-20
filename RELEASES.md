# Releases

Binary releases can be downloaded manually at:
https://github.com/LuchoBazz/quicktest/releases

### 1.0.1 / 2022.11.12

- fix: issue about .memory_limit method not found on Mac OS' was fixed #92
- chore: rust version was updated to v1.60.0
- fix: --tc flag was fixed of cmp docs
- feat: support for mac installation using curl was added
- docs: installation for mac os was supported
- feat: CI only for main, feature and develop branches was added

### 1.0.0 / 2022.05.30

- feat: report table was added using ascii style [4923d80]
- feat: short code examples were added for cmp, stress and check subcommands [08f4cff]
- feat: examples were added for run and setup subcommands #55 [062c3f7]
- feat: Alias of timeout and test-cases options were updated [4d2b6a8, 3559c54]
- feat: show test case input was added in diff [bd366a2, 561590e]
- feat: mle support was added to the cmp, stress, check, run subcommands [2854cc1, c7ccecd, 3c74a6c, 84bd676]
- feat: run subcommand was renamed to output [a73b75e]
- feat: prefix option to run test cases matching that prefix was added to the cmp, stress and check subcommands [545213c]
- feat: Support for rust lang was added [e2be5c9]
- feat: Go lang support was added [4191367]
- feat: java support was added [496f70e]
- feat: C support was added [8f8f22d]
- feat: katlin support was added [16605df]
- feat: installation script via powershell for windows was added [1f7bbde, c604407]
- feat: installation script via shell for linux was added [7220318]
- docs: installation commands on Linux, Windows and macOS were added [d7a6cb0]
- docs: demo of the cmp subcommand was added to the website [e148fb1, 89d06c5, 585def3]]

### 1.0.0-rc.6 / 2022.05.27

- feat: cargo build --release [2293bb1]
- feat: c examples for the cmp command were added [09dfb98]
- feat: rust examples for the cmp command were added [8ff4281]
- feat: java examples for the cmp command were added [89b776c]
- feat: go examples for the cmp command were added [fd604f7]
- feat: kotlin examples for the cmp command were added [85e2c31]
- feat: circleci config for java and go was added [9b2b49c]
- fix: small errors in the documentation was fixed [91a98fe]
- fix: go configuration was fixed [ee9b02b]
- fix: issue about cp and mkdir not found was fixed [5e566b0]
- fix: code smell was fixed [1b64942]
- fix: status variable was renamed [25607b9]
- fix: issue about configuration_commands was fixed [c479316]
- test: test about cmp and output subcommand of c was added [f66eb64]
- test: test cmp subcommand in rust was added II [5f8aeef]
- test: test about cmp and output subcommand in java were added [78fd0da]
- test: test about cmp subcommand in kotlin was added [cdefaaa]
- refactor: extension .o was removed from the unix binaries [9bf6bc1]
- refactor: cpp and python unit tests were refactored [d0ab909]
- docs: intro was updated [531cd51]
- docs: docs/errors was deleted [6c32225]
- docs: documentation icon was updated in the README [9d23ace, 1819dc1, 22cb71d, 4a8a0b7]
- docs: contributing file updated was [de5dc14]
- circleci config was configured [719fb5b]

### 1.0.0-rc.5 /  2022.05.18

- feat: installation script via powershell for windows was added [1f7bbde]
- feat: installation script via shell for linux was added [7220318]
- feat: installation script via powershell for windows was updated [c604407]
- docs: installation commands on Linux, Windows and macOS were added [d7a6cb0]
- docs: installation guide was updated in the README.md [6832582]

### 1.0.0-rc.4 / 2022.05.16

- docs: demo of the cmp subcommand was added to the website [e148fb1]
- docs: demo of the stress subcommand was added to the website [89d06c5]
- docs: demo of check, eample, output and setup subcommands were added to the website [585def3]
- docs: Quick Test CLI Error Mapping was added [6d61bb2]
- docs: command documentation was updated [e9cfa72]
- docs: gif of qt cmp was placed at the beginning of the documentation [e18bd19]
- docs: gifs were updated [43b6196]
- docs: contribution guide was added [61cc8c7]
- docs: How can you contribute? section and future updates were added [138c82a]
- docs: main and check subcommand demo was added [33f9e70]
- docs: README.md was updated with gif [8132666]
- docs: header info was added to the missing files [b60d185]
- refactor: hidden cache folder .qtest was renamed to .qt [f64881f]
- fix: issue about es_status undefined variable in windows was fixed [0bc1a95]
- fix: issue in the contribution docs was fixed [6066b2a]
- fix: code smell were fixed [d5f9c76]
- fix: command docs was fixed [8445068]
- feat: ~/.quicktest/languages.config.json file was updated in website [a6734dd]

### 1.0.0-rc.3 / 2022.05.14

- feat: Support for rust lang was added [e2be5c9]
- feat: Go lang support was added [4191367]
- feat: java support was added [496f70e]
- feat: C support was added [8f8f22d]
- feat: katlin support was added [16605df]
- feat: configuration file was updated [292630a]
- feat: qt setup reset which removes the config file was added [e5eba42]
- feat: I/O files were renamed to a shorter name [bc76ac7]
- fix: issue about text that appears when executing the check installed command was fixed [eda4369]
- docs: documentation of the examples was added in /examples folder [58427eb]
- docs: documentation of examples was added to the web page [cf45782]
- docs: documentation about the subcommands was updated on the web page [968a68a]
- docs: config file documentation was added [4868fbf]
- assets: favicon.ico and logo were added to website [decaa0d]
- examples: rust examples were added [7ef38e3]
- test: test names were changed from tle to stress [703cb87]


### 1.0.0-rc.2 / 2022.05.12

- feat: subcommand folder was renamed to controllers [2d5f859]
- feat: prefix option to run test cases matching that prefix was added to the cmp, stress and check  subcommands [545213c]
- refactor: cli module structures was updated to model and Adapter Command  trait was added [48ee06d]
- refactor: check command was refactored using adapter pattern [b99c8e9]
- refactor: cmp command was refactored using adapter pattern [cbae9bb]
- refactor: stress command was refactored using adapter pattern [f7beebc]
- refactor: output command was refactored using adapter pattern [e4cfefc]

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