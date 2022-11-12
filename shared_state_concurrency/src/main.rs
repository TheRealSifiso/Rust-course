use std::sync::{Arc, Mutex};
use std::thread;

fn main() {

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Result: {}",
        *counter.lock().unwrap()
    );

}

/*

Mutex<T>
    $ Mutual exclusion
    
    $ A common concurrency primitive for shared memory.

    $ A call to Mutex::new(data) creates a new mutual exclusion.

    $ To access the data inside a mutex, we must first acquire the lock
        - which keeps track of who currently has exlusive access to the
        data - using the lock() method.

        The lock() method returns a smart pointer called MutexGuard,
        wrapped in a LockResult.

        Acquiring the lock() blocks the current thread until a lock is
        acquired.

Multiple ownership of mutexes across threads:
    $ The challenge with implementing shared-state concurrency is
        ensuring coordinated access to the same shared data by multiple
        threads. Rust's type system and onwership rules work to ensure
        that shared-state concurrency is performed in a thread-safe way.

    $ By wrapping Mutex<T> in Arc<T>, we can enable shared ownership of 
        mutexes across threads in a safe manner!

    $ Instead of Rc<T>, we use atomics (Arc<T>) because an Rc<T> type
        cannot be shared between threads safely. Rc<T> doesn't use
        concurrency primitives to make sure that changes to the
        reference count cannot be interrupted by another thread

Mutex<T> - interior mutability:
    $ Mutex<T> provides interior mutability in the same way RefCell<T>
        does.

    $ Mutex<T> is a smart pointer!

MutexGuard<T>
    $ The drop implementation of MutexGaurd releases a lock automically
        when a MutexGuard goes out of scope.

    $ The deref implementation of MutexGuard enables us to treat
        MutexGuard as a regular referencence.

*/