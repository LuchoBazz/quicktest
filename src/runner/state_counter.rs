pub struct StateCounter {
    pub tle: u32,
    pub rte: u32,
    pub ac: u32,
    pub mle: u32,
    pub wa: u32,
}

impl StateCounter {
    pub fn new() -> Self {
        Self {
            tle: 0,
            rte: 0,
            ac: 0,
            mle: 0,
            wa: 0,
        }
    }

    pub fn increase_tle(&mut self) {
        self.tle += 1;
    }

    pub fn increase_rte(&mut self) {
        self.rte += 1;
    }

    pub fn increase_ac(&mut self) {
        self.ac += 1;
    }

    pub fn increase_mle(&mut self) {
        self.mle += 1;
    }

    pub fn increase_wa(&mut self) {
        self.wa += 1;
    }

    pub fn has_stress_command_error(&self) -> bool {
        (self.tle + self.rte + self.mle) > 0
    }

    pub fn has_cmp_command_error(&self) -> bool {
        (self.wa + self.tle + self.rte + self.mle) > 0
    }

    pub fn has_checker_command_error(&self) -> bool {
        (self.wa + self.tle + self.rte + self.mle) > 0
    }
}

impl Default for StateCounter {
    fn default() -> Self {
        Self::new()
    }
}
