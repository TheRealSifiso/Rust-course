fn main() {
    
    

}

/*

-> to_string() method is available on any type that implements the
'Display' trait. String::from("") is an equivalent form.

-> strings are UTF-8 encoded

-> push_str() to append to a String; takes String Literal as parameter

-> push() takes only one byte

-> Using '+' operator to concatenate strings works in the following way:

    The '+' operator calls the add() function with following signature:

    fn add(self, s: &str) -> String

    Hence, concatenation works the following way:

    let s3 = s1 + &s2; //We immutably borrow s2 (the second string)

-> Since we can only add &str to a String, if we add two Strings, Rust
    coerces the &String argument into a &str using *deref coercion*.

-> Concantenating multiple strings:

    1) let s = s1 + &s2 + &s3;

    2) let s = format!("{}-{}-{}", s1, s2, s3);
    Result: "!@#-$%^-&*#"

    format! macro uses references, hence, it takes no ownership of any of
    its parameters

-> Rust strings don't support indexing!
    
    A string is a wrapper over Vec<u8>.


*/
