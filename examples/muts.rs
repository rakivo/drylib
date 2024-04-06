extern crate drylib;

use drylib::muts; // And in here let's just use the best library in the world (haha)

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
