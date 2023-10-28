extern crate approx_compare;


#[cfg(test)]
mod relative_eq_f32_tests {
    use approx_compare::{
        assert_relative_eq,
        assert_relative_ne,
    };


    #[test]
    fn test_basic_eq() {
        assert_relative_eq!(1.0_f32, 1.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
    }

    #[test]
    fn test_basic_ne() {
        assert_relative_ne!(1.0_f32, 2.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
    }

    #[test]
    fn test_big() {
        assert_relative_eq!(100000000.0_f32, 100000001.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_eq!(100000001.0_f32, 100000000.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(10000.0_f32, 10001.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(10001.0_f32, 10000.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
    }

    #[test]
    fn test_big_neg() {
        assert_relative_eq!(-100000000.0_f32, -100000001.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_eq!(-100000001.0_f32, -100000000.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(-10000.0_f32, -10001.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(-10001.0_f32, -10000.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
    }

    #[test]
    fn test_mid() {
        assert_relative_eq!(1.0000001_f32, 1.0000002_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_eq!(1.0000002_f32, 1.0000001_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(1.000001_f32, 1.000002_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(1.000002_f32, 1.000001_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
    }

    #[test]
    fn test_mid_neg() {
        assert_relative_eq!(-1.0000001_f32, -1.0000002_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_eq!(-1.0000002_f32, -1.0000001_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(-1.000001_f32, -1.000002_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(-1.000002_f32, -1.000001_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
    }

    #[test]
    fn test_small() {
        assert_relative_eq!(0.000010001_f32, 0.000010002_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_eq!(0.000010002_f32, 0.000010001_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(0.000001002_f32, 0.0000001001_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(0.000001001_f32, 0.0000001002_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
    }

    #[test]
    fn test_small_neg() {
        assert_relative_eq!(-0.000010001_f32, -0.000010002_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_eq!(-0.000010002_f32, -0.000010001_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(-0.000001002_f32, -0.0000001001_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(-0.000001001_f32, -0.0000001002_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
    }

    #[test]
    fn test_zero() {
        assert_relative_eq!(0.0_f32, 0.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_eq!(0.0_f32, -0.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_eq!(-0.0_f32, -0.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);

        assert_relative_ne!(0.000001_f32, 0.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(0.0_f32, 0.000001_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(-0.000001_f32, 0.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(0.0_f32, -0.000001_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
    }

    #[test]
    fn test_tolerance() {
        assert_relative_eq!(0.0_f32, 1e-40_f32,  abs_diff <= 1e-40_f32, relative <= f32::EPSILON);
        assert_relative_eq!(1e-40_f32, 0.0_f32,  abs_diff <= 1e-40_f32, relative <= f32::EPSILON);
        assert_relative_eq!(0.0_f32, -1e-40_f32, abs_diff <= 1e-40_f32, relative <= f32::EPSILON);
        assert_relative_eq!(-1e-40_f32, 0.0_f32, abs_diff <= 1e-40_f32, relative <= f32::EPSILON);

        assert_relative_ne!(1e-40_f32, 0.0_f32,  abs_diff <= 1e-41_f32, relative <= f32::EPSILON);
        assert_relative_ne!(0.0_f32, 1e-40_f32,  abs_diff <= 1e-41_f32, relative <= f32::EPSILON);
        assert_relative_ne!(-1e-40_f32, 0.0_f32, abs_diff <= 1e-41_f32, relative <= f32::EPSILON);
        assert_relative_ne!(0.0_f32, -1e-40_f32, abs_diff <= 1e-41_f32, relative <= f32::EPSILON);
    }

    #[test]
    fn test_max() {
        assert_relative_eq!(f32::MAX, f32::MAX, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(f32::MAX, -f32::MAX, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(-f32::MAX, f32::MAX, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(
            f32::MAX, 
            f32::MAX / 2.0_f32, 
            abs_diff <= f32::EPSILON, 
            relative <= f32::EPSILON
        );
        assert_relative_ne!(
            f32::MAX, 
            -f32::MAX / 2.0_f32, 
            abs_diff <= f32::EPSILON, 
            relative <= f32::EPSILON
        );
        assert_relative_ne!(
            -f32::MAX, 
            f32::MAX / 2.0_f32, 
            abs_diff <= f32::EPSILON, 
            relative <= f32::EPSILON
        );
    }

    #[test]
    fn test_infinity() {
        assert_relative_eq!(f32::INFINITY, f32::INFINITY, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_eq!(-f32::INFINITY, -f32::INFINITY, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(-f32::INFINITY, f32::INFINITY, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
    }

    #[test]
    fn test_zero_infinity() {
        assert_relative_ne!(0_f32, f32::INFINITY, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(0_f32, -f32::INFINITY, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
    }

    #[test]
    fn test_nan() {
        assert_relative_ne!(f32::NAN, f32::NAN, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);

        assert_relative_ne!(f32::NAN, 0.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(-0.0_f32, f32::NAN, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(f32::NAN, -0.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(0.0_f32, f32::NAN, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);

        assert_relative_ne!(f32::NAN, f32::INFINITY, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(f32::INFINITY, f32::NAN, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(f32::NAN, -f32::INFINITY, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(-f32::INFINITY, f32::NAN, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);

        assert_relative_ne!(f32::NAN, f32::MAX, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(f32::MAX, f32::NAN, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(f32::NAN, -f32::MAX, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(-f32::MAX, f32::NAN, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);

        assert_relative_ne!(f32::NAN, f32::MIN_POSITIVE, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(f32::MIN_POSITIVE, f32::NAN, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(f32::NAN, -f32::MIN_POSITIVE, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(-f32::MIN_POSITIVE, f32::NAN, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
    }

    #[test]
    fn test_opposite_signs() {
        assert_relative_ne!(1.000000001_f32, -1.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(-1.0_f32, 1.000000001_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(-1.000000001_f32, 1.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(1.0_f32, -1.000000001_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);

        assert_relative_eq!(
            10.0_f32 *  f32::MIN_POSITIVE, 
            10.0_f32 * -f32::MIN_POSITIVE, 
            abs_diff <= f32::EPSILON, 
            relative <= f32::EPSILON
        );
    }

    #[test]
    fn test_close_to_zero() {
        assert_relative_eq!(f32::MIN_POSITIVE, f32::MIN_POSITIVE, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_eq!(f32::MIN_POSITIVE, -f32::MIN_POSITIVE, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_eq!(-f32::MIN_POSITIVE, f32::MIN_POSITIVE, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);

        assert_relative_eq!(f32::MIN_POSITIVE, 0.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_eq!(0.0_f32, f32::MIN_POSITIVE, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_eq!(-f32::MIN_POSITIVE, 0.0_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_eq!(0.0_f32, -f32::MIN_POSITIVE, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);

        assert_relative_ne!(0.000001_f32, -f32::MIN_POSITIVE, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(0.000001_f32, f32::MIN_POSITIVE, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(f32::MIN_POSITIVE, 0.000001_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
        assert_relative_ne!(-f32::MIN_POSITIVE, 0.000001_f32, abs_diff <= f32::EPSILON, relative <= f32::EPSILON);
    }
}


#[cfg(test)]
mod relative_eq_f64_tests {
    use approx_compare::{
        assert_relative_eq,
        assert_relative_ne,
    };


    #[test]
    fn test_basic_eq() {
        assert_relative_eq!(1.0_f64, 1.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
    }

    #[test]
    fn test_basic_ne() {
        assert_relative_ne!(1.0_f64, 2.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
    }

    #[test]
    fn test_big() {
        assert_relative_eq!(10000000000000000.0_f64, 10000000000000001.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_eq!(10000000000000001.0_f64, 10000000000000000.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(1000000000000000.0_f64, 1000000000000001.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(1000000000000001.0_f64, 1000000000000000.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
    }

    #[test]
    fn test_big_neg() {
        assert_relative_eq!(-10000000000000000.0_f64, -10000000000000001.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_eq!(-10000000000000001.0_f64, -10000000000000000.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(-1000000000000000.0_f64, -1000000000000001.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(-1000000000000001.0_f64, -1000000000000000.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
    }

    #[test]
    fn test_mid() {
        assert_relative_eq!(1.0000000000000001_f64, 1.0000000000000002_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_eq!(1.0000000000000002_f64, 1.0000000000000001_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(1.000000000000001_f64, 1.000000000000002_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(1.000000000000002_f64, 1.000000000000001_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
    }

    #[test]
    fn test_mid_neg() {
        assert_relative_eq!(-1.0000000000000001_f64, -1.0000000000000002_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_eq!(-1.0000000000000002_f64, -1.0000000000000001_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(-1.000000000000001_f64, -1.000000000000002_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(-1.000000000000002_f64, -1.000000000000001_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
    }

    #[test]
    fn test_small() {
        assert_relative_eq!(0.0000000100000001_f64, 0.0000000100000002_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_eq!(0.0000000100000002_f64, 0.0000000100000001_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(0.0000000100000001_f64, 0.0000000010000002_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(0.0000000100000002_f64, 0.0000000010000001_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
    }

    #[test]
    fn test_small_neg() {
        assert_relative_eq!(-0.0000000100000001_f64, -0.0000000100000002_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_eq!(-0.0000000100000002_f64, -0.0000000100000001_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(-0.0000000100000001_f64, -0.0000000010000002_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(-0.0000000100000002_f64, -0.0000000010000001_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
    }

    #[test]
    fn test_zero() {
        assert_relative_eq!(0.0_f64, 0.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_eq!(0.0_f64, -0.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_eq!(-0.0_f64, -0.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);

        assert_relative_ne!(0.000000000000001_f64, 0.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(0.0_f64, 0.000000000000001_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(-0.000000000000001_f64, 0.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(0.0_f64, -0.000000000000001_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
    }

    #[test]
    fn test_tolerance() {
        assert_relative_eq!(0.0_f64, 1e-40_f64,  abs_diff <= 1e-40_f64, relative <= f64::EPSILON);
        assert_relative_eq!(1e-40, 0.0_f64,      abs_diff <= 1e-40_f64, relative <= f64::EPSILON);
        assert_relative_eq!(0.0_f64, -1e-40_f64, abs_diff <= 1e-40_f64, relative <= f64::EPSILON);
        assert_relative_eq!(-1e-40_f64, 0.0_f64, abs_diff <= 1e-40_f64, relative <= f64::EPSILON);

        assert_relative_ne!(1e-40_f64, 0.0_f64,  abs_diff <= 1e-41_f64, relative <= f64::EPSILON);
        assert_relative_ne!(0.0_f64, 1e-40_f64,  abs_diff <= 1e-41_f64, relative <= f64::EPSILON);
        assert_relative_ne!(-1e-40_f64, 0.0_f64, abs_diff <= 1e-41_f64, relative <= f64::EPSILON);
        assert_relative_ne!(0.0_f64, -1e-40_f64, abs_diff <= 1e-41_f64, relative <= f64::EPSILON);
    }

    #[test]
    fn test_max() {
        assert_relative_eq!(f64::MAX, f64::MAX, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(f64::MAX, -f64::MAX, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(-f64::MAX, f64::MAX, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(
            f64::MAX,
            f64::MAX / 2.0_f64,
            abs_diff <= f64::EPSILON, 
            relative <= f64::EPSILON
        );
        assert_relative_ne!(
            f64::MAX,
            -f64::MAX / 2.0_f64,
            abs_diff <= f64::EPSILON, 
            relative <= f64::EPSILON
        );
        assert_relative_ne!(
            -f64::MAX,
            f64::MAX / 2.0_f64,
            abs_diff <= f64::EPSILON, 
            relative <= f64::EPSILON
        );
    }

    #[test]
    fn test_infinity() {
        assert_relative_eq!(f64::INFINITY, f64::INFINITY, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_eq!(-f64::INFINITY, -f64::INFINITY, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(-f64::INFINITY, f64::INFINITY, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
    }

    #[test]
    fn test_nan() {
        assert_relative_ne!(f64::NAN, f64::NAN, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);

        assert_relative_ne!(f64::NAN, 0.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(-0.0_f64, f64::NAN, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(f64::NAN, -0.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(0.0_f64, f64::NAN, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);

        assert_relative_ne!(f64::NAN, f64::INFINITY, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(f64::INFINITY, f64::NAN, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(f64::NAN, -f64::INFINITY, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(-f64::INFINITY, f64::NAN, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);

        assert_relative_ne!(f64::NAN, f64::MAX, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(f64::MAX, f64::NAN, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(f64::NAN, -f64::MAX, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(-f64::MAX, f64::NAN, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);

        assert_relative_ne!(f64::NAN, f64::MIN_POSITIVE, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(f64::MIN_POSITIVE, f64::NAN, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(f64::NAN, -f64::MIN_POSITIVE, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(-f64::MIN_POSITIVE, f64::NAN, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
    }

    #[test]
    fn test_opposite_signs() {
        assert_relative_ne!(1.000000001_f64, -1.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(-1.0_f64, 1.000000001_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(-1.000000001_f64, 1.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(1.0_f64, -1.000000001_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);

        assert_relative_eq!(
            10.0_f64 *  f64::MIN_POSITIVE,
            10.0_f64 * -f64::MIN_POSITIVE,
            abs_diff <= f64::EPSILON, 
            relative <= f64::EPSILON
        );
    }

    #[test]
    fn test_close_to_zero() {
        assert_relative_eq!(f64::MIN_POSITIVE, f64::MIN_POSITIVE, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_eq!(f64::MIN_POSITIVE, -f64::MIN_POSITIVE, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_eq!(-f64::MIN_POSITIVE, f64::MIN_POSITIVE, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);

        assert_relative_eq!(f64::MIN_POSITIVE, 0.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_eq!(0.0_f64, f64::MIN_POSITIVE, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_eq!(-f64::MIN_POSITIVE, 0.0_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_eq!(0.0_f64, -f64::MIN_POSITIVE, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);

        assert_relative_ne!(0.000000000000001_f64, -f64::MIN_POSITIVE, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(0.000000000000001_f64, f64::MIN_POSITIVE, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(f64::MIN_POSITIVE, 0.000000000000001_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
        assert_relative_ne!(-f64::MIN_POSITIVE, 0.000000000000001_f64, abs_diff <= f64::EPSILON, relative <= f64::EPSILON);
    }
}


#[cfg(test)]
mod abs_diff_array_tests {
    use approx_compare::{
        assert_relative_eq,
        assert_relative_ne,
    };


    #[test]
    fn test_basic_eq_array2() {
        let lhs = [1.0_f64, 1.0_f64];
        let rhs = [1.0_f64, 1.0_f64];

        assert_relative_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_ne_array2() {
        let lhs = [1.0_f64, 1.0_f64];
        let rhs = [2.0_f64, 3.0_f64];

        assert_relative_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_eq_array3() {
        let lhs = [1.0_f64, 1.0_f64, 1.0_f64];
        let rhs = [1.0_f64, 1.0_f64, 1.0_f64];

        assert_relative_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_ne_array3() {
        let lhs = [1.0_f64, 1.0_f64, 1.0_f64];
        let rhs = [2.0_f64, 3.0_f64, 4.0_f64];

        assert_relative_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_eq_array4() {
        let lhs = [1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64];
        let rhs = [1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64];

        assert_relative_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_ne_array4() {
        let lhs = [1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64];
        let rhs = [2.0_f64, 3.0_f64, 4.0_f64, 5.0_f64];

        assert_relative_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_eq_array8() {
        let lhs = [1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64];
        let rhs = [1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64];

        assert_relative_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON);
    }
    
    #[test]
    fn test_basic_ne_array8() {
        let lhs = [1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64];
        let rhs = [2.0_f64, 3.0_f64, 4.0_f64, 5.0_f64, 6.0_f64, 7.0_f64, 8.0_f64, 9.0_f64];

        assert_relative_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON);
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

        assert_relative_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON);
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

        assert_relative_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON);
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

        assert_relative_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON);
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

        assert_relative_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON);
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

        assert_relative_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON);
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

        assert_relative_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON, relative_all <= f64::EPSILON);
    }
}

