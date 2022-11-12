use std::fs::File;
use std::io::ErrorKind;

fn main() {

    //The following returns Result<T, E>
    let greeting_file_result = File::open("hello.txt");

    //#[warn(unused_variables)]
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        
        //There many possible errors that need to be dealt with!
        // .kind() method returns the ErrorKind enum where ever variant
        //is a possible 'error'.
        Err(error) => match error.kind() {
            //NotFound is not brought into scope by the prelude
            ErrorKind::NotFound => match File::create("hello.txt") {
                //If a file is successully created, we return its handle
                Ok(fc) => {fc},
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },

            other_error => panic!("Problem opening the file {:?}", other_error),
        }
    };

}

/*

-> Recoverable and unrecoverable errors

-> Rust doesn't have exceptions, but Result<T, E> for recoverable 
errors.

-> panic! macro stops execution - unrecoverable errors

-> When a panic occurs, the programs unwinds and walks back up the stack
to clean up the data in memory. The alternative to unwinding is 'aborting'
which leaves the cleanup procedure to be handled by the operating system.

-> Backtrace can be used to find the source of the panic

-> Buffer overread

-> RUST_BACKTRACE environment variable

*/





/*

-> enum Result<T, E> {
    Ok(T),
    Err(E),
    }

-> T: std::fs::File (file handle)
-> E: std::io::Error

-> Variants of the Result enum are brought into scope by the prelude
    - as is the case with the Option enum.


*/



/*

-> unwrap_or_else():
    an alternative to using Result<T, E>
    this method makes use of closure

    e.g.

    File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            //do something
        }
    });

    In the success case, method returns file handle

*/

/*

-> unwrap() returns file handle or panics

-> expect() does the same thing except that

-> Error Propagation - instead of panicking, return Err(e)
    e.g

    use std::fs::File;

    fn open_file() -> Result(File, io::Error){

        match File::open("hello.txt") {
            Ok(file) => Ok(file),
            Err(e) => Err(e)
        }

    }

-> read_to_string() - return Result; takes a mutable reference to a String,
reads the content of a file handle, passes all the contents onto the
String.

    read_to_string(&mut string).expect("Error!!!!")
    read_to_string(&mut string)?

-> ? operator for propagating errors

*/




/*

-> ? operator can only be within the context of a function that returns
a Result or an Option enum. In the Ok() or Some() case, the value inside
the variant is returned; otherwise, Err(e) or None is returned

-> By default, main() returns () unit-type.
    main() can return Result<(), E)

*/