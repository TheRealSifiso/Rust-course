fn main() {
    println!("Hello, world!");
}

/*

-> cargo test

-> assert! macro - ensure that some condition in a test evaluates to 
    *true*; used for boolean values

-> assert_eq! & assert_ne! - compare arguments for equality or inequality

-> #[should_panic] - to check that our code handles error conditions as
    we expect. The test passes if the code inside the function panics;
    the test fails if the code inside the function does not panic.

        #[test]
        #[should_panic]
        fn this_function_panics {
            panic!() //this may be substituted for a function call that panics
        }

    Given that a function may panic for a variety of reasons, tests that
    use *should_panic* can be imprecise. We add an optional *expected*
    parameter to the *should_panic* attribuute so that a test passes when
    the value in the *should_panic* attribute's *expected* parameter is a
    *subtring* of the message a function panics with:

        #[should_panic(expected = "please enter a valid input")]


-> cargo test -- --help: list the arguments that go to "cargo test",
        followed by the separator, then those that go to the resulting
        binary. This command displays the options you can use after
        the separator.

-> cargo test -- --test-threads=1
        for more fine-grained control over the number of threads
        used

-> cargo test -- --show-output
        to show output of successful tests

-> cargo test [name of test(s)]
        filtering - to run only certain tests

-> #[ignore]
        to ignore a test:

            #[test]
            #[ignore]
            fn expensive_test() {}

-> cargo test -- --ignored
        to run only ognored tests

-> cargo test -- --include-ignored
        run all (ignored and unignored) tests

-> cargo test --test [name of integration test file]
        to run specific integration tests

-> 

*/


/*

->  Rust can compile any code examples that appear in our API 
documentation

-> Tests fail when something in the test function panics.

-> Each test is run in a new thread, and when the main thread
    sees that a test thread has died, the test is marked as failed.

-> When assertions using the assert_eq! & assert_ne! macros fail, their
    arguments are printed using the debug formatting, which means that
    the values being compared must implement the *ParitalEq* and *Debug*
    traits.

    Since *PartialEq* and *Debug* are *derivable traits*, implementing
    these traits for your own custom data types can be done by simply
    adding the
        #[derive(PartialEq, Debug)]
    annotation to your struct or enum definitions.

-> You can add your own custom failure messages as optional arguments to
    the assert!, assert_eq! and assertne! macros:

        assert!(false, "Your function is buggy: {}", stuff)

-> We can write tests that use Result<T, E>:

        #[test]
        fn it_works() -> Result<(), String> {
            if condition{
                Ok(())
            } else {
                Err(String::from("this test fails"))
            }
        }

    We can now use a ? operator or
        assert!(value.is_err())

-> 

*/





/*

-> Since tests runs in parallel, tests that depend on each other or some
    shared enviroment may intefere with one there.

    We set the number of test threads to 1, telling the program not to use 
    any parallelism.

*/




/*

-> Unit vs Intergration tests

->  Integration tests don't need the
        #[cfg(test)]
    annotation

->  Units of code that work correctly on their own could have problems 
    when integrated, so test coverage of the integrated code is important
    as well

-> 

*/