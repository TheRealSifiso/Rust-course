fn main() {
    println!("Hello, world!");
}

/*

-> enums: enumerations - used to define a type by emunerating its
possible *variants*. "To enumerate all possible variants"

-> Structs vs Enums: unlike structs, whcih group together related fields
and data, enums give a way of saying that a value is one of a possible
set of values.

-> The variants of an enum are namespaced under their indentifier:

e.g.

enum IpAddr{
    V4(&str),
    V6(&str),
}

Namespacing:

let four = IpAddr::V4("127.0.0.1"); //"::" syntax for namespacing


---> The name of each enum variant that we define also becomes a function
that constructs an instance of the enum:

e.g. IpAddr::V4() - takes a String argument and returns an instace of
IpAddr type.

-> You can put any kind of data inside an enum variant even if it's
 another enum

*/






/*

-> Option<T> Enum - the alternative to Null Values - encodes the scenario
in which a value could be something or nothing. In other words, it
encodes the concept of a value being present or absent.Hence, Rust has
no Null feature.

enum Option<T> {
    None,
    Some(T),
}

-> The Rust compiler can *infer* the concrete type in place of "T",
however, the compiler can't infer the type that the corresponding *Some*
variant will hold by looking only at a *None* value.

-> We emply the "match" expression - a powerful control flow construct - 
that runs different code depending on which variant of the enum it has.

*/





/*

-> match control flow construct - allows you to compare a value against
a series of patterns and then execute code based on which pattern matches.

-> Matches are Exhaustive: the arms' patterns must cover all possibilities.

-> The catch-all arm must be in last position, otherwise the other arms
will never run.

-> '_' - a special catch-all pattern should we wish to not use the value
in it.

*/







/*

-> The "if let" syntax lets you combine *if* and *let* into a concise way
to handle values that match one pattern while ignoring the rest:

e.g.

let config_max = Some(3u8);
match config_max {
    Some(max) => println!("{}", max),
    _ => ()
}

                        is equivalent to

let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("{}", max)
} // the 'max' binds to the value inside the *Some*

The latter is more concise and contains no boilerplate code!

-> By using *if let*, you lose exhaustive checking that *match* enforces

https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html

*/