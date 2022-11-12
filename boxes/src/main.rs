use boxes::List::{Cons, Nil};

fn main(){

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}", list);
    //Cons(1, Cons(2, Cons(3, Nil)))

}
/*

Smart pointers:
    $ usually implemented using structs, and unlike an ordinary struct,
        smart pointers implement the *Deref* and *Drop* traits

Deref & Drop traits:
    $ Deref trait:
        allows an instance of the smart pointer struct to behave like
        a reference so you can write your code to work with either
        references or smart pointers.

    $ Drop trait:
        allows you to customize the code that's run when an instance of
        the smart pointer goes out of scope.

Box<T>:
    $ provides heap memory allocation or indirection. Indirection means
        storing data 'indirectly' - data is stored on the heap while
        pointer data (the data that refers to the actual data) is stored
        on the stack.

    $ Enabling Recursive Types with Boxes:
        cons-list data structure (lisp version of a linked list)

        A cons list data structure constructs a pair from its two
        arguments: a value and another pair of the same type:

        enum List {
            Cons(i32, List),
            Nil
        }//this code will not compile

        List is a recursively-defined data structure given that we can
        construct new pairs within pairs 'infinitely'.

Computing the size of a non-recursive type:
        $ consider the following enum:

            enum Message {
                Quit,
                Move { x: i32, y: i32 },
                Write(String),
                ChangeColor(i32, i32, i32),
            }

            Since only one variant is used, the most space a Message
            value will need is the space it would take to store the
            largest of its variants

            Unfortunately, List is recursively-defined, and so the
            compiler does not know how much space to allocate. The
            solution to this is indirection!

Indirection:
            $ In order for the rust compiler to determine how much space
                a List value needs, we need to provide indirection:

                enum List{
                    Cons(i32, Box<List>),
                    Nil,
                }

What happens when Box<T> goes out of scope>
                $ Because of the Drop trait implementation, when a Box<T>
                    goes out of scope, not only is the pointer data on
                    the stack cleaned up but the data it points to on
                    the heap is also cleaned up.

*/