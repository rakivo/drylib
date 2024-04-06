# PAY ATTENTION! CRATE IS UNFINISHED!

# Rust macro-library for Don't Repeating Yourself

### DRYlib is a library that designed for reducing the amount of duplicate code.
#### Take a look at clones macros example from examples/clones.rs:
```rust
extern crate drylib;

use drylib::{clones, mutclones}; // Use the best library in the world

fn main() {
    // You can define variables that you want to clone:
    let digit = 2;
    let vector = vec![1, 2, 3];
    let string = "this is a string".to_owned();

    // And you can clone them with the `clones` macro:
    clones!(digit, vector, string); // Just specify what variables you want to clone
    // This ^ creates new variables using the formula: {CLONES_PREFIX}{identifier(name) of the variable}.
    // By default CLONES_PREFIX is 'c', but you can specify it with following features:
    // [clones-prefix-c, clones-prefix-cl, clones-prefix-clo, clones-prefix-clon, clones-prefix-clone]
    // Select the one and prefixes will be appropriate.
    //
    // Therefore, the `clones` macro expands as follows:
    // let cdigit = digit.clone();
    // let cvector = vector.clone();
    // let cstring = string.clone();

    // We can print them:
    println!("cdigit: {cdigit}, cvector: {cvector:?}, cstring: {cstring}");

    // Here is another one, the `mutclones` macro, it does the same thing as the clones macro, but expanded variables are mutable.
    mutclones!(digit, vector, string); // The `mutclones` macro expands in this code:
    // let mut cdigit = digit.clone();
    // let mut cvector = vector.clone();
    // let mut cstring = string.clone();

    // The variables are mutable, therefore you can easily reassign them:
    cdigit = 4;
    cvector = vec![4, 5, 6];
    cstring = "this is a mutable cloned string".to_owned();

    // And print them:
    println!("cdigit: {cdigit}, cvector: {cvector:?}, cstring: {cstring}");
}
```
#### An then at the pubstruct macro example from examples/structs.rs:
```rust
extern crate drylib; // Import the drylib library

use drylib::*; // Bring all of the macros from the drylib into scope

fn main() {
    //  <++++++++++++++++++++++++ Let's start with creating tuple structs  ++++++++++++++++++++++++>

    // You can create a pub tuple struct using the pubstruct macro like that:
    pubstruct!{
        #[derive(Debug)]
        Tuple(usize, i32, u32) // Define a tuple with types usize, i32, and u32
    };
    // This ^ will expand into this:
    // pub struct Tuple(pub usize, pub i32, pub u32);
    // So that's kinda the point of the macro, create pub struct with all of the fields in it are public as well.

    let tuple = Tuple(0, 1, 2); // Let's create an instance of the Tuple
    println!("{tuple:?}");      // Prints: Tuple(0, 1, 2)

    // You can create a pub tuple struct with a generic type T as well:
    pubstruct!{
        #[derive(Debug)]
        TupleT<T>(T, i32, u32)
    };

    let tuple_t = TupleT(0, 1, 2);
    println!("{tuple_t:?}"); // Prints: TupleT(0, 1, 2)

    // Also you create a struct with a lifetime:
    pubstruct!{
        #[derive(Debug)]
        StructureLT<'a> {
            greet: &'a str,
            digit: i32,
        }
    }

    let structure_lt = StructureLT { greet: "hello again", digit: 1 };
    println!("{structure_lt:?}"); // Prints: StructureLT { greet: "hello again", digit: 1 }

    // Create a struct with both of generics and lifetimes:
    pubstruct!{
        #[derive(Debug)]
        StructureLTTU<'a, 'b, T, U> {
            greet: &'a T,
            array: &'b Vec<U>,
        }
    }
    // And this ^ will expand into this(oh wow):
    // pub struct StructureLTTU<'a, 'b, T, U> {
    //     pub greet: &'a T,
    //     pub array: &'b Vec<U>,
    // }

    let structure_lttu = StructureLTTU { greet: &"hello again", array: &vec![1, 2, 3] };
    println!("{structure_lttu:?}"); // Prints: StructureLTTU { greet: "hello again", array: [1, 2, 3] }
    // ... Continued in file examples/structs.rs
}
```
#### In this library we have a lot of convenient little macros, That was just a small part of them.
#### I haven't created the examples for all of them but you can do a PR and help me with that.
#### There will be more and more macros in the future, the library is still far from its final version.
