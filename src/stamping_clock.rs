pub struct StampingClock {
    total :u32
}

impl StampingClock {
    pub fn new() -> StampingClock {
        StampingClock {
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