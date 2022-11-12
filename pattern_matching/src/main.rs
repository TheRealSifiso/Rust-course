fn main() {
    let x = Some(3);

    let mut y = None;

    if let Some(num) = x{
        y = Some(num+1);
    }

    println!("{:?}", y);
}

/*

Catchall Pattern:
    $ '_' pattern will match anything, but it never binds to a variable.

Refutable vs irrefutable patters:
    $ 

    
*/