/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021-present / Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use std::collections::BTreeMap;

pub trait BuildEnvVariables {
    fn build_env_variables(&mut self, env: &BTreeMap<String, String>);
}
