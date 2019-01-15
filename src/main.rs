mod worker;
mod punch_clock;
use crate::punch_clock::PunchClock;
use crate::worker::Worker;
static mut PC :Option<PunchClock> = None;

fn main() {
    unsafe {
        PC=Some(PunchClock::new());
        let mut w1=Worker::new(&mut PC);
        let mut w2=Worker::new(&mut PC);
        w1.stamp(10);
        w1.stamp(5);
        w2.stamp(10);
        match &PC {
            Some(pc) => println!("Totally punched {}",pc.total()),
            None => ()
        }
    }
}

