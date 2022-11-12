fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}


/*

-> HashMap<K, V> stores a mapping of keys of type K to values of type V
using a *hashing function*.

-> .get() method for accessing values in a Hash Map returns Option<&T>

-> For types that implement the Copy trait, values are copied onto the
hashmap. For types that do not, values are moved.

-> 

*/