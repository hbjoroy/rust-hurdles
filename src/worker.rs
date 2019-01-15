use crate::punch_clock::PunchClock;

pub struct Worker {
    pub punch_clock: &'static mut Option<PunchClock>,
}

impl Worker {
    pub fn new(punch_clock: &'static mut Option<PunchClock>) -> Worker {
        Worker {
            punch_clock
        }
    }

    pub fn stamp(&mut self, hours :u32) {
        match self.punch_clock {
            Some(sc) => sc.stamp(hours),
            None => ()
        }
    }
}
