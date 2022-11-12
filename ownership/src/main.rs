fn main() {
    println!("Hello, world!");
}

/*
Ownership
Garbage Collection
Stack vs Heap; last in, first out (LIFO)
Pushing vs Popping

*/

/*
Ownership Explained Breifly

-> Both the Stack and the Heap are parts of memory available for use at
runtime.

-> The function of the stack is based on the LIFO principle: the stack
stores values in the order it gets them and removes the values in the
opposite order.

-> Adding data is called *pushing* onto the stack, and removing data is
called *popping* off the stack

-> All data stored on the stack must have a known, fixed size at compile 
time, whereas, data with an unknown size at compile time or size that
might change must be stored on the heap instead.

-> To store data on the heap, a certain amount of space is requested. The
memory allocator finds big enough space on the heap, marks it as being in
use, and returns a pointer - the address of that location. This is known
as *allocating*.

-> Since the pointer to the heap is a known, fized size, it can be stored
on the stack.

-> Pushing to the stack is faster than allocating on the heap, and
accessing data in the heap is slower than accessing data on the stack.

-> Ownership addresses a number of problems:
    1) Keeping track of what parts of code are using what data on the 
    heap

    2) Minimizing the amount of duplicate data on the heap

    3) Cleaning up unused data on the heap
*/

/*

*Ownership Rules*
1) Each value in Rust has an owner
2) There can only be one owner at a time
3) When the owner goes out of scope, the value is dropped - popped off
the stack or deallocated from the heap???

*/

/*

-> A scope is the range within a program for which an item is valid.

-> String literals are immutable.

What exactly is a literal???

-> String literals vs String type: 

1) in the case of strong literals, contents
are known at compile time, so the text is hardcoded directly into the final
executable. String literals are immutable!

2) To support a mutable, growable piece of text, the momory allocator
needs to allocate an amount of memory on the heap, unknown at compile time.
This memory must be returned to the allocator when we're done using String.
*/

/*

-> Garbage Collection: a principle by which a garbage collector keeps track
of and cldans up memory that isn't being used anymore.

-> Rust has no garbage collection: memory is automatically returned once the
variable that owns it goes out of scope.

-> When a variable goes out of scope, function drop() is called automatically, 
and memory is returned. In C++, this is known as Resource Acquisition Is Initialization
(RAII) - a pattern of deallocating resources at the end of an item's lifetime.

-> A String is made up of a group of three parts stored on the stack:
1) a pointer to the memory that holds the contents of the string,
2) a length,
3) and a capacity.

i) pointer - memory address
ii) length - size in memory used up by the contents of the String
iii) capacity - total amount of memory received from the memory allocator

-> Behind the scenes, assigning a variable to another variable containing
data stored on the heap means that the pointer, length, and capacity - as a group -
are copied on the stack. The actual data stays on the heap in one place. Hence,
Rust does not copy heap data. The is known as *shallow copy*.

-> Rust will automatically make a *move* instead of a *shallow copy*. This means 
that that if a variable is a assigned to another a variable, the first variable becomes invalid.

*/

/*

-> Double Free Error (freeing memory twice) - Memory Safety Bug: when two variables 
both pointing to the same data on the heap go out of scope and attempt to free the 
same memory. The solution to this is avoiding shallow copies of heap data.

*/

/*
Move vs Clone:
1) By default, Rust "moves" data instead of automatically creating "deep" copies.
This means that data on the data is not copied, however, the pointer, capacity and length,
which are all stored on the stack, are.

2) To deeply copy heap data (of the String) and not just the stack data, we use the clone() method.

*/





/*
------------> Please look more into the Copy trait and why a type cannot
implement both the Drop and Copy traits simultaneously.
*/

/*

-> Passing a variable to a function will move or copy, just as assignment does.

-> References enable the use of a value without tranferring ownership.

*/