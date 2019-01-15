### The challenge
So, my first encounter with Rust is brutal. As soon as I have learned my first baby steps, and try to walk, I fall down.

I want to create the easiest application that lets two worker objects use the same "stamping machine" to count the total number of worked hours.
#### Let's get rolling
_stamping_clock.rs_
```Rust
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
```

#### With this main:
main.rs
```Rust
mod worker;
mod stamping_clock;
use crate::stamping_clock::StampingClock;

fn main() {
    let mut sc=StampingClock::new();
    sc.stamp(10);
    println!("Totally stamped {}",sc.total());
}
```
Result:
```
Totally stamped 10
```
### Let's up the game - add a worker
Ok - since we only want one worker, this shouldn't be too hard, should it?
So we start out with high spirit:
_worker.rs_
```Rust
use crate::stamping_clock::StampingClock;

pub struct Worker {
    pub stamping_clock: StampingClock,
}

impl Worker {
    pub fn new(stamping_clock: StampingClock) -> Worker {
        Worker {
            stamping_clock: stamping_clock
        }
    }

    pub fn stamp(&mut self, hours :u32) {
        self.stamping_clock.stamp(hours);
    }
}
```
It looks fine - no warnings or compiler errors.

Then we need to change main.rs to this:
```Rust
mod worker;
mod stamping_clock;
use crate::stamping_clock::StampingClock;
use crate::worker::Worker;

fn main() {
    let sc=StampingClock::new();
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
### Ok, let's try to borrow the sc to the worker
We change the types of the StampingClocks variables to support borrowing:
worker.rs:
```Rust
use crate::stamping_clock::StampingClock;

pub struct Worker {
    pub stamping_clock: &StampingClock,
}

impl Worker {
    pub fn new(stamping_clock: &StampingClock) -> Worker {
        Worker {
            stamping_clock: stamping_clock
        }
    }

    pub fn stamp(&mut self, hours :u32) {
        self.stamping_clock.stamp(hours);
    }
}
```
Here we get a message indicating that the lifetime has to be specified (with a hint that 'static is required)

Main.rs now looks like this (note the &sc)
```Rust
mod worker;
mod stamping_clock;
use crate::stamping_clock::StampingClock;
use crate::worker::Worker;

fn main() {
    let sc=StampingClock::new();
    let mut w1=Worker::new(&sc);
    w1.stamp(10);
    w1.stamp(5);

    println!("Totally stamped {}",sc.total());
}
```
And as such, the error is sort of fixed (sc.total() is executable - since it is not moved in the construction of the worker)

But the worker does not compile - no lifetime...
### Ok - let's add the _'static_ lifetime
Worker.rs changed to this:
```Rust
use crate::stamping_clock::StampingClock;

pub struct Worker {
    pub stamping_clock: &'static mut StampingClock,
}

impl Worker {
    pub fn new(stamping_clock: &'static mut StampingClock) -> Worker {
        Worker {
            stamping_clock: stamping_clock
        }
    }

    pub fn stamp(&mut self, hours :u32) {
        self.stamping_clock.stamp(hours);
    }
}
```
This compiles - note the **'static** and **mut** - now we borrow a value with static lifetime (lives forever) and that is changeable.

Next, the main.rs ... optimistic still?
```Rust
mod worker;
mod stamping_clock;
use crate::stamping_clock::StampingClock;
use crate::worker::Worker;
static mut SC :StampingClock = StampingClock::new();

fn main() {
    let mut w1=Worker::new(&mut SC);
    w1.stamp(10);
    w1.stamp(5);

    println!("Totally stamped {}",SC.total());
}
```
Of course this fails miserably. It was obvious that the SC had to be moved out to be a static variable, also we need to pass it in as a mutable borrow, according to the new signature of the worker.

Here we get the message more or less that everything we do is unsafe - and we need to put it into an "unsafe { }"-block - and it sort of feels like we are not really moving along the right path.

Also, we are not allowed to new up the clock like that. Since the StampingClock is really simple at the moment we can skip the new() and create it ourselves changing the line to this:
```Rust
static mut SC :StampingClock = StampingClock { total: 0};
```
We also have to make the _total_-field public - so we violate the whole StampingClock-struct by making its internal field public, and re-implementing the new()-method in the static-declaration.

We add in a second worker, so the main.rs looks like this:
```Rust
mod worker;
mod stamping_clock;
use crate::stamping_clock::StampingClock;
use crate::worker::Worker;
static mut SC :StampingClock = StampingClock { total: 0};

fn main() {
    unsafe {
        let mut w1=Worker::new(&mut SC);
        let mut w2=Worker::new(&mut SC);
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