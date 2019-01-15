### The challenge
So, my first encounter with Rust is brutal. As soon as I have learned my first baby steps, and try to walk, I fall down.

I want to create the easiest application that lets two worker objects use the same "punch machine" to count the total number of worked hours.
#### Let's get rolling
_punch_clock.rs_
```Rust
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
```

#### With this main:
main.rs
```Rust
mod worker;
mod punch_clock;
use crate::punch_clock::PunchClock;

fn main() {
    let mut sc=PunchClock::new();
    sc.stamp(10);
    println!("Totally stamped {}",sc.total());
}
```
Result:
```
Totally stamped 10
```
#### Let's up the game - add a worker
Ok - since we only want one worker, this shouldn't be too hard, should it?
So we start out with high spirit:
_worker.rs_
```Rust
use crate::punch_clock::PunchClock;

pub struct Worker {
    pub punch_clock: PunchClock,
}

impl Worker {
    pub fn new(punch_clock: PunchClock) -> Worker {
        Worker {
            punch_clock
        }
    }

    pub fn stamp(&mut self, hours :u32) {
        self.punch_clock.stamp(hours);
    }
}
```
It looks fine - no warnings or compiler errors.

Then we need to change main.rs to this:
```Rust
mod worker;
mod punch_clock;
use crate::punch_clock::PunchClock;
use crate::worker::Worker;

fn main() {
    let sc=PunchClock::new();
    let mut w1=Worker::new(sc);
    w1.stamp(10);
    w1.stamp(5);

    println!("Totally stamped {}",sc.total());
}
```
This code crashes because the extraction of the total from sc is not valid, since we moved the sc into the worker, making sc invalid after the line 
```Rust
    let mut w1=Worker::new(sc);
```
#### Ok, let's try to borrow the sc to the worker
We change the types of the PunchClocks variables to support borrowing:
worker.rs:
```Rust
use crate::punch_clock::PunchClock;

pub struct Worker {
    pub punch_clock: &PunchClock,
}

impl Worker {
    pub fn new(punch_clock: &PunchClock) -> Worker {
        Worker {
            punch_clock
        }
    }

    pub fn stamp(&mut self, hours :u32) {
        self.punch_clock.stamp(hours);
    }
}
```
Here we get a message indicating that the lifetime has to be specified (with a hint that 'static is required)

Main.rs now looks like this (note the &sc)
```Rust
mod worker;
mod punch_clock;
use crate::punch_clock::PunchClock;
use crate::worker::Worker;

fn main() {
    let sc=PunchClock::new();
    let mut w1=Worker::new(&sc);
    w1.stamp(10);
    w1.stamp(5);

    println!("Totally stamped {}",sc.total());
}
```
And as such, the error is sort of fixed (sc.total() is executable - since it is not moved in the construction of the worker)

But the worker does not compile - no lifetime...
#### Ok - let's add the _'static_ lifetime
Worker.rs changed to this:
```Rust
use crate::punch_clock::PunchClock;

pub struct Worker {
    pub punch_clock: &'static mut PunchClock,
}

impl Worker {
    pub fn new(punch_clock: &'static mut PunchClock) -> Worker {
        Worker {
            punch_clock
        }
    }

    pub fn stamp(&mut self, hours :u32) {
        self.punch_clock.stamp(hours);
    }
}
```
This compiles - note the **'static** and **mut** - now we borrow a value with static lifetime (lives forever) and that is changeable.

Next, the main.rs ... optimistic still?
```Rust
mod worker;
mod punch_clock;
use crate::punch_clock::PunchClock;
use crate::worker::Worker;
static mut PC :PunchClock = PunchClock::new();

fn main() {
    let mut w1=Worker::new(&mut PC);
    w1.stamp(10);
    w1.stamp(5);

    println!("Totally stamped {}",PC.total());
}
```
Of course this fails miserably. It was obvious that the SC had to be moved out to be a static variable, also we need to pass it in as a mutable borrow, according to the new signature of the worker.

Here we get the message more or less that everything we do is unsafe - and we need to put it into an "unsafe { }"-block - and it sort of feels like we are not really moving along the right path.

Also, we are not allowed to new up the clock like that. Since the PunchClock is really simple at the moment we can skip the new() and create it ourselves changing the line to this:
```Rust
static mut SC :PunchClock = PunchClock { total: 0};
```
We also have to make the _total_-field public - so we violate the whole PunchClock-struct by making its internal field public, and re-implementing the new()-method in the static-declaration.

We add in a second worker, so the main.rs looks like this:
```Rust
mod worker;
mod punch_clock;
use crate::punch_clock::PunchClock;
use crate::worker::Worker;
static mut PC :PunchClock = PunchClock { total: 0};

fn main() {
    unsafe {
        let mut w1=Worker::new(&mut PC);
        let mut w2=Worker::new(&mut PC);
        w1.stamp(10);
        w1.stamp(5);
        w2.stamp(10);
    
        println!("Totally stamped {}",SC.total());
    }
}
```
We finally got a working application, but it has a bitter taste.
```
Totally stamped 25
```

Adding a bit decency back - but still totally unsafe these are the three files (Using Option<> for making it decent)

main.rs
```Rust
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
            Some(pc) => println!("Totally stamped {}",pc.total()),
            None => ()
        }
    }
}
```

worker.rs
```Rust
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
            Some(pc) => pc.stamp(hours),
            None => ()
        }
    }
}
```

and punch_clock.rs
```Rust
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
```