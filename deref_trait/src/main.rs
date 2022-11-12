use std::ops::Deref;

fn main(){

    let m = MyBox::new(String::from("Rust"));

    hello(&m);

}

fn hello(name: &str){
    println!("Hello, {name}")
}

struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
/*

Derefence operator (*):
    $ Consider the following:

        let x = 5;
        let y = &x;

        'y' is bound to a reference to 'x'. A reference is a type of
        pointer.

    $ The dereference operator (*) 'follows' a reference to its value.
        To customize the behaviour of the dereference operator (*),
        we implement the Deref trait which returns a reference to a
        value.

        The * operator calls the deref() method which must be overridden
        should the Deref trait be implemented on a type (struct).

Deref coercions:
    $ convert a reference to a type into a reference to another type.
        e.g. &Box -> &String -> &str
        Box and String must implement the Deref trait.

    $ The Rust compiler automatically performs deref coercion on arguments
        to functions and methods that do not match the parameter type in
        the function or method definition. This done via a sequence of
        calls to the deref method.

    $ The number of times that Deref::deref needs to be inserted is 
        resolved at compile time.
    
    $ The Deref trait overrides the * operator on immutable references,
        where, the DerefMut trait overrrides the * operator on mutable
        references.

    $ Rust performs deref coercion in the following three cases:
        1) &T -> &U when T: Deref<Target=U>
        2) &mut -> &mut U when T: DerefMut<Target=U>
        3) &mut -> &U when T: Deref<Target=U> 

        Rust can coerce a mutable reference to an immutable one, but not
        the opposite is not possible!

*/