use std::{cell::RefCell, borrow::Borrow};
use std::rc::Rc;
fn main() {
    
    let data = RefCell::new(String::from("Hello, "_));

    data.borrow_mut().push_str("World");

    println!("{}", data.borrow());

}

/*

Interior Mutability:
    $ a design pattern that uses unsafe code to allow mutation of data
        even when there are immutable references to that data.
    
    $ By default, it is impossible to make a mutable borrow to an
        immutable value.

RefCell<T>:
    $ represents single ownership over the data it holds.

    $ for use in single threaded scenarios only!

    $ unlike Box<T>, the borrowing rules' invariants of RefCell<T> are
        enforced at runtime, thus, if these rules are broken, the
        program with panic and exit.

        RefCell<T> remains immutable while the data it hold is mutable.
        This is known as Interior Mutability.

        Checking the borrowing rules at runtime ensures that certain
        memory-safe scenarios are allowed, where they would've been
        disallowed by the compile-time checks.

    $ borrow_mut() returns RefMut<T> (a smart pointer) which gives us a
        mutable reference to the value inside RefCell<T>

    $ borrow() returns Ref<T> (a smart pointer) which gives us an
        immutable reference to the value inside RefCell<T>

    $ Ref<T> and RefMut<T> both implement the Deref trait, thus, they
        can be treated like regular references.

    $ RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart
        pointers are currently active. Borrowing rules still apply here
        at runtime: one mutable borrow at any point in time. If rules
        are broken, then the program will panic!

Combining RefCell<T> and Rc<T>:
    $ We can combine RefCell<T> and Rc<T> in order to have multiple
        owners of mutable data:

            Rc<RefCell<T>>

            Rc::new(RefCell::new(data))

*/