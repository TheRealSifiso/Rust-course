use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List{
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // deref coercion
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {

        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }

    }
}

fn main(){

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    // a = [5, Nil]

    println!("'a' initial rc count = {}", Rc::strong_count(&a));
    println!("'a' next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    // b = [10, a]

    println!("'a' rc count after 'b' creation = {}", Rc::strong_count(&a));
    println!("'b' initial rc count = {}", Rc::strong_count(&b));
    println!("'b' next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
        // a = [5, b]
    }

    println!("'b' rc count after changing 'a' = {}", Rc::strong_count(&b));
    println!("'a' rc count after changing 'a' = {}", Rc::strong_count(&a));

    println!("'a' next item = {:?}", a.tail());

}

/*

Question: since RefCell cannot be dereferenced, is RefCell, therefore,
    not a smart pointer. I know RefMut and Ref are.

Memory leak
    $ memory that is never cleaned up!

    $ reference cycles are logical bugs that cause memory leaks since
        reference count (strong_count) is never dropped to 0.

        Calling Rc::clone() inreases the strong_count of an Rc<T>
        instance.

Calling borrow_mut():

    $ consider
        RefCell<Rc<List>>

        calling borrow_mut() gives you
            RefMut<Rc<List>>
        
        Derefencing RefMut<Rc<List>> gives you
            Rc<List>

Weak reference:
    $ Unlile Strong References, Weak References don't express an
        ownership relationship and their count does not affect when an
        Rc<T> instance is cleaned up!

    $ Unlike Strong References, Weak References won't cause a reference
        cycle, since weak_count does not have to be 0 for an Rc<T> 
        instance to be cleaned up.

    $ Calling Rc::downgrade gives you a smart pointer of type Weak<T>,
        and increases weak_count - which keeps track of how many
        Weak<T> references exist - by 1.

    $ The value Weak<T> references may or may not have been dropped.
        To ensure that the ensure still exists, we must call the upgrade
        method on a Weak<T> instance which returns Option<Rc<T>>.
        
        For Option<Rc<T>>, getting a result of Some means that the Rc<T>
        value has not been dropped yet. However, getting a result of
        None means that the value has already been dropped.


#See the trees module for more information about Weak<T>

*/