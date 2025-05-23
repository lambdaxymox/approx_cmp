#[cfg(test)]
mod ulps_eq_f32_tests {
    use ulps_cmp::{
        UlpsAllEq,
        UlpsEq,
        assert_ulps_eq,
        assert_ulps_ne,
        ulps_eq,
        ulps_ne,
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
    fn test_ne_zero1() {
        let range_u32 = (u32::MAX as u64) + 1_u64;
        let max_ulps = (range_u32 / 2_u64) as u32 - 1_u32;

        check_ne(-0.0_f32,  0.0_f32, -f32::EPSILON, max_ulps);
        check_ne( 0.0_f32, -0.0_f32, -f32::EPSILON, max_ulps);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_zero2() {
        check_ne( 0.000001_f32, 0.0_f32,      f32::EPSILON, 4_u32);
        check_ne( 0.0_f32,      0.000001_f32, f32::EPSILON, 4_u32);
        check_ne(-0.000001_f32, 0.0_f32,      f32::EPSILON, 4_u32);
        check_ne( 0.0_f32,     -0.000001_f32, f32::EPSILON, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_tolerance1() {
        check_eq( 0.0_f32,    1e-40_f32, 1e-40_f32, 4_u32);
        check_eq( 1e-40_f32,  0.0_f32,   1e-40_f32, 4_u32);
        check_eq( 0.0_f32,   -1e-40_f32, 1e-40_f32, 4_u32);
        check_eq(-1e-40_f32,  0.0_f32,   1e-40_f32, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_tolerance2() {
        check_eq( 1e-40_f32,  0.0_f32,   0.0_f32, 71362_u32);
        check_eq( 0.0_f32,    1e-40_f32, 0.0_f32, 71362_u32);
        check_eq(-1e-40_f32, -0.0_f32,   0.0_f32, 71362_u32);
        check_eq(-0.0_f32,   -1e-40_f32, 0.0_f32, 71362_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_tolerance1() {
        check_ne( 1e-40_f32,  0.0_f32,   1e-41_f32, 4_u32);
        check_ne( 0.0_f32,    1e-40_f32, 1e-41_f32, 4_u32);
        check_ne(-1e-40_f32,  0.0_f32,   1e-41_f32, 4_u32);
        check_ne( 0.0_f32,   -1e-40_f32, 1e-41_f32, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_tolerance2() {
        check_ne( 1e-40_f32,  0.0_f32,   0.0_f32, 71361_u32);
        check_ne( 0.0_f32,    1e-40_f32, 0.0_f32, 71361_u32);
        check_ne(-1e-40_f32, -0.0_f32,   0.0_f32, 71361_u32);
        check_ne(-0.0_f32,   -1e-40_f32, 0.0_f32, 71361_u32);
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
    fn test_eq_rounding5() {
        check_eq( 10000000.0_f32,  10000001.0_f32, 0.0_f32, 1_u32);
        check_eq( 10000001.0_f32,  10000000.0_f32, 0.0_f32, 1_u32);
        check_eq(-10000000.0_f32, -10000001.0_f32, 0.0_f32, 1_u32);
        check_eq(-10000001.0_f32, -10000000.0_f32, 0.0_f32, 1_u32);

        check_eq( 100000000.0_f32,  100000001.0_f32, 0.0_f32, 1_u32);
        check_eq( 100000001.0_f32,  100000000.0_f32, 0.0_f32, 1_u32);
        check_eq(-100000000.0_f32, -100000001.0_f32, 0.0_f32, 1_u32);
        check_eq(-100000001.0_f32, -100000000.0_f32, 0.0_f32, 1_u32);

        check_eq( 1000000000.0_f32,  1000000001.0_f32, 0.0_f32, 1_u32);
        check_eq( 1000000001.0_f32,  1000000000.0_f32, 0.0_f32, 1_u32);
        check_eq(-1000000000.0_f32, -1000000001.0_f32, 0.0_f32, 1_u32);
        check_eq(-1000000001.0_f32, -1000000000.0_f32, 0.0_f32, 1_u32);

        check_eq( 10000000000.0_f32,  10000000001.0_f32, 0.0_f32, 1_u32);
        check_eq( 10000000001.0_f32,  10000000000.0_f32, 0.0_f32, 1_u32);
        check_eq(-10000000000.0_f32, -10000000001.0_f32, 0.0_f32, 1_u32);
        check_eq(-10000000001.0_f32, -10000000000.0_f32, 0.0_f32, 1_u32);

        check_eq( 100000000000.0_f32,  100000000001.0_f32, 0.0_f32, 1_u32);
        check_eq( 100000000001.0_f32,  100000000000.0_f32, 0.0_f32, 1_u32);
        check_eq(-100000000000.0_f32, -100000000001.0_f32, 0.0_f32, 1_u32);
        check_eq(-100000000001.0_f32, -100000000000.0_f32, 0.0_f32, 1_u32);

        check_eq( 1000000000000.0_f32,  1000000000001.0_f32, 0.0_f32, 1_u32);
        check_eq( 1000000000001.0_f32,  1000000000000.0_f32, 0.0_f32, 1_u32);
        check_eq(-1000000000000.0_f32, -1000000000001.0_f32, 0.0_f32, 1_u32);
        check_eq(-1000000000001.0_f32, -1000000000000.0_f32, 0.0_f32, 1_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding5() {
        check_ne( 1000.0_f32,  1001.0_f32, 0.0_f32, 4_u32);
        check_ne( 1001.0_f32,  1000.0_f32, 0.0_f32, 4_u32);
        check_ne(-1000.0_f32, -1001.0_f32, 0.0_f32, 4_u32);
        check_ne(-1001.0_f32, -1000.0_f32, 0.0_f32, 4_u32);

        check_ne( 10000.0_f32,  10001.0_f32, 0.0_f32, 4_u32);
        check_ne( 10001.0_f32,  10000.0_f32, 0.0_f32, 4_u32);
        check_ne(-10000.0_f32, -10001.0_f32, 0.0_f32, 4_u32);
        check_ne(-10001.0_f32, -10000.0_f32, 0.0_f32, 4_u32);

        check_ne( 100000.0_f32,  100001.0_f32, 0.0_f32, 4_u32);
        check_ne( 100001.0_f32,  100000.0_f32, 0.0_f32, 4_u32);
        check_ne(-100000.0_f32, -100001.0_f32, 0.0_f32, 4_u32);
        check_ne(-100001.0_f32, -100000.0_f32, 0.0_f32, 4_u32);

        check_ne( 1000000.0_f32,  1000001.0_f32, 0.0_f32, 4_u32);
        check_ne( 1000001.0_f32,  1000000.0_f32, 0.0_f32, 4_u32);
        check_ne(-1000000.0_f32, -1000001.0_f32, 0.0_f32, 4_u32);
        check_ne(-1000001.0_f32, -1000000.0_f32, 0.0_f32, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding6() {
        check_eq( 1000.0_f32,  1001.0_f32, 0.0_f32, 16384_u32);
        check_eq( 1001.0_f32,  1000.0_f32, 0.0_f32, 16384_u32);
        check_eq(-1000.0_f32, -1001.0_f32, 0.0_f32, 16384_u32);
        check_eq(-1001.0_f32, -1000.0_f32, 0.0_f32, 16384_u32);

        check_eq( 10000.0_f32,  10001.0_f32, 0.0_f32, 1024_u32);
        check_eq( 10001.0_f32,  10000.0_f32, 0.0_f32, 1024_u32);
        check_eq(-10000.0_f32, -10001.0_f32, 0.0_f32, 1024_u32);
        check_eq(-10001.0_f32, -10000.0_f32, 0.0_f32, 1024_u32);

        check_eq( 100000.0_f32,  100001.0_f32, 0.0_f32, 128_u32);
        check_eq( 100001.0_f32,  100000.0_f32, 0.0_f32, 128_u32);
        check_eq(-100000.0_f32, -100001.0_f32, 0.0_f32, 128_u32);
        check_eq(-100001.0_f32, -100000.0_f32, 0.0_f32, 128_u32);

        check_eq( 1000000.0_f32,  1000001.0_f32, 0.0_f32, 16_u32);
        check_eq( 1000001.0_f32,  1000000.0_f32, 0.0_f32, 16_u32);
        check_eq(-1000000.0_f32, -1000001.0_f32, 0.0_f32, 16_u32);
        check_eq(-1000001.0_f32, -1000000.0_f32, 0.0_f32, 16_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding6() {
        check_ne( 1000.0_f32,  1001.0_f32, 0.0_f32, 16383_u32);
        check_ne( 1001.0_f32,  1000.0_f32, 0.0_f32, 16383_u32);
        check_ne(-1000.0_f32, -1001.0_f32, 0.0_f32, 16383_u32);
        check_ne(-1001.0_f32, -1000.0_f32, 0.0_f32, 16383_u32);

        check_ne( 10000.0_f32,  10001.0_f32, 0.0_f32, 1023_u32);
        check_ne( 10001.0_f32,  10000.0_f32, 0.0_f32, 1023_u32);
        check_ne(-10000.0_f32, -10001.0_f32, 0.0_f32, 1023_u32);
        check_ne(-10001.0_f32, -10000.0_f32, 0.0_f32, 1023_u32);

        check_ne( 100000.0_f32,  100001.0_f32, 0.0_f32, 127_u32);
        check_ne( 100001.0_f32,  100000.0_f32, 0.0_f32, 127_u32);
        check_ne(-100000.0_f32, -100001.0_f32, 0.0_f32, 127_u32);
        check_ne(-100001.0_f32, -100000.0_f32, 0.0_f32, 127_u32);

        check_ne( 1000000.0_f32,  1000001.0_f32, 0.0_f32, 15_u32);
        check_ne( 1000001.0_f32,  1000000.0_f32, 0.0_f32, 15_u32);
        check_ne(-1000000.0_f32, -1000001.0_f32, 0.0_f32, 15_u32);
        check_ne(-1000001.0_f32, -1000000.0_f32, 0.0_f32, 15_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding7() {
        check_eq( 1.0000001_f32,  1.0000002_f32, 0.0_f32, 1_u32);
        check_eq( 1.0000002_f32,  1.0000001_f32, 0.0_f32, 1_u32);
        check_eq(-1.0000001_f32, -1.0000002_f32, 0.0_f32, 1_u32);
        check_eq(-1.0000002_f32, -1.0000001_f32, 0.0_f32, 1_u32);

        check_eq( 1.00000001_f32,  1.00000002_f32, 0.0_f32, 1_u32);
        check_eq( 1.00000002_f32,  1.00000001_f32, 0.0_f32, 1_u32);
        check_eq(-1.00000001_f32, -1.00000002_f32, 0.0_f32, 1_u32);
        check_eq(-1.00000002_f32, -1.00000001_f32, 0.0_f32, 1_u32);

        check_eq( 1.000000001_f32,  1.000000002_f32, 0.0_f32, 1_u32);
        check_eq( 1.000000002_f32,  1.000000001_f32, 0.0_f32, 1_u32);
        check_eq(-1.000000001_f32, -1.000000002_f32, 0.0_f32, 1_u32);
        check_eq(-1.000000002_f32, -1.000000001_f32, 0.0_f32, 1_u32);

        check_eq( 1.0000000001_f32,  1.0000000002_f32, 0.0_f32, 1_u32);
        check_eq( 1.0000000002_f32,  1.0000000001_f32, 0.0_f32, 1_u32);
        check_eq(-1.0000000001_f32, -1.0000000002_f32, 0.0_f32, 1_u32);
        check_eq(-1.0000000002_f32, -1.0000000001_f32, 0.0_f32, 1_u32);

        check_eq( 1.00000000001_f32,  1.00000000002_f32, 0.0_f32, 1_u32);
        check_eq( 1.00000000002_f32,  1.00000000001_f32, 0.0_f32, 1_u32);
        check_eq(-1.00000000001_f32, -1.00000000002_f32, 0.0_f32, 1_u32);
        check_eq(-1.00000000002_f32, -1.00000000001_f32, 0.0_f32, 1_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding7() {
        check_ne( 1.01_f32,  1.02_f32, 0.0_f32, 4_u32);
        check_ne( 1.02_f32,  1.01_f32, 0.0_f32, 4_u32);
        check_ne(-1.01_f32, -1.02_f32, 0.0_f32, 4_u32);
        check_ne(-1.02_f32, -1.01_f32, 0.0_f32, 4_u32);

        check_ne( 1.001_f32,  1.002_f32, 0.0_f32, 4_u32);
        check_ne( 1.002_f32,  1.001_f32, 0.0_f32, 4_u32);
        check_ne(-1.001_f32, -1.002_f32, 0.0_f32, 4_u32);
        check_ne(-1.002_f32, -1.001_f32, 0.0_f32, 4_u32);

        check_ne( 1.0001_f32,  1.0002_f32, 0.0_f32, 4_u32);
        check_ne( 1.0002_f32,  1.0001_f32, 0.0_f32, 4_u32);
        check_ne(-1.0001_f32, -1.0002_f32, 0.0_f32, 4_u32);
        check_ne(-1.0002_f32, -1.0001_f32, 0.0_f32, 4_u32);

        check_ne( 1.00001_f32,  1.00002_f32, 0.0_f32, 4_u32);
        check_ne( 1.00002_f32,  1.00001_f32, 0.0_f32, 4_u32);
        check_ne(-1.00001_f32, -1.00002_f32, 0.0_f32, 4_u32);
        check_ne(-1.00002_f32, -1.00001_f32, 0.0_f32, 4_u32);

        check_ne( 1.000001_f32,  1.000002_f32, 0.0_f32, 4_u32);
        check_ne( 1.000002_f32,  1.000001_f32, 0.0_f32, 4_u32);
        check_ne(-1.000001_f32, -1.000002_f32, 0.0_f32, 4_u32);
        check_ne(-1.000002_f32, -1.000001_f32, 0.0_f32, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding8() {
        check_eq( 1.01_f32,  1.02_f32, 0.0_f32, 83886_u32);
        check_eq( 1.02_f32,  1.01_f32, 0.0_f32, 83886_u32);
        check_eq(-1.01_f32, -1.02_f32, 0.0_f32, 83886_u32);
        check_eq(-1.02_f32, -1.01_f32, 0.0_f32, 83886_u32);

        check_eq( 1.001_f32,  1.002_f32, 0.0_f32, 8388_u32);
        check_eq( 1.002_f32,  1.001_f32, 0.0_f32, 8388_u32);
        check_eq(-1.001_f32, -1.002_f32, 0.0_f32, 8388_u32);
        check_eq(-1.002_f32, -1.001_f32, 0.0_f32, 8388_u32);

        check_eq( 1.0001_f32,  1.0002_f32, 0.0_f32, 839_u32);
        check_eq( 1.0002_f32,  1.0001_f32, 0.0_f32, 839_u32);
        check_eq(-1.0001_f32, -1.0002_f32, 0.0_f32, 839_u32);
        check_eq(-1.0002_f32, -1.0001_f32, 0.0_f32, 4839u32);

        check_eq( 1.00001_f32,  1.00002_f32, 0.0_f32, 84_u32);
        check_eq( 1.00002_f32,  1.00001_f32, 0.0_f32, 84_u32);
        check_eq(-1.00001_f32, -1.00002_f32, 0.0_f32, 84_u32);
        check_eq(-1.00002_f32, -1.00001_f32, 0.0_f32, 84_u32);

        check_eq( 1.000001_f32,  1.000002_f32, 0.0_f32, 9_u32);
        check_eq( 1.000002_f32,  1.000001_f32, 0.0_f32, 9_u32);
        check_eq(-1.000001_f32, -1.000002_f32, 0.0_f32, 9_u32);
        check_eq(-1.000002_f32, -1.000001_f32, 0.0_f32, 9_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding8() {
        check_ne( 1.01_f32,  1.02_f32, 0.0_f32, 83885_u32);
        check_ne( 1.02_f32,  1.01_f32, 0.0_f32, 83885_u32);
        check_ne(-1.01_f32, -1.02_f32, 0.0_f32, 83885_u32);
        check_ne(-1.02_f32, -1.01_f32, 0.0_f32, 83885_u32);

        check_ne( 1.001_f32,  1.002_f32, 0.0_f32, 8387_u32);
        check_ne( 1.002_f32,  1.001_f32, 0.0_f32, 8387_u32);
        check_ne(-1.001_f32, -1.002_f32, 0.0_f32, 8387_u32);
        check_ne(-1.002_f32, -1.001_f32, 0.0_f32, 8387_u32);

        check_ne( 1.0001_f32,  1.0002_f32, 0.0_f32, 838_u32);
        check_ne( 1.0002_f32,  1.0001_f32, 0.0_f32, 838_u32);
        check_ne(-1.0001_f32, -1.0002_f32, 0.0_f32, 838_u32);
        check_ne(-1.0002_f32, -1.0001_f32, 0.0_f32, 838_u32);

        check_ne( 1.00001_f32,  1.00002_f32, 0.0_f32, 83_u32);
        check_ne( 1.00002_f32,  1.00001_f32, 0.0_f32, 83_u32);
        check_ne(-1.00001_f32, -1.00002_f32, 0.0_f32, 83_u32);
        check_ne(-1.00002_f32, -1.00001_f32, 0.0_f32, 83_u32);

        check_ne( 1.000001_f32,  1.000002_f32, 0.0_f32, 8_u32);
        check_ne( 1.000002_f32,  1.000001_f32, 0.0_f32, 8_u32);
        check_ne(-1.000001_f32, -1.000002_f32, 0.0_f32, 8_u32);
        check_ne(-1.000002_f32, -1.000001_f32, 0.0_f32, 8_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding9() {
        check_eq( 0.99999999_f32,  1.00000001_f32, 0.0_f32, 1_u32);
        check_eq( 1.00000001_f32,  0.99999999_f32, 0.0_f32, 1_u32);
        check_eq(-0.99999999_f32, -1.00000001_f32, 0.0_f32, 1_u32);
        check_eq(-1.00000001_f32, -0.99999999_f32, 0.0_f32, 1_u32);

        check_eq( 0.999999999_f32,  1.000000001_f32, 0.0_f32, 1_u32);
        check_eq( 1.000000001_f32,  0.999999999_f32, 0.0_f32, 1_u32);
        check_eq(-0.999999999_f32, -1.000000001_f32, 0.0_f32, 1_u32);
        check_eq(-1.000000001_f32, -0.999999999_f32, 0.0_f32, 1_u32);

        check_eq( 0.9999999999_f32,  1.0000000001_f32, 0.0_f32, 1_u32);
        check_eq( 1.0000000001_f32,  0.9999999999_f32, 0.0_f32, 1_u32);
        check_eq(-0.9999999999_f32, -1.0000000001_f32, 0.0_f32, 1_u32);
        check_eq(-1.0000000001_f32, -0.9999999999_f32, 0.0_f32, 1_u32);

        check_eq( 0.99999999999_f32,  1.00000000001_f32, 0.0_f32, 1_u32);
        check_eq( 1.00000000001_f32,  0.99999999999_f32, 0.0_f32, 1_u32);
        check_eq(-0.99999999999_f32, -1.00000000001_f32, 0.0_f32, 1_u32);
        check_eq(-1.00000000001_f32, -0.99999999999_f32, 0.0_f32, 1_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding9() {
        check_ne( 0.99_f32,  1.01_f32, 0.0_f32, 2_u32);
        check_ne( 1.01_f32,  0.99_f32, 0.0_f32, 2_u32);
        check_ne(-0.99_f32, -1.01_f32, 0.0_f32, 2_u32);
        check_ne(-1.01_f32, -0.99_f32, 0.0_f32, 2_u32);

        check_ne( 0.999_f32,  1.001_f32, 0.0_f32, 2_u32);
        check_ne( 1.001_f32,  0.999_f32, 0.0_f32, 2_u32);
        check_ne(-0.999_f32, -1.001_f32, 0.0_f32, 2_u32);
        check_ne(-1.001_f32, -0.999_f32, 0.0_f32, 2_u32);

        check_ne( 0.9999_f32,  1.0001_f32, 0.0_f32, 2_u32);
        check_ne( 1.0001_f32,  0.9999_f32, 0.0_f32, 2_u32);
        check_ne(-0.9999_f32, -1.0001_f32, 0.0_f32, 2_u32);
        check_ne(-1.0001_f32, -0.9999_f32, 0.0_f32, 2_u32);

        check_ne( 0.99999_f32,  1.00001_f32, 0.0_f32, 2_u32);
        check_ne( 1.00001_f32,  0.99999_f32, 0.0_f32, 2_u32);
        check_ne(-0.99999_f32, -1.00001_f32, 0.0_f32, 2_u32);
        check_ne(-1.00001_f32, -0.99999_f32, 0.0_f32, 2_u32);

        check_ne( 0.999999_f32,  1.000001_f32, 0.0_f32, 2_u32);
        check_ne( 1.000001_f32,  0.999999_f32, 0.0_f32, 2_u32);
        check_ne(-0.999999_f32, -1.000001_f32, 0.0_f32, 2_u32);
        check_ne(-1.000001_f32, -0.999999_f32, 0.0_f32, 2_u32);

        check_ne( 0.9999999_f32,  1.0000001_f32, 0.0_f32, 2_u32);
        check_ne( 1.0000001_f32,  0.9999999_f32, 0.0_f32, 2_u32);
        check_ne(-0.9999999_f32, -1.0000001_f32, 0.0_f32, 2_u32);
        check_ne(-1.0000001_f32, -0.9999999_f32, 0.0_f32, 2_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding10() {
        check_eq( 0.99_f32,  1.01_f32, 0.0_f32, 251658_u32);
        check_eq( 1.01_f32,  0.99_f32, 0.0_f32, 251658_u32);
        check_eq(-0.99_f32, -1.01_f32, 0.0_f32, 251658_u32);
        check_eq(-1.01_f32, -0.99_f32, 0.0_f32, 251658_u32);

        check_eq( 0.999_f32,  1.001_f32, 0.0_f32, 25166_u32);
        check_eq( 1.001_f32,  0.999_f32, 0.0_f32, 25166_u32);
        check_eq(-0.999_f32, -1.001_f32, 0.0_f32, 25166_u32);
        check_eq(-1.001_f32, -0.999_f32, 0.0_f32, 25166_u32);

        check_eq( 0.9999_f32,  1.0001_f32, 0.0_f32, 2517_u32);
        check_eq( 1.0001_f32,  0.9999_f32, 0.0_f32, 2517_u32);
        check_eq(-0.9999_f32, -1.0001_f32, 0.0_f32, 2517_u32);
        check_eq(-1.0001_f32, -0.9999_f32, 0.0_f32, 2517_u32);

        check_eq( 0.99999_f32,  1.00001_f32, 0.0_f32, 252_u32);
        check_eq( 1.00001_f32,  0.99999_f32, 0.0_f32, 252_u32);
        check_eq(-0.99999_f32, -1.00001_f32, 0.0_f32, 252_u32);
        check_eq(-1.00001_f32, -0.99999_f32, 0.0_f32, 252_u32);

        check_eq( 0.999999_f32,  1.000001_f32, 0.0_f32, 25_u32);
        check_eq( 1.000001_f32,  0.999999_f32, 0.0_f32, 25_u32);
        check_eq(-0.999999_f32, -1.000001_f32, 0.0_f32, 25_u32);
        check_eq(-1.000001_f32, -0.999999_f32, 0.0_f32, 25_u32);

        check_eq( 0.9999999_f32,  1.0000001_f32, 0.0_f32, 3_u32);
        check_eq( 1.0000001_f32,  0.9999999_f32, 0.0_f32, 3_u32);
        check_eq(-0.9999999_f32, -1.0000001_f32, 0.0_f32, 3_u32);
        check_eq(-1.0000001_f32, -0.9999999_f32, 0.0_f32, 3_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding10() {
        check_ne( 0.99_f32,  1.01_f32, 0.0_f32, 251657_u32);
        check_ne( 1.01_f32,  0.99_f32, 0.0_f32, 251657_u32);
        check_ne(-0.99_f32, -1.01_f32, 0.0_f32, 251657_u32);
        check_ne(-1.01_f32, -0.99_f32, 0.0_f32, 251657_u32);

        check_ne( 0.999_f32,  1.001_f32, 0.0_f32, 25165_u32);
        check_ne( 1.001_f32,  0.999_f32, 0.0_f32, 25165_u32);
        check_ne(-0.999_f32, -1.001_f32, 0.0_f32, 25165_u32);
        check_ne(-1.001_f32, -0.999_f32, 0.0_f32, 25165_u32);

        check_ne( 0.9999_f32,  1.0001_f32, 0.0_f32, 2516_u32);
        check_ne( 1.0001_f32,  0.9999_f32, 0.0_f32, 2516_u32);
        check_ne(-0.9999_f32, -1.0001_f32, 0.0_f32, 2516_u32);
        check_ne(-1.0001_f32, -0.9999_f32, 0.0_f32, 2516_u32);

        check_ne( 0.99999_f32,  1.00001_f32, 0.0_f32, 251_u32);
        check_ne( 1.00001_f32,  0.99999_f32, 0.0_f32, 251_u32);
        check_ne(-0.99999_f32, -1.00001_f32, 0.0_f32, 251_u32);
        check_ne(-1.00001_f32, -0.99999_f32, 0.0_f32, 251_u32);

        check_ne( 0.999999_f32,  1.000001_f32, 0.0_f32, 24_u32);
        check_ne( 1.000001_f32,  0.999999_f32, 0.0_f32, 24_u32);
        check_ne(-0.999999_f32, -1.000001_f32, 0.0_f32, 24_u32);
        check_ne(-1.000001_f32, -0.999999_f32, 0.0_f32, 24_u32);

        check_ne( 0.9999999_f32,  1.0000001_f32, 0.0_f32, 2_u32);
        check_ne( 1.0000001_f32,  0.9999999_f32, 0.0_f32, 2_u32);
        check_ne(-0.9999999_f32, -1.0000001_f32, 0.0_f32, 2_u32);
        check_ne(-1.0000001_f32, -0.9999999_f32, 0.0_f32, 2_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding11() {
        check_eq( 1.9999999_f32,  2.0000001_f32, 0.0_f32, 1_u32);
        check_eq( 2.0000001_f32,  1.9999999_f32, 0.0_f32, 1_u32);
        check_eq(-1.9999999_f32, -2.0000001_f32, 0.0_f32, 1_u32);
        check_eq(-2.0000001_f32, -1.9999999_f32, 0.0_f32, 1_u32);

        check_eq( 1.99999999_f32,  2.00000001_f32, 0.0_f32, 1_u32);
        check_eq( 2.00000001_f32,  1.99999999_f32, 0.0_f32, 1_u32);
        check_eq(-1.99999999_f32, -2.00000001_f32, 0.0_f32, 1_u32);
        check_eq(-2.00000001_f32, -1.99999999_f32, 0.0_f32, 1_u32);

        check_eq( 1.999999999_f32,  2.000000001_f32, 0.0_f32, 1_u32);
        check_eq( 2.000000001_f32,  1.999999999_f32, 0.0_f32, 1_u32);
        check_eq(-1.999999999_f32, -2.000000001_f32, 0.0_f32, 1_u32);
        check_eq(-2.000000001_f32, -1.999999999_f32, 0.0_f32, 1_u32);

        check_eq( 1.9999999999_f32,  2.0000000001_f32, 0.0_f32, 1_u32);
        check_eq( 2.0000000001_f32,  1.9999999999_f32, 0.0_f32, 1_u32);
        check_eq(-1.9999999999_f32, -2.0000000001_f32, 0.0_f32, 1_u32);
        check_eq(-2.0000000001_f32, -1.9999999999_f32, 0.0_f32, 1_u32);

        check_eq( 1.99999999999_f32,  2.00000000001_f32, 0.0_f32, 1_u32);
        check_eq( 2.00000000001_f32,  1.99999999999_f32, 0.0_f32, 1_u32);
        check_eq(-1.99999999999_f32, -2.00000000001_f32, 0.0_f32, 1_u32);
        check_eq(-2.00000000001_f32, -1.99999999999_f32, 0.0_f32, 1_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding11() {
        check_ne( 1.99_f32,  2.01_f32, 0.0_f32, 4_u32);
        check_ne( 2.01_f32,  1.99_f32, 0.0_f32, 4_u32);
        check_ne(-1.99_f32, -2.01_f32, 0.0_f32, 4_u32);
        check_ne(-2.01_f32, -1.99_f32, 0.0_f32, 4_u32);

        check_ne( 1.999_f32,  2.001_f32, 0.0_f32, 4_u32);
        check_ne( 2.001_f32,  1.999_f32, 0.0_f32, 4_u32);
        check_ne(-1.999_f32, -2.001_f32, 0.0_f32, 4_u32);
        check_ne(-2.001_f32, -1.999_f32, 0.0_f32, 4_u32);

        check_ne( 1.9999_f32,  2.0001_f32, 0.0_f32, 4_u32);
        check_ne( 2.0001_f32,  1.9999_f32, 0.0_f32, 4_u32);
        check_ne(-1.9999_f32, -2.0001_f32, 0.0_f32, 4_u32);
        check_ne(-2.0001_f32, -1.9999_f32, 0.0_f32, 4_u32);

        check_ne( 1.99999_f32,  2.00001_f32, 0.0_f32, 4_u32);
        check_ne( 2.00001_f32,  1.99999_f32, 0.0_f32, 4_u32);
        check_ne(-1.99999_f32, -2.00001_f32, 0.0_f32, 4_u32);
        check_ne(-2.00001_f32, -1.99999_f32, 0.0_f32, 4_u32);

        check_ne( 1.999999_f32,  2.000001_f32, 0.0_f32, 4_u32);
        check_ne( 2.000001_f32,  1.999999_f32, 0.0_f32, 4_u32);
        check_ne(-1.999999_f32, -2.000001_f32, 0.0_f32, 4_u32);
        check_ne(-2.000001_f32, -1.999999_f32, 0.0_f32, 4_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding12() {
        check_eq( 1.99_f32,  2.01_f32, 0.0_f32, 125829_u32);
        check_eq( 2.01_f32,  1.99_f32, 0.0_f32, 125829_u32);
        check_eq(-1.99_f32, -2.01_f32, 0.0_f32, 125829_u32);
        check_eq(-2.01_f32, -1.99_f32, 0.0_f32, 125829_u32);

        check_eq( 1.999_f32,  2.001_f32, 0.0_f32, 12583_u32);
        check_eq( 2.001_f32,  1.999_f32, 0.0_f32, 12583_u32);
        check_eq(-1.999_f32, -2.001_f32, 0.0_f32, 12583_u32);
        check_eq(-2.001_f32, -1.999_f32, 0.0_f32, 12583_u32);

        check_eq( 1.9999_f32,  2.0001_f32, 0.0_f32, 1258_u32);
        check_eq( 2.0001_f32,  1.9999_f32, 0.0_f32, 1258_u32);
        check_eq(-1.9999_f32, -2.0001_f32, 0.0_f32, 1258_u32);
        check_eq(-2.0001_f32, -1.9999_f32, 0.0_f32, 1258_u32);

        check_eq( 1.99999_f32,  2.00001_f32, 0.0_f32, 126_u32);
        check_eq( 2.00001_f32,  1.99999_f32, 0.0_f32, 126_u32);
        check_eq(-1.99999_f32, -2.00001_f32, 0.0_f32, 126_u32);
        check_eq(-2.00001_f32, -1.99999_f32, 0.0_f32, 126_u32);

        check_eq( 1.999999_f32,  2.000001_f32, 0.0_f32, 12_u32);
        check_eq( 2.000001_f32,  1.999999_f32, 0.0_f32, 12_u32);
        check_eq(-1.999999_f32, -2.000001_f32, 0.0_f32, 12_u32);
        check_eq(-2.000001_f32, -1.999999_f32, 0.0_f32, 12_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding12() {
        check_ne( 1.99_f32,  2.01_f32, 0.0_f32, 125828_u32);
        check_ne( 2.01_f32,  1.99_f32, 0.0_f32, 125828_u32);
        check_ne(-1.99_f32, -2.01_f32, 0.0_f32, 125828_u32);
        check_ne(-2.01_f32, -1.99_f32, 0.0_f32, 125828_u32);

        check_ne( 1.999_f32,  2.001_f32, 0.0_f32, 12582_u32);
        check_ne( 2.001_f32,  1.999_f32, 0.0_f32, 12582_u32);
        check_ne(-1.999_f32, -2.001_f32, 0.0_f32, 12582_u32);
        check_ne(-2.001_f32, -1.999_f32, 0.0_f32, 12582_u32);

        check_ne( 1.9999_f32,  2.0001_f32, 0.0_f32, 1257_u32);
        check_ne( 2.0001_f32,  1.9999_f32, 0.0_f32, 1257_u32);
        check_ne(-1.9999_f32, -2.0001_f32, 0.0_f32, 1257_u32);
        check_ne(-2.0001_f32, -1.9999_f32, 0.0_f32, 1257_u32);

        check_ne( 1.99999_f32,  2.00001_f32, 0.0_f32, 125_u32);
        check_ne( 2.00001_f32,  1.99999_f32, 0.0_f32, 125_u32);
        check_ne(-1.99999_f32, -2.00001_f32, 0.0_f32, 125_u32);
        check_ne(-2.00001_f32, -1.99999_f32, 0.0_f32, 125_u32);

        check_ne( 1.999999_f32,  2.000001_f32, 0.0_f32, 11_u32);
        check_ne( 2.000001_f32,  1.999999_f32, 0.0_f32, 11_u32);
        check_ne(-1.999999_f32, -2.000001_f32, 0.0_f32, 11_u32);
        check_ne(-2.000001_f32, -1.999999_f32, 0.0_f32, 11_u32);
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
    fn test_eq_near_zero5() {
        check_eq( f32::MIN_POSITIVE,  0.0_f32,           0.0_f32, 8388608_u32);
        check_eq( 0.0_f32,            f32::MIN_POSITIVE, 0.0_f32, 8388608_u32);
        check_eq(-f32::MIN_POSITIVE, -0.0_f32,           0.0_f32, 8388608_u32);
        check_eq(-0.0_f32,           -f32::MIN_POSITIVE, 0.0_f32, 8388608_u32);

        check_eq( 0.0000001_f32,      f32::MIN_POSITIVE, 0.0_f32, 861323157_u32);
        check_eq( f32::MIN_POSITIVE,  0.0000001_f32,     0.0_f32, 861323157_u32);
        check_eq( -0.0000001_f32,    -f32::MIN_POSITIVE, 0.0_f32, 861323157_u32);
        check_eq(-f32::MIN_POSITIVE, -0.0000001_f32,     0.0_f32, 861323157_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero6() {
        check_eq( 0.000010001_f32,  0.000010002_f32, 0.0_f32, 1099_u32);
        check_eq( 0.000010002_f32,  0.000010001_f32, 0.0_f32, 1099_u32);
        check_eq(-0.000010001_f32, -0.000010002_f32, 0.0_f32, 1099_u32);
        check_eq(-0.000010002_f32, -0.000010001_f32, 0.0_f32, 1099_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero7() {
        check_eq( 0.000011001_f32,  0.000011002_f32, 0.0_f32, 1100_u32);
        check_eq( 0.000011002_f32,  0.000011001_f32, 0.0_f32, 1100_u32);
        check_eq(-0.000011001_f32, -0.000011002_f32, 0.0_f32, 1100_u32);
        check_eq(-0.000011002_f32, -0.000011001_f32, 0.0_f32, 1100_u32);
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

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero4() {
        check_ne(1e-8_f32,  -1e-8_f32,  -f32::EPSILON, 2147483647_u32);
        check_ne(1e-9_f32,  -1e-9_f32,  -f32::EPSILON, 2147483647_u32);
        check_ne(1e-10_f32, -1e-10_f32, -f32::EPSILON, 2147483647_u32);
        check_ne(1e-11_f32, -1e-11_f32, -f32::EPSILON, 2147483647_u32);
        check_ne(1e-12_f32, -1e-12_f32, -f32::EPSILON, 2147483647_u32);
        check_ne(1e-13_f32, -1e-13_f32, -f32::EPSILON, 2147483647_u32);
        check_ne(1e-14_f32, -1e-14_f32, -f32::EPSILON, 2147483647_u32);
        check_ne(1e-15_f32, -1e-15_f32, -f32::EPSILON, 2147483647_u32);
        check_ne(1e-16_f32, -1e-16_f32, -f32::EPSILON, 2147483647_u32);
        check_ne(1e-17_f32, -1e-17_f32, -f32::EPSILON, 2147483647_u32);
        check_ne(1e-18_f32, -1e-18_f32, -f32::EPSILON, 2147483647_u32);
        check_ne(1e-19_f32, -1e-19_f32, -f32::EPSILON, 2147483647_u32);
        check_ne(1e-20_f32, -1e-20_f32, -f32::EPSILON, 2147483647_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero5() {
        check_ne( f32::MIN_POSITIVE,  0.0_f32,           0.0_f32, 8388607_u32);
        check_ne( 0.0_f32,            f32::MIN_POSITIVE, 0.0_f32, 8388607_u32);
        check_ne(-f32::MIN_POSITIVE, -0.0_f32,           0.0_f32, 8388607_u32);
        check_ne(-0.0_f32,           -f32::MIN_POSITIVE, 0.0_f32, 8388607_u32);

        check_ne( 0.0000001_f32,      f32::MIN_POSITIVE, 0.0_f32, 861323156_u32);
        check_ne( f32::MIN_POSITIVE,  0.0000001_f32,     0.0_f32, 861323156_u32);
        check_ne( -0.0000001_f32,    -f32::MIN_POSITIVE, 0.0_f32, 861323156_u32);
        check_ne(-f32::MIN_POSITIVE, -0.0000001_f32,     0.0_f32, 861323156_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero6() {
        check_ne( 0.000010001_f32,  0.000010002_f32, 0.0_f32, 1098_u32);
        check_ne( 0.000010002_f32,  0.000010001_f32, 0.0_f32, 1098_u32);
        check_ne(-0.000010001_f32, -0.000010002_f32, 0.0_f32, 1098_u32);
        check_ne(-0.000010002_f32, -0.000010001_f32, 0.0_f32, 1098_u32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero7() {
        check_ne( 0.000011001_f32,  0.000011002_f32, 0.0_f32, 1099_u32);
        check_ne( 0.000011002_f32,  0.000011001_f32, 0.0_f32, 1099_u32);
        check_ne(-0.000011001_f32, -0.000011002_f32, 0.0_f32, 1099_u32);
        check_ne(-0.000011002_f32, -0.000011001_f32, 0.0_f32, 1099_u32);
    }
}

#[cfg(test)]
mod ulps_eq_f64_tests {
    use ulps_cmp::{
        UlpsAllEq,
        UlpsEq,
        assert_ulps_eq,
        assert_ulps_ne,
        ulps_eq,
        ulps_ne,
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

        check_eq( 0.0_f64,  0.0_f64, f64::EPSILON, 4_u64);
        check_eq(-0.0_f64,  0.0_f64, f64::EPSILON, 4_u64);
        check_eq( 0.0_f64, -0.0_f64, f64::EPSILON, 4_u64);
        check_eq(-0.0_f64, -0.0_f64, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_zero1() {
        let range_u32 = (u64::MAX as u128) + 1_u128;
        let max_ulps = (range_u32 / 2_u128) as u64 - 1_u64;

        check_ne(-0.0_f64,  0.0_f64, -f64::EPSILON, max_ulps);
        check_ne( 0.0_f64, -0.0_f64, -f64::EPSILON, max_ulps);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_zero2() {
        check_ne( 0.000001_f64, 0.0_f64,      f64::EPSILON, 4_u64);
        check_ne( 0.0_f64,      0.000001_f64, f64::EPSILON, 4_u64);
        check_ne(-0.000001_f64, 0.0_f64,      f64::EPSILON, 4_u64);
        check_ne( 0.0_f64,     -0.000001_f64, f64::EPSILON, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_tolerance1() {
        check_eq( 0.0_f64,    1e-40_f64, 1e-40_f64, 4_u64);
        check_eq( 1e-40_f64,  0.0_f64,   1e-40_f64, 4_u64);
        check_eq( 0.0_f64,   -1e-40_f64, 1e-40_f64, 4_u64);
        check_eq(-1e-40_f64,  0.0_f64,   1e-40_f64, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_tolerance2() {
        check_eq( 0.0_f64,    1e-40_f64, 0.0_f64, 4008604054463141788_u64);
        check_eq( 1e-40_f64,  0.0_f64,   0.0_f64, 4008604054463141788_u64);
        check_eq(-0.0_f64,   -1e-40_f64, 0.0_f64, 4008604054463141788_u64);
        check_eq(-1e-40_f64, -0.0_f64,   0.0_f64, 4008604054463141788_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_tolerance1() {
        check_ne( 1e-40_f64,  0.0_f64,   1e-41_f64, 4_u64);
        check_ne( 0.0_f64,    1e-40_f64, 1e-41_f64, 4_u64);
        check_ne(-1e-40_f64,  0.0_f64,   1e-41_f64, 4_u64);
        check_ne( 0.0_f64,   -1e-40_f64, 1e-41_f64, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_tolerance2() {
        check_ne( 0.0_f64,    1e-40_f64, 0.0_f64, 4008604054463141787_u64);
        check_ne( 1e-40_f64,  0.0_f64,   0.0_f64, 4008604054463141787_u64);
        check_ne(-0.0_f64,   -1e-40_f64, 0.0_f64, 4008604054463141787_u64);
        check_ne(-1e-40_f64, -0.0_f64,   0.0_f64, 4008604054463141787_u64);
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
    fn test_eq_rounding5() {
        check_eq( 10000000000000000.0_f64,  10000000000000001.0_f64, 0.0_f64, 1_u64);
        check_eq( 10000000000000001.0_f64,  10000000000000000.0_f64, 0.0_f64, 1_u64);
        check_eq(-10000000000000000.0_f64, -10000000000000001.0_f64, 0.0_f64, 1_u64);
        check_eq(-10000000000000001.0_f64, -10000000000000000.0_f64, 0.0_f64, 1_u64);

        check_eq( 100000000000000000.0_f64,  100000000000000001.0_f64, 0.0_f64, 1_u64);
        check_eq( 100000000000000001.0_f64,  100000000000000000.0_f64, 0.0_f64, 1_u64);
        check_eq(-100000000000000000.0_f64, -100000000000000001.0_f64, 0.0_f64, 1_u64);
        check_eq(-100000000000000001.0_f64, -100000000000000000.0_f64, 0.0_f64, 1_u64);

        check_eq( 1000000000000000000.0_f64,  1000000000000000001.0_f64, 0.0_f64, 1_u64);
        check_eq( 1000000000000000001.0_f64,  1000000000000000000.0_f64, 0.0_f64, 1_u64);
        check_eq(-1000000000000000000.0_f64, -1000000000000000001.0_f64, 0.0_f64, 1_u64);
        check_eq(-1000000000000000001.0_f64, -1000000000000000000.0_f64, 0.0_f64, 1_u64);

        check_eq( 10000000000000000000.0_f64,  10000000000000000001.0_f64, 0.0_f64, 1_u64);
        check_eq( 10000000000000000001.0_f64,  10000000000000000000.0_f64, 0.0_f64, 1_u64);
        check_eq(-10000000000000000000.0_f64, -10000000000000000001.0_f64, 0.0_f64, 1_u64);
        check_eq(-10000000000000000001.0_f64, -10000000000000000000.0_f64, 0.0_f64, 1_u64);

        check_eq( 100000000000000000000.0_f64,  100000000000000000001.0_f64, 0.0_f64, 1_u64);
        check_eq( 100000000000000000001.0_f64,  100000000000000000000.0_f64, 0.0_f64, 1_u64);
        check_eq(-100000000000000000000.0_f64, -100000000000000000001.0_f64, 0.0_f64, 1_u64);
        check_eq(-100000000000000000001.0_f64, -100000000000000000000.0_f64, 0.0_f64, 1_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding5() {
        check_ne( 1000.0_f64,  1001.0_f64, 0.0_f64, 4_u64);
        check_ne( 1001.0_f64,  1000.0_f64, 0.0_f64, 4_u64);
        check_ne(-1000.0_f64, -1001.0_f64, 0.0_f64, 4_u64);
        check_ne(-1001.0_f64, -1000.0_f64, 0.0_f64, 4_u64);

        check_ne( 10000.0_f64,  10001.0_f64, 0.0_f64, 4_u64);
        check_ne( 10001.0_f64,  10000.0_f64, 0.0_f64, 4_u64);
        check_ne(-10000.0_f64, -10001.0_f64, 0.0_f64, 4_u64);
        check_ne(-10001.0_f64, -10000.0_f64, 0.0_f64, 4_u64);

        check_ne( 100000.0_f64,  100001.0_f64, 0.0_f64, 4_u64);
        check_ne( 100001.0_f64,  100000.0_f64, 0.0_f64, 4_u64);
        check_ne(-100000.0_f64, -100001.0_f64, 0.0_f64, 4_u64);
        check_ne(-100001.0_f64, -100000.0_f64, 0.0_f64, 4_u64);

        check_ne( 1000000.0_f64,  1000001.0_f64, 0.0_f64, 4_u64);
        check_ne( 1000001.0_f64,  1000000.0_f64, 0.0_f64, 4_u64);
        check_ne(-1000000.0_f64, -1000001.0_f64, 0.0_f64, 4_u64);
        check_ne(-1000001.0_f64, -1000000.0_f64, 0.0_f64, 4_u64);

        check_ne( 10000000.0_f64,  10000001.0_f64, 0.0_f64, 4_u64);
        check_ne( 10000001.0_f64,  10000000.0_f64, 0.0_f64, 4_u64);
        check_ne(-10000000.0_f64, -10000001.0_f64, 0.0_f64, 4_u64);
        check_ne(-10000001.0_f64, -10000000.0_f64, 0.0_f64, 4_u64);

        check_ne( 100000000.0_f64,  100000001.0_f64, 0.0_f64, 4_u64);
        check_ne( 100000001.0_f64,  100000000.0_f64, 0.0_f64, 4_u64);
        check_ne(-100000000.0_f64, -100000001.0_f64, 0.0_f64, 4_u64);
        check_ne(-100000001.0_f64, -100000000.0_f64, 0.0_f64, 4_u64);

        check_ne( 1000000000.0_f64,  1000000001.0_f64, 0.0_f64, 4_u64);
        check_ne( 1000000001.0_f64,  1000000000.0_f64, 0.0_f64, 4_u64);
        check_ne(-1000000000.0_f64, -1000000001.0_f64, 0.0_f64, 4_u64);
        check_ne(-1000000001.0_f64, -1000000000.0_f64, 0.0_f64, 4_u64);

        check_ne( 10000000000.0_f64,  10000000001.0_f64, 0.0_f64, 4_u64);
        check_ne( 10000000001.0_f64,  10000000000.0_f64, 0.0_f64, 4_u64);
        check_ne(-10000000000.0_f64, -10000000001.0_f64, 0.0_f64, 4_u64);
        check_ne(-10000000001.0_f64, -10000000000.0_f64, 0.0_f64, 4_u64);

        check_ne( 100000000000.0_f64,  100000000001.0_f64, 0.0_f64, 4_u64);
        check_ne( 100000000001.0_f64,  100000000000.0_f64, 0.0_f64, 4_u64);
        check_ne(-100000000000.0_f64, -100000000001.0_f64, 0.0_f64, 4_u64);
        check_ne(-100000000001.0_f64, -100000000000.0_f64, 0.0_f64, 4_u64);

        check_ne( 1000000000000.0_f64,  1000000000001.0_f64, 0.0_f64, 4_u64);
        check_ne( 1000000000001.0_f64,  1000000000000.0_f64, 0.0_f64, 4_u64);
        check_ne(-1000000000000.0_f64, -1000000000001.0_f64, 0.0_f64, 4_u64);
        check_ne(-1000000000001.0_f64, -1000000000000.0_f64, 0.0_f64, 4_u64);

        check_ne( 10000000000000.0_f64,  10000000000001.0_f64, 0.0_f64, 4_u64);
        check_ne( 10000000000001.0_f64,  10000000000000.0_f64, 0.0_f64, 4_u64);
        check_ne(-10000000000000.0_f64, -10000000000001.0_f64, 0.0_f64, 4_u64);
        check_ne(-10000000000001.0_f64, -10000000000000.0_f64, 0.0_f64, 4_u64);

        check_ne( 100000000000000.0_f64,  100000000000001.0_f64, 0.0_f64, 4_u64);
        check_ne( 100000000000001.0_f64,  100000000000000.0_f64, 0.0_f64, 4_u64);
        check_ne(-100000000000000.0_f64, -100000000000001.0_f64, 0.0_f64, 4_u64);
        check_ne(-100000000000001.0_f64, -100000000000000.0_f64, 0.0_f64, 4_u64);

        check_ne( 1000000000000000.0_f64,  1000000000000001.0_f64, 0.0_f64, 4_u64);
        check_ne( 1000000000000001.0_f64,  1000000000000000.0_f64, 0.0_f64, 4_u64);
        check_ne(-1000000000000000.0_f64, -1000000000000001.0_f64, 0.0_f64, 4_u64);
        check_ne(-1000000000000001.0_f64, -1000000000000000.0_f64, 0.0_f64, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding6() {
        check_eq( 1000.0_f64,  1001.0_f64, 0.0_f64, 8796093022208_u64);
        check_eq( 1001.0_f64,  1000.0_f64, 0.0_f64, 8796093022208_u64);
        check_eq(-1000.0_f64, -1001.0_f64, 0.0_f64, 8796093022208_u64);
        check_eq(-1001.0_f64, -1000.0_f64, 0.0_f64, 8796093022208_u64);

        check_eq( 10000.0_f64,  10001.0_f64, 0.0_f64, 549755813888_u64);
        check_eq( 10001.0_f64,  10000.0_f64, 0.0_f64, 549755813888_u64);
        check_eq(-10000.0_f64, -10001.0_f64, 0.0_f64, 549755813888_u64);
        check_eq(-10001.0_f64, -10000.0_f64, 0.0_f64, 549755813888_u64);

        check_eq( 100000.0_f64,  100001.0_f64, 0.0_f64, 68719476736_u64);
        check_eq( 100001.0_f64,  100000.0_f64, 0.0_f64, 68719476736_u64);
        check_eq(-100000.0_f64, -100001.0_f64, 0.0_f64, 68719476736_u64);
        check_eq(-100001.0_f64, -100000.0_f64, 0.0_f64, 68719476736_u64);

        check_eq( 1000000.0_f64,  1000001.0_f64, 0.0_f64, 8589934592_u64);
        check_eq( 1000001.0_f64,  1000000.0_f64, 0.0_f64, 8589934592_u64);
        check_eq(-1000000.0_f64, -1000001.0_f64, 0.0_f64, 8589934592_u64);
        check_eq(-1000001.0_f64, -1000000.0_f64, 0.0_f64, 8589934592_u64);

        check_eq( 10000000.0_f64,  10000001.0_f64, 0.0_f64, 536870912_u64);
        check_eq( 10000001.0_f64,  10000000.0_f64, 0.0_f64, 536870912_u64);
        check_eq(-10000000.0_f64, -10000001.0_f64, 0.0_f64, 536870912_u64);
        check_eq(-10000001.0_f64, -10000000.0_f64, 0.0_f64, 536870912_u64);

        check_eq( 100000000.0_f64,  100000001.0_f64, 0.0_f64, 67108864_u64);
        check_eq( 100000001.0_f64,  100000000.0_f64, 0.0_f64, 67108864_u64);
        check_eq(-100000000.0_f64, -100000001.0_f64, 0.0_f64, 67108864_u64);
        check_eq(-100000001.0_f64, -100000000.0_f64, 0.0_f64, 67108864_u64);

        check_eq( 1000000000.0_f64,  1000000001.0_f64, 0.0_f64, 8388608_u64);
        check_eq( 1000000001.0_f64,  1000000000.0_f64, 0.0_f64, 8388608_u64);
        check_eq(-1000000000.0_f64, -1000000001.0_f64, 0.0_f64, 8388608_u64);
        check_eq(-1000000001.0_f64, -1000000000.0_f64, 0.0_f64, 8388608_u64);

        check_eq( 10000000000.0_f64,  10000000001.0_f64, 0.0_f64, 524288_u64);
        check_eq( 10000000001.0_f64,  10000000000.0_f64, 0.0_f64, 524288_u64);
        check_eq(-10000000000.0_f64, -10000000001.0_f64, 0.0_f64, 524288_u64);
        check_eq(-10000000001.0_f64, -10000000000.0_f64, 0.0_f64, 524288_u64);

        check_eq( 100000000000.0_f64,  100000000001.0_f64, 0.0_f64, 65536_u64);
        check_eq( 100000000001.0_f64,  100000000000.0_f64, 0.0_f64, 65536_u64);
        check_eq(-100000000000.0_f64, -100000000001.0_f64, 0.0_f64, 65536_u64);
        check_eq(-100000000001.0_f64, -100000000000.0_f64, 0.0_f64, 65536_u64);

        check_eq( 1000000000000.0_f64,  1000000000001.0_f64, 0.0_f64, 8192_u64);
        check_eq( 1000000000001.0_f64,  1000000000000.0_f64, 0.0_f64, 8192_u64);
        check_eq(-1000000000000.0_f64, -1000000000001.0_f64, 0.0_f64, 8192_u64);
        check_eq(-1000000000001.0_f64, -1000000000000.0_f64, 0.0_f64, 8192_u64);

        check_eq( 10000000000000.0_f64,  10000000000001.0_f64, 0.0_f64, 512_u64);
        check_eq( 10000000000001.0_f64,  10000000000000.0_f64, 0.0_f64, 512_u64);
        check_eq(-10000000000000.0_f64, -10000000000001.0_f64, 0.0_f64, 512_u64);
        check_eq(-10000000000001.0_f64, -10000000000000.0_f64, 0.0_f64, 512_u64);

        check_eq( 100000000000000.0_f64,  100000000000001.0_f64, 0.0_f64, 64_u64);
        check_eq( 100000000000001.0_f64,  100000000000000.0_f64, 0.0_f64, 64_u64);
        check_eq(-100000000000000.0_f64, -100000000000001.0_f64, 0.0_f64, 64_u64);
        check_eq(-100000000000001.0_f64, -100000000000000.0_f64, 0.0_f64, 64_u64);

        check_eq( 1000000000000000.0_f64,  1000000000000001.0_f64, 0.0_f64, 8_u64);
        check_eq( 1000000000000001.0_f64,  1000000000000000.0_f64, 0.0_f64, 8_u64);
        check_eq(-1000000000000000.0_f64, -1000000000000001.0_f64, 0.0_f64, 8_u64);
        check_eq(-1000000000000001.0_f64, -1000000000000000.0_f64, 0.0_f64, 8_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding6() {
        check_ne( 1000.0_f64,  1001.0_f64, 0.0_f64, 8796093022207_u64);
        check_ne( 1001.0_f64,  1000.0_f64, 0.0_f64, 8796093022207_u64);
        check_ne(-1000.0_f64, -1001.0_f64, 0.0_f64, 8796093022207_u64);
        check_ne(-1001.0_f64, -1000.0_f64, 0.0_f64, 8796093022207_u64);

        check_ne( 10000.0_f64,  10001.0_f64, 0.0_f64, 549755813887_u64);
        check_ne( 10001.0_f64,  10000.0_f64, 0.0_f64, 549755813887_u64);
        check_ne(-10000.0_f64, -10001.0_f64, 0.0_f64, 549755813887_u64);
        check_ne(-10001.0_f64, -10000.0_f64, 0.0_f64, 549755813887_u64);

        check_ne( 100000.0_f64,  100001.0_f64, 0.0_f64, 68719476735_u64);
        check_ne( 100001.0_f64,  100000.0_f64, 0.0_f64, 68719476735_u64);
        check_ne(-100000.0_f64, -100001.0_f64, 0.0_f64, 68719476735_u64);
        check_ne(-100001.0_f64, -100000.0_f64, 0.0_f64, 68719476735_u64);

        check_ne( 1000000.0_f64,  1000001.0_f64, 0.0_f64, 8589934591_u64);
        check_ne( 1000001.0_f64,  1000000.0_f64, 0.0_f64, 8589934591_u64);
        check_ne(-1000000.0_f64, -1000001.0_f64, 0.0_f64, 8589934591_u64);
        check_ne(-1000001.0_f64, -1000000.0_f64, 0.0_f64, 8589934591_u64);

        check_ne( 10000000.0_f64,  10000001.0_f64, 0.0_f64, 536870911_u64);
        check_ne( 10000001.0_f64,  10000000.0_f64, 0.0_f64, 536870911_u64);
        check_ne(-10000000.0_f64, -10000001.0_f64, 0.0_f64, 536870911_u64);
        check_ne(-10000001.0_f64, -10000000.0_f64, 0.0_f64, 536870911_u64);

        check_ne( 100000000.0_f64,  100000001.0_f64, 0.0_f64, 67108863_u64);
        check_ne( 100000001.0_f64,  100000000.0_f64, 0.0_f64, 67108863_u64);
        check_ne(-100000000.0_f64, -100000001.0_f64, 0.0_f64, 67108863_u64);
        check_ne(-100000001.0_f64, -100000000.0_f64, 0.0_f64, 67108863_u64);

        check_ne( 1000000000.0_f64,  1000000001.0_f64, 0.0_f64, 8388607_u64);
        check_ne( 1000000001.0_f64,  1000000000.0_f64, 0.0_f64, 8388607_u64);
        check_ne(-1000000000.0_f64, -1000000001.0_f64, 0.0_f64, 8388607_u64);
        check_ne(-1000000001.0_f64, -1000000000.0_f64, 0.0_f64, 8388607_u64);

        check_ne( 10000000000.0_f64,  10000000001.0_f64, 0.0_f64, 524287_u64);
        check_ne( 10000000001.0_f64,  10000000000.0_f64, 0.0_f64, 524287_u64);
        check_ne(-10000000000.0_f64, -10000000001.0_f64, 0.0_f64, 524287_u64);
        check_ne(-10000000001.0_f64, -10000000000.0_f64, 0.0_f64, 524287_u64);

        check_ne( 100000000000.0_f64,  100000000001.0_f64, 0.0_f64, 65535_u64);
        check_ne( 100000000001.0_f64,  100000000000.0_f64, 0.0_f64, 65535_u64);
        check_ne(-100000000000.0_f64, -100000000001.0_f64, 0.0_f64, 65535_u64);
        check_ne(-100000000001.0_f64, -100000000000.0_f64, 0.0_f64, 65535_u64);

        check_ne( 1000000000000.0_f64,  1000000000001.0_f64, 0.0_f64, 8191_u64);
        check_ne( 1000000000001.0_f64,  1000000000000.0_f64, 0.0_f64, 8191_u64);
        check_ne(-1000000000000.0_f64, -1000000000001.0_f64, 0.0_f64, 8191_u64);
        check_ne(-1000000000001.0_f64, -1000000000000.0_f64, 0.0_f64, 8191_u64);

        check_ne( 10000000000000.0_f64,  10000000000001.0_f64, 0.0_f64, 511_u64);
        check_ne( 10000000000001.0_f64,  10000000000000.0_f64, 0.0_f64, 511_u64);
        check_ne(-10000000000000.0_f64, -10000000000001.0_f64, 0.0_f64, 511_u64);
        check_ne(-10000000000001.0_f64, -10000000000000.0_f64, 0.0_f64, 511_u64);

        check_ne( 100000000000000.0_f64,  100000000000001.0_f64, 0.0_f64, 63_u64);
        check_ne( 100000000000001.0_f64,  100000000000000.0_f64, 0.0_f64, 63_u64);
        check_ne(-100000000000000.0_f64, -100000000000001.0_f64, 0.0_f64, 63_u64);
        check_ne(-100000000000001.0_f64, -100000000000000.0_f64, 0.0_f64, 63_u64);

        check_ne( 1000000000000000.0_f64,  1000000000000001.0_f64, 0.0_f64, 7_u64);
        check_ne( 1000000000000001.0_f64,  1000000000000000.0_f64, 0.0_f64, 7_u64);
        check_ne(-1000000000000000.0_f64, -1000000000000001.0_f64, 0.0_f64, 7_u64);
        check_ne(-1000000000000001.0_f64, -1000000000000000.0_f64, 0.0_f64, 7_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding7() {
        check_eq( 1.000000000000001_f64,  1.000000000000002_f64, 0.0_f64, 4_u64);
        check_eq( 1.000000000000002_f64,  1.000000000000001_f64, 0.0_f64, 4_u64);
        check_eq(-1.000000000000001_f64, -1.000000000000002_f64, 0.0_f64, 4_u64);
        check_eq(-1.000000000000002_f64, -1.000000000000001_f64, 0.0_f64, 4_u64);

        check_eq( 1.0000000000000001_f64,  1.0000000000000002_f64, 0.0_f64, 4_u64);
        check_eq( 1.0000000000000002_f64,  1.0000000000000001_f64, 0.0_f64, 4_u64);
        check_eq(-1.0000000000000001_f64, -1.0000000000000002_f64, 0.0_f64, 4_u64);
        check_eq(-1.0000000000000002_f64, -1.0000000000000001_f64, 0.0_f64, 4_u64);

        check_eq( 1.00000000000000001_f64,  1.00000000000000002_f64, 0.0_f64, 4_u64);
        check_eq( 1.00000000000000002_f64,  1.00000000000000001_f64, 0.0_f64, 4_u64);
        check_eq(-1.00000000000000001_f64, -1.00000000000000002_f64, 0.0_f64, 4_u64);
        check_eq(-1.00000000000000002_f64, -1.00000000000000001_f64, 0.0_f64, 4_u64);

        check_eq( 1.000000000000000001_f64,  1.000000000000000002_f64, 0.0_f64, 4_u64);
        check_eq( 1.000000000000000002_f64,  1.000000000000000001_f64, 0.0_f64, 4_u64);
        check_eq(-1.000000000000000001_f64, -1.000000000000000002_f64, 0.0_f64, 4_u64);
        check_eq(-1.000000000000000002_f64, -1.000000000000000001_f64, 0.0_f64, 4_u64);

        check_eq( 1.0000000000000000001_f64,  1.0000000000000000002_f64, 0.0_f64, 4_u64);
        check_eq( 1.0000000000000000002_f64,  1.0000000000000000001_f64, 0.0_f64, 4_u64);
        check_eq(-1.0000000000000000001_f64, -1.0000000000000000002_f64, 0.0_f64, 4_u64);
        check_eq(-1.0000000000000000002_f64, -1.0000000000000000001_f64, 0.0_f64, 4_u64);

        check_eq( 1.00000000000000000001_f64,  1.00000000000000000002_f64, 0.0_f64, 4_u64);
        check_eq( 1.00000000000000000002_f64,  1.00000000000000000001_f64, 0.0_f64, 4_u64);
        check_eq(-1.00000000000000000001_f64, -1.00000000000000000002_f64, 0.0_f64, 4_u64);
        check_eq(-1.00000000000000000002_f64, -1.00000000000000000001_f64, 0.0_f64, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding7() {
        check_ne( 1.01_f64,  1.02_f64, 0.0_f64, 4_u64);
        check_ne( 1.02_f64,  1.01_f64, 0.0_f64, 4_u64);
        check_ne(-1.01_f64, -1.02_f64, 0.0_f64, 4_u64);
        check_ne(-1.02_f64, -1.01_f64, 0.0_f64, 4_u64);

        check_ne( 1.001_f64,  1.002_f64, 0.0_f64, 4_u64);
        check_ne( 1.002_f64,  1.001_f64, 0.0_f64, 4_u64);
        check_ne(-1.001_f64, -1.002_f64, 0.0_f64, 4_u64);
        check_ne(-1.002_f64, -1.001_f64, 0.0_f64, 4_u64);

        check_ne( 1.0001_f64,  1.0002_f64, 0.0_f64, 4_u64);
        check_ne( 1.0002_f64,  1.0001_f64, 0.0_f64, 4_u64);
        check_ne(-1.0001_f64, -1.0002_f64, 0.0_f64, 4_u64);
        check_ne(-1.0002_f64, -1.0001_f64, 0.0_f64, 4_u64);

        check_ne( 1.00001_f64,  1.00002_f64, 0.0_f64, 4_u64);
        check_ne( 1.00002_f64,  1.00001_f64, 0.0_f64, 4_u64);
        check_ne(-1.00001_f64, -1.00002_f64, 0.0_f64, 4_u64);
        check_ne(-1.00002_f64, -1.00001_f64, 0.0_f64, 4_u64);

        check_ne( 1.000001_f64,  1.000002_f64, 0.0_f64, 4_u64);
        check_ne( 1.000002_f64,  1.000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.000001_f64, -1.000002_f64, 0.0_f64, 4_u64);
        check_ne(-1.000002_f64, -1.000001_f64, 0.0_f64, 4_u64);

        check_ne( 1.0000001_f64,  1.0000002_f64, 0.0_f64, 4_u64);
        check_ne( 1.0000002_f64,  1.0000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.0000001_f64, -1.0000002_f64, 0.0_f64, 4_u64);
        check_ne(-1.0000002_f64, -1.0000001_f64, 0.0_f64, 4_u64);

        check_ne( 1.00000001_f64,  1.00000002_f64, 0.0_f64, 4_u64);
        check_ne( 1.00000002_f64,  1.00000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.00000001_f64, -1.00000002_f64, 0.0_f64, 4_u64);
        check_ne(-1.00000002_f64, -1.00000001_f64, 0.0_f64, 4_u64);

        check_ne( 1.000000001_f64,  1.000000002_f64, 0.0_f64, 4_u64);
        check_ne( 1.000000002_f64,  1.000000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.000000001_f64, -1.000000002_f64, 0.0_f64, 4_u64);
        check_ne(-1.000000002_f64, -1.000000001_f64, 0.0_f64, 4_u64);

        check_ne( 1.0000000001_f64,  1.0000000002_f64, 0.0_f64, 4_u64);
        check_ne( 1.0000000002_f64,  1.0000000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.0000000001_f64, -1.0000000002_f64, 0.0_f64, 4_u64);
        check_ne(-1.0000000002_f64, -1.0000000001_f64, 0.0_f64, 4_u64);

        check_ne( 1.00000000001_f64,  1.00000000002_f64, 0.0_f64, 4_u64);
        check_ne( 1.00000000002_f64,  1.00000000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.00000000001_f64, -1.00000000002_f64, 0.0_f64, 4_u64);
        check_ne(-1.00000000002_f64, -1.00000000001_f64, 0.0_f64, 4_u64);

        check_ne( 1.000000000001_f64,  1.000000000002_f64, 0.0_f64, 4_u64);
        check_ne( 1.000000000002_f64,  1.000000000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.000000000001_f64, -1.000000000002_f64, 0.0_f64, 4_u64);
        check_ne(-1.000000000002_f64, -1.000000000001_f64, 0.0_f64, 4_u64);

        check_ne( 1.0000000000001_f64,  1.0000000000002_f64, 0.0_f64, 4_u64);
        check_ne( 1.0000000000002_f64,  1.0000000000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.0000000000001_f64, -1.0000000000002_f64, 0.0_f64, 4_u64);
        check_ne(-1.0000000000002_f64, -1.0000000000001_f64, 0.0_f64, 4_u64);

        check_ne( 1.00000000000001_f64,  1.00000000000002_f64, 0.0_f64, 4_u64);
        check_ne( 1.00000000000002_f64,  1.00000000000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.00000000000001_f64, -1.00000000000002_f64, 0.0_f64, 4_u64);
        check_ne(-1.00000000000002_f64, -1.00000000000001_f64, 0.0_f64, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding8() {
        check_eq( 1.01_f64,  1.02_f64, 0.0_f64, 45035996273705_u64);
        check_eq( 1.02_f64,  1.01_f64, 0.0_f64, 45035996273705_u64);
        check_eq(-1.01_f64, -1.02_f64, 0.0_f64, 45035996273705_u64);
        check_eq(-1.02_f64, -1.01_f64, 0.0_f64, 45035996273705_u64);

        check_eq( 1.001_f64,  1.002_f64, 0.0_f64, 4503599627371_u64);
        check_eq( 1.002_f64,  1.001_f64, 0.0_f64, 4503599627371_u64);
        check_eq(-1.001_f64, -1.002_f64, 0.0_f64, 4503599627371_u64);
        check_eq(-1.002_f64, -1.001_f64, 0.0_f64, 4503599627371_u64);

        check_eq( 1.0001_f64,  1.0002_f64, 0.0_f64, 450359962737_u64);
        check_eq( 1.0002_f64,  1.0001_f64, 0.0_f64, 450359962737_u64);
        check_eq(-1.0001_f64, -1.0002_f64, 0.0_f64, 450359962737_u64);
        check_eq(-1.0002_f64, -1.0001_f64, 0.0_f64, 450359962737_u64);

        check_eq( 1.00001_f64,  1.00002_f64, 0.0_f64, 45035996273_u64);
        check_eq( 1.00002_f64,  1.00001_f64, 0.0_f64, 45035996273_u64);
        check_eq(-1.00001_f64, -1.00002_f64, 0.0_f64, 45035996273_u64);
        check_eq(-1.00002_f64, -1.00001_f64, 0.0_f64, 45035996273_u64);

        check_eq( 1.000001_f64,  1.000002_f64, 0.0_f64, 4503599628_u64);
        check_eq( 1.000002_f64,  1.000001_f64, 0.0_f64, 4503599628_u64);
        check_eq(-1.000001_f64, -1.000002_f64, 0.0_f64, 4503599628_u64);
        check_eq(-1.000002_f64, -1.000001_f64, 0.0_f64, 4503599628_u64);

        check_eq( 1.0000001_f64,  1.0000002_f64, 0.0_f64, 450359962_u64);
        check_eq( 1.0000002_f64,  1.0000001_f64, 0.0_f64, 450359962_u64);
        check_eq(-1.0000001_f64, -1.0000002_f64, 0.0_f64, 450359962_u64);
        check_eq(-1.0000002_f64, -1.0000001_f64, 0.0_f64, 450359962_u64);

        check_eq( 1.00000001_f64,  1.00000002_f64, 0.0_f64, 45035997_u64);
        check_eq( 1.00000002_f64,  1.00000001_f64, 0.0_f64, 45035997_u64);
        check_eq(-1.00000001_f64, -1.00000002_f64, 0.0_f64, 45035997_u64);
        check_eq(-1.00000002_f64, -1.00000001_f64, 0.0_f64, 45035997_u64);

        check_eq( 1.000000001_f64,  1.000000002_f64, 0.0_f64, 4503599_u64);
        check_eq( 1.000000002_f64,  1.000000001_f64, 0.0_f64, 4503599_u64);
        check_eq(-1.000000001_f64, -1.000000002_f64, 0.0_f64, 4503599_u64);
        check_eq(-1.000000002_f64, -1.000000001_f64, 0.0_f64, 4503599_u64);

        check_eq( 1.0000000001_f64,  1.0000000002_f64, 0.0_f64, 450360_u64);
        check_eq( 1.0000000002_f64,  1.0000000001_f64, 0.0_f64, 450360_u64);
        check_eq(-1.0000000001_f64, -1.0000000002_f64, 0.0_f64, 450360_u64);
        check_eq(-1.0000000002_f64, -1.0000000001_f64, 0.0_f64, 450360_u64);

        check_eq( 1.00000000001_f64,  1.00000000002_f64, 0.0_f64, 45036_u64);
        check_eq( 1.00000000002_f64,  1.00000000001_f64, 0.0_f64, 45036_u64);
        check_eq(-1.00000000001_f64, -1.00000000002_f64, 0.0_f64, 45036_u64);
        check_eq(-1.00000000002_f64, -1.00000000001_f64, 0.0_f64, 45036_u64);

        check_eq( 1.000000000001_f64,  1.000000000002_f64, 0.0_f64, 4503_u64);
        check_eq( 1.000000000002_f64,  1.000000000001_f64, 0.0_f64, 4503_u64);
        check_eq(-1.000000000001_f64, -1.000000000002_f64, 0.0_f64, 4503_u64);
        check_eq(-1.000000000002_f64, -1.000000000001_f64, 0.0_f64, 4503_u64);

        check_eq( 1.0000000000001_f64,  1.0000000000002_f64, 0.0_f64, 451_u64);
        check_eq( 1.0000000000002_f64,  1.0000000000001_f64, 0.0_f64, 451_u64);
        check_eq(-1.0000000000001_f64, -1.0000000000002_f64, 0.0_f64, 451_u64);
        check_eq(-1.0000000000002_f64, -1.0000000000001_f64, 0.0_f64, 451_u64);

        check_eq( 1.00000000000001_f64,  1.00000000000002_f64, 0.0_f64, 45_u64);
        check_eq( 1.00000000000002_f64,  1.00000000000001_f64, 0.0_f64, 45_u64);
        check_eq(-1.00000000000001_f64, -1.00000000000002_f64, 0.0_f64, 45_u64);
        check_eq(-1.00000000000002_f64, -1.00000000000001_f64, 0.0_f64, 45_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding8() {
        check_ne( 1.01_f64,  1.02_f64, 0.0_f64, 45035996273704_u64);
        check_ne( 1.02_f64,  1.01_f64, 0.0_f64, 45035996273704_u64);
        check_ne(-1.01_f64, -1.02_f64, 0.0_f64, 45035996273704_u64);
        check_ne(-1.02_f64, -1.01_f64, 0.0_f64, 45035996273704_u64);

        check_ne( 1.001_f64,  1.002_f64, 0.0_f64, 4503599627370_u64);
        check_ne( 1.002_f64,  1.001_f64, 0.0_f64, 4503599627370_u64);
        check_ne(-1.001_f64, -1.002_f64, 0.0_f64, 4503599627370_u64);
        check_ne(-1.002_f64, -1.001_f64, 0.0_f64, 4503599627370_u64);

        check_ne( 1.0001_f64,  1.0002_f64, 0.0_f64, 450359962736_u64);
        check_ne( 1.0002_f64,  1.0001_f64, 0.0_f64, 450359962736_u64);
        check_ne(-1.0001_f64, -1.0002_f64, 0.0_f64, 450359962736_u64);
        check_ne(-1.0002_f64, -1.0001_f64, 0.0_f64, 450359962736_u64);

        check_ne( 1.00001_f64,  1.00002_f64, 0.0_f64, 45035996272_u64);
        check_ne( 1.00002_f64,  1.00001_f64, 0.0_f64, 45035996272_u64);
        check_ne(-1.00001_f64, -1.00002_f64, 0.0_f64, 45035996272_u64);
        check_ne(-1.00002_f64, -1.00001_f64, 0.0_f64, 45035996272_u64);

        check_ne( 1.000001_f64,  1.000002_f64, 0.0_f64, 4503599627_u64);
        check_ne( 1.000002_f64,  1.000001_f64, 0.0_f64, 4503599627_u64);
        check_ne(-1.000001_f64, -1.000002_f64, 0.0_f64, 4503599627_u64);
        check_ne(-1.000002_f64, -1.000001_f64, 0.0_f64, 4503599627_u64);

        check_ne( 1.0000001_f64,  1.0000002_f64, 0.0_f64, 450359961_u64);
        check_ne( 1.0000002_f64,  1.0000001_f64, 0.0_f64, 450359961_u64);
        check_ne(-1.0000001_f64, -1.0000002_f64, 0.0_f64, 450359961_u64);
        check_ne(-1.0000002_f64, -1.0000001_f64, 0.0_f64, 450359961_u64);

        check_ne( 1.00000001_f64,  1.00000002_f64, 0.0_f64, 45035996_u64);
        check_ne( 1.00000002_f64,  1.00000001_f64, 0.0_f64, 45035996_u64);
        check_ne(-1.00000001_f64, -1.00000002_f64, 0.0_f64, 45035996_u64);
        check_ne(-1.00000002_f64, -1.00000001_f64, 0.0_f64, 45035996_u64);

        check_ne( 1.000000001_f64,  1.000000002_f64, 0.0_f64, 4503598_u64);
        check_ne( 1.000000002_f64,  1.000000001_f64, 0.0_f64, 4503598_u64);
        check_ne(-1.000000001_f64, -1.000000002_f64, 0.0_f64, 4503598_u64);
        check_ne(-1.000000002_f64, -1.000000001_f64, 0.0_f64, 4503598_u64);

        check_ne( 1.0000000001_f64,  1.0000000002_f64, 0.0_f64, 450359_u64);
        check_ne( 1.0000000002_f64,  1.0000000001_f64, 0.0_f64, 450359_u64);
        check_ne(-1.0000000001_f64, -1.0000000002_f64, 0.0_f64, 450359_u64);
        check_ne(-1.0000000002_f64, -1.0000000001_f64, 0.0_f64, 450359_u64);

        check_ne( 1.00000000001_f64,  1.00000000002_f64, 0.0_f64, 45035_u64);
        check_ne( 1.00000000002_f64,  1.00000000001_f64, 0.0_f64, 45035_u64);
        check_ne(-1.00000000001_f64, -1.00000000002_f64, 0.0_f64, 45035_u64);
        check_ne(-1.00000000002_f64, -1.00000000001_f64, 0.0_f64, 45035_u64);

        check_ne( 1.000000000001_f64,  1.000000000002_f64, 0.0_f64, 4502_u64);
        check_ne( 1.000000000002_f64,  1.000000000001_f64, 0.0_f64, 4502_u64);
        check_ne(-1.000000000001_f64, -1.000000000002_f64, 0.0_f64, 4502_u64);
        check_ne(-1.000000000002_f64, -1.000000000001_f64, 0.0_f64, 4502_u64);

        check_ne( 1.0000000000001_f64,  1.0000000000002_f64, 0.0_f64, 450_u64);
        check_ne( 1.0000000000002_f64,  1.0000000000001_f64, 0.0_f64, 450_u64);
        check_ne(-1.0000000000001_f64, -1.0000000000002_f64, 0.0_f64, 450_u64);
        check_ne(-1.0000000000002_f64, -1.0000000000001_f64, 0.0_f64, 450_u64);

        check_ne( 1.00000000000001_f64,  1.00000000000002_f64, 0.0_f64, 44_u64);
        check_ne( 1.00000000000002_f64,  1.00000000000001_f64, 0.0_f64, 44_u64);
        check_ne(-1.00000000000001_f64, -1.00000000000002_f64, 0.0_f64, 44_u64);
        check_ne(-1.00000000000002_f64, -1.00000000000001_f64, 0.0_f64, 44_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding9() {
        check_eq( 0.9999999999999999_f64,  1.0000000000000001_f64, 0.0_f64, 1_u64);
        check_eq( 1.0000000000000001_f64,  0.9999999999999999_f64, 0.0_f64, 1_u64);
        check_eq(-0.9999999999999999_f64, -1.0000000000000001_f64, 0.0_f64, 1_u64);
        check_eq(-1.0000000000000001_f64, -0.9999999999999999_f64, 0.0_f64, 1_u64);

        check_eq( 0.99999999999999999_f64,  1.00000000000000001_f64, 0.0_f64, 1_u64);
        check_eq( 1.00000000000000001_f64,  0.99999999999999999_f64, 0.0_f64, 1_u64);
        check_eq(-0.99999999999999999_f64, -1.00000000000000001_f64, 0.0_f64, 1_u64);
        check_eq(-1.00000000000000001_f64, -0.99999999999999999_f64, 0.0_f64, 1_u64);

        check_eq( 0.999999999999999999_f64,  1.000000000000000001_f64, 0.0_f64, 1_u64);
        check_eq( 1.000000000000000001_f64,  0.999999999999999999_f64, 0.0_f64, 1_u64);
        check_eq(-0.999999999999999999_f64, -1.000000000000000001_f64, 0.0_f64, 1_u64);
        check_eq(-1.000000000000000001_f64, -0.999999999999999999_f64, 0.0_f64, 1_u64);

        check_eq( 0.9999999999999999999_f64,  1.0000000000000000001_f64, 0.0_f64, 1_u64);
        check_eq( 1.0000000000000000001_f64,  0.9999999999999999999_f64, 0.0_f64, 1_u64);
        check_eq(-0.9999999999999999999_f64, -1.0000000000000000001_f64, 0.0_f64, 1_u64);
        check_eq(-1.0000000000000000001_f64, -0.9999999999999999999_f64, 0.0_f64, 1_u64);

        check_eq( 0.99999999999999999999_f64,  1.00000000000000000001_f64, 0.0_f64, 1_u64);
        check_eq( 1.00000000000000000001_f64,  0.99999999999999999999_f64, 0.0_f64, 1_u64);
        check_eq(-0.99999999999999999999_f64, -1.00000000000000000001_f64, 0.0_f64, 1_u64);
        check_eq(-1.00000000000000000001_f64, -0.99999999999999999999_f64, 0.0_f64, 1_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding9() {
        check_ne( 0.99_f64,  1.01_f64, 0.0_f64, 4_u64);
        check_ne( 1.01_f64,  0.99_f64, 0.0_f64, 4_u64);
        check_ne(-0.99_f64, -1.01_f64, 0.0_f64, 4_u64);
        check_ne(-1.01_f64, -0.99_f64, 0.0_f64, 4_u64);

        check_ne( 0.999_f64,  1.001_f64, 0.0_f64, 4_u64);
        check_ne( 1.001_f64,  0.999_f64, 0.0_f64, 4_u64);
        check_ne(-0.999_f64, -1.001_f64, 0.0_f64, 4_u64);
        check_ne(-1.001_f64, -0.999_f64, 0.0_f64, 4_u64);

        check_ne( 0.9999_f64,  1.0001_f64, 0.0_f64, 4_u64);
        check_ne( 1.0001_f64,  0.9999_f64, 0.0_f64, 4_u64);
        check_ne(-0.9999_f64, -1.0001_f64, 0.0_f64, 4_u64);
        check_ne(-1.0001_f64, -0.9999_f64, 0.0_f64, 4_u64);

        check_ne( 0.99999_f64,  1.00001_f64, 0.0_f64, 4_u64);
        check_ne( 1.00001_f64,  0.99999_f64, 0.0_f64, 4_u64);
        check_ne(-0.99999_f64, -1.00001_f64, 0.0_f64, 4_u64);
        check_ne(-1.00001_f64, -0.99999_f64, 0.0_f64, 4_u64);

        check_ne( 0.999999_f64,  1.000001_f64, 0.0_f64, 4_u64);
        check_ne( 1.000001_f64,  0.999999_f64, 0.0_f64, 4_u64);
        check_ne(-0.999999_f64, -1.000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.000001_f64, -0.999999_f64, 0.0_f64, 4_u64);

        check_ne( 0.9999999_f64,  1.0000001_f64, 0.0_f64, 4_u64);
        check_ne( 1.0000001_f64,  0.9999999_f64, 0.0_f64, 4_u64);
        check_ne(-0.9999999_f64, -1.0000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.0000001_f64, -0.9999999_f64, 0.0_f64, 4_u64);

        check_ne( 0.99999999_f64,  1.00000001_f64, 0.0_f64, 4_u64);
        check_ne( 1.00000001_f64,  0.99999999_f64, 0.0_f64, 4_u64);
        check_ne(-0.99999999_f64, -1.00000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.00000001_f64, -0.99999999_f64, 0.0_f64, 4_u64);

        check_ne( 0.999999999_f64,  1.000000001_f64, 0.0_f64, 4_u64);
        check_ne( 1.000000001_f64,  0.999999999_f64, 0.0_f64, 4_u64);
        check_ne(-0.999999999_f64, -1.000000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.000000001_f64, -0.999999999_f64, 0.0_f64, 4_u64);

        check_ne( 0.9999999999_f64,  1.0000000001_f64, 0.0_f64, 4_u64);
        check_ne( 1.0000000001_f64,  0.9999999999_f64, 0.0_f64, 4_u64);
        check_ne(-0.9999999999_f64, -1.0000000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.0000000001_f64, -0.9999999999_f64, 0.0_f64, 4_u64);

        check_ne( 0.99999999999_f64,  1.00000000001_f64, 0.0_f64, 4_u64);
        check_ne( 1.00000000001_f64,  0.99999999999_f64, 0.0_f64, 4_u64);
        check_ne(-0.99999999999_f64, -1.00000000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.00000000001_f64, -0.99999999999_f64, 0.0_f64, 4_u64);

        check_ne( 0.999999999999_f64,  1.000000000001_f64, 0.0_f64, 4_u64);
        check_ne( 1.000000000001_f64,  0.999999999999_f64, 0.0_f64, 4_u64);
        check_ne(-0.999999999999_f64, -1.000000000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.000000000001_f64, -0.999999999999_f64, 0.0_f64, 4_u64);

        check_ne( 0.9999999999999_f64,  1.0000000000001_f64, 0.0_f64, 4_u64);
        check_ne( 1.0000000000001_f64,  0.9999999999999_f64, 0.0_f64, 4_u64);
        check_ne(-0.9999999999999_f64, -1.0000000000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.0000000000001_f64, -0.9999999999999_f64, 0.0_f64, 4_u64);

        check_ne( 0.99999999999999_f64,  1.00000000000001_f64, 0.0_f64, 4_u64);
        check_ne( 1.00000000000001_f64,  0.99999999999999_f64, 0.0_f64, 4_u64);
        check_ne(-0.99999999999999_f64, -1.00000000000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.00000000000001_f64, -0.99999999999999_f64, 0.0_f64, 4_u64);

        check_ne( 0.999999999999999_f64,  1.000000000000001_f64, 0.0_f64, 4_u64);
        check_ne( 1.000000000000001_f64,  0.999999999999999_f64, 0.0_f64, 4_u64);
        check_ne(-0.999999999999999_f64, -1.000000000000001_f64, 0.0_f64, 4_u64);
        check_ne(-1.000000000000001_f64, -0.999999999999999_f64, 0.0_f64, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding10() {
        check_eq( 0.99_f64,  1.01_f64, 0.0_f64, 135107988821115_u64);
        check_eq( 1.01_f64,  0.99_f64, 0.0_f64, 135107988821115_u64);
        check_eq(-0.99_f64, -1.01_f64, 0.0_f64, 135107988821115_u64);
        check_eq(-1.01_f64, -0.99_f64, 0.0_f64, 135107988821115_u64);

        check_eq( 0.999_f64,  1.001_f64, 0.0_f64, 13510798882111_u64);
        check_eq( 1.001_f64,  0.999_f64, 0.0_f64, 13510798882111_u64);
        check_eq(-0.999_f64, -1.001_f64, 0.0_f64, 13510798882111_u64);
        check_eq(-1.001_f64, -0.999_f64, 0.0_f64, 13510798882111_u64);

        check_eq( 0.9999_f64,  1.0001_f64, 0.0_f64, 1351079888211_u64);
        check_eq( 1.0001_f64,  0.9999_f64, 0.0_f64, 1351079888211_u64);
        check_eq(-0.9999_f64, -1.0001_f64, 0.0_f64, 1351079888211_u64);
        check_eq(-1.0001_f64, -0.9999_f64, 0.0_f64, 1351079888211_u64);

        check_eq( 0.99999_f64,  1.00001_f64, 0.0_f64, 135107988821_u64);
        check_eq( 1.00001_f64,  0.99999_f64, 0.0_f64, 135107988821_u64);
        check_eq(-0.99999_f64, -1.00001_f64, 0.0_f64, 135107988821_u64);
        check_eq(-1.00001_f64, -0.99999_f64, 0.0_f64, 135107988821_u64);

        check_eq( 0.999999_f64,  1.000001_f64, 0.0_f64, 13510798882_u64);
        check_eq( 1.000001_f64,  0.999999_f64, 0.0_f64, 13510798882_u64);
        check_eq(-0.999999_f64, -1.000001_f64, 0.0_f64, 13510798882_u64);
        check_eq(-1.000001_f64, -0.999999_f64, 0.0_f64, 13510798882_u64);

        check_eq( 0.9999999_f64,  1.0000001_f64, 0.0_f64, 1351079888_u64);
        check_eq( 1.0000001_f64,  0.9999999_f64, 0.0_f64, 1351079888_u64);
        check_eq(-0.9999999_f64, -1.0000001_f64, 0.0_f64, 1351079888_u64);
        check_eq(-1.0000001_f64, -0.9999999_f64, 0.0_f64, 1351079888_u64);

        check_eq( 0.99999999_f64,  1.00000001_f64, 0.0_f64, 135107989_u64);
        check_eq( 1.00000001_f64,  0.99999999_f64, 0.0_f64, 135107989_u64);
        check_eq(-0.99999999_f64, -1.00000001_f64, 0.0_f64, 135107989_u64);
        check_eq(-1.00000001_f64, -0.99999999_f64, 0.0_f64, 135107989_u64);

        check_eq( 0.999999999_f64,  1.000000001_f64, 0.0_f64, 13510799_u64);
        check_eq( 1.000000001_f64,  0.999999999_f64, 0.0_f64, 13510799_u64);
        check_eq(-0.999999999_f64, -1.000000001_f64, 0.0_f64, 13510799_u64);
        check_eq(-1.000000001_f64, -0.999999999_f64, 0.0_f64, 13510799_u64);

        check_eq( 0.9999999999_f64,  1.0000000001_f64, 0.0_f64, 1351080_u64);
        check_eq( 1.0000000001_f64,  0.9999999999_f64, 0.0_f64, 1351080_u64);
        check_eq(-0.9999999999_f64, -1.0000000001_f64, 0.0_f64, 1351080_u64);
        check_eq(-1.0000000001_f64, -0.9999999999_f64, 0.0_f64, 1351080_u64);

        check_eq( 0.99999999999_f64,  1.00000000001_f64, 0.0_f64, 135108_u64);
        check_eq( 1.00000000001_f64,  0.99999999999_f64, 0.0_f64, 135108_u64);
        check_eq(-0.99999999999_f64, -1.00000000001_f64, 0.0_f64, 135108_u64);
        check_eq(-1.00000000001_f64, -0.99999999999_f64, 0.0_f64, 135108_u64);

        check_eq( 0.999999999999_f64,  1.000000000001_f64, 0.0_f64, 13511_u64);
        check_eq( 1.000000000001_f64,  0.999999999999_f64, 0.0_f64, 13511_u64);
        check_eq(-0.999999999999_f64, -1.000000000001_f64, 0.0_f64, 13511_u64);
        check_eq(-1.000000000001_f64, -0.999999999999_f64, 0.0_f64, 13511_u64);

        check_eq( 0.9999999999999_f64,  1.0000000000001_f64, 0.0_f64, 1351_u64);
        check_eq( 1.0000000000001_f64,  0.9999999999999_f64, 0.0_f64, 1351_u64);
        check_eq(-0.9999999999999_f64, -1.0000000000001_f64, 0.0_f64, 1351_u64);
        check_eq(-1.0000000000001_f64, -0.9999999999999_f64, 0.0_f64, 1351_u64);

        check_eq( 0.99999999999999_f64,  1.00000000000001_f64, 0.0_f64, 135_u64);
        check_eq( 1.00000000000001_f64,  0.99999999999999_f64, 0.0_f64, 135_u64);
        check_eq(-0.99999999999999_f64, -1.00000000000001_f64, 0.0_f64, 135_u64);
        check_eq(-1.00000000000001_f64, -0.99999999999999_f64, 0.0_f64, 135_u64);

        check_eq( 0.999999999999999_f64,  1.000000000000001_f64, 0.0_f64, 14_u64);
        check_eq( 1.000000000000001_f64,  0.999999999999999_f64, 0.0_f64, 14_u64);
        check_eq(-0.999999999999999_f64, -1.000000000000001_f64, 0.0_f64, 14_u64);
        check_eq(-1.000000000000001_f64, -0.999999999999999_f64, 0.0_f64, 14_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding10() {
        check_ne( 0.99_f64,  1.01_f64, 0.0_f64, 135107988821114_u64);
        check_ne( 1.01_f64,  0.99_f64, 0.0_f64, 135107988821114_u64);
        check_ne(-0.99_f64, -1.01_f64, 0.0_f64, 135107988821114_u64);
        check_ne(-1.01_f64, -0.99_f64, 0.0_f64, 135107988821114_u64);

        check_ne( 0.999_f64,  1.001_f64, 0.0_f64, 13510798882110_u64);
        check_ne( 1.001_f64,  0.999_f64, 0.0_f64, 13510798882110_u64);
        check_ne(-0.999_f64, -1.001_f64, 0.0_f64, 13510798882110_u64);
        check_ne(-1.001_f64, -0.999_f64, 0.0_f64, 13510798882110_u64);

        check_ne( 0.9999_f64,  1.0001_f64, 0.0_f64, 1351079888210_u64);
        check_ne( 1.0001_f64,  0.9999_f64, 0.0_f64, 1351079888210_u64);
        check_ne(-0.9999_f64, -1.0001_f64, 0.0_f64, 1351079888210_u64);
        check_ne(-1.0001_f64, -0.9999_f64, 0.0_f64, 1351079888210_u64);

        check_ne( 0.99999_f64,  1.00001_f64, 0.0_f64, 135107988820_u64);
        check_ne( 1.00001_f64,  0.99999_f64, 0.0_f64, 135107988820_u64);
        check_ne(-0.99999_f64, -1.00001_f64, 0.0_f64, 135107988820_u64);
        check_ne(-1.00001_f64, -0.99999_f64, 0.0_f64, 135107988820_u64);

        check_ne( 0.999999_f64,  1.000001_f64, 0.0_f64, 13510798881_u64);
        check_ne( 1.000001_f64,  0.999999_f64, 0.0_f64, 13510798881_u64);
        check_ne(-0.999999_f64, -1.000001_f64, 0.0_f64, 13510798881_u64);
        check_ne(-1.000001_f64, -0.999999_f64, 0.0_f64, 13510798881_u64);

        check_ne( 0.9999999_f64,  1.0000001_f64, 0.0_f64, 1351079887_u64);
        check_ne( 1.0000001_f64,  0.9999999_f64, 0.0_f64, 1351079887_u64);
        check_ne(-0.9999999_f64, -1.0000001_f64, 0.0_f64, 1351079887_u64);
        check_ne(-1.0000001_f64, -0.9999999_f64, 0.0_f64, 1351079887_u64);

        check_ne( 0.99999999_f64,  1.00000001_f64, 0.0_f64, 135107988_u64);
        check_ne( 1.00000001_f64,  0.99999999_f64, 0.0_f64, 135107988_u64);
        check_ne(-0.99999999_f64, -1.00000001_f64, 0.0_f64, 135107988_u64);
        check_ne(-1.00000001_f64, -0.99999999_f64, 0.0_f64, 135107988_u64);

        check_ne( 0.999999999_f64,  1.000000001_f64, 0.0_f64, 13510798_u64);
        check_ne( 1.000000001_f64,  0.999999999_f64, 0.0_f64, 13510798_u64);
        check_ne(-0.999999999_f64, -1.000000001_f64, 0.0_f64, 13510798_u64);
        check_ne(-1.000000001_f64, -0.999999999_f64, 0.0_f64, 13510798_u64);

        check_ne( 0.9999999999_f64,  1.0000000001_f64, 0.0_f64, 1351079_u64);
        check_ne( 1.0000000001_f64,  0.9999999999_f64, 0.0_f64, 1351079_u64);
        check_ne(-0.9999999999_f64, -1.0000000001_f64, 0.0_f64, 1351079_u64);
        check_ne(-1.0000000001_f64, -0.9999999999_f64, 0.0_f64, 1351079_u64);

        check_ne( 0.99999999999_f64,  1.00000000001_f64, 0.0_f64, 135107_u64);
        check_ne( 1.00000000001_f64,  0.99999999999_f64, 0.0_f64, 135107_u64);
        check_ne(-0.99999999999_f64, -1.00000000001_f64, 0.0_f64, 135107_u64);
        check_ne(-1.00000000001_f64, -0.99999999999_f64, 0.0_f64, 135107_u64);

        check_ne( 0.999999999999_f64,  1.000000000001_f64, 0.0_f64, 13510_u64);
        check_ne( 1.000000000001_f64,  0.999999999999_f64, 0.0_f64, 13510_u64);
        check_ne(-0.999999999999_f64, -1.000000000001_f64, 0.0_f64, 13510_u64);
        check_ne(-1.000000000001_f64, -0.999999999999_f64, 0.0_f64, 13510_u64);

        check_ne( 0.9999999999999_f64,  1.0000000000001_f64, 0.0_f64, 1350_u64);
        check_ne( 1.0000000000001_f64,  0.9999999999999_f64, 0.0_f64, 1350_u64);
        check_ne(-0.9999999999999_f64, -1.0000000000001_f64, 0.0_f64, 1350_u64);
        check_ne(-1.0000000000001_f64, -0.9999999999999_f64, 0.0_f64, 1350_u64);

        check_ne( 0.99999999999999_f64,  1.00000000000001_f64, 0.0_f64, 134_u64);
        check_ne( 1.00000000000001_f64,  0.99999999999999_f64, 0.0_f64, 134_u64);
        check_ne(-0.99999999999999_f64, -1.00000000000001_f64, 0.0_f64, 134_u64);
        check_ne(-1.00000000000001_f64, -0.99999999999999_f64, 0.0_f64, 134_u64);

        check_ne( 0.999999999999999_f64,  1.000000000000001_f64, 0.0_f64, 13_u64);
        check_ne( 1.000000000000001_f64,  0.999999999999999_f64, 0.0_f64, 13_u64);
        check_ne(-0.999999999999999_f64, -1.000000000000001_f64, 0.0_f64, 13_u64);
        check_ne(-1.000000000000001_f64, -0.999999999999999_f64, 0.0_f64, 13_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding11() {
        check_eq( 1.9999999999999999_f64,  2.0000000000000001_f64, 0.0_f64, 1_u64);
        check_eq( 2.0000000000000001_f64,  1.9999999999999999_f64, 0.0_f64, 1_u64);
        check_eq(-1.9999999999999999_f64, -2.0000000000000001_f64, 0.0_f64, 1_u64);
        check_eq(-2.0000000000000001_f64, -1.9999999999999999_f64, 0.0_f64, 1_u64);

        check_eq( 1.99999999999999999_f64,  2.00000000000000001_f64, 0.0_f64, 1_u64);
        check_eq( 2.00000000000000001_f64,  1.99999999999999999_f64, 0.0_f64, 1_u64);
        check_eq(-1.99999999999999999_f64, -2.00000000000000001_f64, 0.0_f64, 1_u64);
        check_eq(-2.00000000000000001_f64, -1.99999999999999999_f64, 0.0_f64, 1_u64);

        check_eq( 1.999999999999999999_f64,  2.000000000000000001_f64, 0.0_f64, 1_u64);
        check_eq( 2.000000000000000001_f64,  1.999999999999999999_f64, 0.0_f64, 1_u64);
        check_eq(-1.999999999999999999_f64, -2.000000000000000001_f64, 0.0_f64, 1_u64);
        check_eq(-2.000000000000000001_f64, -1.999999999999999999_f64, 0.0_f64, 1_u64);

        check_eq( 1.9999999999999999999_f64,  2.0000000000000000001_f64, 0.0_f64, 1_u64);
        check_eq( 2.0000000000000000001_f64,  1.9999999999999999999_f64, 0.0_f64, 1_u64);
        check_eq(-1.9999999999999999999_f64, -2.0000000000000000001_f64, 0.0_f64, 1_u64);
        check_eq(-2.0000000000000000001_f64, -1.9999999999999999999_f64, 0.0_f64, 1_u64);

        check_eq( 1.99999999999999999999_f64,  2.00000000000000000001_f64, 0.0_f64, 1_u64);
        check_eq( 2.00000000000000000001_f64,  1.99999999999999999999_f64, 0.0_f64, 1_u64);
        check_eq(-1.99999999999999999999_f64, -2.00000000000000000001_f64, 0.0_f64, 1_u64);
        check_eq(-2.00000000000000000001_f64, -1.99999999999999999999_f64, 0.0_f64, 1_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding11() {
        check_ne( 1.99_f64,  2.01_f64, 0.0_f64, 4_u64);
        check_ne( 2.01_f64,  1.99_f64, 0.0_f64, 4_u64);
        check_ne(-1.99_f64, -2.01_f64, 0.0_f64, 4_u64);
        check_ne(-2.01_f64, -1.99_f64, 0.0_f64, 4_u64);

        check_ne( 1.999_f64,  2.001_f64, 0.0_f64, 4_u64);
        check_ne( 2.001_f64,  1.999_f64, 0.0_f64, 4_u64);
        check_ne(-1.999_f64, -2.001_f64, 0.0_f64, 4_u64);
        check_ne(-2.001_f64, -1.999_f64, 0.0_f64, 4_u64);

        check_ne( 1.9999_f64,  2.0001_f64, 0.0_f64, 4_u64);
        check_ne( 2.0001_f64,  1.9999_f64, 0.0_f64, 4_u64);
        check_ne(-1.9999_f64, -2.0001_f64, 0.0_f64, 4_u64);
        check_ne(-2.0001_f64, -1.9999_f64, 0.0_f64, 4_u64);

        check_ne( 1.99999_f64,  2.00001_f64, 0.0_f64, 4_u64);
        check_ne( 2.00001_f64,  1.99999_f64, 0.0_f64, 4_u64);
        check_ne(-1.99999_f64, -2.00001_f64, 0.0_f64, 4_u64);
        check_ne(-2.00001_f64, -1.99999_f64, 0.0_f64, 4_u64);

        check_ne( 1.999999_f64,  2.000001_f64, 0.0_f64, 4_u64);
        check_ne( 2.000001_f64,  1.999999_f64, 0.0_f64, 4_u64);
        check_ne(-1.999999_f64, -2.000001_f64, 0.0_f64, 4_u64);
        check_ne(-2.000001_f64, -1.999999_f64, 0.0_f64, 4_u64);

        check_ne( 1.9999999_f64,  2.0000001_f64, 0.0_f64, 4_u64);
        check_ne( 2.0000001_f64,  1.9999999_f64, 0.0_f64, 4_u64);
        check_ne(-1.9999999_f64, -2.0000001_f64, 0.0_f64, 4_u64);
        check_ne(-2.0000001_f64, -1.9999999_f64, 0.0_f64, 4_u64);

        check_ne( 1.99999999_f64,  2.00000001_f64, 0.0_f64, 4_u64);
        check_ne( 2.00000001_f64,  1.99999999_f64, 0.0_f64, 4_u64);
        check_ne(-1.99999999_f64, -2.00000001_f64, 0.0_f64, 4_u64);
        check_ne(-2.00000001_f64, -1.99999999_f64, 0.0_f64, 4_u64);

        check_ne( 1.999999999_f64,  2.000000001_f64, 0.0_f64, 4_u64);
        check_ne( 2.000000001_f64,  1.999999999_f64, 0.0_f64, 4_u64);
        check_ne(-1.999999999_f64, -2.000000001_f64, 0.0_f64, 4_u64);
        check_ne(-2.000000001_f64, -1.999999999_f64, 0.0_f64, 4_u64);

        check_ne( 1.9999999999_f64,  2.0000000001_f64, 0.0_f64, 4_u64);
        check_ne( 2.0000000001_f64,  1.9999999999_f64, 0.0_f64, 4_u64);
        check_ne(-1.9999999999_f64, -2.0000000001_f64, 0.0_f64, 4_u64);
        check_ne(-2.0000000001_f64, -1.9999999999_f64, 0.0_f64, 4_u64);

        check_ne( 1.99999999999_f64,  2.00000000001_f64, 0.0_f64, 4_u64);
        check_ne( 2.00000000001_f64,  1.99999999999_f64, 0.0_f64, 4_u64);
        check_ne(-1.99999999999_f64, -2.00000000001_f64, 0.0_f64, 4_u64);
        check_ne(-2.00000000001_f64, -1.99999999999_f64, 0.0_f64, 4_u64);

        check_ne( 1.999999999999_f64,  2.000000000001_f64, 0.0_f64, 4_u64);
        check_ne( 2.000000000001_f64,  1.999999999999_f64, 0.0_f64, 4_u64);
        check_ne(-1.999999999999_f64, -2.000000000001_f64, 0.0_f64, 4_u64);
        check_ne(-2.000000000001_f64, -1.999999999999_f64, 0.0_f64, 4_u64);

        check_ne( 1.9999999999999_f64,  2.0000000000001_f64, 0.0_f64, 4_u64);
        check_ne( 2.0000000000001_f64,  1.9999999999999_f64, 0.0_f64, 4_u64);
        check_ne(-1.9999999999999_f64, -2.0000000000001_f64, 0.0_f64, 4_u64);
        check_ne(-2.0000000000001_f64, -1.9999999999999_f64, 0.0_f64, 4_u64);

        check_ne( 1.99999999999999_f64,  2.00000000000001_f64, 0.0_f64, 4_u64);
        check_ne( 2.00000000000001_f64,  1.99999999999999_f64, 0.0_f64, 4_u64);
        check_ne(-1.99999999999999_f64, -2.00000000000001_f64, 0.0_f64, 4_u64);
        check_ne(-2.00000000000001_f64, -1.99999999999999_f64, 0.0_f64, 4_u64);

        check_ne( 1.999999999999999_f64,  2.000000000000001_f64, 0.0_f64, 4_u64);
        check_ne( 2.000000000000001_f64,  1.999999999999999_f64, 0.0_f64, 4_u64);
        check_ne(-1.999999999999999_f64, -2.000000000000001_f64, 0.0_f64, 4_u64);
        check_ne(-2.000000000000001_f64, -1.999999999999999_f64, 0.0_f64, 4_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_rounding12() {
        check_eq( 1.99_f64,  2.01_f64, 0.0_f64, 67553994410557_u64);
        check_eq( 2.01_f64,  1.99_f64, 0.0_f64, 67553994410557_u64);
        check_eq(-1.99_f64, -2.01_f64, 0.0_f64, 67553994410557_u64);
        check_eq(-2.01_f64, -1.99_f64, 0.0_f64, 67553994410557_u64);

        check_eq( 1.999_f64,  2.001_f64, 0.0_f64, 6755399441055_u64);
        check_eq( 2.001_f64,  1.999_f64, 0.0_f64, 6755399441055_u64);
        check_eq(-1.999_f64, -2.001_f64, 0.0_f64, 6755399441055_u64);
        check_eq(-2.001_f64, -1.999_f64, 0.0_f64, 6755399441055_u64);

        check_eq( 1.9999_f64,  2.0001_f64, 0.0_f64, 675539944106_u64);
        check_eq( 2.0001_f64,  1.9999_f64, 0.0_f64, 675539944106_u64);
        check_eq(-1.9999_f64, -2.0001_f64, 0.0_f64, 675539944106_u64);
        check_eq(-2.0001_f64, -1.9999_f64, 0.0_f64, 675539944106_u64);

        check_eq( 1.99999_f64,  2.00001_f64, 0.0_f64, 67553994411_u64);
        check_eq( 2.00001_f64,  1.99999_f64, 0.0_f64, 67553994411_u64);
        check_eq(-1.99999_f64, -2.00001_f64, 0.0_f64, 67553994411_u64);
        check_eq(-2.00001_f64, -1.99999_f64, 0.0_f64, 67553994411_u64);

        check_eq( 1.999999_f64,  2.000001_f64, 0.0_f64, 6755399441_u64);
        check_eq( 2.000001_f64,  1.999999_f64, 0.0_f64, 6755399441_u64);
        check_eq(-1.999999_f64, -2.000001_f64, 0.0_f64, 6755399441_u64);
        check_eq(-2.000001_f64, -1.999999_f64, 0.0_f64, 6755399441_u64);

        check_eq( 1.9999999_f64,  2.0000001_f64, 0.0_f64, 675539944_u64);
        check_eq( 2.0000001_f64,  1.9999999_f64, 0.0_f64, 675539944_u64);
        check_eq(-1.9999999_f64, -2.0000001_f64, 0.0_f64, 675539944_u64);
        check_eq(-2.0000001_f64, -1.9999999_f64, 0.0_f64, 675539944_u64);

        check_eq( 1.99999999_f64,  2.00000001_f64, 0.0_f64, 67553994_u64);
        check_eq( 2.00000001_f64,  1.99999999_f64, 0.0_f64, 67553994_u64);
        check_eq(-1.99999999_f64, -2.00000001_f64, 0.0_f64, 67553994_u64);
        check_eq(-2.00000001_f64, -1.99999999_f64, 0.0_f64, 67553994_u64);

        check_eq( 1.999999999_f64,  2.000000001_f64, 0.0_f64, 6755400_u64);
        check_eq( 2.000000001_f64,  1.999999999_f64, 0.0_f64, 6755400_u64);
        check_eq(-1.999999999_f64, -2.000000001_f64, 0.0_f64, 6755400_u64);
        check_eq(-2.000000001_f64, -1.999999999_f64, 0.0_f64, 6755400_u64);

        check_eq( 1.9999999999_f64,  2.0000000001_f64, 0.0_f64, 675540_u64);
        check_eq( 2.0000000001_f64,  1.9999999999_f64, 0.0_f64, 675540_u64);
        check_eq(-1.9999999999_f64, -2.0000000001_f64, 0.0_f64, 675540_u64);
        check_eq(-2.0000000001_f64, -1.9999999999_f64, 0.0_f64, 675540_u64);

        check_eq( 1.99999999999_f64,  2.00000000001_f64, 0.0_f64, 67554_u64);
        check_eq( 2.00000000001_f64,  1.99999999999_f64, 0.0_f64, 67554_u64);
        check_eq(-1.99999999999_f64, -2.00000000001_f64, 0.0_f64, 67554_u64);
        check_eq(-2.00000000001_f64, -1.99999999999_f64, 0.0_f64, 67554_u64);

        check_eq( 1.999999999999_f64,  2.000000000001_f64, 0.0_f64, 6756_u64);
        check_eq( 2.000000000001_f64,  1.999999999999_f64, 0.0_f64, 6756_u64);
        check_eq(-1.999999999999_f64, -2.000000000001_f64, 0.0_f64, 6756_u64);
        check_eq(-2.000000000001_f64, -1.999999999999_f64, 0.0_f64, 6756_u64);

        check_eq( 1.9999999999999_f64,  2.0000000000001_f64, 0.0_f64, 675_u64);
        check_eq( 2.0000000000001_f64,  1.9999999999999_f64, 0.0_f64, 675_u64);
        check_eq(-1.9999999999999_f64, -2.0000000000001_f64, 0.0_f64, 675_u64);
        check_eq(-2.0000000000001_f64, -1.9999999999999_f64, 0.0_f64, 675_u64);

        check_eq( 1.99999999999999_f64,  2.00000000000001_f64, 0.0_f64, 68_u64);
        check_eq( 2.00000000000001_f64,  1.99999999999999_f64, 0.0_f64, 68_u64);
        check_eq(-1.99999999999999_f64, -2.00000000000001_f64, 0.0_f64, 68_u64);
        check_eq(-2.00000000000001_f64, -1.99999999999999_f64, 0.0_f64, 68_u64);

        check_eq( 1.999999999999999_f64,  2.000000000000001_f64, 0.0_f64, 7_u64);
        check_eq( 2.000000000000001_f64,  1.999999999999999_f64, 0.0_f64, 7_u64);
        check_eq(-1.999999999999999_f64, -2.000000000000001_f64, 0.0_f64, 7_u64);
        check_eq(-2.000000000000001_f64, -1.999999999999999_f64, 0.0_f64, 7_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_rounding12() {
        check_ne( 1.99_f64,  2.01_f64, 0.0_f64, 67553994410556_u64);
        check_ne( 2.01_f64,  1.99_f64, 0.0_f64, 67553994410556_u64);
        check_ne(-1.99_f64, -2.01_f64, 0.0_f64, 67553994410556_u64);
        check_ne(-2.01_f64, -1.99_f64, 0.0_f64, 67553994410556_u64);

        check_ne( 1.999_f64,  2.001_f64, 0.0_f64, 6755399441054_u64);
        check_ne( 2.001_f64,  1.999_f64, 0.0_f64, 6755399441054_u64);
        check_ne(-1.999_f64, -2.001_f64, 0.0_f64, 6755399441054_u64);
        check_ne(-2.001_f64, -1.999_f64, 0.0_f64, 6755399441054_u64);

        check_ne( 1.9999_f64,  2.0001_f64, 0.0_f64, 675539944105_u64);
        check_ne( 2.0001_f64,  1.9999_f64, 0.0_f64, 675539944105_u64);
        check_ne(-1.9999_f64, -2.0001_f64, 0.0_f64, 675539944105_u64);
        check_ne(-2.0001_f64, -1.9999_f64, 0.0_f64, 675539944105_u64);

        check_ne( 1.99999_f64,  2.00001_f64, 0.0_f64, 67553994410_u64);
        check_ne( 2.00001_f64,  1.99999_f64, 0.0_f64, 67553994410_u64);
        check_ne(-1.99999_f64, -2.00001_f64, 0.0_f64, 67553994410_u64);
        check_ne(-2.00001_f64, -1.99999_f64, 0.0_f64, 67553994410_u64);

        check_ne( 1.999999_f64,  2.000001_f64, 0.0_f64, 6755399440_u64);
        check_ne( 2.000001_f64,  1.999999_f64, 0.0_f64, 6755399440_u64);
        check_ne(-1.999999_f64, -2.000001_f64, 0.0_f64, 6755399440_u64);
        check_ne(-2.000001_f64, -1.999999_f64, 0.0_f64, 6755399440_u64);

        check_ne( 1.9999999_f64,  2.0000001_f64, 0.0_f64, 675539943_u64);
        check_ne( 2.0000001_f64,  1.9999999_f64, 0.0_f64, 675539943_u64);
        check_ne(-1.9999999_f64, -2.0000001_f64, 0.0_f64, 675539943_u64);
        check_ne(-2.0000001_f64, -1.9999999_f64, 0.0_f64, 675539943_u64);

        check_ne( 1.99999999_f64,  2.00000001_f64, 0.0_f64, 67553993_u64);
        check_ne( 2.00000001_f64,  1.99999999_f64, 0.0_f64, 67553993_u64);
        check_ne(-1.99999999_f64, -2.00000001_f64, 0.0_f64, 67553993_u64);
        check_ne(-2.00000001_f64, -1.99999999_f64, 0.0_f64, 67553993_u64);

        check_ne( 1.999999999_f64,  2.000000001_f64, 0.0_f64, 6755399_u64);
        check_ne( 2.000000001_f64,  1.999999999_f64, 0.0_f64, 6755399_u64);
        check_ne(-1.999999999_f64, -2.000000001_f64, 0.0_f64, 6755399_u64);
        check_ne(-2.000000001_f64, -1.999999999_f64, 0.0_f64, 6755399_u64);

        check_ne( 1.9999999999_f64,  2.0000000001_f64, 0.0_f64, 675539_u64);
        check_ne( 2.0000000001_f64,  1.9999999999_f64, 0.0_f64, 675539_u64);
        check_ne(-1.9999999999_f64, -2.0000000001_f64, 0.0_f64, 675539_u64);
        check_ne(-2.0000000001_f64, -1.9999999999_f64, 0.0_f64, 675539_u64);

        check_ne( 1.99999999999_f64,  2.00000000001_f64, 0.0_f64, 67553_u64);
        check_ne( 2.00000000001_f64,  1.99999999999_f64, 0.0_f64, 67553_u64);
        check_ne(-1.99999999999_f64, -2.00000000001_f64, 0.0_f64, 67553_u64);
        check_ne(-2.00000000001_f64, -1.99999999999_f64, 0.0_f64, 67553_u64);

        check_ne( 1.999999999999_f64,  2.000000000001_f64, 0.0_f64, 6755_u64);
        check_ne( 2.000000000001_f64,  1.999999999999_f64, 0.0_f64, 6755_u64);
        check_ne(-1.999999999999_f64, -2.000000000001_f64, 0.0_f64, 6755_u64);
        check_ne(-2.000000000001_f64, -1.999999999999_f64, 0.0_f64, 6755_u64);

        check_ne( 1.9999999999999_f64,  2.0000000000001_f64, 0.0_f64, 674_u64);
        check_ne( 2.0000000000001_f64,  1.9999999999999_f64, 0.0_f64, 674_u64);
        check_ne(-1.9999999999999_f64, -2.0000000000001_f64, 0.0_f64, 674_u64);
        check_ne(-2.0000000000001_f64, -1.9999999999999_f64, 0.0_f64, 674_u64);

        check_ne( 1.99999999999999_f64,  2.00000000000001_f64, 0.0_f64, 67_u64);
        check_ne( 2.00000000000001_f64,  1.99999999999999_f64, 0.0_f64, 67_u64);
        check_ne(-1.99999999999999_f64, -2.00000000000001_f64, 0.0_f64, 67_u64);
        check_ne(-2.00000000000001_f64, -1.99999999999999_f64, 0.0_f64, 67_u64);

        check_ne( 1.999999999999999_f64,  2.000000000000001_f64, 0.0_f64, 6_u64);
        check_ne( 2.000000000000001_f64,  1.999999999999999_f64, 0.0_f64, 6_u64);
        check_ne(-1.999999999999999_f64, -2.000000000000001_f64, 0.0_f64, 6_u64);
        check_ne(-2.000000000000001_f64, -1.999999999999999_f64, 0.0_f64, 6_u64);
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
    fn test_eq_near_zero5() {
        check_eq( f64::MIN_POSITIVE,  0.0_f64,           0.0_f64, 4503599627370496_u64);
        check_eq( 0.0_f64,            f64::MIN_POSITIVE, 0.0_f64, 4503599627370496_u64);
        check_eq(-f64::MIN_POSITIVE, -0.0_f64,           0.0_f64, 4503599627370496_u64);
        check_eq(-0.0_f64,           -f64::MIN_POSITIVE, 0.0_f64, 4503599627370496_u64);

        check_eq( 0.0000000000000001_f64,  f64::MIN_POSITIVE,      0.0_f64, 4363093803508730300_u64);
        check_eq( f64::MIN_POSITIVE,       0.0000000000000001_f64, 0.0_f64, 4363093803508730300_u64);
        check_eq(-0.0000000000000001_f64, -f64::MIN_POSITIVE,      0.0_f64, 4363093803508730300_u64);
        check_eq(-f64::MIN_POSITIVE,      -0.0000000000000001_f64, 0.0_f64, 4363093803508730300_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero6() {
        check_eq( 0.0000000010000001_f64,  0.0000000010000002_f64, 0.0_f64, 483570327_u64);
        check_eq( 0.0000000010000002_f64,  0.0000000010000001_f64, 0.0_f64, 483570327_u64);
        check_eq(-0.0000000010000001_f64, -0.0000000010000002_f64, 0.0_f64, 483570327_u64);
        check_eq(-0.0000000010000002_f64, -0.0000000010000001_f64, 0.0_f64, 483570327_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_near_zero7() {
        check_eq( 0.0000000011000001_f64,  0.0000000011000002_f64, 0.0_f64, 483570328_u64);
        check_eq( 0.0000000011000002_f64,  0.0000000011000001_f64, 0.0_f64, 483570328_u64);
        check_eq(-0.0000000011000001_f64, -0.0000000011000002_f64, 0.0_f64, 483570328_u64);
        check_eq(-0.0000000011000002_f64, -0.0000000011000001_f64, 0.0_f64, 483570328_u64);
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

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero4() {
        check_ne(1e-16_f64, -1e-16_f64, 0.0_f64, 9223372036854775807_u64);
        check_ne(1e-17_f64, -1e-17_f64, 0.0_f64, 9223372036854775807_u64);
        check_ne(1e-18_f64, -1e-18_f64, 0.0_f64, 9223372036854775807_u64);
        check_ne(1e-19_f64, -1e-19_f64, 0.0_f64, 9223372036854775807_u64);
        check_ne(1e-20_f64, -1e-20_f64, 0.0_f64, 9223372036854775807_u64);
        check_ne(1e-21_f64, -1e-21_f64, 0.0_f64, 9223372036854775807_u64);
        check_ne(1e-22_f64, -1e-22_f64, 0.0_f64, 9223372036854775807_u64);
        check_ne(1e-23_f64, -1e-23_f64, 0.0_f64, 9223372036854775807_u64);
        check_ne(1e-24_f64, -1e-24_f64, 0.0_f64, 9223372036854775807_u64);
        check_ne(1e-25_f64, -1e-25_f64, 0.0_f64, 9223372036854775807_u64);
        check_ne(1e-26_f64, -1e-26_f64, 0.0_f64, 9223372036854775807_u64);
        check_ne(1e-27_f64, -1e-27_f64, 0.0_f64, 9223372036854775807_u64);
        check_ne(1e-28_f64, -1e-28_f64, 0.0_f64, 9223372036854775807_u64);
        check_ne(1e-29_f64, -1e-29_f64, 0.0_f64, 9223372036854775807_u64);
        check_ne(1e-30_f64, -1e-30_f64, 0.0_f64, 9223372036854775807_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero5() {
        check_ne( f64::MIN_POSITIVE,  0.0_f64,           0.0_f64, 4503599627370495_u64);
        check_ne( 0.0_f64,            f64::MIN_POSITIVE, 0.0_f64, 4503599627370495_u64);
        check_ne(-f64::MIN_POSITIVE, -0.0_f64,           0.0_f64, 4503599627370495_u64);
        check_ne(-0.0_f64,           -f64::MIN_POSITIVE, 0.0_f64, 4503599627370495_u64);

        check_ne( 0.0000000000000001_f64,  f64::MIN_POSITIVE,      0.0_f64, 4363093803508730299_u64);
        check_ne( f64::MIN_POSITIVE,       0.0000000000000001_f64, 0.0_f64, 4363093803508730299_u64);
        check_ne(-0.0000000000000001_f64, -f64::MIN_POSITIVE,      0.0_f64, 4363093803508730299_u64);
        check_ne(-f64::MIN_POSITIVE,      -0.0000000000000001_f64, 0.0_f64, 4363093803508730299_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero6() {
        check_ne( 0.0000000010000001_f64,  0.0000000010000002_f64, 0.0_f64, 483570326_u64);
        check_ne( 0.0000000010000002_f64,  0.0000000010000001_f64, 0.0_f64, 483570326_u64);
        check_ne(-0.0000000010000001_f64, -0.0000000010000002_f64, 0.0_f64, 483570326_u64);
        check_ne(-0.0000000010000002_f64, -0.0000000010000001_f64, 0.0_f64, 483570326_u64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_near_zero7() {
        check_ne( 0.0000000011000001_f64,  0.0000000011000002_f64, 0.0_f64, 483570327_u64);
        check_ne( 0.0000000011000002_f64,  0.0000000011000001_f64, 0.0_f64, 483570327_u64);
        check_ne(-0.0000000011000001_f64, -0.0000000011000002_f64, 0.0_f64, 483570327_u64);
        check_ne(-0.0000000011000002_f64, -0.0000000011000001_f64, 0.0_f64, 483570327_u64);
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
            fn test_ulps_eq_exactly_representable_exhaustive() {
                let max_ulps = 4;
                for i in <$IntegerType>::MIN..<$IntegerType>::MAX {
                    assert!((i as $FloatType).ulps_eq(&(i as $FloatType), &0.0, &max_ulps));
                    assert!(ulps_eq!(i as $FloatType, i as $FloatType, abs_diff <= 0.0, ulps <= max_ulps));
                    assert_ulps_eq!(i as $FloatType, i as $FloatType, abs_diff <= 0.0, ulps <= max_ulps);
                }
            }

            #[test]
            fn test_ulps_ne_exactly_representable_exhaustive() {
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
