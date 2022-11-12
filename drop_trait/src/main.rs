fn main() {
    println!("Hello, world!");
}

/*

Drop trait:
    $ used to customize what happens when value is about to go out of
        scope.

    $ given that we implement the Drop trait on our own type, Rust
        automatically calls drop() when instances of our type go out of
        scope, in reverse order of their creation.

        The drop() function is called a *destructor* (the opposite of a
        constructor). Dectructors clean up instances; whereas,
        constructors create instances.

    $ We are not allowed to call the Drop trait's drop() method
        explicitly! Instead, we must use the
            std::mem::drop
        function, which takes, as an argument, the value we want to drop.

        Although the Drop trait's drop method is called automatically when
        a value goes out of scope, the ownership systems ensures that drop()
        is only called once to prevent "Double Free error".


*/