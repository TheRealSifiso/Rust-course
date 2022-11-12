fn main(){

    let mut counter = 0;

    counter = 'up_counter: loop {
        counter += 1;

        if counter == 10{
            break 'up_counter counter*2;
        }
    }; 

    println!("{counter}");

}

/*
mismatched return types in arms

three kinds of loops in Rust: loop, while, and for.

loop labels

rev() method to reverse a range
*/