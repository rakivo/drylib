extern crate drylib;

use drylib::*;

fn main() {
    pub_tupstruct!{
        #[derive(Debug)]
        Tuple(usize, i32, u32)
    };

    let tuple = Tuple(0, 1, 2);
    println!("{tuple:?}");

    pub_tupstruct!{
        #[derive(Debug)]
        TupleT<T>(T, i32, u32)
    };

    let tuple_t = TupleT("hello", 1, 2);
    println!("{tuple_t:?}");

    pub_tupstruct!{
        #[derive(Debug)]
        TupleTU<T, U>(T, i32, U)
    };

    let tuple_tu = TupleTU("hello", 1, vec![1, 2, 3]);
    println!("{tuple_tu:?}");
    
    pub_tupstruct!{
        #[derive(Debug)]
        TupleLT<'a>(i32, &'a i32)
    };

    let tuple_lt = TupleLT(0, &1);
    println!("{tuple_lt:?}");

    pub_tupstruct!{
        #[derive(Debug)]
        TupleLTT<'a, T>(T, &'a i32)
    }

    let tuple_ltt = TupleLTT("hello", &1);
    println!("{tuple_ltt:?}");

    pub_tupstruct!{
        #[derive(Debug)]
        TupleLTU<'a, T, U>(T, &'a i32, U)
    }

    let tuple_ltu = TupleLTU("hello", &1, vec![1, 2, 3]);
    println!("{tuple_ltu:?}");

    pubstruct!{
        #[derive(Debug)]
        Structure {
            greet: usize,
            digit: i32,
        }
    }

    let structure = Structure{ greet: 0, digit: 1 };
    println!("{structure:?}");

    pubstruct!{
        #[derive(Debug)]
        StructureT<T> {
            greet: T,
            digit: i32,
        }
    }

    let structure_t = StructureT{ greet: "hello", digit: 1 };
    println!("{structure_t:?}");
    
    pubstruct!{
        #[derive(Debug)]
        StructureTU<T, U> {
            greet: T,
            digit: U,
        }
    }

    let structure_tu = StructureTU{ greet: "hello", digit: 1 };
    println!("{structure_tu:?}");

    pubstruct!{
        #[derive(Debug)]
        StructureLT<'a> {
            greet: &'a str,
            digit: i32,
        }
    }

    let structure_lt = StructureLT{ greet: "hello", digit: 1 };
    println!("{structure_lt:?}");

    pubstruct!{
        #[derive(Debug)]
        StructureLTT<'a, T> {
            greet: &'a T,
            digit: i32,
        }
    }

    let structure_ltt = StructureLTT{ greet: &"hello".to_owned(), digit: 1 };
    println!("{structure_ltt:?}");
    
    pubstruct!{
        #[derive(Debug)]
        StructureLTU<'a, T, U> {
            greet: &'a T,
            digit: U,
        }
    }

    let structure_ltu = StructureLTU{ greet: &"hello".to_owned(), digit: 1 };
    println!("{structure_ltu:?}");
}
