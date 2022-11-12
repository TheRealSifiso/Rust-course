fn main() {
    println!("Hello, world!");
}

/*

-> References -> give the compiler information about how references
    relate to each other.

-> Binary operations such as '>' can only be applied to types whose
    values can be *ordered*.

->  std::cmp::PartialOrd - a trait you can implement on types to enable
    comparisons

-> impl Something<f32> - implement associated functions (including
    methods) only on Something<f32> (only f32 values passed in)

-> Monomorphization - zero performance overhead!

-

*/



#[warn(dead_code)]
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}