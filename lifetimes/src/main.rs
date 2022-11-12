fn main() {
    println!("Hello, world!");
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*

-> lifetimes - the scope for which that reference is valid;
    implicit and inferred

    lifetimes are annotated only when the lifetimes of references could
    be related in a few different ways.

    The main of lifetimes is to prevent dangling references.

    borrow checker - compares scopes to determine whether all borrows
    are valid

    If the subject of a reference does not live as long as the reference,
    the program is rejected.

    Lifetime annotations don't change how long any of the references live.
    Rather, they describe the relationships of the lifetimes of multiple
    references to each other without affecting the lifetimes.

    &i32
    &'a i32
    &'a mut i32

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }

    }

    The returned reference will be valid as long as both the parameters
    are valid.

    The generic lifetime 'a will get the concrete lifetime that is equal
    to the smaller of the lifetimes of x and y. The returned reference
    will also be valid for the length of the smaller of the lifetimes of
    x and y.



    Consider the following struct:
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    The lifetime annotation here means that an instance of
    ImportantExcerpt can't outlive the reference it holds in its 'part'
    field.

    What does it mean for an instance of ImportantExcerpt to outlive the
    reference it holds in its 'part' field?

    consider the following:

    fn main(){

        let text = String::from("Hello Word");

        let instance;

        {
            instance = ImportantExcerpt {
                part: text.as_str(),
            };
        }

        println!("{}", instance.part);
    }

    This code should not compile! Instance outlives reference.

    text.as_str() must not go out of scope before the ImporrtantExcerpt
    does.

    As new deterministic patterns arise, perhaps, fewer lifetime annotations
    will be required.

*/

/*

-> lifetime ellision rules - deterministic patterns programmed into
    Rust's analysis of references.

    These rules apply to fn definitions as well as impl blocks

    Input lifetimes vs Output lifetimes



-> Rules:
    1) each parameter gets its own lifetime:

        fn foo<'a, 'b>(x: &'a i32, y: &'b i32)

    2) if there is exactly one input lifetime parameter, that lifetime is
    assigned to all output lifetime parameters:

        fn foo<'a>(x: &'a i32) -> &'a i32

    3) if there are multiple input lifetime parameters in a method defintion,
    but one of them is &self or &mut self, the lifetime of *self* is assigned
    to all output lifetime parameters.

    Opinion: this makes a lot of sense because references used as arguments
    in a function cannot go out of scope before an instance of a type does.

*/

/*

-> The static lifetime - 'static - denotes that the affected reference
    can live for the entire duration of the program.

    All string literals have the 'static life:

        let s: &'static str = "I have a static lifetime.";

    The text os this string is stored directly in the program's binary,
    making it available always.

*/
