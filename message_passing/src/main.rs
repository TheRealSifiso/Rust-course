use std::sync::mpsc;
use std::time::Duration;
use std::thread;
fn main() {

    let (tx, rx) = mpsc::channel();

    let tx_clone = tx.clone();

    thread::spawn(move ||{
        let vector = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for item in vector {
            tx.send(item).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vector = vec![
            String::from("More"),
            String::from("messages"),
            String::from("for"),
            String::from("you")
        ];

        for item in vector{
            tx_clone.send(item).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

}

/*

Message passing:
    $ a concept whereby data is transferred between threads

    $ mpsc::channel() gives us the ability to have multiple transmitters
        (or producers) and one receiver (a consumer)

            mpsc::channel() returns (tx, rx)

    $ We acheive the ability to have multiple producers by "cloning" tx.

    $ tx.send(something) returns Result<T, E>
        if rx has already been dropped, Err() is returned

        calling unwrap() will automatically a panic in case of an error

    $ rev() and try_recv() are two useful methods available on the
        receiver:

        recv() - 1) returns Result<T, E>;
            2) blocks the current (main) thread's execution and waits until
            a value is sent down the channel;
            3) return Err() when the transmitter closes

        try _recv() - 1) returns Result<T, E> immediately!
            2) Ok(message) if a message is available, or
                Err() is if no message is available yet.

    $ We may index into 'rx' without the need to explicitly call recv()
        or try_recv():

            for received in rx {
                println!("Got: {}", received);
            }

*/