fn main(){}

struct Shoe{
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

/*

    => Always keep in mind the difference between
        consumer adaptor and iterator adaptor,
        and what it means to 'consume'.

    -> iter() -produces an iterator over immutable references
    -> into_iter() - takes and returns ownership of values in a collection
    -> iter_mut() - produces an iterator over mutable references

    -> Iterator trait

    -> associated types

    -> next() - consuming adaptor;
        returns items in Some()
        when iteration is over, returns None

        methods that call "next" are consuming

    -> map(|x| x) - iterator adaptor
        takes a closure argument;
        takes each item in an iterator as 'x'
        returns a new iterator with slightly different items.

    -> filter(|x| x.field = 2) - iterator adaptor
        takes a closure argument;
        takes each item in an iterator as 'x'
        if the condition is true, the current item forms part of a new
        collection after calling collection().

    -> sum() - consuming adaptor
        I think it's pretty obvious by now


*/

/*

iterators:
    are responsible for the logic of iterating over each item in a
    sequence and determing when the sequence has finished.

    iterators are lazy - this meaning they have no effect until you call
    methods that consume the iterator to use it up, such as next(), collect(),
    sum().

Consuming vs iterator adaptors:

    consuming adaptors change the internal state of an interator;
    iterator adaptors don't consume iterators; they produce different
    iteraotrs by changing some aspect of the original iterator.


*/