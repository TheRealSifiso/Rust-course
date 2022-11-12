fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    //borrows_mutably();
    println!("After calling closure: {:?}", list);
}


/*

Closures:
    1) anonymous functions with anonymous types
    2) can be saved in variables or passed as arguments to other functions
    3) capture values from the scope in which they're defined implicitly in three ways:
        i) borrowing immutably
        ii) borrowing mutably - closure variable must be declared as mutable
        iii) taking ownership - in two ways:
                1) assign the captured value to a new variable
                2)  especially for copy types, use the "mov" keyword
                        "moved" variables cannot be used elsewhere
    4) type annotations are usually not required;
    5) closures are dropped only by closure calls e.g. closure();
    6) the true power of closures lies in the ability to pass them as arguments
        to other functions.

    -> The compiler infers one concrete type for the return value and each 
    of the paramaters of a closure definition. 
    
        What this means is that if you assign a closure to another 
        variable but use an argument of a different type from the previous,
        the code will not compile!

        let example_closure = |x| x;

        let s = example_closure(String::from("hello"));
        let n = example_closure(5); //this step is invalid!

    -> For a mutable borrow in a closure to end, the closure must first be called.
        Thus, you are not allowed to have an immutable reference between a closure
        definition and a closure call:

            fn main() {
                let mut list = vec![1, 2, 3];
                println!("Before defining closure: {:?}", list);

                let mut borrows_mutably = || list.push(7);

                borrows_mutably();
                println!("After calling closure: {:?}", list);
            }

    -> *move* keyword:
            to force a closure to take ownership of the values it captures
            from the environment.

            *Look at closures in the context of thread

    -> The way a closure captures and handles values from the environment
        affects which traits the closure implements.

        Closure will automatically implement atleast one of three *Fn traits*,
        in additive fashion, depending on how the closure's body handles the captures
        values in its environment:

            1) FnOnce - Implemented by every closure;
                applies to closures that can be called once.

                ###implemented on closures that capture their environment by *move*

                A closure that moves captured values out of its body will
                only implement *FnOnce* because it can only be called once.

            2) FnMut - applies to closures that don’t move captured values
                out of their body, but that might mutate the captured values.
                These closures can be called more than once.

                ###implemented on closures that capture their environment by mutable
                    or a combination of mutable and immutable borrows

            3) Fn - applies to closures that don’t move captured values out
                of their body and that don’t mutate captured values, as well as
                closures that capture nothing from their environment.

                ###implemented on non-capturing closures or closures that capture their
                    environment only by immutable borrow.

        Supertrait relation between corresponding traits:
            Fn -> FnMut -> FnOnce

        Trait bounds in functions:
            i)  fn immutable_borrow<F: Fn()>(f: F){
                    f();
                }

            ii) fn mutable_borrow<F: FnMut()>(mut f: F){
                    f();
                }

            iii) fn taking_ownership<F: FnOnce()>(f: F){
                    f();
                }

        *move* keyword for transferring ownership (returning a closure):
                fn return_closure() -> impl Fn() {
                    let word = String::from("World!");

                    move || {
                        println!("Hello {}", word)
                    }
                }

                Fn() implies that the closure is either non-capturing
                or performs an immutable borrow. Since String is non-copy
                type, we must transfer its ownership to the closure to
                prevent the existence of a dangling pointer.


    -> Joint Capturing - for complex data types like Structs with multiple fields,
            a closures borrows a whole struct instance as opposed to borrowing one
            of its fields.

            An RFC proposal has been made to change this!

    -> We cannot specify the type of closure argument in a function, thus, the only
            way to define a function that accepts a closure as an argument is through
            trait bounds.
    

unwrap_or_else() method on Option<T>:
    .unwrap_or_else(|| {
        //some action here
        //return something
    });

    takes one argument - a closure

    if Some(T), return T
    else (None case), methods calls closure; returns value returned by
        closure.



*/