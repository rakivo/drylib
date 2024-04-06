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

    // You can create a tuple with a lifetime as well:
    pubstruct!{
        #[derive(Debug)]
        TupleLT<'a>(&'a i32, usize, &'a i32)
    };

    let tuple_lt = TupleLT(&0, 1, &2);
    println!("{tuple_lt:?}"); // Prints: TupleLT(0, 1, 2)

    // And also you can create a tuple with both of generics and lifetimes:
    pubstruct!{
        #[derive(Debug)]
        TupleLTTU<'a, 'b, T, U>(&'a T, U, &'b i32)
    };

    let tuple_lttu = TupleLTTU(&"hello", vec![1, 2, 3], &2);
    println!("{tuple_lttu:?}"); // Prints: TupleLTTU("hello", [1, 2, 3], 2)

    //  <++++++++++++++++++++++++ In here let's create some non-tuple type structs  ++++++++++++++++++++++++>

    // With pubstruct! macro you can also do like that:
    pubstruct!{
        #[derive(Debug)]
        Structure {
            digit: usize,       // By the way, keep in mind that you need to put
            another_digit: i32, // <- this comma after the last 
        }                       // field of your struct to make this work
    }
    // This ^ will expand into this:
    //     pub struct Structure {
    //     pub digit: usize,
    //     pub another_digit: i32,
    // }

    let structure = Structure { digit: 0, another_digit: 1 };
    println!("{structure:?}"); // Prints: Structure { digit: 0, another_digit: 1 }

    // You can create a struct with a generic type T:
    pubstruct!{
        #[derive(Debug)]
        StructureT<T> {
            greet: T,
            digit: i32,
        }
    }

    let structure_t = StructureT { greet: "hello", digit: 1 };
    println!("{structure_t:?}"); // Prints: StructureT { greet: "hello", digit: 1 }

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
}
