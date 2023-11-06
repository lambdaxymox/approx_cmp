extern crate ulps_cmp;


#[cfg(test)]
mod ulps_eq_f32_tests {
    use ulps_cmp::{
        assert_ulps_eq,
        assert_ulps_ne,
        ulps_eq,
        ulps_ne,
        UlpsAllEq,
        UlpsEq,
    };

    fn check_ulps_eq(a: f32, b: f32, max_abs_diff: f32, max_ulps: u32) {
        assert!(a.ulps_eq(&b, &max_abs_diff, &max_ulps));
        assert!(ulps_eq!(a, b, abs_diff <= max_abs_diff, ulps <= max_ulps));
        assert_ulps_eq!(a, b, abs_diff <= max_abs_diff, ulps <= max_ulps);

        assert!(a.ulps_all_eq(&b, &max_abs_diff, &max_ulps));
        assert!(ulps_eq!(a, b, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps));
        assert_ulps_eq!(a, b, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    }

    fn check_ulps_ne(a: f32, b: f32, max_abs_diff: f32, max_ulps: u32) {
        assert!(a.ulps_ne(&b, &max_abs_diff, &max_ulps));
        assert!(ulps_ne!(a, b, abs_diff <= max_abs_diff, ulps <= max_ulps));
        assert_ulps_ne!(a, b, abs_diff <= max_abs_diff, ulps <= max_ulps);

        assert!(a.ulps_all_ne(&b, &max_abs_diff, &max_ulps));
        assert!(ulps_ne!(a, b, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps));
        assert_ulps_ne!(a, b, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    }

    fn check_eq(a: f32, b: f32, max_abs_diff: f32, max_ulps: u32) {
        check_ulps_eq(a, b, max_abs_diff, max_ulps);
        check_ulps_eq(b, a, max_abs_diff, max_ulps);
        check_ulps_eq(-a, -b, max_abs_diff, max_ulps);
        check_ulps_eq(-b, -a, max_abs_diff, max_ulps);
    }

    fn check_ne(a: f32, b: f32, max_abs_diff: f32, max_ulps: u32) {
        check_ulps_ne(a, b, max_abs_diff, max_ulps);
        check_ulps_ne(b, a, max_abs_diff, max_ulps);
        check_ulps_ne(-a, -b, max_abs_diff, max_ulps);
        check_ulps_ne(-b, -a, max_abs_diff, max_ulps);
    }

    #[rustfmt::skip]
    fn check_eq_self(value: f32) {
        check_eq(value, value, 0.0_f32,           0_u32);
        check_eq(value, value, f32::MIN_POSITIVE, 0_u32);
        check_eq(value, value, f32::MAX,          0_u32);
        check_eq(value, value, f32::INFINITY,     0_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_zero() {
        check_eq_self(0.0_f32);

        check_eq( 0.0_f32,  0.0_f32, f32::EPSILON, 4_u32);
        check_eq(-0.0_f32,  0.0_f32, f32::EPSILON, 4_u32);
        check_eq( 0.0_f32, -0.0_f32, f32::EPSILON, 4_u32);
        check_eq(-0.0_f32, -0.0_f32, f32::EPSILON, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_zero() {
        check_ne( 0.000001_f32, 0.0_f32,      f32::EPSILON, 4_u32);
        check_ne( 0.0_f32,      0.000001_f32, f32::EPSILON, 4_u32);
        check_ne(-0.000001_f32, 0.0_f32,      f32::EPSILON, 4_u32);
        check_ne( 0.0_f32,     -0.000001_f32, f32::EPSILON, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_tolerance() {
        check_eq( 0.0_f32,    1e-40_f32, 1e-40_f32, 4_u32);
        check_eq( 1e-40_f32,  0.0_f32,   1e-40_f32, 4_u32);
        check_eq( 0.0_f32,   -1e-40_f32, 1e-40_f32, 4_u32);
        check_eq(-1e-40_f32,  0.0_f32,   1e-40_f32, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_tolerance() {
        check_ne( 1e-40_f32,  0.0_f32,   1e-41_f32, 4_u32);
        check_ne( 0.0_f32,    1e-40_f32, 1e-41_f32, 4_u32);
        check_ne(-1e-40_f32,  0.0_f32,   1e-41_f32, 4_u32);
        check_ne( 0.0_f32,   -1e-40_f32, 1e-41_f32, 4_u32);
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
        check_ne( 1.0_f32,  2.0_f32, f32::EPSILON,      2_u32);
        check_ne( 1.0_f32,  2.0_f32, f32::MIN_POSITIVE, 4_u32);
        check_ne( 1.0_f32, -2.0_f32, f32::EPSILON,      8_u32);
        check_ne( 1.0_f32, -2.0_f32, f32::MIN_POSITIVE, 16_u32);
        check_ne(-1.0_f32,  2.0_f32, f32::EPSILON,      32_u32);
        check_ne(-1.0_f32,  2.0_f32, f32::MIN_POSITIVE, 64_u32);
        check_ne(-1.0_f32, -2.0_f32, f32::EPSILON,      128_u32);
        check_ne(-1.0_f32, -2.0_f32, f32::MIN_POSITIVE, 256_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding1() {
        check_eq( 10000000.0_f32,  10000001.0_f32, f32::EPSILON, 1_u32);
        check_eq( 10000001.0_f32,  10000000.0_f32, f32::EPSILON, 1_u32);
        check_eq(-10000000.0_f32, -10000001.0_f32, f32::EPSILON, 1_u32);
        check_eq(-10000001.0_f32, -10000000.0_f32, f32::EPSILON, 1_u32);

        check_eq( 100000000.0_f32,  100000001.0_f32, f32::EPSILON, 1_u32);
        check_eq( 100000001.0_f32,  100000000.0_f32, f32::EPSILON, 1_u32);
        check_eq(-100000000.0_f32, -100000001.0_f32, f32::EPSILON, 1_u32);
        check_eq(-100000001.0_f32, -100000000.0_f32, f32::EPSILON, 1_u32);

        check_eq( 1000000000.0_f32,  1000000001.0_f32, f32::EPSILON, 1_u32);
        check_eq( 1000000001.0_f32,  1000000000.0_f32, f32::EPSILON, 1_u32);
        check_eq(-1000000000.0_f32, -1000000001.0_f32, f32::EPSILON, 1_u32);
        check_eq(-1000000001.0_f32, -1000000000.0_f32, f32::EPSILON, 1_u32);

        check_eq( 10000000000.0_f32,  10000000001.0_f32, f32::EPSILON, 1_u32);
        check_eq( 10000000001.0_f32,  10000000000.0_f32, f32::EPSILON, 1_u32);
        check_eq(-10000000000.0_f32, -10000000001.0_f32, f32::EPSILON, 1_u32);
        check_eq(-10000000001.0_f32, -10000000000.0_f32, f32::EPSILON, 1_u32);

        check_eq( 100000000000.0_f32,  100000000001.0_f32, f32::EPSILON, 1_u32);
        check_eq( 100000000001.0_f32,  100000000000.0_f32, f32::EPSILON, 1_u32);
        check_eq(-100000000000.0_f32, -100000000001.0_f32, f32::EPSILON, 1_u32);
        check_eq(-100000000001.0_f32, -100000000000.0_f32, f32::EPSILON, 1_u32);

        check_eq( 1000000000000.0_f32,  1000000000001.0_f32, f32::EPSILON, 1_u32);
        check_eq( 1000000000001.0_f32,  1000000000000.0_f32, f32::EPSILON, 1_u32);
        check_eq(-1000000000000.0_f32, -1000000000001.0_f32, f32::EPSILON, 1_u32);
        check_eq(-1000000000001.0_f32, -1000000000000.0_f32, f32::EPSILON, 1_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding1() {
        check_ne( 1000.0_f32,  1001.0_f32, f32::EPSILON, 4_u32);
        check_ne( 1001.0_f32,  1000.0_f32, f32::EPSILON, 4_u32);
        check_ne(-1000.0_f32, -1001.0_f32, f32::EPSILON, 4_u32);
        check_ne(-1001.0_f32, -1000.0_f32, f32::EPSILON, 4_u32);

        check_ne( 10000.0_f32,  10001.0_f32, f32::EPSILON, 4_u32);
        check_ne( 10001.0_f32,  10000.0_f32, f32::EPSILON, 4_u32);
        check_ne(-10000.0_f32, -10001.0_f32, f32::EPSILON, 4_u32);
        check_ne(-10001.0_f32, -10000.0_f32, f32::EPSILON, 4_u32);

        check_ne( 100000.0_f32,  100001.0_f32, f32::EPSILON, 4_u32);
        check_ne( 100001.0_f32,  100000.0_f32, f32::EPSILON, 4_u32);
        check_ne(-100000.0_f32, -100001.0_f32, f32::EPSILON, 4_u32);
        check_ne(-100001.0_f32, -100000.0_f32, f32::EPSILON, 4_u32);

        check_ne( 1000000.0_f32,  1000001.0_f32, f32::EPSILON, 4_u32);
        check_ne( 1000001.0_f32,  1000000.0_f32, f32::EPSILON, 4_u32);
        check_ne(-1000000.0_f32, -1000001.0_f32, f32::EPSILON, 4_u32);
        check_ne(-1000001.0_f32, -1000000.0_f32, f32::EPSILON, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding2() {
        check_eq( 1.0000001_f32,  1.0000002_f32, f32::EPSILON, 4_u32);
        check_eq( 1.0000002_f32,  1.0000001_f32, f32::EPSILON, 4_u32);
        check_eq(-1.0000001_f32, -1.0000002_f32, f32::EPSILON, 4_u32);
        check_eq(-1.0000002_f32, -1.0000001_f32, f32::EPSILON, 4_u32);

        check_eq( 1.00000001_f32,  1.00000002_f32, f32::EPSILON, 4_u32);
        check_eq( 1.00000002_f32,  1.00000001_f32, f32::EPSILON, 4_u32);
        check_eq(-1.00000001_f32, -1.00000002_f32, f32::EPSILON, 4_u32);
        check_eq(-1.00000002_f32, -1.00000001_f32, f32::EPSILON, 4_u32);

        check_eq( 1.000000001_f32,  1.000000002_f32, f32::EPSILON, 4_u32);
        check_eq( 1.000000002_f32,  1.000000001_f32, f32::EPSILON, 4_u32);
        check_eq(-1.000000001_f32, -1.000000002_f32, f32::EPSILON, 4_u32);
        check_eq(-1.000000002_f32, -1.000000001_f32, f32::EPSILON, 4_u32);

        check_eq( 1.0000000001_f32,  1.0000000002_f32, f32::EPSILON, 4_u32);
        check_eq( 1.0000000002_f32,  1.0000000001_f32, f32::EPSILON, 4_u32);
        check_eq(-1.0000000001_f32, -1.0000000002_f32, f32::EPSILON, 4_u32);
        check_eq(-1.0000000002_f32, -1.0000000001_f32, f32::EPSILON, 4_u32);

        check_eq( 1.00000000001_f32,  1.00000000002_f32, f32::EPSILON, 4_u32);
        check_eq( 1.00000000002_f32,  1.00000000001_f32, f32::EPSILON, 4_u32);
        check_eq(-1.00000000001_f32, -1.00000000002_f32, f32::EPSILON, 4_u32);
        check_eq(-1.00000000002_f32, -1.00000000001_f32, f32::EPSILON, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding2() {
        check_ne( 1.01_f32,  1.02_f32, f32::EPSILON, 4_u32);
        check_ne( 1.02_f32,  1.01_f32, f32::EPSILON, 4_u32);
        check_ne(-1.01_f32, -1.02_f32, f32::EPSILON, 4_u32);
        check_ne(-1.02_f32, -1.01_f32, f32::EPSILON, 4_u32);

        check_ne( 1.001_f32,  1.002_f32, f32::EPSILON, 4_u32);
        check_ne( 1.002_f32,  1.001_f32, f32::EPSILON, 4_u32);
        check_ne(-1.001_f32, -1.002_f32, f32::EPSILON, 4_u32);
        check_ne(-1.002_f32, -1.001_f32, f32::EPSILON, 4_u32);

        check_ne( 1.0001_f32,  1.0002_f32, f32::EPSILON, 4_u32);
        check_ne( 1.0002_f32,  1.0001_f32, f32::EPSILON, 4_u32);
        check_ne(-1.0001_f32, -1.0002_f32, f32::EPSILON, 4_u32);
        check_ne(-1.0002_f32, -1.0001_f32, f32::EPSILON, 4_u32);

        check_ne( 1.00001_f32,  1.00002_f32, f32::EPSILON, 4_u32);
        check_ne( 1.00002_f32,  1.00001_f32, f32::EPSILON, 4_u32);
        check_ne(-1.00001_f32, -1.00002_f32, f32::EPSILON, 4_u32);
        check_ne(-1.00002_f32, -1.00001_f32, f32::EPSILON, 4_u32);

        check_ne( 1.000001_f32,  1.000002_f32, f32::EPSILON, 4_u32);
        check_ne( 1.000002_f32,  1.000001_f32, f32::EPSILON, 4_u32);
        check_ne(-1.000001_f32, -1.000002_f32, f32::EPSILON, 4_u32);
        check_ne(-1.000002_f32, -1.000001_f32, f32::EPSILON, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding3() {
        check_eq( 0.99999999_f32,  1.00000001_f32, f32::EPSILON, 2_u32);
        check_eq( 1.00000001_f32,  0.99999999_f32, f32::EPSILON, 2_u32);
        check_eq(-0.99999999_f32, -1.00000001_f32, f32::EPSILON, 2_u32);
        check_eq(-1.00000001_f32, -0.99999999_f32, f32::EPSILON, 2_u32);

        check_eq( 0.999999999_f32,  1.000000001_f32, f32::EPSILON, 2_u32);
        check_eq( 1.000000001_f32,  0.999999999_f32, f32::EPSILON, 2_u32);
        check_eq(-0.999999999_f32, -1.000000001_f32, f32::EPSILON, 2_u32);
        check_eq(-1.000000001_f32, -0.999999999_f32, f32::EPSILON, 2_u32);

        check_eq( 0.9999999999_f32,  1.0000000001_f32, f32::EPSILON, 2_u32);
        check_eq( 1.0000000001_f32,  0.9999999999_f32, f32::EPSILON, 2_u32);
        check_eq(-0.9999999999_f32, -1.0000000001_f32, f32::EPSILON, 2_u32);
        check_eq(-1.0000000001_f32, -0.9999999999_f32, f32::EPSILON, 2_u32);

        check_eq( 0.99999999999_f32,  1.00000000001_f32, f32::EPSILON, 2_u32);
        check_eq( 1.00000000001_f32,  0.99999999999_f32, f32::EPSILON, 2_u32);
        check_eq(-0.99999999999_f32, -1.00000000001_f32, f32::EPSILON, 2_u32);
        check_eq(-1.00000000001_f32, -0.99999999999_f32, f32::EPSILON, 2_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding3() {
        check_ne( 0.99_f32,  1.01_f32, f32::EPSILON, 2_u32);
        check_ne( 1.01_f32,  0.99_f32, f32::EPSILON, 2_u32);
        check_ne(-0.99_f32, -1.01_f32, f32::EPSILON, 2_u32);
        check_ne(-1.01_f32, -0.99_f32, f32::EPSILON, 2_u32);

        check_ne( 0.999_f32,  1.001_f32, f32::EPSILON, 2_u32);
        check_ne( 1.001_f32,  0.999_f32, f32::EPSILON, 2_u32);
        check_ne(-0.999_f32, -1.001_f32, f32::EPSILON, 2_u32);
        check_ne(-1.001_f32, -0.999_f32, f32::EPSILON, 2_u32);

        check_ne( 0.9999_f32,  1.0001_f32, f32::EPSILON, 2_u32);
        check_ne( 1.0001_f32,  0.9999_f32, f32::EPSILON, 2_u32);
        check_ne(-0.9999_f32, -1.0001_f32, f32::EPSILON, 2_u32);
        check_ne(-1.0001_f32, -0.9999_f32, f32::EPSILON, 2_u32);

        check_ne( 0.99999_f32,  1.00001_f32, f32::EPSILON, 2_u32);
        check_ne( 1.00001_f32,  0.99999_f32, f32::EPSILON, 2_u32);
        check_ne(-0.99999_f32, -1.00001_f32, f32::EPSILON, 2_u32);
        check_ne(-1.00001_f32, -0.99999_f32, f32::EPSILON, 2_u32);

        check_ne( 0.999999_f32,  1.000001_f32, f32::EPSILON, 2_u32);
        check_ne( 1.000001_f32,  0.999999_f32, f32::EPSILON, 2_u32);
        check_ne(-0.999999_f32, -1.000001_f32, f32::EPSILON, 2_u32);
        check_ne(-1.000001_f32, -0.999999_f32, f32::EPSILON, 2_u32);

        check_ne( 0.9999999_f32,  1.0000001_f32, f32::EPSILON, 2_u32);
        check_ne( 1.0000001_f32,  0.9999999_f32, f32::EPSILON, 2_u32);
        check_ne(-0.9999999_f32, -1.0000001_f32, f32::EPSILON, 2_u32);
        check_ne(-1.0000001_f32, -0.9999999_f32, f32::EPSILON, 2_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding4() {
        check_eq( 1.9999999_f32,  2.0000001_f32, f32::EPSILON, 4_u32);
        check_eq( 2.0000001_f32,  1.9999999_f32, f32::EPSILON, 4_u32);
        check_eq(-1.9999999_f32, -2.0000001_f32, f32::EPSILON, 4_u32);
        check_eq(-2.0000001_f32, -1.9999999_f32, f32::EPSILON, 4_u32);

        check_eq( 1.99999999_f32,  2.00000001_f32, f32::EPSILON, 4_u32);
        check_eq( 2.00000001_f32,  1.99999999_f32, f32::EPSILON, 4_u32);
        check_eq(-1.99999999_f32, -2.00000001_f32, f32::EPSILON, 4_u32);
        check_eq(-2.00000001_f32, -1.99999999_f32, f32::EPSILON, 4_u32);

        check_eq( 1.999999999_f32,  2.000000001_f32, f32::EPSILON, 4_u32);
        check_eq( 2.000000001_f32,  1.999999999_f32, f32::EPSILON, 4_u32);
        check_eq(-1.999999999_f32, -2.000000001_f32, f32::EPSILON, 4_u32);
        check_eq(-2.000000001_f32, -1.999999999_f32, f32::EPSILON, 4_u32);

        check_eq( 1.9999999999_f32,  2.0000000001_f32, f32::EPSILON, 4_u32);
        check_eq( 2.0000000001_f32,  1.9999999999_f32, f32::EPSILON, 4_u32);
        check_eq(-1.9999999999_f32, -2.0000000001_f32, f32::EPSILON, 4_u32);
        check_eq(-2.0000000001_f32, -1.9999999999_f32, f32::EPSILON, 4_u32);

        check_eq( 1.99999999999_f32,  2.00000000001_f32, f32::EPSILON, 4_u32);
        check_eq( 2.00000000001_f32,  1.99999999999_f32, f32::EPSILON, 4_u32);
        check_eq(-1.99999999999_f32, -2.00000000001_f32, f32::EPSILON, 4_u32);
        check_eq(-2.00000000001_f32, -1.99999999999_f32, f32::EPSILON, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding4() {
        check_ne( 1.99_f32,  2.01_f32, f32::EPSILON, 4_u32);
        check_ne( 2.01_f32,  1.99_f32, f32::EPSILON, 4_u32);
        check_ne(-1.99_f32, -2.01_f32, f32::EPSILON, 4_u32);
        check_ne(-2.01_f32, -1.99_f32, f32::EPSILON, 4_u32);

        check_ne( 1.999_f32,  2.001_f32, f32::EPSILON, 4_u32);
        check_ne( 2.001_f32,  1.999_f32, f32::EPSILON, 4_u32);
        check_ne(-1.999_f32, -2.001_f32, f32::EPSILON, 4_u32);
        check_ne(-2.001_f32, -1.999_f32, f32::EPSILON, 4_u32);

        check_ne( 1.9999_f32,  2.0001_f32, f32::EPSILON, 4_u32);
        check_ne( 2.0001_f32,  1.9999_f32, f32::EPSILON, 4_u32);
        check_ne(-1.9999_f32, -2.0001_f32, f32::EPSILON, 4_u32);
        check_ne(-2.0001_f32, -1.9999_f32, f32::EPSILON, 4_u32);

        check_ne( 1.99999_f32,  2.00001_f32, f32::EPSILON, 4_u32);
        check_ne( 2.00001_f32,  1.99999_f32, f32::EPSILON, 4_u32);
        check_ne(-1.99999_f32, -2.00001_f32, f32::EPSILON, 4_u32);
        check_ne(-2.00001_f32, -1.99999_f32, f32::EPSILON, 4_u32);

        check_ne( 1.999999_f32,  2.000001_f32, f32::EPSILON, 4_u32);
        check_ne( 2.000001_f32,  1.999999_f32, f32::EPSILON, 4_u32);
        check_ne(-1.999999_f32, -2.000001_f32, f32::EPSILON, 4_u32);
        check_ne(-2.000001_f32, -1.999999_f32, f32::EPSILON, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_max() {
        check_eq( f32::MAX,  f32::MAX, f32::EPSILON, 4_u32);
        check_eq(-f32::MAX, -f32::MAX, f32::EPSILON, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_max() {
        check_ne( f32::MAX, -f32::MAX,           f32::EPSILON, 4_u32);
        check_ne(-f32::MAX,  f32::MAX,           f32::EPSILON, 4_u32);
        check_ne( f32::MAX,  f32::MAX / 2.0_f32, f32::EPSILON, 4_u32);
        check_ne( f32::MAX, -f32::MAX / 2.0_f32, f32::EPSILON, 4_u32);
        check_ne(-f32::MAX,  f32::MAX / 2.0_f32, f32::EPSILON, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_nan1() {
        check_ne( f32::NAN,  f32::NAN, 0.0_f32, 128_u32);
        check_ne( f32::NAN, -f32::NAN, 0.0_f32, 128_u32);
        check_ne(-f32::NAN,  f32::NAN, 0.0_f32, 128_u32);
        check_ne(-f32::NAN, -f32::NAN, 0.0_f32, 128_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_nan2() {
        for i in 0..=i16::MAX {
            check_ne( f32::NAN , f32::NAN, 1.0_f32 / (i as f32), i as u32);
            check_ne( f32::NAN, -f32::NAN, 1.0_f32 / (i as f32), i as u32);
            check_ne(-f32::NAN,  f32::NAN, 1.0_f32 / (i as f32), i as u32);
            check_ne(-f32::NAN, -f32::NAN, 1.0_f32 / (i as f32), i as u32);
        }
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_nan3() {
        for i in 0..=i16::MAX {
            check_ne( f32::NAN , f32::NAN, i as f32, i as u32);
            check_ne( f32::NAN, -f32::NAN, i as f32, i as u32);
            check_ne(-f32::NAN,  f32::NAN, i as f32, i as u32);
            check_ne(-f32::NAN, -f32::NAN, i as f32, i as u32);
        }
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_nan4() {
        check_ne(f32::NAN, f32::NAN, f32::EPSILON, 4_u32);
        
        check_ne( f32::NAN,  0.0_f32,  f32::EPSILON, 4_u32);
        check_ne(-0.0_f32,   f32::NAN, f32::EPSILON, 4_u32);
        check_ne( f32::NAN, -0.0_f32,  f32::EPSILON, 4_u32);
        check_ne( 0.0_f32,   f32::NAN, f32::EPSILON, 4_u32);

        check_ne( f32::NAN,       f32::INFINITY, f32::EPSILON, 4_u32);
        check_ne( f32::INFINITY,  f32::NAN,      f32::EPSILON, 4_u32);
        check_ne( f32::NAN,      -f32::INFINITY, f32::EPSILON, 4_u32);
        check_ne(-f32::INFINITY,  f32::NAN,      f32::EPSILON, 4_u32);

        check_ne( f32::NAN,  f32::MAX, f32::EPSILON, 4_u32);
        check_ne( f32::MAX,  f32::NAN, f32::EPSILON, 4_u32);
        check_ne( f32::NAN, -f32::MAX, f32::EPSILON, 4_u32);
        check_ne(-f32::MAX,  f32::NAN, f32::EPSILON, 4_u32);

        check_ne( f32::NAN,           f32::MIN_POSITIVE, f32::EPSILON, 4_u32);
        check_ne( f32::MIN_POSITIVE,  f32::NAN,          f32::EPSILON, 4_u32);
        check_ne( f32::NAN,          -f32::MIN_POSITIVE, f32::EPSILON, 4_u32);
        check_ne(-f32::MIN_POSITIVE,  f32::NAN,          f32::EPSILON, 4_u32);

        check_ne( f32::NAN,  1.0_f32,  f32::EPSILON, 4_u32);
        check_ne( f32::NAN, -1.0_f32,  f32::EPSILON, 4_u32);
        check_ne( 1.0_f32,   f32::NAN, f32::EPSILON, 4_u32);
        check_ne(-1.0_f32,   f32::NAN, f32::EPSILON, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_nan5() {
        check_ne(f32::NAN, f32::NAN, 1.0_f32, 32_u32);
        
        check_ne( f32::NAN,  0.0_f32,  1.0_f32, 32_u32);
        check_ne(-0.0_f32,   f32::NAN, 1.0_f32, 32_u32);
        check_ne( f32::NAN, -0.0_f32,  1.0_f32, 32_u32);
        check_ne( 0.0_f32,   f32::NAN, 1.0_f32, 32_u32);

        check_ne( f32::NAN,       f32::INFINITY, 1.0_f32, 32_u32);
        check_ne( f32::INFINITY,  f32::NAN,      1.0_f32, 32_u32);
        check_ne( f32::NAN,      -f32::INFINITY, 1.0_f32, 32_u32);
        check_ne(-f32::INFINITY,  f32::NAN,      1.0_f32, 32_u32);

        check_ne( f32::NAN,  f32::MAX, 1.0_f32, 32_u32);
        check_ne( f32::MAX,  f32::NAN, 1.0_f32, 32_u32);
        check_ne( f32::NAN, -f32::MAX, 1.0_f32, 32_u32);
        check_ne(-f32::MAX,  f32::NAN, 1.0_f32, 32_u32);

        check_ne( f32::NAN,           f32::MIN_POSITIVE, 1.0_f32, 32_u32);
        check_ne( f32::MIN_POSITIVE,  f32::NAN,          1.0_f32, 32_u32);
        check_ne( f32::NAN,          -f32::MIN_POSITIVE, 1.0_f32, 32_u32);
        check_ne(-f32::MIN_POSITIVE,  f32::NAN,          1.0_f32, 32_u32);

        check_ne( f32::NAN,  1.0_f32,  1.0_f32, 32_u32);
        check_ne( f32::NAN, -1.0_f32,  1.0_f32, 32_u32);
        check_ne( 1.0_f32,   f32::NAN, 1.0_f32, 32_u32);
        check_ne(-1.0_f32,   f32::NAN, 1.0_f32, 32_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_infinity1() {
        check_eq(f32::INFINITY,     f32::INFINITY,     f32::MAX,      u32::MAX);
        check_eq(f32::INFINITY,     f32::INFINITY,     f32::INFINITY, u32::MAX);
        check_eq(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::INFINITY, u32::MAX);
        check_eq(f32::INFINITY,     f32::MAX,          f32::MAX,      u32::MAX);
        check_eq(f32::INFINITY,     f32::MAX,          f32::INFINITY, u32::MAX);
        check_eq(f32::INFINITY,     f32::NEG_INFINITY, f32::INFINITY, u32::MAX);
        check_eq(f32::NEG_INFINITY, f32::MAX,          f32::INFINITY, u32::MAX);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_infinity2() {
        check_eq(f32::INFINITY,      f32::INFINITY,     0.0_f32, 1_u32);
        check_eq(f32::NEG_INFINITY,  f32::NEG_INFINITY, 0.0_f32, 1_u32);
        check_eq(f32::INFINITY,      f32::MAX,          0.0_f32, 1_u32);
        check_eq(f32::NEG_INFINITY, -f32::MAX,          0.0_f32, 1_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_infinity1() {
        check_ne( f32::INFINITY,      f32::NEG_INFINITY, f32::MAX, u32::MAX);
        check_ne( f32::NEG_INFINITY,  f32::MAX,          f32::MAX, u32::MAX);
        check_ne(-f32::MAX,           f32::INFINITY,     f32::MAX, u32::MAX);
        check_ne( f32::INFINITY,     -f32::MAX,          f32::MAX, u32::MAX);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_infinity2() {
        check_ne( f32::INFINITY,      f32::NEG_INFINITY, f32::MAX, 4_u32);
        check_ne( f32::NEG_INFINITY,  f32::MAX,          f32::MAX, 4_u32);
        check_ne(-f32::MAX,           f32::INFINITY,     f32::MAX, 4_u32);
        check_ne( f32::INFINITY,     -f32::MAX,          f32::MAX, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero1() {
        check_eq( f32::MIN_POSITIVE,  f32::MIN_POSITIVE, f32::EPSILON, 4_u32);
        check_eq( f32::MIN_POSITIVE, -f32::MIN_POSITIVE, f32::EPSILON, 4_u32);
        check_eq(-f32::MIN_POSITIVE,  f32::MIN_POSITIVE, f32::EPSILON, 4_u32);

        check_eq( f32::MIN_POSITIVE,  0.0_f32,           f32::EPSILON, 4_u32);
        check_eq( 0.0_f32,            f32::MIN_POSITIVE, f32::EPSILON, 4_u32);
        check_eq(-f32::MIN_POSITIVE,  0.0_f32,           f32::EPSILON, 4_u32);
        check_eq( 0.0_f32,           -f32::MIN_POSITIVE, f32::EPSILON, 4_u32);

        check_eq( 0.0000001_f32,      -f32::MIN_POSITIVE, f32::EPSILON, 4_u32);
        check_eq( 0.0000001_f32,       f32::MIN_POSITIVE, f32::EPSILON, 4_u32);
        check_eq( f32::MIN_POSITIVE,  0.0000001_f32,      f32::EPSILON, 4_u32);
        check_eq(-f32::MIN_POSITIVE,  0.0000001_f32,      f32::EPSILON, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero2() {
        check_eq( 0.000010001_f32,  0.000010002_f32, f32::EPSILON, 4_u32);
        check_eq( 0.000010002_f32,  0.000010001_f32, f32::EPSILON, 4_u32);
        check_eq(-0.000010001_f32, -0.000010002_f32, f32::EPSILON, 4_u32);
        check_eq(-0.000010002_f32, -0.000010001_f32, f32::EPSILON, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero3() {
        check_eq( 0.000011001_f32,  0.000011002_f32, f32::EPSILON, 4_u32);
        check_eq( 0.000011002_f32,  0.000011001_f32, f32::EPSILON, 4_u32);
        check_eq(-0.000011001_f32, -0.000011002_f32, f32::EPSILON, 4_u32);
        check_eq(-0.000011002_f32, -0.000011001_f32, f32::EPSILON, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero4() {
        check_eq(1e-8_f32,  -1e-8_f32,  f32::EPSILON, 4_u32);
        check_eq(1e-9_f32,  -1e-9_f32,  f32::EPSILON, 4_u32);
        check_eq(1e-10_f32, -1e-10_f32, f32::EPSILON, 4_u32);
        check_eq(1e-11_f32, -1e-11_f32, f32::EPSILON, 4_u32);
        check_eq(1e-12_f32, -1e-12_f32, f32::EPSILON, 4_u32);
        check_eq(1e-13_f32, -1e-13_f32, f32::EPSILON, 4_u32);
        check_eq(1e-14_f32, -1e-14_f32, f32::EPSILON, 4_u32);
        check_eq(1e-15_f32, -1e-15_f32, f32::EPSILON, 4_u32);
        check_eq(1e-16_f32, -1e-16_f32, f32::EPSILON, 4_u32);
        check_eq(1e-17_f32, -1e-17_f32, f32::EPSILON, 4_u32);
        check_eq(1e-18_f32, -1e-18_f32, f32::EPSILON, 4_u32);
        check_eq(1e-19_f32, -1e-19_f32, f32::EPSILON, 4_u32);
        check_eq(1e-20_f32, -1e-20_f32, f32::EPSILON, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero1() {
        check_ne( 0.000001_f32,      -f32::MIN_POSITIVE, f32::EPSILON, 4_u32);
        check_ne( 0.000001_f32,       f32::MIN_POSITIVE, f32::EPSILON, 4_u32);
        check_ne( f32::MIN_POSITIVE,  0.000001_f32,      f32::EPSILON, 4_u32);
        check_ne(-f32::MIN_POSITIVE,  0.000001_f32,      f32::EPSILON, 4_u32);

        check_ne(-0.000001_f32,      -f32::MIN_POSITIVE, f32::EPSILON, 4_u32);
        check_ne(-0.000001_f32,       f32::MIN_POSITIVE, f32::EPSILON, 4_u32);
        check_ne( f32::MIN_POSITIVE, -0.000001_f32,      f32::EPSILON, 4_u32);
        check_ne(-f32::MIN_POSITIVE, -0.000001_f32,      f32::EPSILON, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero2() {
        check_ne( 0.000001002_f32,  0.0000001001_f32, f32::EPSILON, 4_u32);
        check_ne( 0.000001001_f32,  0.0000001002_f32, f32::EPSILON, 4_u32);
        check_ne(-0.000001002_f32, -0.0000001001_f32, f32::EPSILON, 4_u32);
        check_ne(-0.000001001_f32, -0.0000001002_f32, f32::EPSILON, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero3() {
        check_ne( 0.000001102_f32,  0.0000001101_f32, f32::EPSILON, 4_u32);
        check_ne( 0.000001101_f32,  0.0000001102_f32, f32::EPSILON, 4_u32);
        check_ne(-0.000001102_f32, -0.0000001101_f32, f32::EPSILON, 4_u32);
        check_ne(-0.000001101_f32, -0.0000001102_f32, f32::EPSILON, 4_u32);
    }
}


#[cfg(test)]
mod ulps_eq_f64_tests {
    use ulps_cmp::{
        assert_ulps_eq,
        assert_ulps_ne,
        ulps_eq,
        ulps_ne,
        UlpsAllEq,
        UlpsEq,
    };

    fn check_ulps_eq(a: f64, b: f64, max_abs_diff: f64, max_ulps: u64) {
        assert!(a.ulps_eq(&b, &max_abs_diff, &max_ulps));
        assert!(ulps_eq!(a, b, abs_diff <= max_abs_diff, ulps <= max_ulps));
        assert_ulps_eq!(a, b, abs_diff <= max_abs_diff, ulps <= max_ulps);

        assert!(a.ulps_all_eq(&b, &max_abs_diff, &max_ulps));
        assert!(ulps_eq!(a, b, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps));
        assert_ulps_eq!(a, b, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    }

    fn check_ulps_ne(a: f64, b: f64, max_abs_diff: f64, max_ulps: u64) {
        assert!(a.ulps_ne(&b, &max_abs_diff, &max_ulps));
        assert!(ulps_ne!(a, b, abs_diff <= max_abs_diff, ulps <= max_ulps));
        assert_ulps_ne!(a, b, abs_diff <= max_abs_diff, ulps <= max_ulps);

        assert!(a.ulps_all_ne(&b, &max_abs_diff, &max_ulps));
        assert!(ulps_ne!(a, b, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps));
        assert_ulps_ne!(a, b, abs_diff_all <= max_abs_diff, ulps_all <= max_ulps);
    }

    fn check_eq(a: f64, b: f64, max_abs_diff: f64, max_ulps: u64) {
        check_ulps_eq(a, b, max_abs_diff, max_ulps);
        check_ulps_eq(b, a, max_abs_diff, max_ulps);
        check_ulps_eq(-a, -b, max_abs_diff, max_ulps);
        check_ulps_eq(-b, -a, max_abs_diff, max_ulps);
    }

    fn check_ne(a: f64, b: f64, max_abs_diff: f64, max_ulps: u64) {
        check_ulps_ne(a, b, max_abs_diff, max_ulps);
        check_ulps_ne(b, a, max_abs_diff, max_ulps);
        check_ulps_ne(-a, -b, max_abs_diff, max_ulps);
        check_ulps_ne(-b, -a, max_abs_diff, max_ulps);
    }

    #[rustfmt::skip]
    fn check_eq_self(value: f64) {
        check_eq(value, value, 0.0_f64,           0_u64);
        check_eq(value, value, f64::MIN_POSITIVE, 0_u64);
        check_eq(value, value, f64::MAX,          0_u64);
        check_eq(value, value, f64::INFINITY,     0_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_zero() {
        check_eq_self(0.0_f64);

        check_eq( 0.0_f64,  0.0_f64, f64::EPSILON, 4_u64);
        check_eq(-0.0_f64,  0.0_f64, f64::EPSILON, 4_u64);
        check_eq( 0.0_f64, -0.0_f64, f64::EPSILON, 4_u64);
        check_eq(-0.0_f64, -0.0_f64, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_zero() {
        check_ne( 0.000001_f64, 0.0_f64,      f64::EPSILON, 4_u64);
        check_ne( 0.0_f64,      0.000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.000001_f64, 0.0_f64,      f64::EPSILON, 4_u64);
        check_ne( 0.0_f64,     -0.000001_f64, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_tolerance() {
        check_eq( 0.0_f64,    1e-40_f64, 1e-40_f64, 4_u64);
        check_eq( 1e-40_f64,  0.0_f64,   1e-40_f64, 4_u64);
        check_eq( 0.0_f64,   -1e-40_f64, 1e-40_f64, 4_u64);
        check_eq(-1e-40_f64,  0.0_f64,   1e-40_f64, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_tolerance() {
        check_ne( 1e-40_f64,  0.0_f64,   1e-41_f64, 4_u64);
        check_ne( 0.0_f64,    1e-40_f64, 1e-41_f64, 4_u64);
        check_ne(-1e-40_f64,  0.0_f64,   1e-41_f64, 4_u64);
        check_ne( 0.0_f64,   -1e-40_f64, 1e-41_f64, 4_u64);
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
        check_ne( 1.0_f64,  2.0_f64, f64::EPSILON,      2_u64);
        check_ne( 1.0_f64,  2.0_f64, f64::MIN_POSITIVE, 4_u64);
        check_ne( 1.0_f64, -2.0_f64, f64::EPSILON,      8_u64);
        check_ne( 1.0_f64, -2.0_f64, f64::MIN_POSITIVE, 16_u64);
        check_ne(-1.0_f64,  2.0_f64, f64::EPSILON,      32_u64);
        check_ne(-1.0_f64,  2.0_f64, f64::MIN_POSITIVE, 64_u64);
        check_ne(-1.0_f64, -2.0_f64, f64::EPSILON,      128_u64);
        check_ne(-1.0_f64, -2.0_f64, f64::MIN_POSITIVE, 256_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding1() {
        check_eq( 10000000000000000.0_f64,  10000000000000001.0_f64, f64::EPSILON, 1_u64);
        check_eq( 10000000000000001.0_f64,  10000000000000000.0_f64, f64::EPSILON, 1_u64);
        check_eq(-10000000000000000.0_f64, -10000000000000001.0_f64, f64::EPSILON, 1_u64);
        check_eq(-10000000000000001.0_f64, -10000000000000000.0_f64, f64::EPSILON, 1_u64);

        check_eq( 100000000000000000.0_f64,  100000000000000001.0_f64, f64::EPSILON, 1_u64);
        check_eq( 100000000000000001.0_f64,  100000000000000000.0_f64, f64::EPSILON, 1_u64);
        check_eq(-100000000000000000.0_f64, -100000000000000001.0_f64, f64::EPSILON, 1_u64);
        check_eq(-100000000000000001.0_f64, -100000000000000000.0_f64, f64::EPSILON, 1_u64);

        check_eq( 1000000000000000000.0_f64,  1000000000000000001.0_f64, f64::EPSILON, 1_u64);
        check_eq( 1000000000000000001.0_f64,  1000000000000000000.0_f64, f64::EPSILON, 1_u64);
        check_eq(-1000000000000000000.0_f64, -1000000000000000001.0_f64, f64::EPSILON, 1_u64);
        check_eq(-1000000000000000001.0_f64, -1000000000000000000.0_f64, f64::EPSILON, 1_u64);

        check_eq( 10000000000000000000.0_f64,  10000000000000000001.0_f64, f64::EPSILON, 1_u64);
        check_eq( 10000000000000000001.0_f64,  10000000000000000000.0_f64, f64::EPSILON, 1_u64);
        check_eq(-10000000000000000000.0_f64, -10000000000000000001.0_f64, f64::EPSILON, 1_u64);
        check_eq(-10000000000000000001.0_f64, -10000000000000000000.0_f64, f64::EPSILON, 1_u64);

        check_eq( 100000000000000000000.0_f64,  100000000000000000001.0_f64, f64::EPSILON, 1_u64);
        check_eq( 100000000000000000001.0_f64,  100000000000000000000.0_f64, f64::EPSILON, 1_u64);
        check_eq(-100000000000000000000.0_f64, -100000000000000000001.0_f64, f64::EPSILON, 1_u64);
        check_eq(-100000000000000000001.0_f64, -100000000000000000000.0_f64, f64::EPSILON, 1_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding1() {
        check_ne( 1000.0_f64,  1001.0_f64, f64::EPSILON, 4_u64);
        check_ne( 1001.0_f64,  1000.0_f64, f64::EPSILON, 4_u64);
        check_ne(-1000.0_f64, -1001.0_f64, f64::EPSILON, 4_u64);
        check_ne(-1001.0_f64, -1000.0_f64, f64::EPSILON, 4_u64);

        check_ne( 10000.0_f64,  10001.0_f64, f64::EPSILON, 4_u64);
        check_ne( 10001.0_f64,  10000.0_f64, f64::EPSILON, 4_u64);
        check_ne(-10000.0_f64, -10001.0_f64, f64::EPSILON, 4_u64);
        check_ne(-10001.0_f64, -10000.0_f64, f64::EPSILON, 4_u64);

        check_ne( 100000.0_f64,  100001.0_f64, f64::EPSILON, 4_u64);
        check_ne( 100001.0_f64,  100000.0_f64, f64::EPSILON, 4_u64);
        check_ne(-100000.0_f64, -100001.0_f64, f64::EPSILON, 4_u64);
        check_ne(-100001.0_f64, -100000.0_f64, f64::EPSILON, 4_u64);

        check_ne( 1000000.0_f64,  1000001.0_f64, f64::EPSILON, 4_u64);
        check_ne( 1000001.0_f64,  1000000.0_f64, f64::EPSILON, 4_u64);
        check_ne(-1000000.0_f64, -1000001.0_f64, f64::EPSILON, 4_u64);
        check_ne(-1000001.0_f64, -1000000.0_f64, f64::EPSILON, 4_u64);
        
        check_ne( 10000000.0_f64,  10000001.0_f64, f64::EPSILON, 4_u64);
        check_ne( 10000001.0_f64,  10000000.0_f64, f64::EPSILON, 4_u64);
        check_ne(-10000000.0_f64, -10000001.0_f64, f64::EPSILON, 4_u64);
        check_ne(-10000001.0_f64, -10000000.0_f64, f64::EPSILON, 4_u64);

        check_ne( 100000000.0_f64,  100000001.0_f64, f64::EPSILON, 4_u64);
        check_ne( 100000001.0_f64,  100000000.0_f64, f64::EPSILON, 4_u64);
        check_ne(-100000000.0_f64, -100000001.0_f64, f64::EPSILON, 4_u64);
        check_ne(-100000001.0_f64, -100000000.0_f64, f64::EPSILON, 4_u64);

        check_ne( 1000000000.0_f64,  1000000001.0_f64, f64::EPSILON, 4_u64);
        check_ne( 1000000001.0_f64,  1000000000.0_f64, f64::EPSILON, 4_u64);
        check_ne(-1000000000.0_f64, -1000000001.0_f64, f64::EPSILON, 4_u64);
        check_ne(-1000000001.0_f64, -1000000000.0_f64, f64::EPSILON, 4_u64);

        check_ne( 10000000000.0_f64,  10000000001.0_f64, f64::EPSILON, 4_u64);
        check_ne( 10000000001.0_f64,  10000000000.0_f64, f64::EPSILON, 4_u64);
        check_ne(-10000000000.0_f64, -10000000001.0_f64, f64::EPSILON, 4_u64);
        check_ne(-10000000001.0_f64, -10000000000.0_f64, f64::EPSILON, 4_u64);

        check_ne( 100000000000.0_f64,  100000000001.0_f64, f64::EPSILON, 4_u64);
        check_ne( 100000000001.0_f64,  100000000000.0_f64, f64::EPSILON, 4_u64);
        check_ne(-100000000000.0_f64, -100000000001.0_f64, f64::EPSILON, 4_u64);
        check_ne(-100000000001.0_f64, -100000000000.0_f64, f64::EPSILON, 4_u64);

        check_ne( 1000000000000.0_f64,  1000000000001.0_f64, f64::EPSILON, 4_u64);
        check_ne( 1000000000001.0_f64,  1000000000000.0_f64, f64::EPSILON, 4_u64);
        check_ne(-1000000000000.0_f64, -1000000000001.0_f64, f64::EPSILON, 4_u64);
        check_ne(-1000000000001.0_f64, -1000000000000.0_f64, f64::EPSILON, 4_u64);

        check_ne( 10000000000000.0_f64,  10000000000001.0_f64, f64::EPSILON, 4_u64);
        check_ne( 10000000000001.0_f64,  10000000000000.0_f64, f64::EPSILON, 4_u64);
        check_ne(-10000000000000.0_f64, -10000000000001.0_f64, f64::EPSILON, 4_u64);
        check_ne(-10000000000001.0_f64, -10000000000000.0_f64, f64::EPSILON, 4_u64);

        check_ne( 100000000000000.0_f64,  100000000000001.0_f64, f64::EPSILON, 4_u64);
        check_ne( 100000000000001.0_f64,  100000000000000.0_f64, f64::EPSILON, 4_u64);
        check_ne(-100000000000000.0_f64, -100000000000001.0_f64, f64::EPSILON, 4_u64);
        check_ne(-100000000000001.0_f64, -100000000000000.0_f64, f64::EPSILON, 4_u64);

        check_ne( 1000000000000000.0_f64,  1000000000000001.0_f64, f64::EPSILON, 4_u64);
        check_ne( 1000000000000001.0_f64,  1000000000000000.0_f64, f64::EPSILON, 4_u64);
        check_ne(-1000000000000000.0_f64, -1000000000000001.0_f64, f64::EPSILON, 4_u64);
        check_ne(-1000000000000001.0_f64, -1000000000000000.0_f64, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding2() {
        check_eq( 1.000000000000001_f64,  1.000000000000002_f64, f64::EPSILON, 4_u64);
        check_eq( 1.000000000000002_f64,  1.000000000000001_f64, f64::EPSILON, 4_u64);
        check_eq(-1.000000000000001_f64, -1.000000000000002_f64, f64::EPSILON, 4_u64);
        check_eq(-1.000000000000002_f64, -1.000000000000001_f64, f64::EPSILON, 4_u64);

        check_eq( 1.0000000000000001_f64,  1.0000000000000002_f64, f64::EPSILON, 4_u64);
        check_eq( 1.0000000000000002_f64,  1.0000000000000001_f64, f64::EPSILON, 4_u64);
        check_eq(-1.0000000000000001_f64, -1.0000000000000002_f64, f64::EPSILON, 4_u64);
        check_eq(-1.0000000000000002_f64, -1.0000000000000001_f64, f64::EPSILON, 4_u64);

        check_eq( 1.00000000000000001_f64,  1.00000000000000002_f64, f64::EPSILON, 4_u64);
        check_eq( 1.00000000000000002_f64,  1.00000000000000001_f64, f64::EPSILON, 4_u64);
        check_eq(-1.00000000000000001_f64, -1.00000000000000002_f64, f64::EPSILON, 4_u64);
        check_eq(-1.00000000000000002_f64, -1.00000000000000001_f64, f64::EPSILON, 4_u64);

        check_eq( 1.000000000000000001_f64,  1.000000000000000002_f64, f64::EPSILON, 4_u64);
        check_eq( 1.000000000000000002_f64,  1.000000000000000001_f64, f64::EPSILON, 4_u64);
        check_eq(-1.000000000000000001_f64, -1.000000000000000002_f64, f64::EPSILON, 4_u64);
        check_eq(-1.000000000000000002_f64, -1.000000000000000001_f64, f64::EPSILON, 4_u64);

        check_eq( 1.0000000000000000001_f64,  1.0000000000000000002_f64, f64::EPSILON, 4_u64);
        check_eq( 1.0000000000000000002_f64,  1.0000000000000000001_f64, f64::EPSILON, 4_u64);
        check_eq(-1.0000000000000000001_f64, -1.0000000000000000002_f64, f64::EPSILON, 4_u64);
        check_eq(-1.0000000000000000002_f64, -1.0000000000000000001_f64, f64::EPSILON, 4_u64);

        check_eq( 1.00000000000000000001_f64,  1.00000000000000000002_f64, f64::EPSILON, 4_u64);
        check_eq( 1.00000000000000000002_f64,  1.00000000000000000001_f64, f64::EPSILON, 4_u64);
        check_eq(-1.00000000000000000001_f64, -1.00000000000000000002_f64, f64::EPSILON, 4_u64);
        check_eq(-1.00000000000000000002_f64, -1.00000000000000000001_f64, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding2() {
        check_ne( 1.01_f64,  1.02_f64, f64::EPSILON, 4_u64);
        check_ne( 1.02_f64,  1.01_f64, f64::EPSILON, 4_u64);
        check_ne(-1.01_f64, -1.02_f64, f64::EPSILON, 4_u64);
        check_ne(-1.02_f64, -1.01_f64, f64::EPSILON, 4_u64);

        check_ne( 1.001_f64,  1.002_f64, f64::EPSILON, 4_u64);
        check_ne( 1.002_f64,  1.001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.001_f64, -1.002_f64, f64::EPSILON, 4_u64);
        check_ne(-1.002_f64, -1.001_f64, f64::EPSILON, 4_u64);

        check_ne( 1.0001_f64,  1.0002_f64, f64::EPSILON, 4_u64);
        check_ne( 1.0002_f64,  1.0001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.0001_f64, -1.0002_f64, f64::EPSILON, 4_u64);
        check_ne(-1.0002_f64, -1.0001_f64, f64::EPSILON, 4_u64);

        check_ne( 1.00001_f64,  1.00002_f64, f64::EPSILON, 4_u64);
        check_ne( 1.00002_f64,  1.00001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.00001_f64, -1.00002_f64, f64::EPSILON, 4_u64);
        check_ne(-1.00002_f64, -1.00001_f64, f64::EPSILON, 4_u64);

        check_ne( 1.000001_f64,  1.000002_f64, f64::EPSILON, 4_u64);
        check_ne( 1.000002_f64,  1.000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.000001_f64, -1.000002_f64, f64::EPSILON, 4_u64);
        check_ne(-1.000002_f64, -1.000001_f64, f64::EPSILON, 4_u64);

        check_ne( 1.0000001_f64,  1.0000002_f64, f64::EPSILON, 4_u64);
        check_ne( 1.0000002_f64,  1.0000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.0000001_f64, -1.0000002_f64, f64::EPSILON, 4_u64);
        check_ne(-1.0000002_f64, -1.0000001_f64, f64::EPSILON, 4_u64);

        check_ne( 1.00000001_f64,  1.00000002_f64, f64::EPSILON, 4_u64);
        check_ne( 1.00000002_f64,  1.00000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.00000001_f64, -1.00000002_f64, f64::EPSILON, 4_u64);
        check_ne(-1.00000002_f64, -1.00000001_f64, f64::EPSILON, 4_u64);

        check_ne( 1.000000001_f64,  1.000000002_f64, f64::EPSILON, 4_u64);
        check_ne( 1.000000002_f64,  1.000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.000000001_f64, -1.000000002_f64, f64::EPSILON, 4_u64);
        check_ne(-1.000000002_f64, -1.000000001_f64, f64::EPSILON, 4_u64);

        check_ne( 1.0000000001_f64,  1.0000000002_f64, f64::EPSILON, 4_u64);
        check_ne( 1.0000000002_f64,  1.0000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.0000000001_f64, -1.0000000002_f64, f64::EPSILON, 4_u64);
        check_ne(-1.0000000002_f64, -1.0000000001_f64, f64::EPSILON, 4_u64);

        check_ne( 1.00000000001_f64,  1.00000000002_f64, f64::EPSILON, 4_u64);
        check_ne( 1.00000000002_f64,  1.00000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.00000000001_f64, -1.00000000002_f64, f64::EPSILON, 4_u64);
        check_ne(-1.00000000002_f64, -1.00000000001_f64, f64::EPSILON, 4_u64);

        check_ne( 1.000000000001_f64,  1.000000000002_f64, f64::EPSILON, 4_u64);
        check_ne( 1.000000000002_f64,  1.000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.000000000001_f64, -1.000000000002_f64, f64::EPSILON, 4_u64);
        check_ne(-1.000000000002_f64, -1.000000000001_f64, f64::EPSILON, 4_u64);

        check_ne( 1.0000000000001_f64,  1.0000000000002_f64, f64::EPSILON, 4_u64);
        check_ne( 1.0000000000002_f64,  1.0000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.0000000000001_f64, -1.0000000000002_f64, f64::EPSILON, 4_u64);
        check_ne(-1.0000000000002_f64, -1.0000000000001_f64, f64::EPSILON, 4_u64);

        check_ne( 1.00000000000001_f64,  1.00000000000002_f64, f64::EPSILON, 4_u64);
        check_ne( 1.00000000000002_f64,  1.00000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.00000000000001_f64, -1.00000000000002_f64, f64::EPSILON, 4_u64);
        check_ne(-1.00000000000002_f64, -1.00000000000001_f64, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding3() {
        check_eq( 0.9999999999999999_f64,  1.0000000000000001_f64, f64::EPSILON, 2_u64);
        check_eq( 1.0000000000000001_f64,  0.9999999999999999_f64, f64::EPSILON, 2_u64);
        check_eq(-0.9999999999999999_f64, -1.0000000000000001_f64, f64::EPSILON, 2_u64);
        check_eq(-1.0000000000000001_f64, -0.9999999999999999_f64, f64::EPSILON, 2_u64);

        check_eq( 0.99999999999999999_f64,  1.00000000000000001_f64, f64::EPSILON, 2_u64);
        check_eq( 1.00000000000000001_f64,  0.99999999999999999_f64, f64::EPSILON, 2_u64);
        check_eq(-0.99999999999999999_f64, -1.00000000000000001_f64, f64::EPSILON, 2_u64);
        check_eq(-1.00000000000000001_f64, -0.99999999999999999_f64, f64::EPSILON, 2_u64);

        check_eq( 0.999999999999999999_f64,  1.000000000000000001_f64, f64::EPSILON, 2_u64);
        check_eq( 1.000000000000000001_f64,  0.999999999999999999_f64, f64::EPSILON, 2_u64);
        check_eq(-0.999999999999999999_f64, -1.000000000000000001_f64, f64::EPSILON, 2_u64);
        check_eq(-1.000000000000000001_f64, -0.999999999999999999_f64, f64::EPSILON, 2_u64);

        check_eq( 0.9999999999999999999_f64,  1.0000000000000000001_f64, f64::EPSILON, 2_u64);
        check_eq( 1.0000000000000000001_f64,  0.9999999999999999999_f64, f64::EPSILON, 2_u64);
        check_eq(-0.9999999999999999999_f64, -1.0000000000000000001_f64, f64::EPSILON, 2_u64);
        check_eq(-1.0000000000000000001_f64, -0.9999999999999999999_f64, f64::EPSILON, 2_u64);

        check_eq( 0.99999999999999999999_f64,  1.00000000000000000001_f64, f64::EPSILON, 2_u64);
        check_eq( 1.00000000000000000001_f64,  0.99999999999999999999_f64, f64::EPSILON, 2_u64);
        check_eq(-0.99999999999999999999_f64, -1.00000000000000000001_f64, f64::EPSILON, 2_u64);
        check_eq(-1.00000000000000000001_f64, -0.99999999999999999999_f64, f64::EPSILON, 2_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding3() {
        check_ne( 0.99_f64,  1.01_f64, f64::EPSILON, 4_u64);
        check_ne( 1.01_f64,  0.99_f64, f64::EPSILON, 4_u64);
        check_ne(-0.99_f64, -1.01_f64, f64::EPSILON, 4_u64);
        check_ne(-1.01_f64, -0.99_f64, f64::EPSILON, 4_u64);

        check_ne( 0.999_f64,  1.001_f64, f64::EPSILON, 4_u64);
        check_ne( 1.001_f64,  0.999_f64, f64::EPSILON, 4_u64);
        check_ne(-0.999_f64, -1.001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.001_f64, -0.999_f64, f64::EPSILON, 4_u64);

        check_ne( 0.9999_f64,  1.0001_f64, f64::EPSILON, 4_u64);
        check_ne( 1.0001_f64,  0.9999_f64, f64::EPSILON, 4_u64);
        check_ne(-0.9999_f64, -1.0001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.0001_f64, -0.9999_f64, f64::EPSILON, 4_u64);

        check_ne( 0.99999_f64,  1.00001_f64, f64::EPSILON, 4_u64);
        check_ne( 1.00001_f64,  0.99999_f64, f64::EPSILON, 4_u64);
        check_ne(-0.99999_f64, -1.00001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.00001_f64, -0.99999_f64, f64::EPSILON, 4_u64);

        check_ne( 0.999999_f64,  1.000001_f64, f64::EPSILON, 4_u64);
        check_ne( 1.000001_f64,  0.999999_f64, f64::EPSILON, 4_u64);
        check_ne(-0.999999_f64, -1.000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.000001_f64, -0.999999_f64, f64::EPSILON, 4_u64);

        check_ne( 0.9999999_f64,  1.0000001_f64, f64::EPSILON, 4_u64);
        check_ne( 1.0000001_f64,  0.9999999_f64, f64::EPSILON, 4_u64);
        check_ne(-0.9999999_f64, -1.0000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.0000001_f64, -0.9999999_f64, f64::EPSILON, 4_u64);

        check_ne( 0.99999999_f64,  1.00000001_f64, f64::EPSILON, 4_u64);
        check_ne( 1.00000001_f64,  0.99999999_f64, f64::EPSILON, 4_u64);
        check_ne(-0.99999999_f64, -1.00000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.00000001_f64, -0.99999999_f64, f64::EPSILON, 4_u64);

        check_ne( 0.999999999_f64,  1.000000001_f64, f64::EPSILON, 4_u64);
        check_ne( 1.000000001_f64,  0.999999999_f64, f64::EPSILON, 4_u64);
        check_ne(-0.999999999_f64, -1.000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.000000001_f64, -0.999999999_f64, f64::EPSILON, 4_u64);

        check_ne( 0.9999999999_f64,  1.0000000001_f64, f64::EPSILON, 4_u64);
        check_ne( 1.0000000001_f64,  0.9999999999_f64, f64::EPSILON, 4_u64);
        check_ne(-0.9999999999_f64, -1.0000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.0000000001_f64, -0.9999999999_f64, f64::EPSILON, 4_u64);

        check_ne( 0.99999999999_f64,  1.00000000001_f64, f64::EPSILON, 4_u64);
        check_ne( 1.00000000001_f64,  0.99999999999_f64, f64::EPSILON, 4_u64);
        check_ne(-0.99999999999_f64, -1.00000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.00000000001_f64, -0.99999999999_f64, f64::EPSILON, 4_u64);

        check_ne( 0.999999999999_f64,  1.000000000001_f64, f64::EPSILON, 4_u64);
        check_ne( 1.000000000001_f64,  0.999999999999_f64, f64::EPSILON, 4_u64);
        check_ne(-0.999999999999_f64, -1.000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.000000000001_f64, -0.999999999999_f64, f64::EPSILON, 4_u64);

        check_ne( 0.9999999999999_f64,  1.0000000000001_f64, f64::EPSILON, 4_u64);
        check_ne( 1.0000000000001_f64,  0.9999999999999_f64, f64::EPSILON, 4_u64);
        check_ne(-0.9999999999999_f64, -1.0000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.0000000000001_f64, -0.9999999999999_f64, f64::EPSILON, 4_u64);

        check_ne( 0.99999999999999_f64,  1.00000000000001_f64, f64::EPSILON, 4_u64);
        check_ne( 1.00000000000001_f64,  0.99999999999999_f64, f64::EPSILON, 4_u64);
        check_ne(-0.99999999999999_f64, -1.00000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.00000000000001_f64, -0.99999999999999_f64, f64::EPSILON, 4_u64);

        check_ne( 0.999999999999999_f64,  1.000000000000001_f64, f64::EPSILON, 4_u64);
        check_ne( 1.000000000000001_f64,  0.999999999999999_f64, f64::EPSILON, 4_u64);
        check_ne(-0.999999999999999_f64, -1.000000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-1.000000000000001_f64, -0.999999999999999_f64, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding4() {
        check_eq( 1.9999999999999999_f64,  2.0000000000000001_f64, f64::EPSILON, 1_u64);
        check_eq( 2.0000000000000001_f64,  1.9999999999999999_f64, f64::EPSILON, 1_u64);
        check_eq(-1.9999999999999999_f64, -2.0000000000000001_f64, f64::EPSILON, 1_u64);
        check_eq(-2.0000000000000001_f64, -1.9999999999999999_f64, f64::EPSILON, 1_u64);

        check_eq( 1.99999999999999999_f64,  2.00000000000000001_f64, f64::EPSILON, 1_u64);
        check_eq( 2.00000000000000001_f64,  1.99999999999999999_f64, f64::EPSILON, 1_u64);
        check_eq(-1.99999999999999999_f64, -2.00000000000000001_f64, f64::EPSILON, 1_u64);
        check_eq(-2.00000000000000001_f64, -1.99999999999999999_f64, f64::EPSILON, 1_u64);

        check_eq( 1.999999999999999999_f64,  2.000000000000000001_f64, f64::EPSILON, 1_u64);
        check_eq( 2.000000000000000001_f64,  1.999999999999999999_f64, f64::EPSILON, 1_u64);
        check_eq(-1.999999999999999999_f64, -2.000000000000000001_f64, f64::EPSILON, 1_u64);
        check_eq(-2.000000000000000001_f64, -1.999999999999999999_f64, f64::EPSILON, 1_u64);

        check_eq( 1.9999999999999999999_f64,  2.0000000000000000001_f64, f64::EPSILON, 1_u64);
        check_eq( 2.0000000000000000001_f64,  1.9999999999999999999_f64, f64::EPSILON, 1_u64);
        check_eq(-1.9999999999999999999_f64, -2.0000000000000000001_f64, f64::EPSILON, 1_u64);
        check_eq(-2.0000000000000000001_f64, -1.9999999999999999999_f64, f64::EPSILON, 1_u64);

        check_eq( 1.99999999999999999999_f64,  2.00000000000000000001_f64, f64::EPSILON, 1_u64);
        check_eq( 2.00000000000000000001_f64,  1.99999999999999999999_f64, f64::EPSILON, 1_u64);
        check_eq(-1.99999999999999999999_f64, -2.00000000000000000001_f64, f64::EPSILON, 1_u64);
        check_eq(-2.00000000000000000001_f64, -1.99999999999999999999_f64, f64::EPSILON, 1_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding4() {
        check_ne( 1.99_f64,  2.01_f64, f64::EPSILON, 4_u64);
        check_ne( 2.01_f64,  1.99_f64, f64::EPSILON, 4_u64);
        check_ne(-1.99_f64, -2.01_f64, f64::EPSILON, 4_u64);
        check_ne(-2.01_f64, -1.99_f64, f64::EPSILON, 4_u64);

        check_ne( 1.999_f64,  2.001_f64, f64::EPSILON, 4_u64);
        check_ne( 2.001_f64,  1.999_f64, f64::EPSILON, 4_u64);
        check_ne(-1.999_f64, -2.001_f64, f64::EPSILON, 4_u64);
        check_ne(-2.001_f64, -1.999_f64, f64::EPSILON, 4_u64);

        check_ne( 1.9999_f64,  2.0001_f64, f64::EPSILON, 4_u64);
        check_ne( 2.0001_f64,  1.9999_f64, f64::EPSILON, 4_u64);
        check_ne(-1.9999_f64, -2.0001_f64, f64::EPSILON, 4_u64);
        check_ne(-2.0001_f64, -1.9999_f64, f64::EPSILON, 4_u64);

        check_ne( 1.99999_f64,  2.00001_f64, f64::EPSILON, 4_u64);
        check_ne( 2.00001_f64,  1.99999_f64, f64::EPSILON, 4_u64);
        check_ne(-1.99999_f64, -2.00001_f64, f64::EPSILON, 4_u64);
        check_ne(-2.00001_f64, -1.99999_f64, f64::EPSILON, 4_u64);

        check_ne( 1.999999_f64,  2.000001_f64, f64::EPSILON, 4_u64);
        check_ne( 2.000001_f64,  1.999999_f64, f64::EPSILON, 4_u64);
        check_ne(-1.999999_f64, -2.000001_f64, f64::EPSILON, 4_u64);
        check_ne(-2.000001_f64, -1.999999_f64, f64::EPSILON, 4_u64);

        check_ne( 1.9999999_f64,  2.0000001_f64, f64::EPSILON, 4_u64);
        check_ne( 2.0000001_f64,  1.9999999_f64, f64::EPSILON, 4_u64);
        check_ne(-1.9999999_f64, -2.0000001_f64, f64::EPSILON, 4_u64);
        check_ne(-2.0000001_f64, -1.9999999_f64, f64::EPSILON, 4_u64);

        check_ne( 1.99999999_f64,  2.00000001_f64, f64::EPSILON, 4_u64);
        check_ne( 2.00000001_f64,  1.99999999_f64, f64::EPSILON, 4_u64);
        check_ne(-1.99999999_f64, -2.00000001_f64, f64::EPSILON, 4_u64);
        check_ne(-2.00000001_f64, -1.99999999_f64, f64::EPSILON, 4_u64);

        check_ne( 1.999999999_f64,  2.000000001_f64, f64::EPSILON, 4_u64);
        check_ne( 2.000000001_f64,  1.999999999_f64, f64::EPSILON, 4_u64);
        check_ne(-1.999999999_f64, -2.000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-2.000000001_f64, -1.999999999_f64, f64::EPSILON, 4_u64);

        check_ne( 1.9999999999_f64,  2.0000000001_f64, f64::EPSILON, 4_u64);
        check_ne( 2.0000000001_f64,  1.9999999999_f64, f64::EPSILON, 4_u64);
        check_ne(-1.9999999999_f64, -2.0000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-2.0000000001_f64, -1.9999999999_f64, f64::EPSILON, 4_u64);

        check_ne( 1.99999999999_f64,  2.00000000001_f64, f64::EPSILON, 4_u64);
        check_ne( 2.00000000001_f64,  1.99999999999_f64, f64::EPSILON, 4_u64);
        check_ne(-1.99999999999_f64, -2.00000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-2.00000000001_f64, -1.99999999999_f64, f64::EPSILON, 4_u64);

        check_ne( 1.999999999999_f64,  2.000000000001_f64, f64::EPSILON, 4_u64);
        check_ne( 2.000000000001_f64,  1.999999999999_f64, f64::EPSILON, 4_u64);
        check_ne(-1.999999999999_f64, -2.000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-2.000000000001_f64, -1.999999999999_f64, f64::EPSILON, 4_u64);

        check_ne( 1.9999999999999_f64,  2.0000000000001_f64, f64::EPSILON, 4_u64);
        check_ne( 2.0000000000001_f64,  1.9999999999999_f64, f64::EPSILON, 4_u64);
        check_ne(-1.9999999999999_f64, -2.0000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-2.0000000000001_f64, -1.9999999999999_f64, f64::EPSILON, 4_u64);

        check_ne( 1.99999999999999_f64,  2.00000000000001_f64, f64::EPSILON, 4_u64);
        check_ne( 2.00000000000001_f64,  1.99999999999999_f64, f64::EPSILON, 4_u64);
        check_ne(-1.99999999999999_f64, -2.00000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-2.00000000000001_f64, -1.99999999999999_f64, f64::EPSILON, 4_u64);

        check_ne( 1.999999999999999_f64,  2.000000000000001_f64, f64::EPSILON, 4_u64);
        check_ne( 2.000000000000001_f64,  1.999999999999999_f64, f64::EPSILON, 4_u64);
        check_ne(-1.999999999999999_f64, -2.000000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-2.000000000000001_f64, -1.999999999999999_f64, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_max() {
        check_eq( f64::MAX,  f64::MAX, f64::EPSILON, 4_u64);
        check_eq(-f64::MAX, -f64::MAX, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_max() {
        check_ne( f64::MAX, -f64::MAX,           f64::EPSILON, 4_u64);
        check_ne(-f64::MAX,  f64::MAX,           f64::EPSILON, 4_u64);
        check_ne( f64::MAX,  f64::MAX / 2.0_f64, f64::EPSILON, 4_u64);
        check_ne( f64::MAX, -f64::MAX / 2.0_f64, f64::EPSILON, 4_u64);
        check_ne(-f64::MAX,  f64::MAX / 2.0_f64, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_nan1() {
        check_ne( f64::NAN,  f64::NAN, 0.0_f64, 128_u64);
        check_ne( f64::NAN, -f64::NAN, 0.0_f64, 128_u64);
        check_ne(-f64::NAN,  f64::NAN, 0.0_f64, 128_u64);
        check_ne(-f64::NAN, -f64::NAN, 0.0_f64, 128_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_nan2() {
        for i in 0..=i16::MAX {
            check_ne( f64::NAN , f64::NAN, 1.0_f64 / (i as f64), i as u64);
            check_ne( f64::NAN, -f64::NAN, 1.0_f64 / (i as f64), i as u64);
            check_ne(-f64::NAN,  f64::NAN, 1.0_f64 / (i as f64), i as u64);
            check_ne(-f64::NAN, -f64::NAN, 1.0_f64 / (i as f64), i as u64);
        }
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_nan3() {
        for i in 0..=i16::MAX {
            check_ne( f64::NAN , f64::NAN, i as f64, i as u64);
            check_ne( f64::NAN, -f64::NAN, i as f64, i as u64);
            check_ne(-f64::NAN,  f64::NAN, i as f64, i as u64);
            check_ne(-f64::NAN, -f64::NAN, i as f64, i as u64);
        }
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_nan4() {
        check_ne(f64::NAN, f64::NAN, f64::EPSILON, 4_u64);
        
        check_ne( f64::NAN,  0.0_f64,  f64::EPSILON, 4_u64);
        check_ne(-0.0_f64,   f64::NAN, f64::EPSILON, 4_u64);
        check_ne( f64::NAN, -0.0_f64,  f64::EPSILON, 4_u64);
        check_ne( 0.0_f64,   f64::NAN, f64::EPSILON, 4_u64);

        check_ne( f64::NAN,       f64::INFINITY, f64::EPSILON, 4_u64);
        check_ne( f64::INFINITY,  f64::NAN,      f64::EPSILON, 4_u64);
        check_ne( f64::NAN,      -f64::INFINITY, f64::EPSILON, 4_u64);
        check_ne(-f64::INFINITY,  f64::NAN,      f64::EPSILON, 4_u64);

        check_ne( f64::NAN,  f64::MAX, f64::EPSILON, 4_u64);
        check_ne( f64::MAX,  f64::NAN, f64::EPSILON, 4_u64);
        check_ne( f64::NAN, -f64::MAX, f64::EPSILON, 4_u64);
        check_ne(-f64::MAX,  f64::NAN, f64::EPSILON, 4_u64);

        check_ne( f64::NAN,           f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE,  f64::NAN,          f64::EPSILON, 4_u64);
        check_ne( f64::NAN,          -f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE,  f64::NAN,          f64::EPSILON, 4_u64);

        check_ne( f64::NAN,  1.0_f64,  f64::EPSILON, 4_u64);
        check_ne( f64::NAN, -1.0_f64,  f64::EPSILON, 4_u64);
        check_ne( 1.0_f64,   f64::NAN, f64::EPSILON, 4_u64);
        check_ne(-1.0_f64,   f64::NAN, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_nan5() {
        check_ne(f64::NAN, f64::NAN, 1.0_f64, 64_u64);
        
        check_ne( f64::NAN,  0.0_f64,  1.0_f64, 64_u64);
        check_ne(-0.0_f64,   f64::NAN, 1.0_f64, 64_u64);
        check_ne( f64::NAN, -0.0_f64,  1.0_f64, 64_u64);
        check_ne( 0.0_f64,   f64::NAN, 1.0_f64, 64_u64);

        check_ne( f64::NAN,       f64::INFINITY, 1.0_f64, 64_u64);
        check_ne( f64::INFINITY,  f64::NAN,      1.0_f64, 64_u64);
        check_ne( f64::NAN,      -f64::INFINITY, 1.0_f64, 64_u64);
        check_ne(-f64::INFINITY,  f64::NAN,      1.0_f64, 64_u64);

        check_ne( f64::NAN,  f64::MAX, 1.0_f64, 64_u64);
        check_ne( f64::MAX,  f64::NAN, 1.0_f64, 64_u64);
        check_ne( f64::NAN, -f64::MAX, 1.0_f64, 64_u64);
        check_ne(-f64::MAX,  f64::NAN, 1.0_f64, 64_u64);

        check_ne( f64::NAN,           f64::MIN_POSITIVE, 1.0_f64, 64_u64);
        check_ne( f64::MIN_POSITIVE,  f64::NAN,          1.0_f64, 64_u64);
        check_ne( f64::NAN,          -f64::MIN_POSITIVE, 1.0_f64, 64_u64);
        check_ne(-f64::MIN_POSITIVE,  f64::NAN,          1.0_f64, 64_u64);

        check_ne( f64::NAN,  1.0_f64,  1.0_f64, 64_u64);
        check_ne( f64::NAN, -1.0_f64,  1.0_f64, 64_u64);
        check_ne( 1.0_f64,   f64::NAN, 1.0_f64, 64_u64);
        check_ne(-1.0_f64,   f64::NAN, 1.0_f64, 64_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_infinity1() {
        check_eq(f64::INFINITY,     f64::INFINITY,     f64::MAX,      u64::MAX);
        check_eq(f64::INFINITY,     f64::INFINITY,     f64::INFINITY, u64::MAX);
        check_eq(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::INFINITY, u64::MAX);
        check_eq(f64::INFINITY,     f64::MAX,          f64::MAX,      u64::MAX);
        check_eq(f64::INFINITY,     f64::MAX,          f64::INFINITY, u64::MAX);
        check_eq(f64::INFINITY,     f64::NEG_INFINITY, f64::INFINITY, u64::MAX);
        check_eq(f64::NEG_INFINITY, f64::MAX,          f64::INFINITY, u64::MAX);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_infinity2() {
        check_eq(f64::INFINITY,      f64::INFINITY,     0.0_f64, 1_u64);
        check_eq(f64::NEG_INFINITY,  f64::NEG_INFINITY, 0.0_f64, 1_u64);
        check_eq(f64::INFINITY,      f64::MAX,          0.0_f64, 1_u64);
        check_eq(f64::NEG_INFINITY, -f64::MAX,          0.0_f64, 1_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_infinity1() {
        check_ne( f64::INFINITY,      f64::NEG_INFINITY, f64::MAX, u64::MAX);
        check_ne( f64::NEG_INFINITY,  f64::MAX,          f64::MAX, u64::MAX);
        check_ne(-f64::MAX,           f64::INFINITY,     f64::MAX, u64::MAX);
        check_ne( f64::INFINITY,     -f64::MAX,          f64::MAX, u64::MAX);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_infinity2() {
        check_ne( f64::INFINITY,      f64::NEG_INFINITY, f64::MAX, 4_u64);
        check_ne( f64::NEG_INFINITY,  f64::MAX,          f64::MAX, 4_u64);
        check_ne(-f64::MAX,           f64::INFINITY,     f64::MAX, 4_u64);
        check_ne( f64::INFINITY,     -f64::MAX,          f64::MAX, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero1() {
        check_eq( f64::MIN_POSITIVE,  f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_eq( f64::MIN_POSITIVE, -f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_eq(-f64::MIN_POSITIVE,  f64::MIN_POSITIVE, f64::EPSILON, 4_u64);

        check_eq( f64::MIN_POSITIVE,  0.0_f64,           f64::EPSILON, 4_u64);
        check_eq( 0.0_f64,            f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_eq(-f64::MIN_POSITIVE,  0.0_f64,           f64::EPSILON, 4_u64);
        check_eq( 0.0_f64,           -f64::MIN_POSITIVE, f64::EPSILON, 4_u64);

        check_eq( 0.0000000000000001_f64, -f64::MIN_POSITIVE,      f64::EPSILON, 4_u64);
        check_eq( 0.0000000000000001_f64,  f64::MIN_POSITIVE,      f64::EPSILON, 4_u64);
        check_eq( f64::MIN_POSITIVE,       0.0000000000000001_f64, f64::EPSILON, 4_u64);
        check_eq(-f64::MIN_POSITIVE,       0.0000000000000001_f64, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero2() {
        check_eq( 0.0000000010000001_f64,  0.0000000010000002_f64, f64::EPSILON, 4_u64);
        check_eq( 0.0000000010000002_f64,  0.0000000010000001_f64, f64::EPSILON, 4_u64);
        check_eq(-0.0000000010000001_f64, -0.0000000010000002_f64, f64::EPSILON, 4_u64);
        check_eq(-0.0000000010000002_f64, -0.0000000010000001_f64, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero3() {
        check_eq( 0.0000000011000001_f64,  0.0000000011000002_f64, f64::EPSILON, 4_u64);
        check_eq( 0.0000000011000002_f64,  0.0000000011000001_f64, f64::EPSILON, 4_u64);
        check_eq(-0.0000000011000001_f64, -0.0000000011000002_f64, f64::EPSILON, 4_u64);
        check_eq(-0.0000000011000002_f64, -0.0000000011000001_f64, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero4() {
        check_eq(1e-16_f64, -1e-16_f64, f64::EPSILON, 4_u64);
        check_eq(1e-17_f64, -1e-17_f64, f64::EPSILON, 4_u64);
        check_eq(1e-18_f64, -1e-18_f64, f64::EPSILON, 4_u64);
        check_eq(1e-19_f64, -1e-19_f64, f64::EPSILON, 4_u64);
        check_eq(1e-20_f64, -1e-20_f64, f64::EPSILON, 4_u64);
        check_eq(1e-21_f64, -1e-21_f64, f64::EPSILON, 4_u64);
        check_eq(1e-22_f64, -1e-22_f64, f64::EPSILON, 4_u64);
        check_eq(1e-23_f64, -1e-23_f64, f64::EPSILON, 4_u64);
        check_eq(1e-24_f64, -1e-24_f64, f64::EPSILON, 4_u64);
        check_eq(1e-25_f64, -1e-25_f64, f64::EPSILON, 4_u64);
        check_eq(1e-26_f64, -1e-26_f64, f64::EPSILON, 4_u64);
        check_eq(1e-27_f64, -1e-27_f64, f64::EPSILON, 4_u64);
        check_eq(1e-28_f64, -1e-28_f64, f64::EPSILON, 4_u64);
        check_eq(1e-29_f64, -1e-29_f64, f64::EPSILON, 4_u64);
        check_eq(1e-30_f64, -1e-30_f64, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero1() {
        check_ne( 0.000001_f64,      -f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( 0.000001_f64,       f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE,  0.000001_f64,      f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE,  0.000001_f64,      f64::EPSILON, 4_u64);

        check_ne(-0.000001_f64,      -f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne(-0.000001_f64,       f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE, -0.000001_f64,      f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE, -0.000001_f64,      f64::EPSILON, 4_u64);

        check_ne( 0.0000001_f64,     -f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( 0.0000001_f64,      f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE,  0.0000001_f64,     f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE,  0.0000001_f64,     f64::EPSILON, 4_u64);

        check_ne(-0.0000001_f64,     -f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne(-0.0000001_f64,      f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE, -0.0000001_f64,     f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE, -0.0000001_f64,     f64::EPSILON, 4_u64);

        check_ne( 0.00000001_f64,    -f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( 0.00000001_f64,     f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE,  0.00000001_f64,    f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE,  0.00000001_f64,    f64::EPSILON, 4_u64);

        check_ne(-0.00000001_f64,    -f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne(-0.00000001_f64,     f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE, -0.00000001_f64,    f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE, -0.00000001_f64,    f64::EPSILON, 4_u64);

        check_ne( 0.000000001_f64,   -f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( 0.000000001_f64,    f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE,  0.000000001_f64,   f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE,  0.000000001_f64,   f64::EPSILON, 4_u64);

        check_ne(-0.000000001_f64,   -f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne(-0.000000001_f64,    f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE, -0.000000001_f64,   f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE, -0.000000001_f64,   f64::EPSILON, 4_u64);

        check_ne( 0.0000000001_f64,  -f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( 0.0000000001_f64,   f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE,  0.0000000001_f64,  f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE,  0.0000000001_f64,  f64::EPSILON, 4_u64);

        check_ne(-0.0000000001_f64,  -f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne(-0.0000000001_f64,   f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE, -0.0000000001_f64,  f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE, -0.0000000001_f64,  f64::EPSILON, 4_u64);

        check_ne( 0.00000000001_f64, -f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( 0.00000000001_f64,  f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE,  0.00000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE,  0.00000000001_f64, f64::EPSILON, 4_u64);

        check_ne(-0.00000000001_f64, -f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne(-0.00000000001_f64,  f64::MIN_POSITIVE, f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE, -0.00000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE, -0.00000000001_f64, f64::EPSILON, 4_u64);

        check_ne( 0.000000000001_f64, -f64::MIN_POSITIVE,  f64::EPSILON, 4_u64);
        check_ne( 0.000000000001_f64,  f64::MIN_POSITIVE,  f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE,   0.000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE,   0.000000000001_f64, f64::EPSILON, 4_u64);

        check_ne(-0.000000000001_f64, -f64::MIN_POSITIVE,  f64::EPSILON, 4_u64);
        check_ne(-0.000000000001_f64,  f64::MIN_POSITIVE,  f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE,  -0.000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE,  -0.000000000001_f64, f64::EPSILON, 4_u64);

        check_ne( 0.0000000000001_f64, -f64::MIN_POSITIVE,   f64::EPSILON, 4_u64);
        check_ne( 0.0000000000001_f64,  f64::MIN_POSITIVE,   f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE,    0.0000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE,    0.0000000000001_f64, f64::EPSILON, 4_u64);

        check_ne(-0.0000000000001_f64, -f64::MIN_POSITIVE,   f64::EPSILON, 4_u64);
        check_ne(-0.0000000000001_f64,  f64::MIN_POSITIVE,   f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE,   -0.0000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE,   -0.0000000000001_f64, f64::EPSILON, 4_u64);

        check_ne( 0.00000000000001_f64, -f64::MIN_POSITIVE,    f64::EPSILON, 4_u64);
        check_ne( 0.00000000000001_f64,  f64::MIN_POSITIVE,    f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE,     0.00000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE,     0.00000000000001_f64, f64::EPSILON, 4_u64);

        check_ne(-0.00000000000001_f64, -f64::MIN_POSITIVE,    f64::EPSILON, 4_u64);
        check_ne(-0.00000000000001_f64,  f64::MIN_POSITIVE,    f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE,    -0.00000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE,    -0.00000000000001_f64, f64::EPSILON, 4_u64);

        check_ne( 0.000000000000001_f64, -f64::MIN_POSITIVE,     f64::EPSILON, 4_u64);
        check_ne( 0.000000000000001_f64,  f64::MIN_POSITIVE,     f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE,      0.000000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE,      0.000000000000001_f64, f64::EPSILON, 4_u64);

        check_ne(-0.000000000000001_f64, -f64::MIN_POSITIVE,     f64::EPSILON, 4_u64);
        check_ne(-0.000000000000001_f64,  f64::MIN_POSITIVE,     f64::EPSILON, 4_u64);
        check_ne( f64::MIN_POSITIVE,     -0.000000000000001_f64, f64::EPSILON, 4_u64);
        check_ne(-f64::MIN_POSITIVE,     -0.000000000000001_f64, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero2() {
        check_ne( 0.0000010000002_f64,  0.00000010000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.0000010000001_f64,  0.00000010000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.0000010000002_f64, -0.00000010000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.0000010000001_f64, -0.00000010000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.00000010000002_f64,  0.000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.00000010000001_f64,  0.000000010000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.00000010000002_f64, -0.000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.00000010000001_f64, -0.000000010000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.000000010000002_f64,  0.0000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.000000010000001_f64,  0.0000000010000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.000000010000002_f64, -0.0000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.000000010000001_f64, -0.0000000010000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.0000000010000002_f64,  0.00000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.0000000010000001_f64,  0.00000000010000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.0000000010000002_f64, -0.00000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.0000000010000001_f64, -0.00000000010000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.00000000010000002_f64,  0.000000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.00000000010000001_f64,  0.000000000010000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.00000000010000002_f64, -0.000000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.00000000010000001_f64, -0.000000000010000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.000000000010000002_f64,  0.0000000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.000000000010000001_f64,  0.0000000000010000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.000000000010000002_f64, -0.0000000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.000000000010000001_f64, -0.0000000000010000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.0000000000010000002_f64,  0.00000000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.0000000000010000001_f64,  0.00000000000010000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.0000000000010000002_f64, -0.00000000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.0000000000010000001_f64, -0.00000000000010000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.00000000000010000002_f64,  0.000000000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.00000000000010000001_f64,  0.000000000000010000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.00000000000010000002_f64, -0.000000000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.00000000000010000001_f64, -0.000000000000010000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.000000000000010000002_f64,  0.0000000000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.000000000000010000001_f64,  0.0000000000000010000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.000000000000010000002_f64, -0.0000000000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.000000000000010000001_f64, -0.0000000000000010000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.0000000000000010000002_f64,  0.00000000000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.0000000000000010000001_f64,  0.00000000000000010000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.0000000000000010000002_f64, -0.00000000000000010000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.0000000000000010000001_f64, -0.00000000000000010000002_f64, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero3() {
        check_ne( 0.0000011000002_f64,  0.00000011000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.0000011000001_f64,  0.00000011000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.0000011000002_f64, -0.00000011000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.0000011000001_f64, -0.00000011000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.00000011000002_f64,  0.000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.00000011000001_f64,  0.000000011000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.00000011000002_f64, -0.000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.00000011000001_f64, -0.000000011000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.000000011000002_f64,  0.0000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.000000011000001_f64,  0.0000000011000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.000000011000002_f64, -0.0000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.000000011000001_f64, -0.0000000011000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.0000000011000002_f64,  0.00000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.0000000011000001_f64,  0.00000000011000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.0000000011000002_f64, -0.00000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.0000000011000001_f64, -0.00000000011000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.00000000011000002_f64,  0.000000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.00000000011000001_f64,  0.000000000011000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.00000000011000002_f64, -0.000000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.00000000011000001_f64, -0.000000000011000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.000000000011000002_f64,  0.0000000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.000000000011000001_f64,  0.0000000000011000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.000000000011000002_f64, -0.0000000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.000000000011000001_f64, -0.0000000000011000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.0000000000011000002_f64,  0.00000000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.0000000000011000001_f64,  0.00000000000011000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.0000000000011000002_f64, -0.00000000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.0000000000011000001_f64, -0.00000000000011000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.00000000000011000002_f64,  0.000000000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.00000000000011000001_f64,  0.000000000000011000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.00000000000011000002_f64, -0.000000000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.00000000000011000001_f64, -0.000000000000011000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.000000000000011000002_f64,  0.0000000000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.000000000000011000001_f64,  0.0000000000000011000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.000000000000011000002_f64, -0.0000000000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.000000000000011000001_f64, -0.0000000000000011000002_f64, f64::EPSILON, 4_u64);

        check_ne( 0.0000000000000011000002_f64,  0.00000000000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne( 0.0000000000000011000001_f64,  0.00000000000000011000002_f64, f64::EPSILON, 4_u64);
        check_ne(-0.0000000000000011000002_f64, -0.00000000000000011000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.0000000000000011000001_f64, -0.00000000000000011000002_f64, f64::EPSILON, 4_u64);
    }
}

macro_rules! impl_ulps_eq_float_exact_exhaustive_tests {
    ($(($module_name:ident, $FloatType:ty, $IntegerType:ty)),*) => {$(
        #[cfg(test)]
        mod $module_name {
            use ulps_cmp::{
                UlpsEq,
                assert_ulps_eq,
                assert_ulps_ne,
                ulps_eq,
                ulps_ne,
            };

            #[test]
            fn test_abs_relative_exactly_representable_exhaustive() {
                let max_ulps = 4;
                for i in <$IntegerType>::MIN..<$IntegerType>::MAX {
                    assert!((i as $FloatType).ulps_eq(&(i as $FloatType), &0.0, &max_ulps));
                    assert!(ulps_eq!(i as $FloatType, i as $FloatType, abs_diff <= 0.0, ulps <= max_ulps));
                    assert_ulps_eq!(i as $FloatType, i as $FloatType, abs_diff <= 0.0, ulps <= max_ulps);
                }
            }

            #[test]
            fn test_ulps_ne_exactly_representable_exhaustive1() {
                let max_ulps = 4;
                for i in <$IntegerType>::MIN..<$IntegerType>::MAX {
                    assert!(((i + 1) as $FloatType).ulps_ne(&(i as $FloatType), &0.0, &max_ulps));
                    assert!(ulps_ne!((i + 1) as $FloatType, i as $FloatType, abs_diff <= 0.0, ulps <= max_ulps));
                    assert_ulps_ne!((i + 1) as $FloatType, i as $FloatType, abs_diff <= 0.0, ulps <= max_ulps);

                    assert!((i as $FloatType).ulps_ne(&((i + 1) as $FloatType), &0.0, &max_ulps));
                    assert!(ulps_ne!(i as $FloatType, (i + 1) as $FloatType, abs_diff <= 0.0, ulps <= max_ulps));
                    assert_ulps_ne!(i as $FloatType, (i + 1) as $FloatType, abs_diff <= 0.0, ulps <= max_ulps);
                }
            }
        }
    )*};
}

impl_ulps_eq_float_exact_exhaustive_tests!(
    (ulps_eq_f32_u8_exact_exhaustive_tests, f32, u8),
    (ulps_eq_f32_u16_exact_exhaustive_tests, f32, u16),
    (ulps_eq_f32_i8_exact_exhaustive_tests, f32, i8),
    (ulps_eq_f32_i16_exact_exhaustive_tests, f32, i16),
    (ulps_eq_f64_u8_exact_exhaustive_tests, f64, u8),
    (ulps_eq_f64_u16_exact_exhaustive_tests, f64, u16),
    (ulps_eq_f64_i8_exact_exhaustive_tests, f64, i8),
    (ulps_eq_f64_i16_exact_exhaustive_tests, f64, i16)
);
