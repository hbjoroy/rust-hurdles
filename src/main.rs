mod worker;
mod stamping_clock;
use crate::stamping_clock::StampingClock;
use crate::worker::Worker;
static mut SC :Option<StampingClock> = None;

fn main() {
    unsafe {
        SC=Some(StampingClock::new());
        let mut w1=Worker::new(&mut SC);
        let mut w2=Worker::new(&mut SC);
        w1.stamp(10);
        w1.stamp(5);
        w2.stamp(10);
        match &SC {
            Some(sc) => println!("Totally stamped {}",sc.total()),
            None => ()
        }
    }
}

