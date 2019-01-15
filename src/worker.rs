use crate::stamping_clock::StampingClock;

pub struct Worker {
    pub stamping_clock: &'static mut Option<StampingClock>,
}

impl Worker {
    pub fn new(stamping_clock: &'static mut Option<StampingClock>) -> Worker {
        Worker {
            stamping_clock: stamping_clock
        }
    }

    pub fn stamp(&mut self, hours :u32) {
        match self.stamping_clock {
            Some(sc) => sc.stamp(hours),
            None => ()
        }
    }
}
