fn main() {
    let x = 5;

    let x = x + 1; //6

    {
        let x = x * 2; //12
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}"); //6
}


/*
-> Mutability: by default, variables in Rust are immutable, which means you cannot
assign twice to an immutable variable. By adding "mut" keyword, a variable
becomes mutable.

-> Constants: like immutable variables, "constants" are not allowed to change. They
are valid within the scope they are declared in and they are used to name
hardcoded values. To name constants, we use all uppercase with underscores
between words. The type of the value must be annotated. Constants may be
set only to a constant expression; not the result of a computed at
runtime.

-> Shadowing: the practice of declaring a new variable with the same
name as a previous variable. The advantage of variable shadowing: since
a variable type cannot be mutated, shadowing spares us from having to
come up with different variable names for the same thing. e.g.
----> spaces = "   " (String type) && spaces = 3 (Integer type)
*/