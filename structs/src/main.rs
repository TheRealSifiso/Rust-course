struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        //'email' gets a new value
        //user1.email remains valid
        email: String::from("another@example.com"),
        //With the exception of types that implement the "Copy trait",
        //values from all other fields are "moved" into this new instance
        //thus, *user1.username* is invalid
        ..user1
    };
}


/*
-> Struct(structure) -

-> Field Init Shorthand syntax -> used to avoid tedious repetition of
field and function parameter names.
*/

/*
-> Struct Update Syntax -> the use of ".." syntax to create a new instance
of a struct that includes most of the values from another instance but
changes some without the need to explicitly set the remaining fields.


e.g.

struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

let user1 = User{
    active: ###,
    username: ###,
    email: ###,
    sign_in_count: ###,
};

Using the Struct Update Syntax:

let user2 = User{
    email: ############(something different),
    ..user1, // notice the "..user1" syntax
};



---> The Struct Update Syntax uses "=" like an assignment, thus, data
from the previous instance is *moved* to the new instance. Field types
that implement the *Copy trait*, thus, the values in the corresponding fields
from the previous instance remain "valid".

SEE EXAMPLE ABOVE!

*/

/*

--> Tuple Structs - look similar to tuples, however, they have the added
meaning the struct name provides but don't have names associated with their
fields; rather, they just have the types of the fields

These are useful for giving tuples a "name" and a different "type"

-->Tuple struct instances are similar to tuples in that you can destructure
them into their individual pieces, and you can use a . followed by the index
 to access an individual value.

*/

/*
-> Unit-like structs - behave similarly to (), the unit type, and can
be useful when you need to implement a trait on some type but don't have
any data that you want to store in the type itself.

e.g.

struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}

*/

/*
-> Lifetimes - to store *references* to data owned by something else,
the use of lifetimes is required to ensire that the data referenced by
a struct is valid for as long as the struct is:

e.g.

struct User {
    active: bool,
    username: &str, //&str is a string slice type - a reference type
    email: &str,
    sign_in_count: u64,
}

The deliberate choice to use Owned Data Types is justified by the want to
have each instance of a struct to own all of its data and for that data
to be valid for as long as the entire struct is valid:

e.g.

struct User {
    active: bool,
    username: String, //instead of &str, we used String - an Owned Data Type
    email: String,
    sign_in_count: u64,
}
*/