fn main() {
    println!("Hello, world!");
}

/*

-> A reference, like a pointer, is an address used to access data stored
at that address.

-> Data races occur in the following situations:
1) Two or more simultaneous pointers access the same data at the same time.
2) At least one of the pointers is being used to write to the data
3) There's no mechanism being used to synchronize access to the data.

-> Rust prevents data races at compile time by disallowing multiple simultaneous
mutable references and a combinatation of simultaneous mutable and immutable 
references to the same data

-> A reference's scope begins from where it is introduced and continues through
the last time that reference is used.

PLEASE LOOK INTO THIS

-> Non-Lexical Lifetimes (NLL) - the ability of the compiler to tell that a reference
is no longer being used at a point before the end of the scope.

-> Dangling reference - a pointer that references a location in memory that may
have been given to another entity - by freeing some memory while preserving a pointer
to that memory.

To prevent dangling references, the rust compiler will ensure that the data will not
go out of scope before the reference to the data does.

*/