use state_pattern::Post;

fn main(){
    
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    post.request_review();
    post.approve();

}

/*

take() method
    takes a Some value and leaves a None in its former place

The state pattern:
    $ copy-paste:
        The crux of the pattern is that we define a set of states a
        value can have internally; the states are represented by a set
        of *state objects*, and the value's behaviour changes based on
        its states.

    $ State objects may share functionality - however - each state
        object is responsible for its behavior and for governing when
        it should change into another state. 
        
        The value that holds a state object knows nothing about the
        different behaviors of the state or when to transition between
        states.

*/