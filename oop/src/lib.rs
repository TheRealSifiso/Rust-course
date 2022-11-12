pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self){
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

/*

    $ Four principles of Object-oriented programming:
        1) Encapsulation:

            By default, types, fields, methods, and associated functions
            are private. Using the 'pub' keyword, we can 'unhide'
            certain implementation details while keeping others hidden.

        2) Inheritance:

            In other programming languages, a child class can inherit
            behaviours from the parent class. Rust - however - does
            not allow structs to inherit data and procedures from other
            structs directly.

            Rust provides a limited way of using the principle of
            inheritance, by leveraging the functionality of traits.
            
        3) Polymorphism:

            A child type be used in the same places as the parent type
            if the child type inherits from the parent type. This is
            known as polymorphism.

            Sometimes, we want to use instances of different types and
            call methods on them as if they were of the same type.
            This is where trait objects are useful, and can be defined
            in two ways:

                i) Box<dyn Trait>
                ii) &dyn Trait

            A useful application of trait objects is creating a
            collection of non-homogeneous items in the sense that
            each item is an instance of a different type:

            pub struct Screen {
                pub components: Vec<Box<dyn Draw>>,
            }

            #-----------------------------------------------------------
            
            An alternative to using trait objects is the application of
            trait bounds:

            pub struct Screen<T: Draw> {
                pub components: Vec<T>
            }

            the method is only useful for homogeneous collections as
            each of item in the vector is of exactly the same type
            due to monomorphization (where one concrete type is 
            substituted for a generic one)

    $ Static vs Dynamic Dispatch
        When using trait bounds with generic types instead of trait 
        objects, Rust uses 'monomorphization' to perform static
        dispatch. Thus, Rust replaces functions with ones containing
        concrete types on their definitions.

        Static dispatch has zero performance overhead, however, the
        downsides are that it causes "code bloat" due to many copies
        of the same function existing in the binary, one for each type.

        #---------------------------------------------------------------

        Since Trait objects (&dyn Foo or Box<dyn Foo>) store a value
        of any type that implements the given trait, the precise type
        can only be known at runtime.

        The methods of this trait can be called on a trait object via a
        special record of function pointers (created and managed by the
        compiler). This is known as Dynamic dispatch.

        Dynamic dispatch incurs performance overhead, since the precise
        type can only be own at runtime?

        There's so much more to dynamic dispatch to learn about!

*/