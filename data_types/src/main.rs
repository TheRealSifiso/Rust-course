fn main() {

    //let num: u32 = "3".parse();

    let num: u32 = if let Ok(num) = "3".parse(){
        return num
    };

    println!("{}", num);
}

/*
Data type subsets: scalar vs compound
parse() function
signed and unsigned numbers
two's complement representation
integer literals
integer overflows: panicking vs
two's complement wrapping
What is Unicode? ASCII?
Pattern Matching and Destructuring
Unit tuple
array vs tuple
fixed length/size
stack allocation
compile-time vs runtime errors
*/