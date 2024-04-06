# PAY ATTENTION! CRATE IS UNFINISHED!

# Rust macro-library for Don't Repeating Yourself

### DRYlib is a library that designed for reducing the amount of duplicate code.
#### Take a look at clones macros example from examples/clones.rs:
```rust
extern crate drylib;

use drylib_procs::{clones, mutclones};

fn main() {
    // You can define variables that you want to clone:
    let digit = 2;
    let vector = vec![1, 2, 3];
    let string = "this is a string".to_owned();

    // And you can clone them with the `clones` macro:
    clones!(digit, vector, string); // Just specify what variables you want to clone
    // This ^ creates new variables using the formula: {CLONES_PREFIX}{identifier(name) of the variable}.
    // By default CLONES_PREFIX is 'c', but you can specify it with following features: [clones-prefix-c, clones-prefix-cl, clones-prefix-clo, clones-prefix-clon, clones-prefix-clone], select the one and prefixes will be appropriate
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
#### An then at the muts macro example from examples/muts.rs:
```rust
extern crate drylib;

use drylib_procs::muts;

fn main() {
    // With the muts macro you can create mutable variables as follows:
    muts!(vector = vec![7, 8, 9]; array = [1; 3]; string = "this is a mutable string".to_owned());
    // But you cannot do it like that:
    // muts!(a; b; c); 
    // You will see this error:
    //     error: proc macro panicked
    //    |
    //    |     muts!(a; b; c); 
    //    |     ^^^^^^^^^^^^^^
    //    |
    //    = help: message: Expected '=' after identifier
    // Assign variables in-place as showed in here       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

    // The `muts` macro expands as follows:
    // let mut vector = vec![7, 8, 9];
    // let mut array = [1; 3];
    // let mut string = "this is a mutable string".to_owned();
        
    // Let's print them:
    println!("vector: {vector:?}, array: {array:?}, string: {string}");

    // Reassign because we can
    vector = vec![10, 11, 12];
    array = [2; 3];
    string = "this is a reassigned mutable string".to_owned();

    // And print them again:
    println!("vector: {vector:?}, array: {array:?}, string: {string}");
}
```
