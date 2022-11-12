fn main() {
    let mut v = vec![1, 2, 3, 4];
    //Two ways to access an element in a vector:
    #[warn(unused_variables)]
    let third: &i32 = &v[2]; //indexing

    //get() method return Option<T>
    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => print!("{third}"),
        None => print!("")
    }

    for i in &mut v {
        *i += 50;
    }
}

/*

-> A collection of data can grow or shrink since collections point to
    data stored on the heap:
    Vectors, Strings, and Hash Maps.

-> Vec<T>: stores multiple values of the same type;
    push() to add elements onto a vector
    pop() - removes and returns the last element in a vector

-> Storing multiple types in a vector:
    we accomplish this with the use of enums:
    teh variants of an enum are all defined under the same enum type

    e.g.

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12)];


-> If you don't know the exhaustive set of types a program will get
    at runtime to store in a vector, the enum technique won't work.
    Instead, you can use trait objects.

*/