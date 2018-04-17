  use chapter1;

    #[test]
   fn test_sum_integration() {
       assert_eq!( 3 + 11, 14);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(15, 14), 1);
    }