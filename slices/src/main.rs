fn main() {
    println!("Hello, world!");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }

    s.len()
}

/*

-> byte literal syntax

-> A string slice is a reference to part of a String:
&name[starting_index...ending_index (not inclusive)]

-> length = ending_index - starting_index

-> A string slice contains two components:
1) a pointer to the 'byte' at the starting_index
2) the length of the slice

-> emumerate() returns a tuple containing an index and a reference to a
value from a collection: (index, &value)

-> String Literals are Slices: &str.

-> For slice type &[i32], only stores a reference to the first element
and length

*/