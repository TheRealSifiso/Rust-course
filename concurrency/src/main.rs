use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{} - spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    //--------------------------------------------------------

    for i in 1..5 {
        println!("{} - main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}

/*

Message-passing concurrency:
    $ where channels send messages between threads



Shared-state concurrency:
    $ where multiple threads have access to some piece of data

Issues pertaining to multi-threaded code:
    $ There is no guarantee on the order in which threads run

    $ Race conditions

    $ Deadlocks

Multi-threading:
    $ thread::spawn(|| {}) - spawns a new thread

    $ thread::sleep() - to force a thread to stop its execution for
        a short duration.


Fearless Concurrency:
    $ when the main thread of a Rust program completes execution,
        all spawned threads are shut down, sometimes prematurely.

        We solve this problem using Join Handles:
            
            thread::spawn returns JoinHandle

            calling join() on JoinHandle will ensure a spawned thread
            finishes execution.

            thread::spawn(||{}).join().unwrap();

    $ move keyword - transfer ownership of values from one thread to
        another.

    $
*/