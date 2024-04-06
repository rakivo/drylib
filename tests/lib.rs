#[cfg(test)]
mod tests {
    #[allow(unused)]
    use drylib::*;
    use drylib::drylib_proc_macros::*;

    #[test]
    fn muting() {
        muts!(a = 5; b = 2; c = 3);

        assert_eq!(a, 5);
        assert_eq!(b, 2);
        assert_eq!(c, 3);

        a = a * a;
        b = b * b;
        c = c * c;
        assert_eq!(a, 5 * 5);
        assert_eq!(b, 2 * 2);
        assert_eq!(c, 3 * 3);
    }

    #[test]
    fn cloning() {
        let t1 = 1;
        let t2 = 2;
        let t3 = 3;
        clones!(t1, t2, t3);

        assert_eq!(ct1, t1);
        assert_eq!(ct2, t2);
        assert_eq!(ct3, t3);
    }

    #[test]
    fn mut_cloning() {
        let t1 = 1;
        let t2 = 2;
        let t3 = 3;
        clones!(mut t1, mut t2, mut t3);

        assert_eq!(ct1, t1);
        assert_eq!(ct2, t2);
        assert_eq!(ct3, t3);
        
        ct1 = 2024;
        ct2 = 9;
        ct3 = 15;
        
        assert_eq!(ct1, 2024);
        assert_eq!(ct2, 9);
        assert_eq!(ct3, 15);
    }

    #[test]
    fn muting_and_cloning() {
        muts!(a = 5; b = 2; c = 3);

        assert_eq!(a, 5);
        assert_eq!(b, 2);
        assert_eq!(c, 3);

        a = a * a;
        b = b * b;
        c = c * c;
        assert_eq!(a, 5 * 5);
        assert_eq!(b, 2 * 2);
        assert_eq!(c, 3 * 3);

        clones!(a, b, c);

        assert_eq!(ca, a);
        assert_eq!(cb, b);
        assert_eq!(cc, c);
    }

    #[test]
    fn muting_and_mut_cloning() {
        muts!(a = 5; b = 2; c = 3);

        assert_eq!(a, 5);
        assert_eq!(b, 2);
        assert_eq!(c, 3);

        a = a * a;
        b = b * b;
        c = c * c;
        assert_eq!(a, 5 * 5);
        assert_eq!(b, 2 * 2);
        assert_eq!(c, 3 * 3);

        clones!(mut a, mut b, mut c);
        
        assert_eq!(ca, a);
        assert_eq!(cb, b);
        assert_eq!(cc, c);
        
        ca = 15;
        cb = 63;
        cc = 92;
        assert_eq!(ca, 15);
        assert_eq!(cb, 63);
        assert_eq!(cc, 92);
    }

    #[test]
    fn muting_and_muts_cloning() {
        muts!(a = 5; b = 2; c = 3);

        assert_eq!(a, 5);
        assert_eq!(b, 2);
        assert_eq!(c, 3);

        a = a * a;
        b = b * b;
        c = c * c;
        assert_eq!(a, 5 * 5);
        assert_eq!(b, 2 * 2);
        assert_eq!(c, 3 * 3);

        mutclones!(a, b, c);
        
        assert_eq!(ca, a);
        assert_eq!(cb, b);
        assert_eq!(cc, c);
        
        ca = 15;
        cb = 63;
        cc = 92;
        assert_eq!(ca, 15);
        assert_eq!(cb, 63);
        assert_eq!(cc, 92);
    }

    #[test]
    fn creating_structs() {
        pub_tupstruct!{
            #[derive(Debug)]
            Tuple(usize, i32, u32)
        };

        #[allow(unused)]
        let tuple = Tuple(0, 1, 2);

        pub_tupstruct!{
            #[derive(Debug)]
            TupleT<T>(T, i32, u32)
        };

        #[allow(unused)]
        let tuple_t = TupleT("hello", 1, 2);

        pub_tupstruct!{
            #[derive(Debug)]
            TupleTU<T, U>(T, i32, U)
        };

        #[allow(unused)]
        let tuple_tu = TupleTU("hello", 1, vec![1, 2, 3]);
        
        pub_tupstruct!{
            #[derive(Debug)]
            TupleLT<'a>(i32, &'a i32)
        };

        #[allow(unused)]
        let tuple_lt = TupleLT(0, &1);

        pub_tupstruct!{
            #[derive(Debug)]
            TupleLTT<'a, T>(T, &'a i32)
        }

        #[allow(unused)]
        let tuple_ltt = TupleLTT("hello", &1);

        pub_tupstruct!{
            #[derive(Debug)]
            TupleLTU<'a, T, U>(T, &'a i32, U)
        }

        #[allow(unused)]
        let tuple_ltu = TupleLTU("hello", &1, vec![1, 2, 3]);

        pubstruct!{
            #[derive(Debug)]
            Structure {
                greet: usize,
                digit: i32,
            }
        }

        #[allow(unused)]
        let structure = Structure{ greet: 0, digit: 1 };

        pubstruct!{
            #[derive(Debug)]
            StructureT<T> {
                greet: T,
                digit: i32,
            }
        }

        #[allow(unused)]
        let structure_t = StructureT{ greet: "hello", digit: 1 };
        
        pubstruct!{
            #[derive(Debug)]
            StructureTU<T, U> {
                greet: T,
                digit: U,
            }
        }

        #[allow(unused)]
        let structure_tu = StructureTU{ greet: "hello", digit: 1 };

        pubstruct!{
            #[derive(Debug)]
            StructureLT<'a> {
                greet: &'a str,
                digit: i32,
            }
        }

        #[allow(unused)]
        let structure_lt = StructureLT{ greet: "hello", digit: 1 };

        pubstruct!{
            #[derive(Debug)]
            StructureLTT<'a, T> {
                greet: &'a T,
                digit: i32,
            }
        }

        #[allow(unused)]
        let structure_ltt = StructureLTT{ greet: &"hello".to_owned(), digit: 1 };
        
        pubstruct!{
            #[derive(Debug)]
            StructureLTU<'a, T, U> {
                greet: &'a T,
                digit: U,
            }
        }

        #[allow(unused)]
        let structure_ltu = StructureLTU{ greet: &"hello".to_owned(), digit: 1 };
    }
}
