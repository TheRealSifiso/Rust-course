fn main() {

    <Dog as Animal>::scream();

}

struct Dog {}

trait Animal {
    fn scream();
}

impl Animal for Dog {
    fn scream() {
        
    }
}

/*

Conservativeness:
    $ Much of Rust's safety comes from compile-time checks, thus, Rust's
        memory safety guarantees are enforced at compile time. This is
        known as static analysis which is conservative.

    $ Since static analysis is conservative, if the Rust compiler does
        not have enough information to be confident about whether or not
        code upholds Rust's memory safety guarantees, it will reject code
        regardless of whether it is valid or invalid.

Unsafe Rust:
    $ Given that the underlying computer hardware is inherently unsafe,
        unsafe operations in Rust enable you to do low-level systems
        programming such as writing your own operating system.

    $ Unsafe superpowers:
        1) Dereference a raw pointer
        2) Call an unsafe function or method
        3) Access or modify a mutable static variable
        4) Implement an unsafe trait
        5) Access fields of *union*s

    $ You may wrap unsafe code within a safe abstraction and provide a
        safe API.

    $ Raw pointers:
        1) Immutable raw pointer: *const T
        2) Mutable raw pointer: *mut T

        A mutable reference is a borrow to any type mut T, allowing
        mutation of T through that reference.

        BUT,

        In the context of raw pointers, Immutable means that the pointer
        can't be directly assigned to after being dereferenced.

    $ Unlike references and smart pointers, raw pointers:

        1) are allowed to ignore the borrowing rules by having both
            immutable and mutable pointers or multiple mutable pointers
            to the same location.

        2) aren't guaranteed to point to valid memory

        3) are allowed to be null

        4) don't implement any automatic cleanup

    $ Creation of raw pointers in safe code is allowed. However,
        dereferencing raw pointers outside an unsafe block is not allowed

    

*/