fn main() {
    println!("Hello, world!");
}

trait Barking {
    fn bark(){

    }
}

//Blanket Implementation
use std::fmt::Display;
impl <T: Display> Barking for T{

}

/*

-> traits - define shared behaviour

-> trait bouds - specificy that a generic type can be any type that has
    certain behaviour

-> coherence and orphan rule

-> Accept and/or return types that implement specific traits

-> You can only use "impl Trait" if you're returning a single type!
    If a function returns one of many possible types that implement
    a certain trait, trait objects must be used to accomplish such.

    e.g. fn returns_summarizable(switch: bool) -> impl Summary {
        if switch {
            NewsArticle {
                //fields
            }
        } else {
            Tweet {
                //fields
            }
        }
    }

    Although NewsArticle and Tweet implement Summary (trait), this is
    NOT allowed!

    Use *trait objects* for this.

-> By using a trait bound with an *impl* block that uses generic
    type parameters, we can implement methods conditionally for types
    that implement the specificied traits

-> We can also conditionally implement a trait for any type that
    implements another trait.

    Implementations of a trait on any type that satisfies the trait
    bounds are called "blanket implementations"

    e.g. The standard library implements the ToString trait on any type
    that implements the Display trait

    impl<T: Display> ToString for t {
        
    }

    Since integer types implement the Display trait, we can turn integers
    into their corresponding String values like so:

    let s = 3.to_string();



    It's called a Blanket Implementation because the implementation block
    doesn't have to be provided for a specific type - custom or not;
    instead, the implementation block is provided - even in the absence
    of a custom type - for all types brought in locally that implement
    a specific trait!

    In other words, following the previous example, all types that 
    implement the Display trait should implement ToString

-> All three variants put together:
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> impl Summary {

    }

    fn some_function(t: &(impl Display + Clone), u: &(impl Clone + Debug)) -> impl Summary {

    }

    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        
    }
    

*/