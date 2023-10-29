extern crate approx_cmp;


macro_rules! impl_abs_diff_eq_integer_exhaustive_tests {
    ($(($module_name:ident, $T:ty)),*) => {$(
        #[cfg(test)]
        mod $module_name {
            use approx_cmp::{
                AbsDiffEq,
                assert_abs_diff_eq,
                assert_abs_diff_ne,
                abs_diff_eq,
                abs_diff_ne,
            };
    
    
            #[test]
            fn test_abs_diff_eq_exhaustive() {
                for i in <$T>::MIN..=<$T>::MAX {
                    assert!(i.abs_diff_eq(&i, &0));
                    assert!(abs_diff_eq!(i, i, abs_diff <= 0));
                    assert_abs_diff_eq!(i, i, abs_diff <= 0);
                }
            }
    
            #[test]
            fn test_abs_diff_ne_exhaustive() {
                for i in <$T>::MIN..<$T>::MAX {
                    assert!(i.abs_diff_ne(&(i + 1), &0));
                    assert!(abs_diff_ne!(i, i + 1, abs_diff <= 0));
                    assert_abs_diff_ne!(i, i + 1, abs_diff <= 0);

                    assert!((i + 1).abs_diff_ne(&i, &0));
                    assert!(abs_diff_ne!(i + 1, i, abs_diff <= 0));
                    assert_abs_diff_ne!(i + 1, i, abs_diff <= 0);
                }
            }
        }
    )*};
}

impl_abs_diff_eq_integer_exhaustive_tests!(
    (abs_diff_eq_u8_exhaustive_tests,  u8), 
    (abs_diff_eq_u16_exhaustive_tests, u16), 
    (abs_diff_eq_i8_exhaustive_tests,  i8), 
    (abs_diff_eq_i16_exhaustive_tests, i16)
);

macro_rules! impl_abs_diff_eq_float_exact_exhaustive_tests {
    ($(($module_name:ident, $FloatType:ty, $IntegerType:ty)),*) => {$(
        #[cfg(test)]
        mod $module_name {
            use approx_cmp::{
                AbsDiffEq,
                assert_abs_diff_eq,
                assert_abs_diff_ne,
                abs_diff_eq,
                abs_diff_ne,
            };
    
            #[test]
            fn test_abs_diff_eq_exactly_representable_exhaustive1() {
                for i in <$IntegerType>::MIN..<$IntegerType>::MAX {
                    assert!((i as $FloatType).abs_diff_eq(&(i as $FloatType), &0.0));
                    assert!(abs_diff_eq!(i as $FloatType, i as $FloatType, abs_diff <= 0.0));
                    assert_abs_diff_eq!(i as $FloatType, i as $FloatType, abs_diff <= 0.0);
                }
            }
        
            #[test]
            fn test_abs_diff_eq_exactly_representable_exhaustive2() {
                for i in <$IntegerType>::MIN..<$IntegerType>::MAX {
                    assert_abs_diff_eq!(i as $FloatType, i as $FloatType, abs_diff <= 0.0);
                }
            }
        
            #[test]
            fn test_abs_diff_ne_exactly_representable_exhaustive1() {
                for i in <$IntegerType>::MIN..<$IntegerType>::MAX {
                    assert!(((i + 1) as $FloatType).abs_diff_ne(&(i as $FloatType), &0.0));
                    assert!(abs_diff_ne!((i + 1) as $FloatType, i as $FloatType, abs_diff <= 0.0));
                    assert_abs_diff_ne!((i + 1) as $FloatType, i as $FloatType, abs_diff <= 0.0);

                    assert!((i as $FloatType).abs_diff_ne(&((i + 1) as $FloatType), &0.0));
                    assert!(abs_diff_ne!(i as $FloatType, (i + 1) as $FloatType, abs_diff <= 0.0));
                    assert_abs_diff_ne!(i as $FloatType, (i + 1) as $FloatType, abs_diff <= 0.0);
                }
            }
        
            #[test]
            fn test_abs_diff_ne_exactly_representable_exhaustive2() {
                for i in <$IntegerType>::MIN..<$IntegerType>::MAX {
                    assert_abs_diff_ne!((i + 1) as $FloatType, i as $FloatType, abs_diff <= 0.0);
                    assert_abs_diff_ne!(i as $FloatType, (i + 1) as $FloatType, abs_diff <= 0.0);
                }
            }
        }
    )*};
}

impl_abs_diff_eq_float_exact_exhaustive_tests!(
    (abs_diff_eq_f32_u8_exact_exhaustive_tests,  f32, u8),
    (abs_diff_eq_f32_u16_exact_exhaustive_tests, f32, u16),
    (abs_diff_eq_f32_i8_exact_exhaustive_tests,  f32, i8),
    (abs_diff_eq_f32_i16_exact_exhaustive_tests, f32, i16),
    (abs_diff_eq_f64_u8_exact_exhaustive_tests,  f64, u8),
    (abs_diff_eq_f64_u16_exact_exhaustive_tests, f64, u16),
    (abs_diff_eq_f64_i8_exact_exhaustive_tests,  f64, i8),
    (abs_diff_eq_f64_i16_exact_exhaustive_tests, f64, i16)
);

#[cfg(test)]
mod abs_diff_compare_f32_tests {
    use approx_cmp::{
        AbsDiffEq,
        AbsDiffAllEq,
        assert_abs_diff_eq,
        assert_abs_diff_ne,
        abs_diff_eq,
        abs_diff_ne,
    };

    fn check_abs_diff_eq(a: f32, b: f32, tolerance: f32) {
        assert!(a.abs_diff_eq(&b, &tolerance));
        assert!(abs_diff_eq!(a, b, abs_diff <= tolerance));
        assert_abs_diff_eq!(a, b, abs_diff <= tolerance);
        
        assert!(a.abs_diff_all_eq(&b, &tolerance));
        assert!(abs_diff_eq!(a, b, abs_diff_all <= tolerance));
        assert_abs_diff_eq!(a, b, abs_diff_all <= tolerance);
    }

    fn check_abs_diff_ne(a: f32, b: f32, tolerance: f32) {
        assert!(a.abs_diff_ne(&b, &tolerance));
        assert!(abs_diff_ne!(a, b, abs_diff <= tolerance));
        assert_abs_diff_ne!(a, b, abs_diff <= tolerance);

        assert!(a.abs_diff_all_ne(&b, &tolerance));
        assert!(abs_diff_ne!(a, b, abs_diff_all <= tolerance));
        assert_abs_diff_ne!(a, b, abs_diff_all <= tolerance);
    }

    fn check_eq(a: f32, b: f32, tolerance: f32) {
        check_abs_diff_eq(a, b, tolerance);
        check_abs_diff_eq(b, a, tolerance);
        check_abs_diff_eq(-a, -b, tolerance);
        check_abs_diff_eq(-b, -a, tolerance);
    }

    fn check_ne(a: f32, b: f32, tolerance: f32) {
        check_abs_diff_ne(a, b, tolerance);
        check_abs_diff_ne(b, a, tolerance);
        check_abs_diff_ne(-a, -b, tolerance);
        check_abs_diff_ne(-b, -a, tolerance);
    }

    fn check_eq_self(value: f32) {
        check_eq(value, value, 0.0);
        check_eq(value, value, f32::MIN_POSITIVE);
        check_eq(value, value, f32::MAX);
        check_eq(value, value, f32::INFINITY);
    }

    #[test]
    fn test_eq_zero() {
        check_eq_self(0.0);

        check_eq( 0.0,  0.0, f32::EPSILON);
        check_eq(-0.0,  0.0, f32::EPSILON);
        check_eq( 0.0, -0.0, f32::EPSILON);
        check_eq(-0.0, -0.0, f32::EPSILON);
    }

    #[test]
    fn test_ne_zero() {
        check_ne( 0.000001_f32, 0.0_f32,      f32::EPSILON);
        check_ne( 0.0_f32,      0.000001_f32, f32::EPSILON);
        check_ne(-0.000001_f32, 0.0_f32,      f32::EPSILON);
        check_ne( 0.0_f32,     -0.000001_f32, f32::EPSILON);
    }

    #[test]
    fn test_eq_tolerance() {
        check_eq( 0.0_f32,    1e-40_f32, 1e-40_f32);
        check_eq( 1e-40_f32,  0.0_f32,   1e-40_f32);
        check_eq( 0.0_f32,   -1e-40_f32, 1e-40_f32);
        check_eq(-1e-40_f32,  0.0_f32,   1e-40_f32);
    }

    #[test]
    fn test_ne_tolerance() {
        check_ne( 1e-40_f32,  0.0_f32,   1e-41_f32);
        check_ne( 0.0_f32,    1e-40_f32, 1e-41_f32);
        check_ne(-1e-40_f32,  0.0_f32,   1e-41_f32);
        check_ne( 0.0_f32,   -1e-40_f32, 1e-41_f32);
    }

    #[test]
    fn test_eq_self() {
        check_eq_self(-1.0_f32);
        check_eq_self(-2.0_f32);
        check_eq_self(-3.0_f32);
        check_eq_self(-4.0_f32);
        check_eq_self(-5.0_f32);
        check_eq_self(-6.0_f32);
        check_eq_self(-7.0_f32);
        check_eq_self(-8.0_f32);
        check_eq_self(-9.0_f32);
        check_eq_self(-10.0_f32);
        check_eq_self(-11.0_f32);
        check_eq_self(-12.0_f32);
        check_eq_self(-13.0_f32);
        check_eq_self(-14.0_f32);
        check_eq_self(-15.0_f32);
        check_eq_self(-16.0_f32);

        check_eq_self(1.0_f32);
        check_eq_self(2.0_f32);
        check_eq_self(3.0_f32);
        check_eq_self(4.0_f32);
        check_eq_self(5.0_f32);
        check_eq_self(6.0_f32);
        check_eq_self(7.0_f32);
        check_eq_self(8.0_f32);
        check_eq_self(9.0_f32);
        check_eq_self(10.0_f32);
        check_eq_self(11.0_f32);
        check_eq_self(12.0_f32);
        check_eq_self(13.0_f32);
        check_eq_self(14.0_f32);
        check_eq_self(15.0_f32);
        check_eq_self(16.0_f32);
    }

    #[test]
    fn test_ne() {
        check_ne( 1.0_f32,  2.0_f32, f32::EPSILON);
        check_ne( 1.0_f32,  2.0_f32, f32::MIN_POSITIVE);
        check_ne( 1.0_f32, -2.0_f32, f32::EPSILON);
        check_ne( 1.0_f32, -2.0_f32, f32::MIN_POSITIVE);
        check_ne(-1.0_f32,  2.0_f32, f32::EPSILON);
        check_ne(-1.0_f32,  2.0_f32, f32::MIN_POSITIVE);
        check_ne(-1.0_f32, -2.0_f32, f32::EPSILON);
        check_ne(-1.0_f32, -2.0_f32, f32::MIN_POSITIVE);
    }

    #[test]
    fn test_eq_rounding1() {
        check_eq( 100000000.0_f32,  100000001.0_f32, f32::EPSILON);
        check_eq( 100000001.0_f32,  100000000.0_f32, f32::EPSILON);
        check_eq(-100000000.0_f32, -100000001.0_f32, f32::EPSILON);
        check_eq(-100000001.0_f32, -100000000.0_f32, f32::EPSILON);

        check_eq( 1000000000.0_f32,  1000000001.0_f32, f32::EPSILON);
        check_eq( 1000000001.0_f32,  1000000000.0_f32, f32::EPSILON);
        check_eq(-1000000000.0_f32, -1000000001.0_f32, f32::EPSILON);
        check_eq(-1000000001.0_f32, -1000000000.0_f32, f32::EPSILON);

        check_eq( 10000000000.0_f32,  10000000001.0_f32, f32::EPSILON);
        check_eq( 10000000001.0_f32,  10000000000.0_f32, f32::EPSILON);
        check_eq(-10000000000.0_f32, -10000000001.0_f32, f32::EPSILON);
        check_eq(-10000000001.0_f32, -10000000000.0_f32, f32::EPSILON);

        check_eq( 100000000000.0_f32,  100000000001.0_f32, f32::EPSILON);
        check_eq( 100000000001.0_f32,  100000000000.0_f32, f32::EPSILON);
        check_eq(-100000000000.0_f32, -100000000001.0_f32, f32::EPSILON);
        check_eq(-100000000001.0_f32, -100000000000.0_f32, f32::EPSILON);

        check_eq( 1000000000000.0_f32,  1000000000001.0_f32, f32::EPSILON);
        check_eq( 1000000000001.0_f32,  1000000000000.0_f32, f32::EPSILON);
        check_eq(-1000000000000.0_f32, -1000000000001.0_f32, f32::EPSILON);
        check_eq(-1000000000001.0_f32, -1000000000000.0_f32, f32::EPSILON);
    }

    #[test]
    fn test_ne_rounding1() {
        check_ne( 1000.0_f32,  1001.0_f32, f32::EPSILON);
        check_ne( 1001.0_f32,  1000.0_f32, f32::EPSILON);
        check_ne(-1000.0_f32, -1001.0_f32, f32::EPSILON);
        check_ne(-1001.0_f32, -1000.0_f32, f32::EPSILON);

        check_ne( 10000.0_f32,  10001.0_f32, f32::EPSILON);
        check_ne( 10001.0_f32,  10000.0_f32, f32::EPSILON);
        check_ne(-10000.0_f32, -10001.0_f32, f32::EPSILON);
        check_ne(-10001.0_f32, -10000.0_f32, f32::EPSILON);

        check_ne( 100000.0_f32,  100001.0_f32, f32::EPSILON);
        check_ne( 100001.0_f32,  100000.0_f32, f32::EPSILON);
        check_ne(-100000.0_f32, -100001.0_f32, f32::EPSILON);
        check_ne(-100001.0_f32, -100000.0_f32, f32::EPSILON);

        check_ne( 1000000.0_f32,  1000001.0_f32, f32::EPSILON);
        check_ne( 1000001.0_f32,  1000000.0_f32, f32::EPSILON);
        check_ne(-1000000.0_f32, -1000001.0_f32, f32::EPSILON);
        check_ne(-1000001.0_f32, -1000000.0_f32, f32::EPSILON);
        
        check_ne( 10000000.0_f32,  10000001.0_f32, f32::EPSILON);
        check_ne( 10000001.0_f32,  10000000.0_f32, f32::EPSILON);
        check_ne(-10000000.0_f32, -10000001.0_f32, f32::EPSILON);
        check_ne(-10000001.0_f32, -10000000.0_f32, f32::EPSILON);
    }

    #[test]
    fn test_eq_rounding2() {
        check_eq( 1.0000001_f32,  1.0000002_f32, f32::EPSILON);
        check_eq( 1.0000002_f32,  1.0000001_f32, f32::EPSILON);
        check_eq(-1.0000001_f32, -1.0000002_f32, f32::EPSILON);
        check_eq(-1.0000002_f32, -1.0000001_f32, f32::EPSILON);

        check_eq( 1.00000001_f32,  1.00000002_f32, f32::EPSILON);
        check_eq( 1.00000002_f32,  1.00000001_f32, f32::EPSILON);
        check_eq(-1.00000001_f32, -1.00000002_f32, f32::EPSILON);
        check_eq(-1.00000002_f32, -1.00000001_f32, f32::EPSILON);

        check_eq( 1.000000001_f32,  1.000000002_f32, f32::EPSILON);
        check_eq( 1.000000002_f32,  1.000000001_f32, f32::EPSILON);
        check_eq(-1.000000001_f32, -1.000000002_f32, f32::EPSILON);
        check_eq(-1.000000002_f32, -1.000000001_f32, f32::EPSILON);

        check_eq( 1.0000000001_f32,  1.0000000002_f32, f32::EPSILON);
        check_eq( 1.0000000002_f32,  1.0000000001_f32, f32::EPSILON);
        check_eq(-1.0000000001_f32, -1.0000000002_f32, f32::EPSILON);
        check_eq(-1.0000000002_f32, -1.0000000001_f32, f32::EPSILON);

        check_eq( 1.00000000001_f32,  1.00000000002_f32, f32::EPSILON);
        check_eq( 1.00000000002_f32,  1.00000000001_f32, f32::EPSILON);
        check_eq(-1.00000000001_f32, -1.00000000002_f32, f32::EPSILON);
        check_eq(-1.00000000002_f32, -1.00000000001_f32, f32::EPSILON);
    }

    #[test]
    fn test_ne_rounding2() {
        check_ne( 1.01_f32,  1.02_f32, f32::EPSILON);
        check_ne( 1.02_f32,  1.01_f32, f32::EPSILON);
        check_ne(-1.01_f32, -1.02_f32, f32::EPSILON);
        check_ne(-1.02_f32, -1.01_f32, f32::EPSILON);

        check_ne( 1.001_f32,  1.002_f32, f32::EPSILON);
        check_ne( 1.002_f32,  1.001_f32, f32::EPSILON);
        check_ne(-1.001_f32, -1.002_f32, f32::EPSILON);
        check_ne(-1.002_f32, -1.001_f32, f32::EPSILON);

        check_ne( 1.0001_f32,  1.0002_f32, f32::EPSILON);
        check_ne( 1.0002_f32,  1.0001_f32, f32::EPSILON);
        check_ne(-1.0001_f32, -1.0002_f32, f32::EPSILON);
        check_ne(-1.0002_f32, -1.0001_f32, f32::EPSILON);

        check_ne( 1.00001_f32,  1.00002_f32, f32::EPSILON);
        check_ne( 1.00002_f32,  1.00001_f32, f32::EPSILON);
        check_ne(-1.00001_f32, -1.00002_f32, f32::EPSILON);
        check_ne(-1.00002_f32, -1.00001_f32, f32::EPSILON);

        check_ne( 1.000001_f32,  1.000002_f32, f32::EPSILON);
        check_ne( 1.000002_f32,  1.000001_f32, f32::EPSILON);
        check_ne(-1.000001_f32, -1.000002_f32, f32::EPSILON);
        check_ne(-1.000002_f32, -1.000001_f32, f32::EPSILON);
    }

    #[test]
    fn test_eq_max() {
        check_eq( f32::MAX,  f32::MAX, f32::EPSILON);
    }

    #[test]
    fn test_ne_max() {
        check_ne( f32::MAX, -f32::MAX,           f32::EPSILON);
        check_ne(-f32::MAX,  f32::MAX,           f32::EPSILON);
        check_ne( f32::MAX,  f32::MAX / 2.0_f32, f32::EPSILON);
        check_ne( f32::MAX, -f32::MAX / 2.0_f32, f32::EPSILON);
        check_ne(-f32::MAX,  f32::MAX / 2.0_f32, f32::EPSILON);
    }

    #[test]
    fn test_ne_nan1() {
        check_ne( f32::NAN,  f32::NAN, 0_f32);
        check_ne( f32::NAN, -f32::NAN, 0_f32);
        check_ne(-f32::NAN,  f32::NAN, 0_f32);
        check_ne(-f32::NAN, -f32::NAN, 0_f32);
    }

    #[test]
    fn test_ne_nan2() {
        for i in 0..=i16::MAX {
            check_ne( f32::NAN , f32::NAN, 1_f32 / (i as f32));
            check_ne( f32::NAN, -f32::NAN, 1_f32 / (i as f32));
            check_ne(-f32::NAN,  f32::NAN, 1_f32 / (i as f32));
            check_ne(-f32::NAN, -f32::NAN, 1_f32 / (i as f32));
        }
    }

    #[test]
    fn test_ne_nan3() {
        for i in 0..=i16::MAX {
            check_ne( f32::NAN , f32::NAN, i as f32);
            check_ne( f32::NAN, -f32::NAN, i as f32);
            check_ne(-f32::NAN,  f32::NAN, i as f32);
            check_ne(-f32::NAN, -f32::NAN, i as f32);
        }
    }

    #[test]
    fn test_ne_nan4() {
        check_ne(f32::NAN, f32::NAN, f32::EPSILON);
        
        check_ne( f32::NAN,  0.0_f32,  f32::EPSILON);
        check_ne(-0_f32,     f32::NAN, f32::EPSILON);
        check_ne( f32::NAN, -0.0_f32,  f32::EPSILON);
        check_ne( 0_f32,     f32::NAN, f32::EPSILON);

        check_ne( f32::NAN, f32::INFINITY,  f32::EPSILON);
        check_ne( f32::INFINITY, f32::NAN,  f32::EPSILON);
        check_ne( f32::NAN, -f32::INFINITY, f32::EPSILON);
        check_ne(-f32::INFINITY, f32::NAN,  f32::EPSILON);

        check_ne( f32::NAN, f32::MAX,  f32::EPSILON);
        check_ne( f32::MAX, f32::NAN,  f32::EPSILON);
        check_ne( f32::NAN, -f32::MAX, f32::EPSILON);
        check_ne(-f32::MAX, f32::NAN,  f32::EPSILON);

        check_ne( f32::NAN,          f32::MIN_POSITIVE,  f32::EPSILON);
        check_ne( f32::MIN_POSITIVE, f32::NAN,           f32::EPSILON);
        check_ne( f32::NAN,          -f32::MIN_POSITIVE, f32::EPSILON);
        check_ne(-f32::MIN_POSITIVE, f32::NAN,           f32::EPSILON);

        check_ne( f32::NAN,  1_f32,    f32::EPSILON);
        check_ne( f32::NAN, -1_f32,    f32::EPSILON);
        check_ne( 1_f32,     f32::NAN, f32::EPSILON);
        check_ne(-1_f32,     f32::NAN, f32::EPSILON);
    }

    #[test]
    fn test_nan5() {
        check_ne(f32::NAN, f32::NAN, 1_f32);
        
        check_ne( f32::NAN,  0.0_f32,  1_f32);
        check_ne(-0_f32,     f32::NAN, 1_f32);
        check_ne( f32::NAN, -0.0_f32,  1_f32);
        check_ne( 0_f32,     f32::NAN, 1_f32);

        check_ne( f32::NAN, f32::INFINITY,  1_f32);
        check_ne( f32::INFINITY, f32::NAN,  1_f32);
        check_ne( f32::NAN, -f32::INFINITY, 1_f32);
        check_ne(-f32::INFINITY, f32::NAN,  1_f32);

        check_ne( f32::NAN, f32::MAX,  1_f32);
        check_ne( f32::MAX, f32::NAN,  1_f32);
        check_ne( f32::NAN, -f32::MAX, 1_f32);
        check_ne(-f32::MAX, f32::NAN,  1_f32);

        check_ne( f32::NAN,          f32::MIN_POSITIVE,  1_f32);
        check_ne( f32::MIN_POSITIVE, f32::NAN,           1_f32);
        check_ne( f32::NAN,          -f32::MIN_POSITIVE, 1_f32);
        check_ne(-f32::MIN_POSITIVE, f32::NAN,           1_f32);

        check_ne( f32::NAN,  1_f32,    1_f32);
        check_ne( f32::NAN, -1_f32,    1_f32);
        check_ne( 1_f32,     f32::NAN, 1_f32);
        check_ne(-1_f32,     f32::NAN, 1_f32);
    }

    #[test]
    fn test_eq_infinity() {
        check_eq(f32::INFINITY,     f32::INFINITY,     f32::INFINITY);
        check_eq(f32::INFINITY,     f32::MAX,          f32::INFINITY);
        check_eq(f32::INFINITY,     f32::NEG_INFINITY, f32::INFINITY);
        check_eq(f32::NEG_INFINITY, f32::MAX,          f32::INFINITY);
    }

    #[test]
    fn test_ne_infinity() {
        // check_ne(f32::INFINITY,     f32::INFINITY,     f32::MAX);
        check_ne(f32::INFINITY,     f32::MAX,          f32::MAX);
        check_ne(f32::INFINITY,     f32::NEG_INFINITY, f32::MAX);
        check_ne(f32::NEG_INFINITY, f32::MAX,          f32::MAX);
    }

    #[test]
    fn test_eq_near_zero1() {
        check_eq( f32::MIN_POSITIVE,  f32::MIN_POSITIVE, f32::EPSILON);
        check_eq( f32::MIN_POSITIVE, -f32::MIN_POSITIVE, f32::EPSILON);
        check_eq(-f32::MIN_POSITIVE,  f32::MIN_POSITIVE, f32::EPSILON);

        check_eq( f32::MIN_POSITIVE,  0_f32,             f32::EPSILON);
        check_eq( 0_f32,              f32::MIN_POSITIVE, f32::EPSILON);
        check_eq(-f32::MIN_POSITIVE,  0_f32,             f32::EPSILON);
        check_eq( 0_f32,             -f32::MIN_POSITIVE, f32::EPSILON);

        check_eq( 0.0000001_f32,      -f32::MIN_POSITIVE, f32::EPSILON);
        check_eq( 0.0000001_f32,       f32::MIN_POSITIVE, f32::EPSILON);
        check_eq( f32::MIN_POSITIVE,  0.0000001_f32,      f32::EPSILON);
        check_eq(-f32::MIN_POSITIVE,  0.0000001_f32,      f32::EPSILON);
    }

    #[test]
    fn test_eq_near_zero2() {
        check_eq( 0.000010001_f32,  0.000010002_f32, f32::EPSILON);
        check_eq( 0.000010002_f32,  0.000010001_f32, f32::EPSILON);
        check_eq(-0.000010001_f32, -0.000010002_f32, f32::EPSILON);
        check_eq(-0.000010002_f32, -0.000010001_f32, f32::EPSILON);
    }

    #[test]
    fn test_eq_near_zero3() {
        check_eq(1e-8_f32,  -1e-8_f32,  f32::EPSILON);
        check_eq(1e-9_f32,  -1e-9_f32,  f32::EPSILON);
        check_eq(1e-10_f32, -1e-10_f32, f32::EPSILON);
        check_eq(1e-11_f32, -1e-11_f32, f32::EPSILON);
        check_eq(1e-12_f32, -1e-12_f32, f32::EPSILON);
        check_eq(1e-13_f32, -1e-13_f32, f32::EPSILON);
        check_eq(1e-14_f32, -1e-14_f32, f32::EPSILON);
        check_eq(1e-15_f32, -1e-15_f32, f32::EPSILON);
        check_eq(1e-16_f32, -1e-16_f32, f32::EPSILON);
        check_eq(1e-17_f32, -1e-17_f32, f32::EPSILON);
        check_eq(1e-18_f32, -1e-18_f32, f32::EPSILON);
        check_eq(1e-19_f32, -1e-19_f32, f32::EPSILON);
        check_eq(1e-20_f32, -1e-20_f32, f32::EPSILON);
    }

    #[test]
    fn test_ne_near_zero1() {
        check_ne( 0.000001_f32,      -f32::MIN_POSITIVE, f32::EPSILON);
        check_ne( 0.000001_f32,       f32::MIN_POSITIVE, f32::EPSILON);
        check_ne( f32::MIN_POSITIVE,  0.000001_f32,      f32::EPSILON);
        check_ne(-f32::MIN_POSITIVE,  0.000001_f32,      f32::EPSILON);

        check_ne(-0.000001_f32,      -f32::MIN_POSITIVE, f32::EPSILON);
        check_ne(-0.000001_f32,       f32::MIN_POSITIVE, f32::EPSILON);
        check_ne( f32::MIN_POSITIVE, -0.000001_f32,      f32::EPSILON);
        check_ne(-f32::MIN_POSITIVE, -0.000001_f32,      f32::EPSILON);
    }

    #[test]
    fn test_ne_near_zero2() {
        check_ne( 0.000001002_f32,  0.0000001001_f32, f32::EPSILON);
        check_ne( 0.000001001_f32,  0.0000001002_f32, f32::EPSILON);
        check_ne(-0.000001002_f32, -0.0000001001_f32, f32::EPSILON);
        check_ne(-0.000001001_f32, -0.0000001002_f32, f32::EPSILON);
    }
}

#[cfg(test)]
mod abs_diff_compare_f64_tests {
    use approx_cmp::{
        AbsDiffEq,
        AbsDiffAllEq,
        assert_abs_diff_eq,
        assert_abs_diff_ne,
        abs_diff_eq,
        abs_diff_ne,
    };

    fn check_abs_diff_eq(a: f64, b: f64, tolerance: f64) {
        assert!(a.abs_diff_eq(&b, &tolerance));
        assert!(abs_diff_eq!(a, b, abs_diff <= tolerance));
        assert_abs_diff_eq!(a, b, abs_diff <= tolerance);
        
        assert!(a.abs_diff_all_eq(&b, &tolerance));
        assert!(abs_diff_eq!(a, b, abs_diff_all <= tolerance));
        assert_abs_diff_eq!(a, b, abs_diff_all <= tolerance);
    }

    fn check_abs_diff_ne(a: f64, b: f64, tolerance: f64) {
        assert!(a.abs_diff_ne(&b, &tolerance));
        assert!(abs_diff_ne!(a, b, abs_diff <= tolerance));
        assert_abs_diff_ne!(a, b, abs_diff <= tolerance);

        assert!(a.abs_diff_all_ne(&b, &tolerance));
        assert!(abs_diff_ne!(a, b, abs_diff_all <= tolerance));
        assert_abs_diff_ne!(a, b, abs_diff_all <= tolerance);
    }

    fn check_eq(a: f64, b: f64, tolerance: f64) {
        check_abs_diff_eq(a, b, tolerance);
        check_abs_diff_eq(b, a, tolerance);
        check_abs_diff_eq(-a, -b, tolerance);
        check_abs_diff_eq(-b, -a, tolerance);
    }

    fn check_ne(a: f64, b: f64, tolerance: f64) {
        check_abs_diff_ne(a, b, tolerance);
        check_abs_diff_ne(b, a, tolerance);
        check_abs_diff_ne(-a, -b, tolerance);
        check_abs_diff_ne(-b, -a, tolerance);
    }

    fn check_eq_self(value: f64) {
        check_eq(value, value, 0.0);
        check_eq(value, value, f64::MIN_POSITIVE);
        check_eq(value, value, f64::MAX);
        check_eq(value, value, f64::INFINITY);
    }

    #[test]
    fn test_eq_zero() {
        check_eq_self(0.0);

        check_eq( 0.0,  0.0, f64::EPSILON);
        check_eq(-0.0,  0.0, f64::EPSILON);
        check_eq( 0.0, -0.0, f64::EPSILON);
        check_eq(-0.0, -0.0, f64::EPSILON);
    }

    #[test]
    fn test_ne_zero() {
        check_ne( 0.000001_f64, 0.0_f64,      f64::EPSILON);
        check_ne( 0.0_f64,      0.000001_f64, f64::EPSILON);
        check_ne(-0.000001_f64, 0.0_f64,      f64::EPSILON);
        check_ne( 0.0_f64,     -0.000001_f64, f64::EPSILON);
    }

    #[test]
    fn test_eq_tolerance() {
        check_eq( 0.0_f64,    1e-40_f64, 1e-40_f64);
        check_eq( 1e-40_f64,  0.0_f64,   1e-40_f64);
        check_eq( 0.0_f64,   -1e-40_f64, 1e-40_f64);
        check_eq(-1e-40_f64,  0.0_f64,   1e-40_f64);
    }

    #[test]
    fn test_ne_tolerance() {
        check_ne( 1e-40_f64,  0.0_f64,   1e-41_f64);
        check_ne( 0.0_f64,    1e-40_f64, 1e-41_f64);
        check_ne(-1e-40_f64,  0.0_f64,   1e-41_f64);
        check_ne( 0.0_f64,   -1e-40_f64, 1e-41_f64);
    }

    #[test]
    fn test_eq_self() {
        check_eq_self(-1.0_f64);
        check_eq_self(-2.0_f64);
        check_eq_self(-3.0_f64);
        check_eq_self(-4.0_f64);
        check_eq_self(-5.0_f64);
        check_eq_self(-6.0_f64);
        check_eq_self(-7.0_f64);
        check_eq_self(-8.0_f64);
        check_eq_self(-9.0_f64);
        check_eq_self(-10.0_f64);
        check_eq_self(-11.0_f64);
        check_eq_self(-12.0_f64);
        check_eq_self(-13.0_f64);
        check_eq_self(-14.0_f64);
        check_eq_self(-15.0_f64);
        check_eq_self(-16.0_f64);

        check_eq_self(1.0_f64);
        check_eq_self(2.0_f64);
        check_eq_self(3.0_f64);
        check_eq_self(4.0_f64);
        check_eq_self(5.0_f64);
        check_eq_self(6.0_f64);
        check_eq_self(7.0_f64);
        check_eq_self(8.0_f64);
        check_eq_self(9.0_f64);
        check_eq_self(10.0_f64);
        check_eq_self(11.0_f64);
        check_eq_self(12.0_f64);
        check_eq_self(13.0_f64);
        check_eq_self(14.0_f64);
        check_eq_self(15.0_f64);
        check_eq_self(16.0_f64);
    }

    #[test]
    fn test_ne() {
        check_ne( 1.0_f64,  2.0_f64, f64::EPSILON);
        check_ne( 1.0_f64,  2.0_f64, f64::MIN_POSITIVE);
        check_ne( 1.0_f64, -2.0_f64, f64::EPSILON);
        check_ne( 1.0_f64, -2.0_f64, f64::MIN_POSITIVE);
        check_ne(-1.0_f64,  2.0_f64, f64::EPSILON);
        check_ne(-1.0_f64,  2.0_f64, f64::MIN_POSITIVE);
        check_ne(-1.0_f64, -2.0_f64, f64::EPSILON);
        check_ne(-1.0_f64, -2.0_f64, f64::MIN_POSITIVE);
    }

    #[test]
    fn test_eq_rounding1() {
        check_eq( 10000000000000000.0_f64,  10000000000000001.0_f64, f64::EPSILON);
        check_eq( 10000000000000001.0_f64,  10000000000000000.0_f64, f64::EPSILON);
        check_eq(-10000000000000000.0_f64, -10000000000000001.0_f64, f64::EPSILON);
        check_eq(-10000000000000001.0_f64, -10000000000000000.0_f64, f64::EPSILON);

        check_eq( 100000000000000000.0_f64,  100000000000000001.0_f64, f64::EPSILON);
        check_eq( 100000000000000001.0_f64,  100000000000000000.0_f64, f64::EPSILON);
        check_eq(-100000000000000000.0_f64, -100000000000000001.0_f64, f64::EPSILON);
        check_eq(-100000000000000001.0_f64, -100000000000000000.0_f64, f64::EPSILON);

        check_eq( 1000000000000000000.0_f64,  1000000000000000001.0_f64, f64::EPSILON);
        check_eq( 1000000000000000001.0_f64,  1000000000000000000.0_f64, f64::EPSILON);
        check_eq(-1000000000000000000.0_f64, -1000000000000000001.0_f64, f64::EPSILON);
        check_eq(-1000000000000000001.0_f64, -1000000000000000000.0_f64, f64::EPSILON);

        check_eq( 10000000000000000000.0_f64,  10000000000000000001.0_f64, f64::EPSILON);
        check_eq( 10000000000000000001.0_f64,  10000000000000000000.0_f64, f64::EPSILON);
        check_eq(-10000000000000000000.0_f64, -10000000000000000001.0_f64, f64::EPSILON);
        check_eq(-10000000000000000001.0_f64, -10000000000000000000.0_f64, f64::EPSILON);

        check_eq( 100000000000000000000.0_f64,  100000000000000000001.0_f64, f64::EPSILON);
        check_eq( 100000000000000000001.0_f64,  100000000000000000000.0_f64, f64::EPSILON);
        check_eq(-100000000000000000000.0_f64, -100000000000000000001.0_f64, f64::EPSILON);
        check_eq(-100000000000000000001.0_f64, -100000000000000000000.0_f64, f64::EPSILON);
    }

    #[test]
    fn test_ne_rounding1() {
        check_ne( 1000.0_f64,  1001.0_f64, f64::EPSILON);
        check_ne( 1001.0_f64,  1000.0_f64, f64::EPSILON);
        check_ne(-1000.0_f64, -1001.0_f64, f64::EPSILON);
        check_ne(-1001.0_f64, -1000.0_f64, f64::EPSILON);

        check_ne( 10000.0_f64,  10001.0_f64, f64::EPSILON);
        check_ne( 10001.0_f64,  10000.0_f64, f64::EPSILON);
        check_ne(-10000.0_f64, -10001.0_f64, f64::EPSILON);
        check_ne(-10001.0_f64, -10000.0_f64, f64::EPSILON);

        check_ne( 100000.0_f64,  100001.0_f64, f64::EPSILON);
        check_ne( 100001.0_f64,  100000.0_f64, f64::EPSILON);
        check_ne(-100000.0_f64, -100001.0_f64, f64::EPSILON);
        check_ne(-100001.0_f64, -100000.0_f64, f64::EPSILON);

        check_ne( 1000000.0_f64,  1000001.0_f64, f64::EPSILON);
        check_ne( 1000001.0_f64,  1000000.0_f64, f64::EPSILON);
        check_ne(-1000000.0_f64, -1000001.0_f64, f64::EPSILON);
        check_ne(-1000001.0_f64, -1000000.0_f64, f64::EPSILON);
        
        check_ne( 10000000.0_f64,  10000001.0_f64, f64::EPSILON);
        check_ne( 10000001.0_f64,  10000000.0_f64, f64::EPSILON);
        check_ne(-10000000.0_f64, -10000001.0_f64, f64::EPSILON);
        check_ne(-10000001.0_f64, -10000000.0_f64, f64::EPSILON);

        check_ne( 100000000.0_f64,  100000001.0_f64, f64::EPSILON);
        check_ne( 100000001.0_f64,  100000000.0_f64, f64::EPSILON);
        check_ne(-100000000.0_f64, -100000001.0_f64, f64::EPSILON);
        check_ne(-100000001.0_f64, -100000000.0_f64, f64::EPSILON);

        check_ne( 1000000000.0_f64,  1000000001.0_f64, f64::EPSILON);
        check_ne( 1000000001.0_f64,  1000000000.0_f64, f64::EPSILON);
        check_ne(-1000000000.0_f64, -1000000001.0_f64, f64::EPSILON);
        check_ne(-1000000001.0_f64, -1000000000.0_f64, f64::EPSILON);

        check_ne( 10000000000.0_f64,  10000000001.0_f64, f64::EPSILON);
        check_ne( 10000000001.0_f64,  10000000000.0_f64, f64::EPSILON);
        check_ne(-10000000000.0_f64, -10000000001.0_f64, f64::EPSILON);
        check_ne(-10000000001.0_f64, -10000000000.0_f64, f64::EPSILON);

        check_ne( 100000000000.0_f64,  100000000001.0_f64, f64::EPSILON);
        check_ne( 100000000001.0_f64,  100000000000.0_f64, f64::EPSILON);
        check_ne(-100000000000.0_f64, -100000000001.0_f64, f64::EPSILON);
        check_ne(-100000000001.0_f64, -100000000000.0_f64, f64::EPSILON);

        check_ne( 1000000000000.0_f64,  1000000000001.0_f64, f64::EPSILON);
        check_ne( 1000000000001.0_f64,  1000000000000.0_f64, f64::EPSILON);
        check_ne(-1000000000000.0_f64, -1000000000001.0_f64, f64::EPSILON);
        check_ne(-1000000000001.0_f64, -1000000000000.0_f64, f64::EPSILON);

        check_ne( 10000000000000.0_f64,  10000000000001.0_f64, f64::EPSILON);
        check_ne( 10000000000001.0_f64,  10000000000000.0_f64, f64::EPSILON);
        check_ne(-10000000000000.0_f64, -10000000000001.0_f64, f64::EPSILON);
        check_ne(-10000000000001.0_f64, -10000000000000.0_f64, f64::EPSILON);

        check_ne( 100000000000000.0_f64,  100000000000001.0_f64, f64::EPSILON);
        check_ne( 100000000000001.0_f64,  100000000000000.0_f64, f64::EPSILON);
        check_ne(-100000000000000.0_f64, -100000000000001.0_f64, f64::EPSILON);
        check_ne(-100000000000001.0_f64, -100000000000000.0_f64, f64::EPSILON);

        check_ne( 1000000000000000.0_f64,  1000000000000001.0_f64, f64::EPSILON);
        check_ne( 1000000000000001.0_f64,  1000000000000000.0_f64, f64::EPSILON);
        check_ne(-1000000000000000.0_f64, -1000000000000001.0_f64, f64::EPSILON);
        check_ne(-1000000000000001.0_f64, -1000000000000000.0_f64, f64::EPSILON);
    }

    #[test]
    fn test_eq_rounding2() {
        check_eq( 1.0000000000000001_f64,  1.0000000000000002_f64, f64::EPSILON);
        check_eq( 1.0000000000000002_f64,  1.0000000000000001_f64, f64::EPSILON);
        check_eq(-1.0000000000000001_f64, -1.0000000000000002_f64, f64::EPSILON);
        check_eq(-1.0000000000000002_f64, -1.0000000000000001_f64, f64::EPSILON);

        check_eq( 1.00000000000000001_f64,  1.00000000000000002_f64, f64::EPSILON);
        check_eq( 1.00000000000000002_f64,  1.00000000000000001_f64, f64::EPSILON);
        check_eq(-1.00000000000000001_f64, -1.00000000000000002_f64, f64::EPSILON);
        check_eq(-1.00000000000000002_f64, -1.00000000000000001_f64, f64::EPSILON);

        check_eq( 1.000000000000000001_f64,  1.000000000000000002_f64, f64::EPSILON);
        check_eq( 1.000000000000000002_f64,  1.000000000000000001_f64, f64::EPSILON);
        check_eq(-1.000000000000000001_f64, -1.000000000000000002_f64, f64::EPSILON);
        check_eq(-1.000000000000000002_f64, -1.000000000000000001_f64, f64::EPSILON);

        check_eq( 1.0000000000000000001_f64,  1.0000000000000000002_f64, f64::EPSILON);
        check_eq( 1.0000000000000000002_f64,  1.0000000000000000001_f64, f64::EPSILON);
        check_eq(-1.0000000000000000001_f64, -1.0000000000000000002_f64, f64::EPSILON);
        check_eq(-1.0000000000000000002_f64, -1.0000000000000000001_f64, f64::EPSILON);

        check_eq( 1.00000000000000000001_f64,  1.00000000000000000002_f64, f64::EPSILON);
        check_eq( 1.00000000000000000002_f64,  1.00000000000000000001_f64, f64::EPSILON);
        check_eq(-1.00000000000000000001_f64, -1.00000000000000000002_f64, f64::EPSILON);
        check_eq(-1.00000000000000000002_f64, -1.00000000000000000001_f64, f64::EPSILON);
    }

    #[test]
    fn test_ne_rounding2() {
        check_ne( 1.01_f64,  1.02_f64, f64::EPSILON);
        check_ne( 1.02_f64,  1.01_f64, f64::EPSILON);
        check_ne(-1.01_f64, -1.02_f64, f64::EPSILON);
        check_ne(-1.02_f64, -1.01_f64, f64::EPSILON);

        check_ne( 1.001_f64,  1.002_f64, f64::EPSILON);
        check_ne( 1.002_f64,  1.001_f64, f64::EPSILON);
        check_ne(-1.001_f64, -1.002_f64, f64::EPSILON);
        check_ne(-1.002_f64, -1.001_f64, f64::EPSILON);

        check_ne( 1.0001_f64,  1.0002_f64, f64::EPSILON);
        check_ne( 1.0002_f64,  1.0001_f64, f64::EPSILON);
        check_ne(-1.0001_f64, -1.0002_f64, f64::EPSILON);
        check_ne(-1.0002_f64, -1.0001_f64, f64::EPSILON);

        check_ne( 1.00001_f64,  1.00002_f64, f64::EPSILON);
        check_ne( 1.00002_f64,  1.00001_f64, f64::EPSILON);
        check_ne(-1.00001_f64, -1.00002_f64, f64::EPSILON);
        check_ne(-1.00002_f64, -1.00001_f64, f64::EPSILON);

        check_ne( 1.000001_f64,  1.000002_f64, f64::EPSILON);
        check_ne( 1.000002_f64,  1.000001_f64, f64::EPSILON);
        check_ne(-1.000001_f64, -1.000002_f64, f64::EPSILON);
        check_ne(-1.000002_f64, -1.000001_f64, f64::EPSILON);

        check_ne( 1.0000001_f64,  1.0000002_f64, f64::EPSILON);
        check_ne( 1.0000002_f64,  1.0000001_f64, f64::EPSILON);
        check_ne(-1.0000001_f64, -1.0000002_f64, f64::EPSILON);
        check_ne(-1.0000002_f64, -1.0000001_f64, f64::EPSILON);

        check_ne( 1.00000001_f64,  1.00000002_f64, f64::EPSILON);
        check_ne( 1.00000002_f64,  1.00000001_f64, f64::EPSILON);
        check_ne(-1.00000001_f64, -1.00000002_f64, f64::EPSILON);
        check_ne(-1.00000002_f64, -1.00000001_f64, f64::EPSILON);

        check_ne( 1.000000001_f64,  1.000000002_f64, f64::EPSILON);
        check_ne( 1.000000002_f64,  1.000000001_f64, f64::EPSILON);
        check_ne(-1.000000001_f64, -1.000000002_f64, f64::EPSILON);
        check_ne(-1.000000002_f64, -1.000000001_f64, f64::EPSILON);

        check_ne( 1.0000000001_f64,  1.0000000002_f64, f64::EPSILON);
        check_ne( 1.0000000002_f64,  1.0000000001_f64, f64::EPSILON);
        check_ne(-1.0000000001_f64, -1.0000000002_f64, f64::EPSILON);
        check_ne(-1.0000000002_f64, -1.0000000001_f64, f64::EPSILON);

        check_ne( 1.00000000001_f64,  1.00000000002_f64, f64::EPSILON);
        check_ne( 1.00000000002_f64,  1.00000000001_f64, f64::EPSILON);
        check_ne(-1.00000000001_f64, -1.00000000002_f64, f64::EPSILON);
        check_ne(-1.00000000002_f64, -1.00000000001_f64, f64::EPSILON);

        check_ne( 1.000000000001_f64,  1.000000000002_f64, f64::EPSILON);
        check_ne( 1.000000000002_f64,  1.000000000001_f64, f64::EPSILON);
        check_ne(-1.000000000001_f64, -1.000000000002_f64, f64::EPSILON);
        check_ne(-1.000000000002_f64, -1.000000000001_f64, f64::EPSILON);

        check_ne( 1.0000000000001_f64,  1.0000000000002_f64, f64::EPSILON);
        check_ne( 1.0000000000002_f64,  1.0000000000001_f64, f64::EPSILON);
        check_ne(-1.0000000000001_f64, -1.0000000000002_f64, f64::EPSILON);
        check_ne(-1.0000000000002_f64, -1.0000000000001_f64, f64::EPSILON);

        check_ne( 1.00000000000001_f64,  1.00000000000002_f64, f64::EPSILON);
        check_ne( 1.00000000000002_f64,  1.00000000000001_f64, f64::EPSILON);
        check_ne(-1.00000000000001_f64, -1.00000000000002_f64, f64::EPSILON);
        check_ne(-1.00000000000002_f64, -1.00000000000001_f64, f64::EPSILON);

        check_ne( 1.000000000000001_f64,  1.000000000000002_f64, f64::EPSILON);
        check_ne( 1.000000000000002_f64,  1.000000000000001_f64, f64::EPSILON);
        check_ne(-1.000000000000001_f64, -1.000000000000002_f64, f64::EPSILON);
        check_ne(-1.000000000000002_f64, -1.000000000000001_f64, f64::EPSILON);
    }

    #[test]
    fn test_eq_max() {
        check_eq( f64::MAX,  f64::MAX, f64::EPSILON);
    }

    #[test]
    fn test_ne_max() {
        check_ne( f64::MAX, -f64::MAX,           f64::EPSILON);
        check_ne(-f64::MAX,  f64::MAX,           f64::EPSILON);
        check_ne( f64::MAX,  f64::MAX / 2.0_f64, f64::EPSILON);
        check_ne( f64::MAX, -f64::MAX / 2.0_f64, f64::EPSILON);
        check_ne(-f64::MAX,  f64::MAX / 2.0_f64, f64::EPSILON);
    }

    #[test]
    fn test_ne_nan1() {
        check_ne( f64::NAN,  f64::NAN, 0_f64);
        check_ne( f64::NAN, -f64::NAN, 0_f64);
        check_ne(-f64::NAN,  f64::NAN, 0_f64);
        check_ne(-f64::NAN, -f64::NAN, 0_f64);
    }

    #[test]
    fn test_ne_nan2() {
        for i in 0..=i16::MAX {
            check_ne( f64::NAN , f64::NAN, 1_f64 / (i as f64));
            check_ne( f64::NAN, -f64::NAN, 1_f64 / (i as f64));
            check_ne(-f64::NAN,  f64::NAN, 1_f64 / (i as f64));
            check_ne(-f64::NAN, -f64::NAN, 1_f64 / (i as f64));
        }
    }

    #[test]
    fn test_ne_nan3() {
        for i in 0..=i16::MAX {
            check_ne( f64::NAN , f64::NAN, i as f64);
            check_ne( f64::NAN, -f64::NAN, i as f64);
            check_ne(-f64::NAN,  f64::NAN, i as f64);
            check_ne(-f64::NAN, -f64::NAN, i as f64);
        }
    }

    #[test]
    fn test_ne_nan4() {
        check_ne(f64::NAN, f64::NAN, f64::EPSILON);
        
        check_ne( f64::NAN,  0.0_f64,  f64::EPSILON);
        check_ne(-0_f64,     f64::NAN, f64::EPSILON);
        check_ne( f64::NAN, -0.0_f64,  f64::EPSILON);
        check_ne( 0_f64,     f64::NAN, f64::EPSILON);

        check_ne( f64::NAN, f64::INFINITY,  f64::EPSILON);
        check_ne( f64::INFINITY, f64::NAN,  f64::EPSILON);
        check_ne( f64::NAN, -f64::INFINITY, f64::EPSILON);
        check_ne(-f64::INFINITY, f64::NAN,  f64::EPSILON);

        check_ne( f64::NAN, f64::MAX,  f64::EPSILON);
        check_ne( f64::MAX, f64::NAN,  f64::EPSILON);
        check_ne( f64::NAN, -f64::MAX, f64::EPSILON);
        check_ne(-f64::MAX, f64::NAN,  f64::EPSILON);

        check_ne( f64::NAN,          f64::MIN_POSITIVE,  f64::EPSILON);
        check_ne( f64::MIN_POSITIVE, f64::NAN,           f64::EPSILON);
        check_ne( f64::NAN,          -f64::MIN_POSITIVE, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE, f64::NAN,           f64::EPSILON);

        check_ne( f64::NAN,  1_f64,    f64::EPSILON);
        check_ne( f64::NAN, -1_f64,    f64::EPSILON);
        check_ne( 1_f64,     f64::NAN, f64::EPSILON);
        check_ne(-1_f64,     f64::NAN, f64::EPSILON);
    }

    #[test]
    fn test_nan5() {
        check_ne(f64::NAN, f64::NAN, 1_f64);
        
        check_ne( f64::NAN,  0.0_f64,  1_f64);
        check_ne(-0_f64,     f64::NAN, 1_f64);
        check_ne( f64::NAN, -0.0_f64,  1_f64);
        check_ne( 0_f64,     f64::NAN, 1_f64);

        check_ne( f64::NAN, f64::INFINITY,  1_f64);
        check_ne( f64::INFINITY, f64::NAN,  1_f64);
        check_ne( f64::NAN, -f64::INFINITY, 1_f64);
        check_ne(-f64::INFINITY, f64::NAN,  1_f64);

        check_ne( f64::NAN, f64::MAX,  1_f64);
        check_ne( f64::MAX, f64::NAN,  1_f64);
        check_ne( f64::NAN, -f64::MAX, 1_f64);
        check_ne(-f64::MAX, f64::NAN,  1_f64);

        check_ne( f64::NAN,          f64::MIN_POSITIVE,  1_f64);
        check_ne( f64::MIN_POSITIVE, f64::NAN,           1_f64);
        check_ne( f64::NAN,          -f64::MIN_POSITIVE, 1_f64);
        check_ne(-f64::MIN_POSITIVE, f64::NAN,           1_f64);

        check_ne( f64::NAN,  1_f64,    1_f64);
        check_ne( f64::NAN, -1_f64,    1_f64);
        check_ne( 1_f64,     f64::NAN, 1_f64);
        check_ne(-1_f64,     f64::NAN, 1_f64);
    }

    #[test]
    fn test_eq_infinity() {
        check_eq(f64::INFINITY,     f64::INFINITY,     f64::INFINITY);
        check_eq(f64::INFINITY,     f64::MAX,          f64::INFINITY);
        check_eq(f64::INFINITY,     f64::NEG_INFINITY, f64::INFINITY);
        check_eq(f64::NEG_INFINITY, f64::MAX,          f64::INFINITY);
    }

    #[test]
    fn test_ne_infinity() {
        // check_ne(f64::INFINITY,     f64::INFINITY,     f64::MAX);
        check_ne(f64::INFINITY,     f64::MAX,          f64::MAX);
        check_ne(f64::INFINITY,     f64::NEG_INFINITY, f64::MAX);
        check_ne(f64::NEG_INFINITY, f64::MAX,          f64::MAX);
    }

    #[test]
    fn test_eq_near_zero1() {
        check_eq( f64::MIN_POSITIVE,  f64::MIN_POSITIVE, f64::EPSILON);
        check_eq( f64::MIN_POSITIVE, -f64::MIN_POSITIVE, f64::EPSILON);
        check_eq(-f64::MIN_POSITIVE,  f64::MIN_POSITIVE, f64::EPSILON);

        check_eq( f64::MIN_POSITIVE,  0_f64,             f64::EPSILON);
        check_eq( 0_f64,              f64::MIN_POSITIVE, f64::EPSILON);
        check_eq(-f64::MIN_POSITIVE,  0_f64,             f64::EPSILON);
        check_eq( 0_f64,             -f64::MIN_POSITIVE, f64::EPSILON);

        check_eq( 0.0000000000000001_f64, -f64::MIN_POSITIVE,      f64::EPSILON);
        check_eq( 0.0000000000000001_f64,  f64::MIN_POSITIVE,      f64::EPSILON);
        check_eq( f64::MIN_POSITIVE,       0.0000000000000001_f64, f64::EPSILON);
        check_eq(-f64::MIN_POSITIVE,       0.0000000000000001_f64, f64::EPSILON);
    }

    #[test]
    fn test_eq_near_zero2() {
        check_eq( 0.0000000010000001_f64,  0.0000000010000002_f64, f64::EPSILON);
        check_eq( 0.0000000010000002_f64,  0.0000000010000001_f64, f64::EPSILON);
        check_eq(-0.0000000010000001_f64, -0.0000000010000002_f64, f64::EPSILON);
        check_eq(-0.0000000010000002_f64, -0.0000000010000001_f64, f64::EPSILON);
    }

    #[test]
    fn test_eq_near_zero3() {
        check_eq(1e-16_f64, -1e-16_f64, f64::EPSILON);
        check_eq(1e-17_f64, -1e-17_f64, f64::EPSILON);
        check_eq(1e-18_f64, -1e-18_f64, f64::EPSILON);
        check_eq(1e-19_f64, -1e-19_f64, f64::EPSILON);
        check_eq(1e-20_f64, -1e-20_f64, f64::EPSILON);
        check_eq(1e-21_f64, -1e-21_f64, f64::EPSILON);
        check_eq(1e-22_f64, -1e-22_f64, f64::EPSILON);
        check_eq(1e-23_f64, -1e-23_f64, f64::EPSILON);
        check_eq(1e-24_f64, -1e-24_f64, f64::EPSILON);
        check_eq(1e-25_f64, -1e-25_f64, f64::EPSILON);
        check_eq(1e-26_f64, -1e-26_f64, f64::EPSILON);
        check_eq(1e-27_f64, -1e-27_f64, f64::EPSILON);
        check_eq(1e-28_f64, -1e-28_f64, f64::EPSILON);
    }

    #[test]
    fn test_ne_near_zero1() {
        check_ne( 0.000001_f64,      -f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( 0.000001_f64,       f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,  0.000001_f64,      f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,  0.000001_f64,      f64::EPSILON);

        check_ne(-0.000001_f64,      -f64::MIN_POSITIVE, f64::EPSILON);
        check_ne(-0.000001_f64,       f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE, -0.000001_f64,      f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE, -0.000001_f64,      f64::EPSILON);

        check_ne( 0.0000001_f64,     -f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( 0.0000001_f64,      f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,  0.0000001_f64,     f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,  0.0000001_f64,     f64::EPSILON);

        check_ne(-0.0000001_f64,     -f64::MIN_POSITIVE, f64::EPSILON);
        check_ne(-0.0000001_f64,      f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE, -0.0000001_f64,     f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE, -0.0000001_f64,     f64::EPSILON);

        check_ne( 0.00000001_f64,    -f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( 0.00000001_f64,     f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,  0.00000001_f64,    f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,  0.00000001_f64,    f64::EPSILON);

        check_ne(-0.00000001_f64,    -f64::MIN_POSITIVE, f64::EPSILON);
        check_ne(-0.00000001_f64,     f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE, -0.00000001_f64,    f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE, -0.00000001_f64,    f64::EPSILON);

        check_ne( 0.000000001_f64,   -f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( 0.000000001_f64,    f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,  0.000000001_f64,    f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,  0.000000001_f64,    f64::EPSILON);

        check_ne(-0.000000001_f64,   -f64::MIN_POSITIVE, f64::EPSILON);
        check_ne(-0.000000001_f64,    f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE, -0.000000001_f64,   f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE, -0.000000001_f64,   f64::EPSILON);

        check_ne( 0.0000000001_f64,  -f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( 0.0000000001_f64,   f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,  0.0000000001_f64,  f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,  0.0000000001_f64,  f64::EPSILON);

        check_ne(-0.0000000001_f64,  -f64::MIN_POSITIVE, f64::EPSILON);
        check_ne(-0.0000000001_f64,   f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE, -0.0000000001_f64,  f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE, -0.0000000001_f64,  f64::EPSILON);

        check_ne( 0.00000000001_f64, -f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( 0.00000000001_f64,  f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,  0.00000000001_f64, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,  0.00000000001_f64, f64::EPSILON);

        check_ne(-0.00000000001_f64, -f64::MIN_POSITIVE, f64::EPSILON);
        check_ne(-0.00000000001_f64,  f64::MIN_POSITIVE, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE, -0.00000000001_f64, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE, -0.00000000001_f64, f64::EPSILON);

        check_ne( 0.000000000001_f64, -f64::MIN_POSITIVE,  f64::EPSILON);
        check_ne( 0.000000000001_f64,  f64::MIN_POSITIVE,  f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,   0.000000000001_f64, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,   0.000000000001_f64, f64::EPSILON);

        check_ne(-0.000000000001_f64, -f64::MIN_POSITIVE,  f64::EPSILON);
        check_ne(-0.000000000001_f64,  f64::MIN_POSITIVE,  f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,  -0.000000000001_f64, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,  -0.000000000001_f64, f64::EPSILON);

        check_ne( 0.0000000000001_f64, -f64::MIN_POSITIVE,   f64::EPSILON);
        check_ne( 0.0000000000001_f64,  f64::MIN_POSITIVE,   f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,    0.0000000000001_f64, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,    0.0000000000001_f64, f64::EPSILON);

        check_ne(-0.0000000000001_f64, -f64::MIN_POSITIVE,   f64::EPSILON);
        check_ne(-0.0000000000001_f64,  f64::MIN_POSITIVE,   f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,   -0.0000000000001_f64, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,   -0.0000000000001_f64, f64::EPSILON);

        check_ne( 0.00000000000001_f64, -f64::MIN_POSITIVE,    f64::EPSILON);
        check_ne( 0.00000000000001_f64,  f64::MIN_POSITIVE,    f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,     0.00000000000001_f64, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,     0.00000000000001_f64, f64::EPSILON);

        check_ne(-0.00000000000001_f64, -f64::MIN_POSITIVE,    f64::EPSILON);
        check_ne(-0.00000000000001_f64,  f64::MIN_POSITIVE,    f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,    -0.00000000000001_f64, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,    -0.00000000000001_f64, f64::EPSILON);

        check_ne( 0.000000000000001_f64, -f64::MIN_POSITIVE,     f64::EPSILON);
        check_ne( 0.000000000000001_f64,  f64::MIN_POSITIVE,     f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,      0.000000000000001_f64, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,      0.000000000000001_f64, f64::EPSILON);

        check_ne(-0.000000000000001_f64, -f64::MIN_POSITIVE,     f64::EPSILON);
        check_ne(-0.000000000000001_f64,  f64::MIN_POSITIVE,     f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,     -0.000000000000001_f64, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,     -0.000000000000001_f64, f64::EPSILON);
    }

    #[test]
    fn test_ne_near_zero2() {
        check_ne( 0.0000010000002_f64,  0.00000010000001_f64, f64::EPSILON);
        check_ne( 0.0000010000001_f64,  0.00000010000002_f64, f64::EPSILON);
        check_ne(-0.0000010000002_f64, -0.00000010000001_f64, f64::EPSILON);
        check_ne(-0.0000010000001_f64, -0.00000010000002_f64, f64::EPSILON);

        check_ne( 0.00000010000002_f64,  0.000000010000001_f64, f64::EPSILON);
        check_ne( 0.00000010000001_f64,  0.000000010000002_f64, f64::EPSILON);
        check_ne(-0.00000010000002_f64, -0.000000010000001_f64, f64::EPSILON);
        check_ne(-0.00000010000001_f64, -0.000000010000002_f64, f64::EPSILON);

        check_ne( 0.000000010000002_f64,  0.0000000010000001_f64, f64::EPSILON);
        check_ne( 0.000000010000001_f64,  0.0000000010000002_f64, f64::EPSILON);
        check_ne(-0.000000010000002_f64, -0.0000000010000001_f64, f64::EPSILON);
        check_ne(-0.000000010000001_f64, -0.0000000010000002_f64, f64::EPSILON);

        check_ne( 0.0000000010000002_f64,  0.00000000010000001_f64, f64::EPSILON);
        check_ne( 0.0000000010000001_f64,  0.00000000010000002_f64, f64::EPSILON);
        check_ne(-0.0000000010000002_f64, -0.00000000010000001_f64, f64::EPSILON);
        check_ne(-0.0000000010000001_f64, -0.00000000010000002_f64, f64::EPSILON);

        check_ne( 0.00000000010000002_f64,  0.000000000010000001_f64, f64::EPSILON);
        check_ne( 0.00000000010000001_f64,  0.000000000010000002_f64, f64::EPSILON);
        check_ne(-0.00000000010000002_f64, -0.000000000010000001_f64, f64::EPSILON);
        check_ne(-0.00000000010000001_f64, -0.000000000010000002_f64, f64::EPSILON);

        check_ne( 0.000000000010000002_f64,  0.0000000000010000001_f64, f64::EPSILON);
        check_ne( 0.000000000010000001_f64,  0.0000000000010000002_f64, f64::EPSILON);
        check_ne(-0.000000000010000002_f64, -0.0000000000010000001_f64, f64::EPSILON);
        check_ne(-0.000000000010000001_f64, -0.0000000000010000002_f64, f64::EPSILON);

        check_ne( 0.0000000000010000002_f64,  0.00000000000010000001_f64, f64::EPSILON);
        check_ne( 0.0000000000010000001_f64,  0.00000000000010000002_f64, f64::EPSILON);
        check_ne(-0.0000000000010000002_f64, -0.00000000000010000001_f64, f64::EPSILON);
        check_ne(-0.0000000000010000001_f64, -0.00000000000010000002_f64, f64::EPSILON);

        check_ne( 0.00000000000010000002_f64,  0.000000000000010000001_f64, f64::EPSILON);
        check_ne( 0.00000000000010000001_f64,  0.000000000000010000002_f64, f64::EPSILON);
        check_ne(-0.00000000000010000002_f64, -0.000000000000010000001_f64, f64::EPSILON);
        check_ne(-0.00000000000010000001_f64, -0.000000000000010000002_f64, f64::EPSILON);

        check_ne( 0.000000000000010000002_f64,  0.0000000000000010000001_f64, f64::EPSILON);
        check_ne( 0.000000000000010000001_f64,  0.0000000000000010000002_f64, f64::EPSILON);
        check_ne(-0.000000000000010000002_f64, -0.0000000000000010000001_f64, f64::EPSILON);
        check_ne(-0.000000000000010000001_f64, -0.0000000000000010000002_f64, f64::EPSILON);

        check_ne( 0.0000000000000010000002_f64,  0.00000000000000010000001_f64, f64::EPSILON);
        check_ne( 0.0000000000000010000001_f64,  0.00000000000000010000002_f64, f64::EPSILON);
        check_ne(-0.0000000000000010000002_f64, -0.00000000000000010000001_f64, f64::EPSILON);
        check_ne(-0.0000000000000010000001_f64, -0.00000000000000010000002_f64, f64::EPSILON);
    }
}

#[cfg(test)]
mod abs_diff_array_tests {
    use approx_cmp::{
        assert_abs_diff_eq,
        assert_abs_diff_ne
    };


    #[test]
    fn test_basic_eq_array2() {
        let lhs = [1.0_f64, 1.0_f64];
        let rhs = [1.0_f64, 1.0_f64];

        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_ne_array2() {
        let lhs = [1.0_f64, 1.0_f64];
        let rhs = [2.0_f64, 3.0_f64];

        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_eq_array3() {
        let lhs = [1.0_f64, 1.0_f64, 1.0_f64];
        let rhs =  [1.0_f64, 1.0_f64, 1.0_f64];

        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_ne_array3() {
        let lhs = [1.0_f64, 1.0_f64, 1.0_f64];
        let rhs = [2.0_f64, 3.0_f64, 4.0_f64];

        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_eq_array4() {
        let lhs = [1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64];
        let rhs = [1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64];
        
        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_ne_array4() {
        let lhs = [1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64];
        let rhs = [2.0_f64, 3.0_f64, 4.0_f64, 5.0_f64];

        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_eq_array8() {
        let lhs = [1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64];
        let rhs = [1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64];
            
        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_ne_array8() {
        let lhs = [1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64];
        let rhs = [2.0_f64, 3.0_f64, 4.0_f64, 5.0_f64, 6.0_f64, 7.0_f64, 8.0_f64, 9.0_f64];

        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_eq_array16() {
        let lhs = [
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64
        ];
        let rhs = [
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64
        ];
        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_ne_array16() {
        let lhs = [
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64
        ];
        let rhs = [
            2.0_f64,  3.0_f64,  4.0_f64,  5.0_f64,  6.0_f64,  7.0_f64,  8.0_f64,  9.0_f64,
            10.0_f64, 11.0_f64, 12.0_f64, 13.0_f64, 14.0_f64, 15.0_f64, 16.0_f64, 17.0_f64
        ];
        
        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_eq_array32() {
        let lhs = [
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64
        ];
        let rhs = [
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64
        ];
            
        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_ne_array32() {
        let lhs = [
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64
        ];
        let rhs = [
            2.0_f64,  3.0_f64,  4.0_f64,  5.0_f64,  6.0_f64,  7.0_f64,  8.0_f64,  9.0_f64,
            10.0_f64, 11.0_f64, 12.0_f64, 13.0_f64, 14.0_f64, 15.0_f64, 16.0_f64, 17.0_f64,
            18.0_f64, 19.0_f64, 20.0_f64, 21.0_f64, 22.0_f64, 23.0_f64, 24.0_f64, 25.0_f64,
            26.0_f64, 27.0_f64, 28.0_f64, 29.0_f64, 30.0_f64, 31.0_f64, 32.0_f64, 33.0_f64
        ];

        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_eq_array64() {
        let lhs = [
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64
        ];
        let rhs = [
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64
        ];
            
        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_ne_array64() {
        let lhs = [
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
            1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64
        ];
        let rhs = [
            2.0_f64,  3.0_f64,  4.0_f64,  5.0_f64,  6.0_f64,  7.0_f64,  8.0_f64,  9.0_f64,
            10.0_f64, 11.0_f64, 12.0_f64, 13.0_f64, 14.0_f64, 15.0_f64, 16.0_f64, 17.0_f64,
            18.0_f64, 19.0_f64, 20.0_f64, 21.0_f64, 22.0_f64, 23.0_f64, 24.0_f64, 25.0_f64,
            26.0_f64, 27.0_f64, 28.0_f64, 29.0_f64, 30.0_f64, 31.0_f64, 32.0_f64, 33.0_f64,
            34.0_f64, 35.0_f64, 36.0_f64, 37.0_f64, 38.0_f64, 39.0_f64, 40.0_f64, 41.0_f64,
            42.0_f64, 43.0_f64, 44.0_f64, 45.0_f64, 46.0_f64, 47.0_f64, 48.0_f64, 49.0_f64,
            50.0_f64, 51.0_f64, 52.0_f64, 53.0_f64, 54.0_f64, 55.0_f64, 56.0_f64, 57.0_f64,
            58.0_f64, 59.0_f64, 60.0_f64, 61.0_f64, 62.0_f64, 63.0_f64, 64.0_f64, 65.0_f64
        ];
        
        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON);
    }
}

