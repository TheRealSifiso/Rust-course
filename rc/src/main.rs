use std::rc::Rc;
use std::cell::RefCell;

enum List{
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main(){

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    let holder = Something {
        value: vec![1, 2]
    };

    let mut owner = Something{
        value: vec![1, 2, 3]
    };

    owner.value = vec![1, 4, 3];

}

struct Something{
    value: Vec<i32>
}

/*

Rc<T>:
    $ multiple ownership by reference counting

    $ heap memory allocation
    
    $ non-atomic reference counting, thus very low overhead.

    $ for use in *single-threaded* scenarios only!

    $ Creation of an Rc type using Rc::new() increases the reference
        from 0 to 1.

        Calling Rc::clone() - which takes an immutable reference to
        an Rc type - increases the reference count by 1. The alternative
        is to call .clone() which makes deep-copies of the data stored in Rc.
        The convention is to use Rc::clone() instead.

        Passing an immutable reference to an Rc type as an argument to
        Rc::strong_count() returns the reference count (the number of owners).

    $ Rc<T> for multiple owernship works for immutable references only.
        If multiple mutable references were allowed, the borrowing rules
        would be violated since multiple mutable borrows to the same
        place can cause data races and inconsistences.

        The interior mutability pattern via the combined use of the 
        RefCell<T> and Rc<T> can address this immutability
        restriction.

        This means placing a RefCell<T> inside a Rc<T>:
            Rc<RefCell<T>>

*/
