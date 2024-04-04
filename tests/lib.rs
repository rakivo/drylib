#[cfg(test)]
mod tests {
    use drylib::*;

    #[test]
    fn muting() {
        muts!(a = 5, b = 2, c = 3);

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
        muts!(a = 5, b = 2, c = 3);

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
        muts!(a = 5, b = 2, c = 3);

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
        muts!(a = 5, b = 2, c = 3);

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
}
