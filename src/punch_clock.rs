pub struct PunchClock {
    total :u32
}

impl PunchClock {
    pub fn new() -> PunchClock {
        PunchClock {
            total: 0
        }
    }

    pub fn stamp(&mut self, hours :u32) {
        self.total+=hours;
    }

    pub fn total(&self) -> u32 {
        self.total
    }
}