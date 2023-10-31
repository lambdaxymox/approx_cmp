extern crate approx_cmp;


#[cfg(test)]
mod relative_eq_f32_tests {
    use approx_cmp::{
        RelativeEq,
        RelativeAllEq,
        assert_relative_eq,
        assert_relative_ne,
        relative_eq,
        relative_ne,
    };

    fn check_relative_eq(a: f32, b: f32, max_abs_diff: f32, max_relative: f32) {
        assert!(a.relative_eq(&b, &max_abs_diff, &max_relative));
        assert!(relative_eq!(a, b, abs_diff <= max_abs_diff, relative <= max_relative));
        assert_relative_eq!(a, b, abs_diff <= max_abs_diff, relative <= max_relative);
        
        assert!(a.relative_all_eq(&b, &max_abs_diff, &max_relative));
        assert!(relative_eq!(a, b, abs_diff_all <= max_abs_diff, relative_all <= max_relative));
        assert_relative_eq!(a, b, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    fn check_relative_ne(a: f32, b: f32, max_abs_diff: f32, max_relative: f32) {
        assert!(a.relative_ne(&b, &max_abs_diff, &max_relative));
        assert!(relative_ne!(a, b, abs_diff <= max_abs_diff, relative <= max_relative));
        assert_relative_ne!(a, b, abs_diff <= max_abs_diff, relative <= max_relative);

        assert!(a.relative_all_ne(&b, &max_abs_diff, &max_relative));
        assert!(relative_ne!(a, b, abs_diff_all <= max_abs_diff, relative_all <= max_relative));
        assert_relative_ne!(a, b, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    fn check_eq(a: f32, b: f32, max_abs_diff: f32, max_relative: f32) {
        check_relative_eq(a, b, max_abs_diff, max_relative);
        check_relative_eq(b, a, max_abs_diff, max_relative);
        check_relative_eq(-a, -b, max_abs_diff, max_relative);
        check_relative_eq(-b, -a, max_abs_diff, max_relative);
    }

    fn check_ne(a: f32, b: f32, max_abs_diff: f32, max_relative: f32) {
        check_relative_ne(a, b, max_abs_diff, max_relative);
        check_relative_ne(b, a, max_abs_diff, max_relative);
        check_relative_ne(-a, -b, max_abs_diff, max_relative);
        check_relative_ne(-b, -a, max_abs_diff, max_relative);
    }

    #[rustfmt::skip]
    fn check_eq_self(value: f32) {
        check_eq(value, value, 0.0,               f32::EPSILON);
        check_eq(value, value, f32::MIN_POSITIVE, f32::MIN_POSITIVE);
        check_eq(value, value, f32::MAX,          f32::MAX);
        check_eq(value, value, f32::INFINITY,     f32::INFINITY);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_zero() {
        check_eq_self(0.0);

        check_eq( 0.0,  0.0, f32::EPSILON, f32::EPSILON);
        check_eq(-0.0,  0.0, f32::EPSILON, f32::EPSILON);
        check_eq( 0.0, -0.0, f32::EPSILON, f32::EPSILON);
        check_eq(-0.0, -0.0, f32::EPSILON, f32::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_zero() {
        check_ne( 0.000001_f32, 0.0_f32,      f32::EPSILON, f32::EPSILON);
        check_ne( 0.0_f32,      0.000001_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-0.000001_f32, 0.0_f32,      f32::EPSILON, f32::EPSILON);
        check_ne( 0.0_f32,     -0.000001_f32, f32::EPSILON, f32::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_tolerance() {
        check_eq( 0.0_f32,    1e-40_f32, 1e-40_f32, 1e-40_f32);
        check_eq( 1e-40_f32,  0.0_f32,   1e-40_f32, 1e-40_f32);
        check_eq( 0.0_f32,   -1e-40_f32, 1e-40_f32, 1e-40_f32);
        check_eq(-1e-40_f32,  0.0_f32,   1e-40_f32, 1e-40_f32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_tolerance() {
        check_ne( 1e-40_f32,  0.0_f32,   1e-41_f32, 1e-41_f32);
        check_ne( 0.0_f32,    1e-40_f32, 1e-41_f32, 1e-41_f32);
        check_ne(-1e-40_f32,  0.0_f32,   1e-41_f32, 1e-41_f32);
        check_ne( 0.0_f32,   -1e-40_f32, 1e-41_f32, 1e-41_f32);
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

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        check_ne( 1.0_f32,  2.0_f32, f32::EPSILON,      f32::EPSILON);
        check_ne( 1.0_f32,  2.0_f32, f32::MIN_POSITIVE, f32::MIN_POSITIVE);
        check_ne( 1.0_f32, -2.0_f32, f32::EPSILON,      f32::EPSILON);
        check_ne( 1.0_f32, -2.0_f32, f32::MIN_POSITIVE, f32::MIN_POSITIVE);
        check_ne(-1.0_f32,  2.0_f32, f32::EPSILON,      f32::EPSILON);
        check_ne(-1.0_f32,  2.0_f32, f32::MIN_POSITIVE, f32::MIN_POSITIVE);
        check_ne(-1.0_f32, -2.0_f32, f32::EPSILON,      f32::EPSILON);
        check_ne(-1.0_f32, -2.0_f32, f32::MIN_POSITIVE, f32::MIN_POSITIVE);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding1() {
        check_eq( 10000000.0_f32,  10000001.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq( 10000001.0_f32,  10000000.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-10000000.0_f32, -10000001.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-10000001.0_f32, -10000000.0_f32, f32::EPSILON, f32::EPSILON);

        check_eq( 100000000.0_f32,  100000001.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq( 100000001.0_f32,  100000000.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-100000000.0_f32, -100000001.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-100000001.0_f32, -100000000.0_f32, f32::EPSILON, f32::EPSILON);

        check_eq( 1000000000.0_f32,  1000000001.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq( 1000000001.0_f32,  1000000000.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-1000000000.0_f32, -1000000001.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-1000000001.0_f32, -1000000000.0_f32, f32::EPSILON, f32::EPSILON);

        check_eq( 10000000000.0_f32,  10000000001.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq( 10000000001.0_f32,  10000000000.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-10000000000.0_f32, -10000000001.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-10000000001.0_f32, -10000000000.0_f32, f32::EPSILON, f32::EPSILON);

        check_eq( 100000000000.0_f32,  100000000001.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq( 100000000001.0_f32,  100000000000.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-100000000000.0_f32, -100000000001.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-100000000001.0_f32, -100000000000.0_f32, f32::EPSILON, f32::EPSILON);

        check_eq( 1000000000000.0_f32,  1000000000001.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq( 1000000000001.0_f32,  1000000000000.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-1000000000000.0_f32, -1000000000001.0_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-1000000000001.0_f32, -1000000000000.0_f32, f32::EPSILON, f32::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding1() {
        check_ne( 1000.0_f32,  1001.0_f32, f32::EPSILON, f32::EPSILON);
        check_ne( 1001.0_f32,  1000.0_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-1000.0_f32, -1001.0_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-1001.0_f32, -1000.0_f32, f32::EPSILON, f32::EPSILON);

        check_ne( 10000.0_f32,  10001.0_f32, f32::EPSILON, f32::EPSILON);
        check_ne( 10001.0_f32,  10000.0_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-10000.0_f32, -10001.0_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-10001.0_f32, -10000.0_f32, f32::EPSILON, f32::EPSILON);

        check_ne( 100000.0_f32,  100001.0_f32, f32::EPSILON, f32::EPSILON);
        check_ne( 100001.0_f32,  100000.0_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-100000.0_f32, -100001.0_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-100001.0_f32, -100000.0_f32, f32::EPSILON, f32::EPSILON);

        check_ne( 1000000.0_f32,  1000001.0_f32, f32::EPSILON, f32::EPSILON);
        check_ne( 1000001.0_f32,  1000000.0_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-1000000.0_f32, -1000001.0_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-1000001.0_f32, -1000000.0_f32, f32::EPSILON, f32::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding2() {
        check_eq( 1.0000001_f32,  1.0000002_f32, f32::EPSILON, f32::EPSILON);
        check_eq( 1.0000002_f32,  1.0000001_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-1.0000001_f32, -1.0000002_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-1.0000002_f32, -1.0000001_f32, f32::EPSILON, f32::EPSILON);

        check_eq( 1.00000001_f32,  1.00000002_f32, f32::EPSILON, f32::EPSILON);
        check_eq( 1.00000002_f32,  1.00000001_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-1.00000001_f32, -1.00000002_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-1.00000002_f32, -1.00000001_f32, f32::EPSILON, f32::EPSILON);

        check_eq( 1.000000001_f32,  1.000000002_f32, f32::EPSILON, f32::EPSILON);
        check_eq( 1.000000002_f32,  1.000000001_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-1.000000001_f32, -1.000000002_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-1.000000002_f32, -1.000000001_f32, f32::EPSILON, f32::EPSILON);

        check_eq( 1.0000000001_f32,  1.0000000002_f32, f32::EPSILON, f32::EPSILON);
        check_eq( 1.0000000002_f32,  1.0000000001_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-1.0000000001_f32, -1.0000000002_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-1.0000000002_f32, -1.0000000001_f32, f32::EPSILON, f32::EPSILON);

        check_eq( 1.00000000001_f32,  1.00000000002_f32, f32::EPSILON, f32::EPSILON);
        check_eq( 1.00000000002_f32,  1.00000000001_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-1.00000000001_f32, -1.00000000002_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-1.00000000002_f32, -1.00000000001_f32, f32::EPSILON, f32::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding2() {
        check_ne( 1.01_f32,  1.02_f32, f32::EPSILON, f32::EPSILON);
        check_ne( 1.02_f32,  1.01_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-1.01_f32, -1.02_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-1.02_f32, -1.01_f32, f32::EPSILON, f32::EPSILON);

        check_ne( 1.001_f32,  1.002_f32, f32::EPSILON, f32::EPSILON);
        check_ne( 1.002_f32,  1.001_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-1.001_f32, -1.002_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-1.002_f32, -1.001_f32, f32::EPSILON, f32::EPSILON);

        check_ne( 1.0001_f32,  1.0002_f32, f32::EPSILON, f32::EPSILON);
        check_ne( 1.0002_f32,  1.0001_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-1.0001_f32, -1.0002_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-1.0002_f32, -1.0001_f32, f32::EPSILON, f32::EPSILON);

        check_ne( 1.00001_f32,  1.00002_f32, f32::EPSILON, f32::EPSILON);
        check_ne( 1.00002_f32,  1.00001_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-1.00001_f32, -1.00002_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-1.00002_f32, -1.00001_f32, f32::EPSILON, f32::EPSILON);

        check_ne( 1.000001_f32,  1.000002_f32, f32::EPSILON, f32::EPSILON);
        check_ne( 1.000002_f32,  1.000001_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-1.000001_f32, -1.000002_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-1.000002_f32, -1.000001_f32, f32::EPSILON, f32::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_max() {
        check_eq( f32::MAX,  f32::MAX, f32::EPSILON, f32::EPSILON);
        check_eq(-f32::MAX, -f32::MAX, f32::EPSILON, f32::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_max() {
        check_ne( f32::MAX, -f32::MAX,           f32::EPSILON, f32::EPSILON);
        check_ne(-f32::MAX,  f32::MAX,           f32::EPSILON, f32::EPSILON);
        check_ne( f32::MAX,  f32::MAX / 2.0_f32, f32::EPSILON, f32::EPSILON);
        check_ne( f32::MAX, -f32::MAX / 2.0_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-f32::MAX,  f32::MAX / 2.0_f32, f32::EPSILON, f32::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_nan1() {
        check_ne( f32::NAN,  f32::NAN, 0_f32, f32::EPSILON);
        check_ne( f32::NAN, -f32::NAN, 0_f32, f32::EPSILON);
        check_ne(-f32::NAN,  f32::NAN, 0_f32, f32::EPSILON);
        check_ne(-f32::NAN, -f32::NAN, 0_f32, f32::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_nan2() {
        for i in 0..=i16::MAX {
            check_ne( f32::NAN , f32::NAN, 1_f32 / (i as f32), 1_f32 / (i as f32));
            check_ne( f32::NAN, -f32::NAN, 1_f32 / (i as f32), 1_f32 / (i as f32));
            check_ne(-f32::NAN,  f32::NAN, 1_f32 / (i as f32), 1_f32 / (i as f32));
            check_ne(-f32::NAN, -f32::NAN, 1_f32 / (i as f32), 1_f32 / (i as f32));
        }
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_nan3() {
        for i in 0..=i16::MAX {
            check_ne( f32::NAN , f32::NAN, i as f32, i as f32);
            check_ne( f32::NAN, -f32::NAN, i as f32, i as f32);
            check_ne(-f32::NAN,  f32::NAN, i as f32, i as f32);
            check_ne(-f32::NAN, -f32::NAN, i as f32, i as f32);
        }
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_nan4() {
        check_ne(f32::NAN, f32::NAN, f32::EPSILON, f32::EPSILON);
        
        check_ne( f32::NAN,  0.0_f32,  f32::EPSILON, f32::EPSILON);
        check_ne(-0_f32,     f32::NAN, f32::EPSILON, f32::EPSILON);
        check_ne( f32::NAN, -0.0_f32,  f32::EPSILON, f32::EPSILON);
        check_ne( 0_f32,     f32::NAN, f32::EPSILON, f32::EPSILON);

        check_ne( f32::NAN, f32::INFINITY,  f32::EPSILON, f32::EPSILON);
        check_ne( f32::INFINITY, f32::NAN,  f32::EPSILON, f32::EPSILON);
        check_ne( f32::NAN, -f32::INFINITY, f32::EPSILON, f32::EPSILON);
        check_ne(-f32::INFINITY, f32::NAN,  f32::EPSILON, f32::EPSILON);

        check_ne( f32::NAN, f32::MAX,  f32::EPSILON, f32::EPSILON);
        check_ne( f32::MAX, f32::NAN,  f32::EPSILON, f32::EPSILON);
        check_ne( f32::NAN, -f32::MAX, f32::EPSILON, f32::EPSILON);
        check_ne(-f32::MAX, f32::NAN,  f32::EPSILON, f32::EPSILON);

        check_ne( f32::NAN,          f32::MIN_POSITIVE,  f32::EPSILON, f32::EPSILON);
        check_ne( f32::MIN_POSITIVE, f32::NAN,           f32::EPSILON, f32::EPSILON);
        check_ne( f32::NAN,          -f32::MIN_POSITIVE, f32::EPSILON, f32::EPSILON);
        check_ne(-f32::MIN_POSITIVE, f32::NAN,           f32::EPSILON, f32::EPSILON);

        check_ne( f32::NAN,  1_f32,    f32::EPSILON, f32::EPSILON);
        check_ne( f32::NAN, -1_f32,    f32::EPSILON, f32::EPSILON);
        check_ne( 1_f32,     f32::NAN, f32::EPSILON, f32::EPSILON);
        check_ne(-1_f32,     f32::NAN, f32::EPSILON, f32::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_nan5() {
        check_ne(f32::NAN, f32::NAN, 1_f32, 2_f32);
        
        check_ne( f32::NAN,  0.0_f32,  1_f32, 2_f32);
        check_ne(-0_f32,     f32::NAN, 1_f32, 2_f32);
        check_ne( f32::NAN, -0.0_f32,  1_f32, 2_f32);
        check_ne( 0_f32,     f32::NAN, 1_f32, 2_f32);

        check_ne( f32::NAN, f32::INFINITY,  1_f32, 2_f32);
        check_ne( f32::INFINITY, f32::NAN,  1_f32, 2_f32);
        check_ne( f32::NAN, -f32::INFINITY, 1_f32, 2_f32);
        check_ne(-f32::INFINITY, f32::NAN,  1_f32, 2_f32);

        check_ne( f32::NAN, f32::MAX,  1_f32, 2_f32);
        check_ne( f32::MAX, f32::NAN,  1_f32, 2_f32);
        check_ne( f32::NAN, -f32::MAX, 1_f32, 2_f32);
        check_ne(-f32::MAX, f32::NAN,  1_f32, 2_f32);

        check_ne( f32::NAN,          f32::MIN_POSITIVE,  1_f32, 2_f32);
        check_ne( f32::MIN_POSITIVE, f32::NAN,           1_f32, 2_f32);
        check_ne( f32::NAN,          -f32::MIN_POSITIVE, 1_f32, 2_f32);
        check_ne(-f32::MIN_POSITIVE, f32::NAN,           1_f32, 2_f32);

        check_ne( f32::NAN,  1_f32,    1_f32, 2_f32);
        check_ne( f32::NAN, -1_f32,    1_f32, 2_f32);
        check_ne( 1_f32,     f32::NAN, 1_f32, 2_f32);
        check_ne(-1_f32,     f32::NAN, 1_f32, 2_f32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_infinity() {
        check_eq(f32::INFINITY,     f32::INFINITY,     f32::MAX,      f32::MAX);
        check_eq(f32::INFINITY,     f32::INFINITY,     f32::INFINITY, f32::INFINITY);
        check_eq(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::INFINITY, f32::INFINITY);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_infinity() {
        check_ne(f32::INFINITY,     f32::MAX,          f32::MAX, f32::MAX);
        check_ne(f32::INFINITY,     f32::NEG_INFINITY, f32::MAX, f32::MAX);
        check_ne(f32::NEG_INFINITY, f32::MAX,          f32::MAX, f32::MAX);
        check_ne(f32::INFINITY,     f32::MAX,          f32::INFINITY, f32::INFINITY);
        check_ne(f32::INFINITY,     f32::NEG_INFINITY, f32::INFINITY, f32::INFINITY);
        check_ne(f32::NEG_INFINITY, f32::MAX,          f32::INFINITY, f32::INFINITY);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero1() {
        check_eq( f32::MIN_POSITIVE,  f32::MIN_POSITIVE, f32::EPSILON, f32::EPSILON);
        check_eq( f32::MIN_POSITIVE, -f32::MIN_POSITIVE, f32::EPSILON, f32::EPSILON);
        check_eq(-f32::MIN_POSITIVE,  f32::MIN_POSITIVE, f32::EPSILON, f32::EPSILON);

        check_eq( f32::MIN_POSITIVE,  0_f32,             f32::EPSILON, f32::EPSILON);
        check_eq( 0_f32,              f32::MIN_POSITIVE, f32::EPSILON, f32::EPSILON);
        check_eq(-f32::MIN_POSITIVE,  0_f32,             f32::EPSILON, f32::EPSILON);
        check_eq( 0_f32,             -f32::MIN_POSITIVE, f32::EPSILON, f32::EPSILON);

        check_eq( 0.0000001_f32,      -f32::MIN_POSITIVE, f32::EPSILON, f32::EPSILON);
        check_eq( 0.0000001_f32,       f32::MIN_POSITIVE, f32::EPSILON, f32::EPSILON);
        check_eq( f32::MIN_POSITIVE,  0.0000001_f32,      f32::EPSILON, f32::EPSILON);
        check_eq(-f32::MIN_POSITIVE,  0.0000001_f32,      f32::EPSILON, f32::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero2() {
        check_eq( 0.000010001_f32,  0.000010002_f32, f32::EPSILON, f32::EPSILON);
        check_eq( 0.000010002_f32,  0.000010001_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-0.000010001_f32, -0.000010002_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-0.000010002_f32, -0.000010001_f32, f32::EPSILON, f32::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero3() {
        check_eq( 0.000011001_f32,  0.000011002_f32, f32::EPSILON, f32::EPSILON);
        check_eq( 0.000011002_f32,  0.000011001_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-0.000011001_f32, -0.000011002_f32, f32::EPSILON, f32::EPSILON);
        check_eq(-0.000011002_f32, -0.000011001_f32, f32::EPSILON, f32::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero4() {
        check_eq(1e-8_f32,  -1e-8_f32,  f32::EPSILON, f32::EPSILON);
        check_eq(1e-9_f32,  -1e-9_f32,  f32::EPSILON, f32::EPSILON);
        check_eq(1e-10_f32, -1e-10_f32, f32::EPSILON, f32::EPSILON);
        check_eq(1e-11_f32, -1e-11_f32, f32::EPSILON, f32::EPSILON);
        check_eq(1e-12_f32, -1e-12_f32, f32::EPSILON, f32::EPSILON);
        check_eq(1e-13_f32, -1e-13_f32, f32::EPSILON, f32::EPSILON);
        check_eq(1e-14_f32, -1e-14_f32, f32::EPSILON, f32::EPSILON);
        check_eq(1e-15_f32, -1e-15_f32, f32::EPSILON, f32::EPSILON);
        check_eq(1e-16_f32, -1e-16_f32, f32::EPSILON, f32::EPSILON);
        check_eq(1e-17_f32, -1e-17_f32, f32::EPSILON, f32::EPSILON);
        check_eq(1e-18_f32, -1e-18_f32, f32::EPSILON, f32::EPSILON);
        check_eq(1e-19_f32, -1e-19_f32, f32::EPSILON, f32::EPSILON);
        check_eq(1e-20_f32, -1e-20_f32, f32::EPSILON, f32::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero1() {
        check_ne( 0.000001_f32,      -f32::MIN_POSITIVE, f32::EPSILON, f32::EPSILON);
        check_ne( 0.000001_f32,       f32::MIN_POSITIVE, f32::EPSILON, f32::EPSILON);
        check_ne( f32::MIN_POSITIVE,  0.000001_f32,      f32::EPSILON, f32::EPSILON);
        check_ne(-f32::MIN_POSITIVE,  0.000001_f32,      f32::EPSILON, f32::EPSILON);

        check_ne(-0.000001_f32,      -f32::MIN_POSITIVE, f32::EPSILON, f32::EPSILON);
        check_ne(-0.000001_f32,       f32::MIN_POSITIVE, f32::EPSILON, f32::EPSILON);
        check_ne( f32::MIN_POSITIVE, -0.000001_f32,      f32::EPSILON, f32::EPSILON);
        check_ne(-f32::MIN_POSITIVE, -0.000001_f32,      f32::EPSILON, f32::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero2() {
        check_ne( 0.000001002_f32,  0.0000001001_f32, f32::EPSILON, f32::EPSILON);
        check_ne( 0.000001001_f32,  0.0000001002_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-0.000001002_f32, -0.0000001001_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-0.000001001_f32, -0.0000001002_f32, f32::EPSILON, f32::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero3() {
        check_ne( 0.000001102_f32,  0.0000001101_f32, f32::EPSILON, f32::EPSILON);
        check_ne( 0.000001101_f32,  0.0000001102_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-0.000001102_f32, -0.0000001101_f32, f32::EPSILON, f32::EPSILON);
        check_ne(-0.000001101_f32, -0.0000001102_f32, f32::EPSILON, f32::EPSILON);
    }
}


#[cfg(test)]
mod relative_eq_f64_tests {
    use approx_cmp::{
        RelativeEq,
        RelativeAllEq,
        assert_relative_eq,
        assert_relative_ne,
        relative_eq,
        relative_ne,
    };

    fn check_relative_eq(a: f64, b: f64, max_abs_diff: f64, max_relative: f64) {
        assert!(a.relative_eq(&b, &max_abs_diff, &max_relative));
        assert!(relative_eq!(a, b, abs_diff <= max_abs_diff, relative <= max_relative));
        assert_relative_eq!(a, b, abs_diff <= max_abs_diff, relative <= max_relative);
        
        assert!(a.relative_all_eq(&b, &max_abs_diff, &max_relative));
        assert!(relative_eq!(a, b, abs_diff_all <= max_abs_diff, relative_all <= max_relative));
        assert_relative_eq!(a, b, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    fn check_relative_ne(a: f64, b: f64, max_abs_diff: f64, max_relative: f64) {
        assert!(a.relative_ne(&b, &max_abs_diff, &max_relative));
        assert!(relative_ne!(a, b, abs_diff <= max_abs_diff, relative <= max_relative));
        assert_relative_ne!(a, b, abs_diff <= max_abs_diff, relative <= max_relative);

        assert!(a.relative_all_ne(&b, &max_abs_diff, &max_relative));
        assert!(relative_ne!(a, b, abs_diff_all <= max_abs_diff, relative_all <= max_relative));
        assert_relative_ne!(a, b, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    fn check_eq(a: f64, b: f64, max_abs_diff: f64, max_relative: f64) {
        check_relative_eq(a, b, max_abs_diff, max_relative);
        check_relative_eq(b, a, max_abs_diff, max_relative);
        check_relative_eq(-a, -b, max_abs_diff, max_relative);
        check_relative_eq(-b, -a, max_abs_diff, max_relative);
    }

    fn check_ne(a: f64, b: f64, max_abs_diff: f64, max_relative: f64) {
        check_relative_ne(a, b, max_abs_diff, max_relative);
        check_relative_ne(b, a, max_abs_diff, max_relative);
        check_relative_ne(-a, -b, max_abs_diff, max_relative);
        check_relative_ne(-b, -a, max_abs_diff, max_relative);
    }

    #[rustfmt::skip]
    fn check_eq_self(value: f64) {
        check_eq(value, value, 0.0,               f64::EPSILON);
        check_eq(value, value, f64::MIN_POSITIVE, f64::MIN_POSITIVE);
        check_eq(value, value, f64::MAX,          f64::MAX);
        check_eq(value, value, f64::INFINITY,     f64::INFINITY);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_zero() {
        check_eq_self(0.0);

        check_eq( 0.0,  0.0, f64::EPSILON, f64::EPSILON);
        check_eq(-0.0,  0.0, f64::EPSILON, f64::EPSILON);
        check_eq( 0.0, -0.0, f64::EPSILON, f64::EPSILON);
        check_eq(-0.0, -0.0, f64::EPSILON, f64::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_zero() {
        check_ne( 0.000001_f64, 0.0_f64,      f64::EPSILON, f64::EPSILON);
        check_ne( 0.0_f64,      0.000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.000001_f64, 0.0_f64,      f64::EPSILON, f64::EPSILON);
        check_ne( 0.0_f64,     -0.000001_f64, f64::EPSILON, f64::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_tolerance() {
        check_eq( 0.0_f64,    1e-40_f64, 1e-40_f64, 1e-40_f64);
        check_eq( 1e-40_f64,  0.0_f64,   1e-40_f64, 1e-40_f64);
        check_eq( 0.0_f64,   -1e-40_f64, 1e-40_f64, 1e-40_f64);
        check_eq(-1e-40_f64,  0.0_f64,   1e-40_f64, 1e-40_f64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_tolerance() {
        check_ne( 1e-40_f64,  0.0_f64,   1e-41_f64, 1e-41_f64);
        check_ne( 0.0_f64,    1e-40_f64, 1e-41_f64, 1e-41_f64);
        check_ne(-1e-40_f64,  0.0_f64,   1e-41_f64, 1e-41_f64);
        check_ne( 0.0_f64,   -1e-40_f64, 1e-41_f64, 1e-41_f64);
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

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        check_ne( 1.0_f64,  2.0_f64, f64::EPSILON,      f64::EPSILON);
        check_ne( 1.0_f64,  2.0_f64, f64::MIN_POSITIVE, f64::MIN_POSITIVE);
        check_ne( 1.0_f64, -2.0_f64, f64::EPSILON,      f64::EPSILON);
        check_ne( 1.0_f64, -2.0_f64, f64::MIN_POSITIVE, f64::MIN_POSITIVE);
        check_ne(-1.0_f64,  2.0_f64, f64::EPSILON,      f64::EPSILON);
        check_ne(-1.0_f64,  2.0_f64, f64::MIN_POSITIVE, f64::MIN_POSITIVE);
        check_ne(-1.0_f64, -2.0_f64, f64::EPSILON,      f64::EPSILON);
        check_ne(-1.0_f64, -2.0_f64, f64::MIN_POSITIVE, f64::MIN_POSITIVE);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding1() {
        check_eq( 10000000000000000.0_f64,  10000000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_eq( 10000000000000001.0_f64,  10000000000000000.0_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-10000000000000000.0_f64, -10000000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-10000000000000001.0_f64, -10000000000000000.0_f64, f64::EPSILON, f64::EPSILON);

        check_eq( 100000000000000000.0_f64,  100000000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_eq( 100000000000000001.0_f64,  100000000000000000.0_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-100000000000000000.0_f64, -100000000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-100000000000000001.0_f64, -100000000000000000.0_f64, f64::EPSILON, f64::EPSILON);

        check_eq( 1000000000000000000.0_f64,  1000000000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_eq( 1000000000000000001.0_f64,  1000000000000000000.0_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-1000000000000000000.0_f64, -1000000000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-1000000000000000001.0_f64, -1000000000000000000.0_f64, f64::EPSILON, f64::EPSILON);

        check_eq( 10000000000000000000.0_f64,  10000000000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_eq( 10000000000000000001.0_f64,  10000000000000000000.0_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-10000000000000000000.0_f64, -10000000000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-10000000000000000001.0_f64, -10000000000000000000.0_f64, f64::EPSILON, f64::EPSILON);

        check_eq( 100000000000000000000.0_f64,  100000000000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_eq( 100000000000000000001.0_f64,  100000000000000000000.0_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-100000000000000000000.0_f64, -100000000000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-100000000000000000001.0_f64, -100000000000000000000.0_f64, f64::EPSILON, f64::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding1() {
        check_ne( 1000.0_f64,  1001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1001.0_f64,  1000.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1000.0_f64, -1001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1001.0_f64, -1000.0_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 10000.0_f64,  10001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 10001.0_f64,  10000.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-10000.0_f64, -10001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-10001.0_f64, -10000.0_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 100000.0_f64,  100001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 100001.0_f64,  100000.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-100000.0_f64, -100001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-100001.0_f64, -100000.0_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 1000000.0_f64,  1000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1000001.0_f64,  1000000.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1000000.0_f64, -1000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1000001.0_f64, -1000000.0_f64, f64::EPSILON, f64::EPSILON);
        
        check_ne( 10000000.0_f64,  10000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 10000001.0_f64,  10000000.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-10000000.0_f64, -10000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-10000001.0_f64, -10000000.0_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 100000000.0_f64,  100000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 100000001.0_f64,  100000000.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-100000000.0_f64, -100000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-100000001.0_f64, -100000000.0_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 1000000000.0_f64,  1000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1000000001.0_f64,  1000000000.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1000000000.0_f64, -1000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1000000001.0_f64, -1000000000.0_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 10000000000.0_f64,  10000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 10000000001.0_f64,  10000000000.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-10000000000.0_f64, -10000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-10000000001.0_f64, -10000000000.0_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 100000000000.0_f64,  100000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 100000000001.0_f64,  100000000000.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-100000000000.0_f64, -100000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-100000000001.0_f64, -100000000000.0_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 1000000000000.0_f64,  1000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1000000000001.0_f64,  1000000000000.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1000000000000.0_f64, -1000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1000000000001.0_f64, -1000000000000.0_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 10000000000000.0_f64,  10000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 10000000000001.0_f64,  10000000000000.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-10000000000000.0_f64, -10000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-10000000000001.0_f64, -10000000000000.0_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 100000000000000.0_f64,  100000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 100000000000001.0_f64,  100000000000000.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-100000000000000.0_f64, -100000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-100000000000001.0_f64, -100000000000000.0_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 1000000000000000.0_f64,  1000000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1000000000000001.0_f64,  1000000000000000.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1000000000000000.0_f64, -1000000000000001.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1000000000000001.0_f64, -1000000000000000.0_f64, f64::EPSILON, f64::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding2() {
        check_eq( 1.0000000000000001_f64,  1.0000000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_eq( 1.0000000000000002_f64,  1.0000000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-1.0000000000000001_f64, -1.0000000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-1.0000000000000002_f64, -1.0000000000000001_f64, f64::EPSILON, f64::EPSILON);

        check_eq( 1.00000000000000001_f64,  1.00000000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_eq( 1.00000000000000002_f64,  1.00000000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-1.00000000000000001_f64, -1.00000000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-1.00000000000000002_f64, -1.00000000000000001_f64, f64::EPSILON, f64::EPSILON);

        check_eq( 1.000000000000000001_f64,  1.000000000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_eq( 1.000000000000000002_f64,  1.000000000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-1.000000000000000001_f64, -1.000000000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-1.000000000000000002_f64, -1.000000000000000001_f64, f64::EPSILON, f64::EPSILON);

        check_eq( 1.0000000000000000001_f64,  1.0000000000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_eq( 1.0000000000000000002_f64,  1.0000000000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-1.0000000000000000001_f64, -1.0000000000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-1.0000000000000000002_f64, -1.0000000000000000001_f64, f64::EPSILON, f64::EPSILON);

        check_eq( 1.00000000000000000001_f64,  1.00000000000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_eq( 1.00000000000000000002_f64,  1.00000000000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-1.00000000000000000001_f64, -1.00000000000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-1.00000000000000000002_f64, -1.00000000000000000001_f64, f64::EPSILON, f64::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding2() {
        check_ne( 1.01_f64,  1.02_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1.02_f64,  1.01_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.01_f64, -1.02_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.02_f64, -1.01_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 1.001_f64,  1.002_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1.002_f64,  1.001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.001_f64, -1.002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.002_f64, -1.001_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 1.0001_f64,  1.0002_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1.0002_f64,  1.0001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.0001_f64, -1.0002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.0002_f64, -1.0001_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 1.00001_f64,  1.00002_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1.00002_f64,  1.00001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.00001_f64, -1.00002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.00002_f64, -1.00001_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 1.000001_f64,  1.000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1.000002_f64,  1.000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.000001_f64, -1.000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.000002_f64, -1.000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 1.0000001_f64,  1.0000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1.0000002_f64,  1.0000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.0000001_f64, -1.0000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.0000002_f64, -1.0000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 1.00000001_f64,  1.00000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1.00000002_f64,  1.00000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.00000001_f64, -1.00000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.00000002_f64, -1.00000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 1.000000001_f64,  1.000000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1.000000002_f64,  1.000000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.000000001_f64, -1.000000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.000000002_f64, -1.000000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 1.0000000001_f64,  1.0000000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1.0000000002_f64,  1.0000000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.0000000001_f64, -1.0000000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.0000000002_f64, -1.0000000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 1.00000000001_f64,  1.00000000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1.00000000002_f64,  1.00000000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.00000000001_f64, -1.00000000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.00000000002_f64, -1.00000000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 1.000000000001_f64,  1.000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1.000000000002_f64,  1.000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.000000000001_f64, -1.000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.000000000002_f64, -1.000000000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 1.0000000000001_f64,  1.0000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1.0000000000002_f64,  1.0000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.0000000000001_f64, -1.0000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.0000000000002_f64, -1.0000000000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 1.00000000000001_f64,  1.00000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1.00000000000002_f64,  1.00000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.00000000000001_f64, -1.00000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.00000000000002_f64, -1.00000000000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 1.000000000000001_f64,  1.000000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 1.000000000000002_f64,  1.000000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.000000000000001_f64, -1.000000000000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-1.000000000000002_f64, -1.000000000000001_f64, f64::EPSILON, f64::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_max() {
        check_eq( f64::MAX,    f64::MAX, f64::EPSILON, f64::EPSILON);
        check_eq(-f64::MAX,  -f64::MAX, f64::EPSILON, f64::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_max() {
        check_ne( f64::MAX, -f64::MAX,           f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MAX,  f64::MAX,           f64::EPSILON, f64::EPSILON);
        check_ne( f64::MAX,  f64::MAX / 2.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne( f64::MAX, -f64::MAX / 2.0_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MAX,  f64::MAX / 2.0_f64, f64::EPSILON, f64::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_nan1() {
        check_ne( f64::NAN,  f64::NAN, 0_f64, f64::EPSILON);
        check_ne( f64::NAN, -f64::NAN, 0_f64, f64::EPSILON);
        check_ne(-f64::NAN,  f64::NAN, 0_f64, f64::EPSILON);
        check_ne(-f64::NAN, -f64::NAN, 0_f64, f64::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_nan2() {
        for i in 0..=i16::MAX {
            check_ne( f64::NAN , f64::NAN, 1_f64 / (i as f64), 1_f64 / (i as f64));
            check_ne( f64::NAN, -f64::NAN, 1_f64 / (i as f64), 1_f64 / (i as f64));
            check_ne(-f64::NAN,  f64::NAN, 1_f64 / (i as f64), 1_f64 / (i as f64));
            check_ne(-f64::NAN, -f64::NAN, 1_f64 / (i as f64), 1_f64 / (i as f64));
        }
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_nan3() {
        for i in 0..=i16::MAX {
            check_ne( f64::NAN , f64::NAN, i as f64, i as f64);
            check_ne( f64::NAN, -f64::NAN, i as f64, i as f64);
            check_ne(-f64::NAN,  f64::NAN, i as f64, i as f64);
            check_ne(-f64::NAN, -f64::NAN, i as f64, i as f64);
        }
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_nan4() {
        check_ne(f64::NAN, f64::NAN, f64::EPSILON, f64::EPSILON);
        
        check_ne( f64::NAN,  0.0_f64,  f64::EPSILON, f64::EPSILON);
        check_ne(-0_f64,     f64::NAN, f64::EPSILON, f64::EPSILON);
        check_ne( f64::NAN, -0.0_f64,  f64::EPSILON, f64::EPSILON);
        check_ne( 0_f64,     f64::NAN, f64::EPSILON, f64::EPSILON);

        check_ne( f64::NAN, f64::INFINITY,  f64::EPSILON, f64::EPSILON);
        check_ne( f64::INFINITY, f64::NAN,  f64::EPSILON, f64::EPSILON);
        check_ne( f64::NAN, -f64::INFINITY, f64::EPSILON, f64::EPSILON);
        check_ne(-f64::INFINITY, f64::NAN,  f64::EPSILON, f64::EPSILON);

        check_ne( f64::NAN, f64::MAX,  f64::EPSILON, f64::EPSILON);
        check_ne( f64::MAX, f64::NAN,  f64::EPSILON, f64::EPSILON);
        check_ne( f64::NAN, -f64::MAX, f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MAX, f64::NAN,  f64::EPSILON, f64::EPSILON);

        check_ne( f64::NAN,          f64::MIN_POSITIVE,  f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE, f64::NAN,           f64::EPSILON, f64::EPSILON);
        check_ne( f64::NAN,          -f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE, f64::NAN,           f64::EPSILON, f64::EPSILON);

        check_ne( f64::NAN,  1_f64,    f64::EPSILON, f64::EPSILON);
        check_ne( f64::NAN, -1_f64,    f64::EPSILON, f64::EPSILON);
        check_ne( 1_f64,     f64::NAN, f64::EPSILON, f64::EPSILON);
        check_ne(-1_f64,     f64::NAN, f64::EPSILON, f64::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_nan5() {
        check_ne(f64::NAN, f64::NAN, 1_f64, 2_f64);
        
        check_ne( f64::NAN,  0.0_f64,  1_f64, 2_f64);
        check_ne(-0_f64,     f64::NAN, 1_f64, 2_f64);
        check_ne( f64::NAN, -0.0_f64,  1_f64, 2_f64);
        check_ne( 0_f64,     f64::NAN, 1_f64, 2_f64);

        check_ne( f64::NAN, f64::INFINITY,  1_f64, 2_f64);
        check_ne( f64::INFINITY, f64::NAN,  1_f64, 2_f64);
        check_ne( f64::NAN, -f64::INFINITY, 1_f64, 2_f64);
        check_ne(-f64::INFINITY, f64::NAN,  1_f64, 2_f64);

        check_ne( f64::NAN, f64::MAX,  1_f64, 2_f64);
        check_ne( f64::MAX, f64::NAN,  1_f64, 2_f64);
        check_ne( f64::NAN, -f64::MAX, 1_f64, 2_f64);
        check_ne(-f64::MAX, f64::NAN,  1_f64, 2_f64);

        check_ne( f64::NAN,          f64::MIN_POSITIVE,  1_f64, 2_f64);
        check_ne( f64::MIN_POSITIVE, f64::NAN,           1_f64, 2_f64);
        check_ne( f64::NAN,          -f64::MIN_POSITIVE, 1_f64, 2_f64);
        check_ne(-f64::MIN_POSITIVE, f64::NAN,           1_f64, 2_f64);

        check_ne( f64::NAN,  1_f64,    1_f64, 2_f64);
        check_ne( f64::NAN, -1_f64,    1_f64, 2_f64);
        check_ne( 1_f64,     f64::NAN, 1_f64, 2_f64);
        check_ne(-1_f64,     f64::NAN, 1_f64, 2_f64);
    }
    
    #[rustfmt::skip]
    #[test]
    fn test_eq_infinity() {
        check_eq(f64::INFINITY,     f64::INFINITY,     f64::MAX,      f64::MAX);
        check_eq(f64::INFINITY,     f64::INFINITY,     f64::INFINITY, f64::INFINITY);
        check_eq(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::INFINITY);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_infinity() {
        check_ne(f64::INFINITY,     f64::MAX,          f64::MAX, f64::MAX);
        check_ne(f64::INFINITY,     f64::NEG_INFINITY, f64::MAX, f64::MAX);
        check_ne(f64::NEG_INFINITY, f64::MAX,          f64::MAX, f64::MAX);
        check_ne(f64::INFINITY,     f64::MAX,          f64::INFINITY, f64::INFINITY);
        check_ne(f64::INFINITY,     f64::NEG_INFINITY, f64::INFINITY, f64::INFINITY);
        check_ne(f64::NEG_INFINITY, f64::MAX,          f64::INFINITY, f64::INFINITY);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero1() {
        check_eq( f64::MIN_POSITIVE,  f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_eq( f64::MIN_POSITIVE, -f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_eq(-f64::MIN_POSITIVE,  f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);

        check_eq( f64::MIN_POSITIVE,  0_f64,             f64::EPSILON, f64::EPSILON);
        check_eq( 0_f64,              f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_eq(-f64::MIN_POSITIVE,  0_f64,             f64::EPSILON, f64::EPSILON);
        check_eq( 0_f64,             -f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);

        check_eq( 0.0000000000000001_f64, -f64::MIN_POSITIVE,      f64::EPSILON, f64::EPSILON);
        check_eq( 0.0000000000000001_f64,  f64::MIN_POSITIVE,      f64::EPSILON, f64::EPSILON);
        check_eq( f64::MIN_POSITIVE,       0.0000000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-f64::MIN_POSITIVE,       0.0000000000000001_f64, f64::EPSILON, f64::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero2() {
        check_eq( 0.0000000010000001_f64,  0.0000000010000002_f64, f64::EPSILON, f64::EPSILON);
        check_eq( 0.0000000010000002_f64,  0.0000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-0.0000000010000001_f64, -0.0000000010000002_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-0.0000000010000002_f64, -0.0000000010000001_f64, f64::EPSILON, f64::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero3() {
        check_eq( 0.0000000011000001_f64,  0.0000000011000002_f64, f64::EPSILON, f64::EPSILON);
        check_eq( 0.0000000011000002_f64,  0.0000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-0.0000000011000001_f64, -0.0000000011000002_f64, f64::EPSILON, f64::EPSILON);
        check_eq(-0.0000000011000002_f64, -0.0000000011000001_f64, f64::EPSILON, f64::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero4() {
        check_eq(1e-16_f64, -1e-16_f64, f64::EPSILON, f64::EPSILON);
        check_eq(1e-17_f64, -1e-17_f64, f64::EPSILON, f64::EPSILON);
        check_eq(1e-18_f64, -1e-18_f64, f64::EPSILON, f64::EPSILON);
        check_eq(1e-19_f64, -1e-19_f64, f64::EPSILON, f64::EPSILON);
        check_eq(1e-20_f64, -1e-20_f64, f64::EPSILON, f64::EPSILON);
        check_eq(1e-21_f64, -1e-21_f64, f64::EPSILON, f64::EPSILON);
        check_eq(1e-22_f64, -1e-22_f64, f64::EPSILON, f64::EPSILON);
        check_eq(1e-23_f64, -1e-23_f64, f64::EPSILON, f64::EPSILON);
        check_eq(1e-24_f64, -1e-24_f64, f64::EPSILON, f64::EPSILON);
        check_eq(1e-25_f64, -1e-25_f64, f64::EPSILON, f64::EPSILON);
        check_eq(1e-26_f64, -1e-26_f64, f64::EPSILON, f64::EPSILON);
        check_eq(1e-27_f64, -1e-27_f64, f64::EPSILON, f64::EPSILON);
        check_eq(1e-28_f64, -1e-28_f64, f64::EPSILON, f64::EPSILON);
        check_eq(1e-29_f64, -1e-29_f64, f64::EPSILON, f64::EPSILON);
        check_eq(1e-30_f64, -1e-30_f64, f64::EPSILON, f64::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero1() {
        check_ne( 0.000001_f64,      -f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( 0.000001_f64,       f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,  0.000001_f64,      f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,  0.000001_f64,      f64::EPSILON, f64::EPSILON);

        check_ne(-0.000001_f64,      -f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne(-0.000001_f64,       f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE, -0.000001_f64,      f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE, -0.000001_f64,      f64::EPSILON, f64::EPSILON);

        check_ne( 0.0000001_f64,     -f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( 0.0000001_f64,      f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,  0.0000001_f64,     f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,  0.0000001_f64,     f64::EPSILON, f64::EPSILON);

        check_ne(-0.0000001_f64,     -f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000001_f64,      f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE, -0.0000001_f64,     f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE, -0.0000001_f64,     f64::EPSILON, f64::EPSILON);

        check_ne( 0.00000001_f64,    -f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( 0.00000001_f64,     f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,  0.00000001_f64,    f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,  0.00000001_f64,    f64::EPSILON, f64::EPSILON);

        check_ne(-0.00000001_f64,    -f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne(-0.00000001_f64,     f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE, -0.00000001_f64,    f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE, -0.00000001_f64,    f64::EPSILON, f64::EPSILON);

        check_ne( 0.000000001_f64,   -f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( 0.000000001_f64,    f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,  0.000000001_f64,   f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,  0.000000001_f64,   f64::EPSILON, f64::EPSILON);

        check_ne(-0.000000001_f64,   -f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne(-0.000000001_f64,    f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE, -0.000000001_f64,   f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE, -0.000000001_f64,   f64::EPSILON, f64::EPSILON);

        check_ne( 0.0000000001_f64,  -f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( 0.0000000001_f64,   f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,  0.0000000001_f64,  f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,  0.0000000001_f64,  f64::EPSILON, f64::EPSILON);

        check_ne(-0.0000000001_f64,  -f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000000001_f64,   f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE, -0.0000000001_f64,  f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE, -0.0000000001_f64,  f64::EPSILON, f64::EPSILON);

        check_ne( 0.00000000001_f64, -f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( 0.00000000001_f64,  f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,  0.00000000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,  0.00000000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne(-0.00000000001_f64, -f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne(-0.00000000001_f64,  f64::MIN_POSITIVE, f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE, -0.00000000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE, -0.00000000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.000000000001_f64, -f64::MIN_POSITIVE,  f64::EPSILON, f64::EPSILON);
        check_ne( 0.000000000001_f64,  f64::MIN_POSITIVE,  f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,   0.000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,   0.000000000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne(-0.000000000001_f64, -f64::MIN_POSITIVE,  f64::EPSILON, f64::EPSILON);
        check_ne(-0.000000000001_f64,  f64::MIN_POSITIVE,  f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,  -0.000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,  -0.000000000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.0000000000001_f64, -f64::MIN_POSITIVE,   f64::EPSILON, f64::EPSILON);
        check_ne( 0.0000000000001_f64,  f64::MIN_POSITIVE,   f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,    0.0000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,    0.0000000000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne(-0.0000000000001_f64, -f64::MIN_POSITIVE,   f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000000000001_f64,  f64::MIN_POSITIVE,   f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,   -0.0000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,   -0.0000000000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.00000000000001_f64, -f64::MIN_POSITIVE,    f64::EPSILON, f64::EPSILON);
        check_ne( 0.00000000000001_f64,  f64::MIN_POSITIVE,    f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,     0.00000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,     0.00000000000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne(-0.00000000000001_f64, -f64::MIN_POSITIVE,    f64::EPSILON, f64::EPSILON);
        check_ne(-0.00000000000001_f64,  f64::MIN_POSITIVE,    f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,    -0.00000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,    -0.00000000000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.000000000000001_f64, -f64::MIN_POSITIVE,     f64::EPSILON, f64::EPSILON);
        check_ne( 0.000000000000001_f64,  f64::MIN_POSITIVE,     f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,      0.000000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,      0.000000000000001_f64, f64::EPSILON, f64::EPSILON);

        check_ne(-0.000000000000001_f64, -f64::MIN_POSITIVE,     f64::EPSILON, f64::EPSILON);
        check_ne(-0.000000000000001_f64,  f64::MIN_POSITIVE,     f64::EPSILON, f64::EPSILON);
        check_ne( f64::MIN_POSITIVE,     -0.000000000000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-f64::MIN_POSITIVE,     -0.000000000000001_f64, f64::EPSILON, f64::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero2() {
        check_ne( 0.0000010000002_f64,  0.00000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.0000010000001_f64,  0.00000010000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000010000002_f64, -0.00000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000010000001_f64, -0.00000010000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.00000010000002_f64,  0.000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.00000010000001_f64,  0.000000010000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.00000010000002_f64, -0.000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.00000010000001_f64, -0.000000010000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.000000010000002_f64,  0.0000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.000000010000001_f64,  0.0000000010000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.000000010000002_f64, -0.0000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.000000010000001_f64, -0.0000000010000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.0000000010000002_f64,  0.00000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.0000000010000001_f64,  0.00000000010000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000000010000002_f64, -0.00000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000000010000001_f64, -0.00000000010000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.00000000010000002_f64,  0.000000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.00000000010000001_f64,  0.000000000010000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.00000000010000002_f64, -0.000000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.00000000010000001_f64, -0.000000000010000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.000000000010000002_f64,  0.0000000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.000000000010000001_f64,  0.0000000000010000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.000000000010000002_f64, -0.0000000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.000000000010000001_f64, -0.0000000000010000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.0000000000010000002_f64,  0.00000000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.0000000000010000001_f64,  0.00000000000010000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000000000010000002_f64, -0.00000000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000000000010000001_f64, -0.00000000000010000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.00000000000010000002_f64,  0.000000000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.00000000000010000001_f64,  0.000000000000010000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.00000000000010000002_f64, -0.000000000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.00000000000010000001_f64, -0.000000000000010000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.000000000000010000002_f64,  0.0000000000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.000000000000010000001_f64,  0.0000000000000010000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.000000000000010000002_f64, -0.0000000000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.000000000000010000001_f64, -0.0000000000000010000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.0000000000000010000002_f64,  0.00000000000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.0000000000000010000001_f64,  0.00000000000000010000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000000000000010000002_f64, -0.00000000000000010000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000000000000010000001_f64, -0.00000000000000010000002_f64, f64::EPSILON, f64::EPSILON);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero3() {
        check_ne( 0.0000011000002_f64,  0.00000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.0000011000001_f64,  0.00000011000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000011000002_f64, -0.00000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000011000001_f64, -0.00000011000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.00000011000002_f64,  0.000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.00000011000001_f64,  0.000000011000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.00000011000002_f64, -0.000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.00000011000001_f64, -0.000000011000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.000000011000002_f64,  0.0000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.000000011000001_f64,  0.0000000011000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.000000011000002_f64, -0.0000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.000000011000001_f64, -0.0000000011000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.0000000011000002_f64,  0.00000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.0000000011000001_f64,  0.00000000011000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000000011000002_f64, -0.00000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000000011000001_f64, -0.00000000011000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.00000000011000002_f64,  0.000000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.00000000011000001_f64,  0.000000000011000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.00000000011000002_f64, -0.000000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.00000000011000001_f64, -0.000000000011000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.000000000011000002_f64,  0.0000000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.000000000011000001_f64,  0.0000000000011000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.000000000011000002_f64, -0.0000000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.000000000011000001_f64, -0.0000000000011000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.0000000000011000002_f64,  0.00000000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.0000000000011000001_f64,  0.00000000000011000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000000000011000002_f64, -0.00000000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000000000011000001_f64, -0.00000000000011000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.00000000000011000002_f64,  0.000000000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.00000000000011000001_f64,  0.000000000000011000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.00000000000011000002_f64, -0.000000000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.00000000000011000001_f64, -0.000000000000011000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.000000000000011000002_f64,  0.0000000000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.000000000000011000001_f64,  0.0000000000000011000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.000000000000011000002_f64, -0.0000000000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.000000000000011000001_f64, -0.0000000000000011000002_f64, f64::EPSILON, f64::EPSILON);

        check_ne( 0.0000000000000011000002_f64,  0.00000000000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne( 0.0000000000000011000001_f64,  0.00000000000000011000002_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000000000000011000002_f64, -0.00000000000000011000001_f64, f64::EPSILON, f64::EPSILON);
        check_ne(-0.0000000000000011000001_f64, -0.00000000000000011000002_f64, f64::EPSILON, f64::EPSILON);
    }
}

macro_rules! impl_relative_eq_float_exact_exhaustive_tests {
    ($(($module_name:ident, $FloatType:ty, $IntegerType:ty)),*) => {$(
        #[cfg(test)]
        mod $module_name {
            use approx_cmp::{
                RelativeEq,
                assert_relative_eq,
                assert_relative_ne,
                relative_eq,
                relative_ne,
            };
    
            #[test]
            fn test_abs_relative_exactly_representable_exhaustive() {
                for i in <$IntegerType>::MIN..<$IntegerType>::MAX {
                    assert!((i as $FloatType).relative_eq(&(i as $FloatType), &0.0, &<$FloatType>::EPSILON));
                    assert!(relative_eq!(i as $FloatType, i as $FloatType, abs_diff <= 0.0, relative <= <$FloatType>::EPSILON));
                    assert_relative_eq!(i as $FloatType, i as $FloatType, abs_diff <= 0.0, relative <= <$FloatType>::EPSILON);
                }
            }
        
            #[test]
            fn test_relative_ne_exactly_representable_exhaustive1() {
                for i in <$IntegerType>::MIN..<$IntegerType>::MAX {
                    assert!(((i + 1) as $FloatType).relative_ne(&(i as $FloatType), &0.0, &<$FloatType>::EPSILON));
                    assert!(relative_ne!((i + 1) as $FloatType, i as $FloatType, abs_diff <= 0.0, relative <= <$FloatType>::EPSILON));
                    assert_relative_ne!((i + 1) as $FloatType, i as $FloatType, abs_diff <= 0.0, relative <= <$FloatType>::EPSILON);

                    assert!((i as $FloatType).relative_ne(&((i + 1) as $FloatType), &0.0, &<$FloatType>::EPSILON));
                    assert!(relative_ne!(i as $FloatType, (i + 1) as $FloatType, abs_diff <= 0.0, relative <= <$FloatType>::EPSILON));
                    assert_relative_ne!(i as $FloatType, (i + 1) as $FloatType, abs_diff <= 0.0, relative <= <$FloatType>::EPSILON);
                }
            }
        }
    )*};
}

impl_relative_eq_float_exact_exhaustive_tests!(
    (relative_eq_f32_u8_exact_exhaustive_tests,  f32, u8),
    (relative_eq_f32_u16_exact_exhaustive_tests, f32, u16),
    (relative_eq_f32_i8_exact_exhaustive_tests,  f32, i8),
    (relative_eq_f32_i16_exact_exhaustive_tests, f32, i16),
    (relative_eq_f64_u8_exact_exhaustive_tests,  f64, u8),
    (relative_eq_f64_u16_exact_exhaustive_tests, f64, u16),
    (relative_eq_f64_i8_exact_exhaustive_tests,  f64, i8),
    (relative_eq_f64_i16_exact_exhaustive_tests, f64, i16)
);


#[cfg(test)]
mod relative_eq_array_f32_tests {
    use approx_cmp::{
        RelativeEq,
        RelativeAllEq,
        assert_relative_eq,
        assert_relative_ne,
        relative_eq,
        relative_ne,
    };

    fn array_uniform<const N: usize>(value: f32) -> [f32; N] {
        [value; N]
    }

    fn array_range<const N: usize>(min_value: f32) -> [f32; N] {
        let mut array = [0_f32; N];
        for i in 0..N {
            array[i] = (min_value + (i as f32)) as f32;
        }

        array
    }

    fn check_eq_array<const N: usize>(value: f32) {
        let lhs = array_uniform::<N>(value);
        let rhs = array_uniform::<N>(value);
        let max_abs_diff = array_uniform::<N>(f32::EPSILON);
        let max_relative = array_uniform::<N>(f32::EPSILON);

        assert!(lhs.relative_eq(&rhs, &max_abs_diff, &max_relative));
        assert!(relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative));
        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    fn check_ne_array<const N: usize>(value: f32, min_value: f32) {
        let lhs = array_uniform::<N>(value);
        let rhs = array_range::<N>(min_value);
        let max_abs_diff = array_uniform::<N>(f32::EPSILON);
        let max_relative = array_uniform::<N>(f32::EPSILON);

        assert!(lhs.relative_ne(&rhs, &max_abs_diff, &max_relative));
        assert!(relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative));
        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    fn check_all_eq_array<const N: usize>(value: f32) {
        let lhs = array_uniform::<N>(value);
        let rhs = array_uniform::<N>(value);

        assert!(lhs.relative_all_eq(&rhs, &f32::EPSILON, &f32::EPSILON));
        assert!(relative_eq!(lhs, rhs, abs_diff_all <= f32::EPSILON, relative_all <= f32::EPSILON));
        assert_relative_eq!(lhs, rhs, abs_diff_all <= f32::EPSILON, relative_all <= f32::EPSILON);
    }

    fn check_all_ne_array<const N: usize>(value: f32, min_value: f32) {
        let lhs = array_uniform::<N>(value);
        let rhs = array_range::<N>(min_value);

        assert!(lhs.relative_all_ne(&rhs, &f32::EPSILON, &f32::EPSILON));
        assert!(relative_ne!(lhs, rhs, abs_diff_all <= f32::EPSILON, relative_all <= f32::EPSILON));
        assert_relative_ne!(lhs, rhs, abs_diff_all <= f32::EPSILON, relative_all <= f32::EPSILON);
    }

    #[test]
    fn test_eq_array_empty() {
        check_eq_array::<0>(1_f32);
        check_all_eq_array::<0>(1_f32);
    }

    #[test]
    fn test_eq_array() {
        check_eq_array::<1>(1_f32);
        check_eq_array::<2>(1_f32);
        check_eq_array::<3>(1_f32);
        check_eq_array::<4>(1_f32);
        check_eq_array::<8>(1_f32);
        check_eq_array::<16>(1_f32);
        check_eq_array::<32>(1_f32);
        check_eq_array::<64>(1_f32);
    }

    #[test]
    fn test_ne_array() {
        check_ne_array::<1>(1_f32, 2_f32);
        check_ne_array::<2>(1_f32, 2_f32);
        check_ne_array::<3>(1_f32, 2_f32);
        check_ne_array::<4>(1_f32, 2_f32);
        check_ne_array::<8>(1_f32, 2_f32);
        check_ne_array::<16>(1_f32, 2_f32);
        check_ne_array::<32>(1_f32, 2_f32);
        check_ne_array::<64>(1_f32, 2_f32);
    }

    #[test]
    fn test_all_eq_array() {
        check_all_eq_array::<1>(1_f32);
        check_all_eq_array::<2>(1_f32);
        check_all_eq_array::<3>(1_f32);
        check_all_eq_array::<4>(1_f32);
        check_all_eq_array::<8>(1_f32);
        check_all_eq_array::<16>(1_f32);
        check_all_eq_array::<32>(1_f32);
        check_all_eq_array::<64>(1_f32);
    }

    #[test]
    fn test_all_ne_array() {
        check_all_ne_array::<1>(1_f32, 2_f32);
        check_all_ne_array::<2>(1_f32, 2_f32);
        check_all_ne_array::<3>(1_f32, 2_f32);
        check_all_ne_array::<4>(1_f32, 2_f32);
        check_all_ne_array::<8>(1_f32, 2_f32);
        check_all_ne_array::<16>(1_f32, 2_f32);
        check_all_ne_array::<32>(1_f32, 2_f32);
        check_all_ne_array::<64>(1_f32, 2_f32);
    }
}


#[cfg(test)]
mod relative_eq_array_f64_tests {
    use approx_cmp::{
        RelativeEq,
        RelativeAllEq,
        assert_relative_eq,
        assert_relative_ne,
        relative_eq,
        relative_ne,
    };

    fn array_uniform<const N: usize>(value: f64) -> [f64; N] {
        [value; N]
    }

    fn array_range<const N: usize>(min_value: f64) -> [f64; N] {
        let mut array = [0_f64; N];
        for i in 0..N {
            array[i] = (min_value + (i as f64)) as f64;
        }

        array
    }

    fn check_eq_array<const N: usize>(value: f64) {
        let lhs = array_uniform::<N>(value);
        let rhs = array_uniform::<N>(value);
        let max_abs_diff = array_uniform::<N>(f64::EPSILON);
        let max_relative = array_uniform::<N>(f64::EPSILON);

        assert!(lhs.relative_eq(&rhs, &max_abs_diff, &max_relative));
        assert!(relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative));
        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    fn check_ne_array<const N: usize>(value: f64, min_value: f64) {
        let lhs = array_uniform::<N>(value);
        let rhs = array_range::<N>(min_value);
        let max_abs_diff = array_uniform::<N>(f64::EPSILON);
        let max_relative = array_uniform::<N>(f64::EPSILON);

        assert!(lhs.relative_ne(&rhs, &max_abs_diff, &max_relative));
        assert!(relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative));
        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    fn check_all_eq_array<const N: usize>(value: f64) {
        let lhs = array_uniform::<N>(value);
        let rhs = array_uniform::<N>(value);

        assert!(lhs.relative_all_eq(&rhs, &f64::EPSILON, &f64::EPSILON));
        assert!(relative_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON));
        assert_relative_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON);
    }

    fn check_all_ne_array<const N: usize>(value: f64, min_value: f64) {
        let lhs = array_uniform::<N>(value);
        let rhs = array_range::<N>(min_value);

        assert!(lhs.relative_all_ne(&rhs, &f64::EPSILON, &f64::EPSILON));
        assert!(relative_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON));
        assert_relative_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON);
    }


    #[test]
    fn test_eq_array_empty() {
        check_eq_array::<0>(1_f64);
        check_all_eq_array::<0>(1_f64);
    }

    #[test]
    fn test_eq_array() {
        check_eq_array::<1>(1_f64);
        check_eq_array::<2>(1_f64);
        check_eq_array::<3>(1_f64);
        check_eq_array::<4>(1_f64);
        check_eq_array::<8>(1_f64);
        check_eq_array::<16>(1_f64);
        check_eq_array::<32>(1_f64);
        check_eq_array::<64>(1_f64);
    }

    #[test]
    fn test_ne_array() {
        check_ne_array::<1>(1_f64, 2_f64);
        check_ne_array::<2>(1_f64, 2_f64);
        check_ne_array::<3>(1_f64, 2_f64);
        check_ne_array::<4>(1_f64, 2_f64);
        check_ne_array::<8>(1_f64, 2_f64);
        check_ne_array::<16>(1_f64, 2_f64);
        check_ne_array::<32>(1_f64, 2_f64);
        check_ne_array::<64>(1_f64, 2_f64);
    }

    #[test]
    fn test_all_eq_array() {
        check_all_eq_array::<1>(1_f64);
        check_all_eq_array::<2>(1_f64);
        check_all_eq_array::<3>(1_f64);
        check_all_eq_array::<4>(1_f64);
        check_all_eq_array::<8>(1_f64);
        check_all_eq_array::<16>(1_f64);
        check_all_eq_array::<32>(1_f64);
        check_all_eq_array::<64>(1_f64);
    }

    #[test]
    fn test_all_ne_array() {
        check_all_ne_array::<1>(1_f64, 2_f64);
        check_all_ne_array::<2>(1_f64, 2_f64);
        check_all_ne_array::<3>(1_f64, 2_f64);
        check_all_ne_array::<4>(1_f64, 2_f64);
        check_all_ne_array::<8>(1_f64, 2_f64);
        check_all_ne_array::<16>(1_f64, 2_f64);
        check_all_ne_array::<32>(1_f64, 2_f64);
        check_all_ne_array::<64>(1_f64, 2_f64);
    }
}


#[cfg(test)]
mod relative_eq_array_f32_debug_tests {
    use approx_cmp::{
        AssertRelativeEq,
        AssertRelativeAllEq,
    };

    fn array_uniform<const N: usize>(value: f32) -> [f32; N] {
        [value; N]
    }

    fn array_range<const N: usize>(min_value: f32) -> [f32; N] {
        let mut array = [0_f32; N];
        for i in 0..N {
            array[i] = (min_value + (i as f32)) as f32;
        }

        array
    }


    #[test]
    fn test_debug_abs_diff1() {
        let lhs = array_uniform::<32>(1_f32);
        let rhs = array_uniform::<32>(1_f32);
        let abs_diff = [0_f32; 32];

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff2() {
        let lhs = array_uniform::<32>(1_f32);
        let rhs = array_range::<32>(2_f32);
        let abs_diff = array_range::<32>(1_f32);

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_tolerance1() {
        let lhs = array_uniform::<32>(1_f32);
        let rhs = array_uniform::<32>(1_f32);
        let tolerance = 4.0_f32 * f32::EPSILON;
        let max_abs_diff = array_uniform::<32>(tolerance);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_tolerance2() {
        let lhs = array_uniform::<32>(1_f32);
        let rhs = array_range::<32>(2_f32);
        let tolerance = 4.0_f32 * f32::EPSILON;
        let max_abs_diff = array_uniform::<32>(tolerance);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_tolerance1() {
        let lhs = array_uniform::<32>(1_f32);
        let rhs = array_uniform::<32>(1_f32);
        let tolerance = 4.0_f32 * f32::EPSILON;
        let max_relative = array_uniform::<32>(tolerance);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
    }

    #[test]
    fn test_debug_relative_tolerance2() {
        let lhs = array_uniform::<32>(1_f32);
        let rhs = array_range::<32>(2_f32);
        let tolerance = 4.0_f32 * f32::EPSILON;
        let max_relative = array_uniform::<32>(tolerance);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance1() {
        let lhs = array_uniform::<32>(1_f32);
        let rhs = array_uniform::<32>(1_f32);
        let tolerance = 4.0_f32 * f32::EPSILON;
        let max_abs_diff = array_uniform::<32>(tolerance);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &tolerance), max_abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance2() {
        let lhs = array_uniform::<32>(1_f32);
        let rhs = array_range::<32>(2_f32);
        let tolerance = 4.0_f32 * f32::EPSILON;
        let max_abs_diff = array_uniform::<32>(tolerance);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &tolerance), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_all_tolerance1() {
        let lhs = array_uniform::<32>(1_f32);
        let rhs = array_uniform::<32>(1_f32);
        let tolerance = 4.0_f32 * f32::EPSILON;
        let max_relative = array_uniform::<32>(tolerance);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &tolerance), max_relative);
    }

    #[test]
    fn test_debug_relative_all_tolerance2() {
        let lhs = array_uniform::<32>(1_f32);
        let rhs = array_range::<32>(2_f32);
        let tolerance = 4.0_f32 * f32::EPSILON;
        let max_relative = array_uniform::<32>(tolerance);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &tolerance), max_relative);
    }
}


#[cfg(test)]
mod relative_eq_array_f64_debug_tests {
    use approx_cmp::{
        AssertRelativeEq,
        AssertRelativeAllEq,
    };

    fn array_uniform<const N: usize>(value: f64) -> [f64; N] {
        [value; N]
    }

    fn array_range<const N: usize>(min_value: f64) -> [f64; N] {
        let mut array = [0_f64; N];
        for i in 0..N {
            array[i] = (min_value + (i as f64)) as f64;
        }

        array
    }


    #[test]
    fn test_debug_abs_diff1() {
        let lhs = array_uniform::<32>(1_f64);
        let rhs = array_uniform::<32>(1_f64);
        let abs_diff = [0_f64; 32];

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff2() {
        let lhs = array_uniform::<32>(1_f64);
        let rhs = array_range::<32>(2_f64);
        let abs_diff = array_range::<32>(1_f64);

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_tolerance1() {
        let lhs = array_uniform::<32>(1_f64);
        let rhs = array_uniform::<32>(1_f64);
        let tolerance = 4.0_f64 * f64::EPSILON;
        let max_abs_diff = array_uniform::<32>(tolerance);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_tolerance2() {
        let lhs = array_uniform::<32>(1_f64);
        let rhs = array_range::<32>(2_f64);
        let tolerance = 4.0_f64 * f64::EPSILON;
        let max_abs_diff = array_uniform::<32>(tolerance);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_tolerance1() {
        let lhs = array_uniform::<32>(1_f64);
        let rhs = array_uniform::<32>(1_f64);
        let tolerance = 4.0_f64 * f64::EPSILON;
        let max_relative = array_uniform::<32>(tolerance);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
    }

    #[test]
    fn test_debug_relative_tolerance2() {
        let lhs = array_uniform::<32>(1_f64);
        let rhs = array_range::<32>(2_f64);
        let tolerance = 4.0_f64 * f64::EPSILON;
        let max_relative = array_uniform::<32>(tolerance);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance1() {
        let lhs = array_uniform::<32>(1_f64);
        let rhs = array_uniform::<32>(1_f64);
        let tolerance = 4.0_f64 * f64::EPSILON;
        let max_abs_diff = array_uniform::<32>(tolerance);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &tolerance), max_abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance2() {
        let lhs = array_uniform::<32>(1_f64);
        let rhs = array_range::<32>(2_f64);
        let tolerance = 4.0_f64 * f64::EPSILON;
        let max_abs_diff = array_uniform::<32>(tolerance);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &tolerance), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_all_tolerance1() {
        let lhs = array_uniform::<32>(1_f64);
        let rhs = array_uniform::<32>(1_f64);
        let tolerance = 4.0_f64 * f64::EPSILON;
        let max_relative = array_uniform::<32>(tolerance);

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &tolerance), max_relative);
    }

    #[test]
    fn test_debug_relative_all_tolerance2() {
        let lhs = array_uniform::<32>(1_f64);
        let rhs = array_range::<32>(2_f64);
        let tolerance = 4.0_f64 * f64::EPSILON;
        let max_relative = array_uniform::<32>(tolerance);

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &tolerance), max_relative);
    }
}


#[cfg(test)]
mod relative_eq_ref_tests {
    use approx_cmp::{
        AssertRelativeEq,
        AssertRelativeAllEq,
        assert_relative_eq,
        assert_relative_ne,
    };


    #[rustfmt::skip]
    #[test]
    fn test_eq1() {
        let a = &1.0_f32;
        let b = &1.5_f32;
        let mut a_mut = &mut 1.0_f32;
        let mut b_mut = &mut 1.5_f32;
        let max_abs_diff = 1.0_f32;
        let max_relative = 1.0_f32;

        assert_relative_eq!(&a,         &b,         abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(&a,         &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(&mut a_mut, &b,         abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq2() {
        let a = &1.0_f32;
        let b = &0.9999999_f32;
        let mut a_mut = &mut 1.0_f32;
        let mut b_mut = &0.9999999_f32;
        let max_abs_diff = 1.0 * f32::EPSILON;
        let max_relative = 1.0 * f32::EPSILON;

        assert_relative_eq!(&a,         &b,         abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(&a,         &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(&mut a_mut, &b,         abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq3() {
        let a = &1.0_f32;
        let b = &0.9999999_f32;
        let mut a_mut = &mut 1.0_f32;
        let mut b_mut = &0.9999999_f32;
        let max_abs_diff = 2.0 * f32::EPSILON;
        let max_relative = 2.0 * f32::EPSILON;

        assert_relative_eq!(&a,         &b,         abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(&a,         &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(&mut a_mut, &b,         abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne1() {
        let a = &1.0_f32;
        let b = &1.5_f32;
        let mut a_mut = &mut 1.0_f32;
        let mut b_mut = &mut 1.5_f32;
        let max_abs_diff = 0.499_f32;
        let max_relative = 0.0499_f32;

        assert_relative_ne!(&a,         &b,         abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(&a,         &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(&mut a_mut, &b,         abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne2() {
        let a = &1.0_f32;
        let b = &1.5_f32;
        let mut a_mut = &mut 1.0_f32;
        let mut b_mut = &mut 1.5_f32;
        let max_abs_diff = 0.49999_f32;
        let max_relative = 0.049999_f32;

        assert_relative_ne!(&a,         &b,         abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(&a,         &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(&mut a_mut, &b,         abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne3() {
        let a = &1.0_f32;
        let b = &1.5_f32;
        let mut a_mut = &mut 1.0_f32;
        let mut b_mut = &mut 1.5_f32;
        let max_abs_diff = 0.4999999_f32;
        let max_relative = 0.04999999_f32;

        assert_relative_ne!(&a,         &b,         abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(&a,         &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(&mut a_mut, &b,         abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(&mut a_mut, &mut b_mut, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq1() {
        let a = &1.0_f32;
        let b = &1.5_f32;
        let mut a_mut = &mut 1.0_f32;
        let mut b_mut = &mut 1.5_f32;
        let max_abs_diff = 1.0_f32;
        let max_relative = 1.0_f32;

        assert_relative_eq!(&a,         &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq2() {
        let a = &1.0_f32;
        let b = &0.9999999_f32;
        let mut a_mut = &mut 1.0_f32;
        let mut b_mut = &0.9999999_f32;
        let max_abs_diff = 1.0 * f32::EPSILON;
        let max_relative = 1.0 * f32::EPSILON;

        assert_relative_eq!(&a,         &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq3() {
        let a = &1.0_f32;
        let b = &0.9999999_f32;
        let mut a_mut = &mut 1.0_f32;
        let mut b_mut = &0.9999999_f32;
        let max_abs_diff = 2.0 * f32::EPSILON;
        let max_relative = 2.0 * f32::EPSILON;

        assert_relative_eq!(&a,         &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne1() {
        let a = &1.0_f32;
        let b = &1.5_f32;
        let mut a_mut = &mut 1.0_f32;
        let mut b_mut = &mut 1.5_f32;
        let max_abs_diff = 0.499_f32;
        let max_relative = 0.0499_f32;

        assert_relative_ne!(&a,         &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne2() {
        let a = &1.0_f32;
        let b = &1.5_f32;
        let mut a_mut = &mut 1.0_f32;
        let mut b_mut = &mut 1.5_f32;
        let max_abs_diff = 0.49999_f32;
        let max_relative = 0.049999_f32;

        assert_relative_ne!(&a,         &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne3() {
        let a = &1.0_f32;
        let b = &1.5_f32;
        let mut a_mut = &mut 1.0_f32;
        let mut b_mut = &mut 1.5_f32;
        let max_abs_diff = 0.4999999_f32;
        let max_relative = 0.04999999_f32;

        assert_relative_ne!(&a,         &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(&a,         &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(&mut a_mut, &b,         abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(&mut a_mut, &mut b_mut, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[test]
    fn test_debug_abs_diff() {
        let a = &1.0_f32;
        let b = &1.5_f32;
        let mut a_mut = &mut 1.0_f32;
        let mut b_mut = &mut 1.5_f32;

        assert_eq!(AssertRelativeEq::debug_abs_diff(&a, &b), 0.5_f32);
        assert_eq!(AssertRelativeEq::debug_abs_diff(&a, &mut b_mut), 0.5_f32);
        assert_eq!(AssertRelativeEq::debug_abs_diff(&mut a_mut, &b), 0.5_f32);
        assert_eq!(AssertRelativeEq::debug_abs_diff(&mut a_mut, &mut b_mut), 0.5_f32);
    }

    #[test]
    fn test_debug_abs_diff_tolerance() {
        let a = &1.0_f32;
        let b = &2.0_f32;
        let a_mut = &mut 1.0_f32;
        let b_mut = &mut 2.0_f32;

        assert_eq!(AssertRelativeEq::debug_abs_diff_tolerance(&a, &b, &0.5_f32), 0.5_f32);
        assert_eq!(AssertRelativeEq::debug_abs_diff_tolerance(&a, &b_mut, &0.5_f32), 0.5_f32);
        assert_eq!(AssertRelativeEq::debug_abs_diff_tolerance(&a_mut, &b, &0.5_f32), 0.5_f32);
        assert_eq!(AssertRelativeEq::debug_abs_diff_tolerance(&a_mut, &b_mut, &0.5_f32), 0.5_f32);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let a = &1.0_f32;
        let b = &2.0_f32;
        let a_mut = &mut 1.0_f32;
        let b_mut = &mut 2.0_f32;

        assert_eq!(AssertRelativeAllEq::debug_abs_diff_all_tolerance(&a, &b, &0.5_f32), 0.5_f32);
        assert_eq!(AssertRelativeAllEq::debug_abs_diff_all_tolerance(&a, &b_mut, &0.5_f32), 0.5_f32);
        assert_eq!(AssertRelativeAllEq::debug_abs_diff_all_tolerance(&a_mut, &b, &0.5_f32), 0.5_f32);
        assert_eq!(AssertRelativeAllEq::debug_abs_diff_all_tolerance(&a_mut, &b_mut, &0.5_f32), 0.5_f32);
    }

    #[test]
    fn test_debug_relative_tolerance() {
        let a = &1.0_f32;
        let b = &2.0_f32;
        let a_mut = &mut 1.0_f32;
        let b_mut = &mut 2.0_f32;

        assert_eq!(AssertRelativeEq::debug_relative_tolerance(&a, &b, &0.5_f32), 0.5_f32);
        assert_eq!(AssertRelativeEq::debug_relative_tolerance(&a, &b_mut, &0.5_f32), 0.5_f32);
        assert_eq!(AssertRelativeEq::debug_relative_tolerance(&a_mut, &b, &0.5_f32), 0.5_f32);
        assert_eq!(AssertRelativeEq::debug_relative_tolerance(&a_mut, &b_mut, &0.5_f32), 0.5_f32);
    }

    #[test]
    fn test_debug_relative_all_tolerance() {
        let a = &1.0_f32;
        let b = &2.0_f32;
        let a_mut = &mut 1.0_f32;
        let b_mut = &mut 2.0_f32;

        assert_eq!(AssertRelativeAllEq::debug_relative_all_tolerance(&a, &b, &0.5_f32), 0.5_f32);
        assert_eq!(AssertRelativeAllEq::debug_relative_all_tolerance(&a, &b_mut, &0.5_f32), 0.5_f32);
        assert_eq!(AssertRelativeAllEq::debug_relative_all_tolerance(&a_mut, &b, &0.5_f32), 0.5_f32);
        assert_eq!(AssertRelativeAllEq::debug_relative_all_tolerance(&a_mut, &b_mut, &0.5_f32), 0.5_f32);
    }
}


#[cfg(test)]
mod relative_eq_cell_tests {
    use approx_cmp::{
        AssertRelativeEq,
        AssertRelativeAllEq,
        assert_relative_eq,
        assert_relative_ne,
    };
    use core::cell;


    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = cell::Cell::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = cell::Cell::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            1.0 * eps, 4.0 * eps, 4.0 * eps, 4.0 * eps,
            1.0 * eps, 1.0 * eps, 4.0 * eps, 4.0 * eps,
        ];
        let max_relative = [
            1.0 * eps, 2.0 * eps, 2.0 * eps, 2.0 * eps,
            1.0 * eps, 1.0 * eps, 2.0 * eps, 2.0 * eps,
        ];

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne1() {
        let lhs = cell::Cell::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = cell::Cell::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            0.5 * eps, 4.0 * eps, 4.0 * eps, 4.0 * eps,
            0.5 * eps, 0.5 * eps, 4.0 * eps, 4.0 * eps,
        ];
        let max_relative = [
            0.5 * eps, 4.0 * eps, 4.0 * eps, 4.0 * eps,
            0.5 * eps, 0.5 * eps, 4.0 * eps, 4.0 * eps,
        ];

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne2() {
        let lhs = cell::Cell::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = cell::Cell::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            1.0 * eps, 2.0 * eps, 2.0 * eps, 2.0 * eps,
            1.0 * eps, 1.0 * eps, 2.0 * eps, 2.0 * eps,
        ];
        let max_relative = [eps; 8];

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = cell::Cell::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = cell::Cell::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 4.0 * f32::EPSILON;
        let max_relative = 4.0 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = cell::Cell::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = cell::Cell::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 2.0 * f32::EPSILON;
        let max_relative = 1.0 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff1() {
        let lhs = cell::Cell::new([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let rhs = cell::Cell::new([
            1.10_f32, 1.15_f32, 1.70_f32, 1.80_f32,
            2.80_f32, 2.70_f32, 4.40_f32, 4.60_f32,
        ]);
        let abs_diff_self = [0.0000000_f32; 8];
        let abs_diff = [
            0.100000024_f32, 0.100000024_f32, 0.20000005_f32, 0.20000005_f32,
            0.299999950_f32, 0.299999950_f32, 0.40000010_f32, 0.40000010_f32,
        ];

        assert_eq!(lhs.debug_abs_diff(&lhs), abs_diff_self);
        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff2() {
        let lhs = cell::Cell::new([
            0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
            4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
        ]);
        let rhs = cell::Cell::new([
            1.0000000_f32, 1.9999500_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000000_f32, 7.0000000_f32, 8.0000000_f32,
        ]);
        let abs_diff_self = [0.00000000000000_f32; 8];
        let abs_diff = [
            0.00005000829700_f32, 0.00004994869200_f32, 0.00005006790000_f32, 0.00000047683716_f32,
            0.00005006790000_f32, 0.00000000000000_f32, 0.00005006790000_f32, 0.00000000000000_f32,
        ];

        assert_eq!(lhs.debug_abs_diff(&lhs), abs_diff_self);
        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = cell::Cell::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = cell::Cell::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = cell::Cell::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = cell::Cell::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff_all = 0.20_f32;
        let max_abs_diff = [max_abs_diff_all; 4];

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = cell::Cell::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = cell::Cell::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
    }

    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = cell::Cell::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = cell::Cell::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative_all = 0.20_f32;
        let max_relative = [max_relative_all; 4];

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), max_relative);
    }
}


#[cfg(test)]
mod relative_eq_refcell_tests {
    use approx_cmp::{
        AssertRelativeEq,
        AssertRelativeAllEq,
        assert_relative_eq,
        assert_relative_ne,
    };
    use core::cell;


    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = cell::RefCell::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = cell::RefCell::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            1.0 * eps, 4.0 * eps, 4.0 * eps, 4.0 * eps,
            1.0 * eps, 1.0 * eps, 4.0 * eps, 4.0 * eps,
        ];
        let max_relative = [
            1.0 * eps, 2.0 * eps, 2.0 * eps, 2.0 * eps,
            1.0 * eps, 1.0 * eps, 2.0 * eps, 2.0 * eps,
        ];

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne1() {
        let lhs = cell::RefCell::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = cell::RefCell::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            0.5 * eps, 4.0 * eps, 4.0 * eps, 4.0 * eps,
            0.5 * eps, 0.5 * eps, 4.0 * eps, 4.0 * eps,
        ];
        let max_relative = [
            0.5 * eps, 4.0 * eps, 4.0 * eps, 4.0 * eps,
            0.5 * eps, 0.5 * eps, 4.0 * eps, 4.0 * eps,
        ];

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne2() {
        let lhs = cell::RefCell::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = cell::RefCell::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            1.0 * eps, 2.0 * eps, 2.0 * eps, 2.0 * eps,
            1.0 * eps, 1.0 * eps, 2.0 * eps, 2.0 * eps,
        ];
        let max_relative = [eps; 8];

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = cell::RefCell::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = cell::RefCell::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 4.0 * f32::EPSILON;
        let max_relative = 4.0 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = cell::RefCell::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = cell::RefCell::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 2.0 * f32::EPSILON;
        let max_relative = 1.0 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff1() {
        let lhs = cell::RefCell::new([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let rhs = cell::RefCell::new([
            1.10_f32, 1.15_f32, 1.70_f32, 1.80_f32,
            2.80_f32, 2.70_f32, 4.40_f32, 4.60_f32,
        ]);
        let abs_diff_self = [0.0000000_f32; 8];
        let abs_diff = [
            0.100000024_f32, 0.100000024_f32, 0.20000005_f32, 0.20000005_f32,
            0.299999950_f32, 0.299999950_f32, 0.40000010_f32, 0.40000010_f32,
        ];

        assert_eq!(lhs.debug_abs_diff(&lhs), abs_diff_self);
        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff2() {
        let lhs = cell::RefCell::new([
            0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
            4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
        ]);
        let rhs = cell::RefCell::new([
            1.0000000_f32, 1.9999500_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000000_f32, 7.0000000_f32, 8.0000000_f32,
        ]);
        let abs_diff_self = [0.00000000000000_f32; 8];
        let abs_diff = [
            0.00005000829700_f32, 0.00004994869200_f32, 0.00005006790000_f32, 0.00000047683716_f32,
            0.00005006790000_f32, 0.00000000000000_f32, 0.00005006790000_f32, 0.00000000000000_f32,
        ];

        assert_eq!(lhs.debug_abs_diff(&lhs), abs_diff_self);
        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = cell::RefCell::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = cell::RefCell::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = cell::RefCell::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = cell::RefCell::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff_all = 0.20_f32;
        let max_abs_diff = [max_abs_diff_all; 4];

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = cell::RefCell::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = cell::RefCell::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
    }

    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = cell::RefCell::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = cell::RefCell::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative_all = 0.20_f32;
        let max_relative = [max_relative_all; 4];

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), max_relative);
    }
}


#[cfg(test)]
mod relative_eq_option_tests {
    use approx_cmp::{
        AssertRelativeEq,
        AssertRelativeAllEq,
        assert_relative_eq,
        assert_relative_ne,
    };


    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = Some([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Some([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = Some([
            1.0 * eps, 4.0 * eps, 4.0 * eps, 4.0 * eps,
            1.0 * eps, 1.0 * eps, 4.0 * eps, 4.0 * eps,
        ]);
        let max_relative = Some([
            1.0 * eps, 2.0 * eps, 2.0 * eps, 2.0 * eps,
            1.0 * eps, 1.0 * eps, 2.0 * eps, 2.0 * eps,
        ]);

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne1() {
        let lhs = Some([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Some([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = Some([
            0.5 * eps, 4.0 * eps, 4.0 * eps, 4.0 * eps,
            0.5 * eps, 0.5 * eps, 4.0 * eps, 4.0 * eps,
        ]);
        let max_relative = Some([
            0.5 * eps, 4.0 * eps, 4.0 * eps, 4.0 * eps,
            0.5 * eps, 0.5 * eps, 4.0 * eps, 4.0 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne2() {
        let lhs = Some([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Some([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = Some([
            1.0 * eps, 2.0 * eps, 2.0 * eps, 2.0 * eps,
            1.0 * eps, 1.0 * eps, 2.0 * eps, 2.0 * eps,
        ]);
        let max_relative = Some([eps; 8]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = Some([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Some([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = Some(4.0 * f32::EPSILON);
        let max_relative = Some(4.0 * f32::EPSILON);

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = Some([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Some([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = Some(2.0 * f32::EPSILON);
        let max_relative = Some(1.0 * f32::EPSILON);

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff1() {
        let lhs = Some([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let rhs = Some([
            1.10_f32, 1.15_f32, 1.70_f32, 1.80_f32,
            2.80_f32, 2.70_f32, 4.40_f32, 4.60_f32,
        ]);
        let abs_diff_self = Some([0.0000000_f32; 8]);
        let abs_diff = Some([
            0.100000024_f32, 0.100000024_f32, 0.20000005_f32, 0.20000005_f32,
            0.299999950_f32, 0.299999950_f32, 0.40000010_f32, 0.40000010_f32,
        ]);

        assert_eq!(lhs.debug_abs_diff(&lhs), abs_diff_self);
        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff2() {
        let lhs = Some([
            0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
            4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
        ]);
        let rhs = Some([
            1.0000000_f32, 1.9999500_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000000_f32, 7.0000000_f32, 8.0000000_f32,
        ]);
        let abs_diff_self = Some([0.00000000000000_f32; 8]);
        let abs_diff = Some([
            0.00005000829700_f32, 0.00004994869200_f32, 0.00005006790000_f32, 0.00000047683716_f32,
            0.00005006790000_f32, 0.00000000000000_f32, 0.00005006790000_f32, 0.00000000000000_f32,
        ]);

        assert_eq!(lhs.debug_abs_diff(&lhs), abs_diff_self);
        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = Some([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Some([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff = Some([0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32]);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = Some([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Some([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff_all = 0.20_f32;
        let max_abs_diff = Some([max_abs_diff_all; 4]);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &Some(max_abs_diff_all)), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = Some([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Some([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative = Some([0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32]);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
    }

    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = Some([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Some([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative_all = 0.20_f32;
        let max_relative = Some([max_relative_all; 4]);

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &Some(max_relative_all)), max_relative);
    }

    #[test]
    fn test_eq_none() {
        let lhs: Option<f32> = Some(1.0_f32);
        let rhs: Option<f32> = Some(1.0_f32);
        let max_abs_diff: Option<f32> = Some(f32::EPSILON);
        let max_relative: Option<f32> = Some(f32::EPSILON);

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none1() {
        let lhs: Option<f32> = None;
        let rhs: Option<f32> = None;
        let max_abs_diff: Option<f32> = None;
        let max_relative: Option<f32> = None;

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none2() {
        let lhs: Option<f32> = None;
        let rhs: Option<f32> = None;
        let max_abs_diff: Option<f32> = None;
        let max_relative: Option<f32> = Some(f32::EPSILON);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none3() {
        let lhs: Option<f32> = None;
        let rhs: Option<f32> = None;
        let max_abs_diff: Option<f32> = Some(f32::EPSILON);
        let max_relative: Option<f32> = None;

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none4() {
        let lhs: Option<f32> = None;
        let rhs: Option<f32> = None;
        let max_abs_diff: Option<f32> = Some(f32::EPSILON);
        let max_relative: Option<f32> = Some(f32::EPSILON);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none5() {
        let lhs: Option<f32> = None;
        let rhs: Option<f32> = Some(1.0_f32);
        let max_abs_diff: Option<f32> = None;
        let max_relative: Option<f32> = None;

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none6() {
        let lhs: Option<f32> = None;
        let rhs: Option<f32> = Some(1.0_f32);
        let max_abs_diff: Option<f32> = None;
        let max_relative: Option<f32> = Some(f32::EPSILON);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none7() {
        let lhs: Option<f32> = None;
        let rhs: Option<f32> = Some(1.0_f32);
        let max_abs_diff: Option<f32> = Some(f32::EPSILON);
        let max_relative: Option<f32> = None;

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none8() {
        let lhs: Option<f32> = None;
        let rhs: Option<f32> = Some(1.0_f32);
        let max_abs_diff: Option<f32> = Some(f32::EPSILON);
        let max_relative: Option<f32> = Some(f32::EPSILON);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none9() {
        let lhs: Option<f32> = None;
        let rhs: Option<f32> = None;
        let max_abs_diff: Option<f32> = None;
        let max_relative: Option<f32> = None;

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none10() {
        let lhs: Option<f32> = None;
        let rhs: Option<f32> = None;
        let max_abs_diff: Option<f32> = None;
        let max_relative: Option<f32> = Some(f32::EPSILON);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none11() {
        let lhs: Option<f32> = None;
        let rhs: Option<f32> = None;
        let max_abs_diff: Option<f32> = Some(f32::EPSILON);
        let max_relative: Option<f32> = None;

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none12() {
        let lhs: Option<f32> = None;
        let rhs: Option<f32> = None;
        let max_abs_diff: Option<f32> = Some(f32::EPSILON);
        let max_relative: Option<f32> = Some(f32::EPSILON);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none13() {
        let lhs: Option<f32> = None;
        let rhs: Option<f32> = Some(1.0_f32);
        let max_abs_diff: Option<f32> = None;
        let max_relative: Option<f32> = None;

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none14() {
        let lhs: Option<f32> = None;
        let rhs: Option<f32> = Some(1.0_f32);
        let max_abs_diff: Option<f32> = None;
        let max_relative: Option<f32> = Some(f32::EPSILON);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none15() {
        let lhs: Option<f32> = None;
        let rhs: Option<f32> = Some(1.0_f32);
        let max_abs_diff: Option<f32> = Some(f32::EPSILON);
        let max_relative: Option<f32> = None;

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_debug_abs_diff_tolerance_none() {
        let lhs = Some([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Some([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff = Some([0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32]);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &None), None);
        assert_ne!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), None);
        assert_ne!(lhs.debug_abs_diff_tolerance(&rhs, &None), max_abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance_none() {
        let lhs = Some([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Some([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff_all = 0.20_f32;
        let max_abs_diff = Some([max_abs_diff_all; 4]);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &None), None);
        assert_ne!(lhs.debug_abs_diff_all_tolerance(&rhs, &Some(max_abs_diff_all)), None);
        assert_ne!(lhs.debug_abs_diff_all_tolerance(&rhs, &None), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_tolerance_none() {
        let lhs = Some([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Some([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff = Some([0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32]);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &None), None);
        assert_ne!(lhs.debug_relative_tolerance(&rhs, &max_abs_diff), None);
        assert_ne!(lhs.debug_relative_tolerance(&rhs, &None), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_all_tolerance_none() {
        let lhs = Some([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Some([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff_all = 0.20_f32;
        let max_abs_diff = Some([max_abs_diff_all; 4]);

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &None), None);
        assert_ne!(lhs.debug_relative_all_tolerance(&rhs, &Some(max_abs_diff_all)), None);
        assert_ne!(lhs.debug_relative_all_tolerance(&rhs, &None), max_abs_diff);
    }
}

#[cfg(test)]
mod relative_eq_oncecell_tests {
    use approx_cmp::{
        AssertRelativeEq,
        AssertRelativeAllEq,
        assert_relative_eq,
        assert_relative_ne,
    };
    use core::cell;


    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = cell::OnceCell::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = cell::OnceCell::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            1.0 * eps, 4.0 * eps, 4.0 * eps, 4.0 * eps,
            1.0 * eps, 1.0 * eps, 4.0 * eps, 4.0 * eps,
        ];
        let max_relative = [
            1.0 * eps, 2.0 * eps, 2.0 * eps, 2.0 * eps,
            1.0 * eps, 1.0 * eps, 2.0 * eps, 2.0 * eps,
        ];

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne1() {
        let lhs = cell::OnceCell::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = cell::OnceCell::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            0.5 * eps, 4.0 * eps, 4.0 * eps, 4.0 * eps,
            0.5 * eps, 0.5 * eps, 4.0 * eps, 4.0 * eps,
        ];
        let max_relative = [
            0.5 * eps, 4.0 * eps, 4.0 * eps, 4.0 * eps,
            0.5 * eps, 0.5 * eps, 4.0 * eps, 4.0 * eps,
        ];

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne2() {
        let lhs = cell::OnceCell::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = cell::OnceCell::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            1.0 * eps, 2.0 * eps, 2.0 * eps, 2.0 * eps,
            1.0 * eps, 1.0 * eps, 2.0 * eps, 2.0 * eps,
        ];
        let max_relative = [eps; 8];

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = cell::OnceCell::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = cell::OnceCell::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 4.0 * f32::EPSILON;
        let max_relative = 4.0 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = cell::OnceCell::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = cell::OnceCell::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 2.0 * f32::EPSILON;
        let max_relative = 1.0 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff1() {
        let lhs = cell::OnceCell::from([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let rhs = cell::OnceCell::from([
            1.10_f32, 1.15_f32, 1.70_f32, 1.80_f32,
            2.80_f32, 2.70_f32, 4.40_f32, 4.60_f32,
        ]);
        let abs_diff_self = [0.0000000_f32; 8];
        let abs_diff = [
            0.100000024_f32, 0.100000024_f32, 0.20000005_f32, 0.20000005_f32,
            0.299999950_f32, 0.299999950_f32, 0.40000010_f32, 0.40000010_f32,
        ];

        assert_eq!(lhs.debug_abs_diff(&lhs), Some(abs_diff_self));
        assert_eq!(lhs.debug_abs_diff(&rhs), Some(abs_diff));
        assert_eq!(rhs.debug_abs_diff(&lhs), Some(abs_diff));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff2() {
        let lhs = cell::OnceCell::from([
            0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
            4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
        ]);
        let rhs = cell::OnceCell::from([
            1.0000000_f32, 1.9999500_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000000_f32, 7.0000000_f32, 8.0000000_f32,
        ]);
        let abs_diff_self = [0.00000000000000_f32; 8];
        let abs_diff = [
            0.00005000829700_f32, 0.00004994869200_f32, 0.00005006790000_f32, 0.00000047683716_f32,
            0.00005006790000_f32, 0.00000000000000_f32, 0.00005006790000_f32, 0.00000000000000_f32,
        ];

        assert_eq!(lhs.debug_abs_diff(&lhs), Some(abs_diff_self));
        assert_eq!(lhs.debug_abs_diff(&rhs), Some(abs_diff));
        assert_eq!(rhs.debug_abs_diff(&lhs), Some(abs_diff));
    }

    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = cell::OnceCell::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = cell::OnceCell::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), Some(max_abs_diff));
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = cell::OnceCell::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = cell::OnceCell::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff_all = 0.20_f32;
        let max_abs_diff = [max_abs_diff_all; 4];

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), Some(max_abs_diff));
    }

    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = cell::OnceCell::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = cell::OnceCell::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), Some(max_relative));
    }

    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = cell::OnceCell::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = cell::OnceCell::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative_all = 0.20_f32;
        let max_relative = [max_relative_all; 4];

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), Some(max_relative));
    }

    #[test]
    fn test_eq_none() {
        let lhs: cell::OnceCell<f32> = cell::OnceCell::from(1.0_f32);
        let rhs: cell::OnceCell<f32> = cell::OnceCell::from(1.0_f32);
        let max_abs_diff = f32::EPSILON;
        let max_relative = f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none1() {
        let lhs: cell::OnceCell<f32> = cell::OnceCell::new();
        let rhs: cell::OnceCell<f32> = cell::OnceCell::new();
        let max_abs_diff = f32::EPSILON;
        let max_relative = f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none2() {
        let lhs: cell::OnceCell<f32> = cell::OnceCell::new();
        let rhs: cell::OnceCell<f32> = cell::OnceCell::from(1.0_f32);
        let max_abs_diff = f32::EPSILON;
        let max_relative = f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne_none3() {
        let lhs: cell::OnceCell<f32> = cell::OnceCell::from(1.0_f32);
        let rhs: cell::OnceCell<f32> = cell::OnceCell::new();
        let max_abs_diff = f32::EPSILON;
        let max_relative = f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_debug_abs_diff_tolerance_none() {
        let lhs = cell::OnceCell::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = cell::OnceCell::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let empty_cell = cell::OnceCell::new();
        let max_abs_diff = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), Some(max_abs_diff));
        assert_eq!(lhs.debug_abs_diff_tolerance(&empty_cell, &max_abs_diff), None);
        assert_eq!(empty_cell.debug_abs_diff_tolerance(&rhs, &max_abs_diff), None);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance_none() {
        let lhs = cell::OnceCell::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = cell::OnceCell::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let empty_cell = cell::OnceCell::new();
        let max_abs_diff_all = 0.20_f32;
        let max_abs_diff = [max_abs_diff_all; 4];

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), Some(max_abs_diff));
        assert_eq!(lhs.debug_abs_diff_all_tolerance(&empty_cell, &max_abs_diff_all), None);
        assert_eq!(empty_cell.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), None);
    }

    #[test]
    fn test_debug_relative_tolerance_none() {
        let lhs = cell::OnceCell::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = cell::OnceCell::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let empty_cell = cell::OnceCell::new();
        let max_abs_diff = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_abs_diff), Some(max_abs_diff));
        assert_eq!(lhs.debug_relative_tolerance(&empty_cell, &max_abs_diff), None);
        assert_eq!(empty_cell.debug_relative_tolerance(&rhs, &max_abs_diff), None);
    }

    #[test]
    fn test_debug_relative_all_tolerance_none() {
        let lhs = cell::OnceCell::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = cell::OnceCell::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let empty_cell = cell::OnceCell::new();
        let max_abs_diff_all = 0.20_f32;
        let max_abs_diff = [max_abs_diff_all; 4];

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_abs_diff_all), Some(max_abs_diff));
        assert_eq!(lhs.debug_relative_all_tolerance(&empty_cell, &max_abs_diff_all), None);
        assert_eq!(empty_cell.debug_relative_all_tolerance(&rhs, &max_abs_diff_all), None);
    }
}

