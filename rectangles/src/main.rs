#[derive(Debug)]
struct Rectangle{
    width: u64,
    height: u64,
}

//I prefer separating associated functions depending whether they
//are just associated functions or methods.
impl Rectangle{ //all methods
    fn area(&self) -> u64{
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

}

impl Rectangle { //other associated functions
    fn square(size: u64) -> Self { //use 'Self' instead of 'Rectangle' incase the name of the struct is changed
        Self{
            width: size,
            height: size,
        }
    }
}
fn main(){

    let field = Rectangle::square(3);

}

/*
-> All primitive types implement the "Display" trait. Structs on the
otherhand do not and they have display possibilities.

-> To print out Debugging Information, we must add the outer attribute
#[derive(Debug)] just before the struct definition. Inside the println!
macro, we use the ":?" or ":#?" *specifier* inside the curly brackets to
tell the println! macro to use an output format called *Debug*.

-> The Debug trait enables us to print our struct in a way so we can
see its value(s) while we're debugging our code

*/

// dbg!() macro takes and returns ownership. LOOK INTO THIS!!!!



/*

---> Methods are similar to functions, however, they are defined within
the context of a struct (or enum or a trait object) and their first
parameter is always "self", which represents the instance of the struct
the method is being called on.

---> "&self" is actually a shorthand for "self: &Self"

-> "Self" is an alias for the type that the implementation block is for

-> Methods may:
1) take ownership of "self" (self), 
2) borrow "self" immutably (&self),
3) borrow "self" mutably (&mut self)

-> Getters are useful because you can make the field private but the
method public and thus enable read-only access to that field as part
of the type’s public API.

*/








/*

-> Automatic Referencing and Dereferencing: PLEASE LOOK INTO THIS!!!!

p1.distance(&p2);
(&p1).distance(&p2);

^^^^^^Both of these are same. Above is an example of Rust's
Automatic Referencing and Dereferencing feature

REMEMBER:
impl Type_name{
    fn method_name(&self) -> type_name {
        expression
    }
}

"&self" indicates that the method is making an immutable borrow of an
instance of Self (the type that the implementation block is for). We'll
call this instance "object", hence, a method call resembles the following:
object.something()

Rust automatically adds in &, &mut, or * so "object" matches the signature 
of the method. In other words, the following are the same:

p1.distance(&p2);
(&p1).distance(&p2);

*/







/*

-> Associated functions - are all functions defined within an
implementation block because they’re associated with the type 
named after the "impl" keyword.

-> Associated functons that don't have "self" as their first parameter
are not methods because they don't need an instance of the type to work
with. e.g. String::from() - String is a type; from() is an associated
function.

-> "::" syntax to call associated functions

-> Associated functions (including methods) may be split into multiple
implementation blocks.

*/