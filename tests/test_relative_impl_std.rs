extern crate approx_cmp;


#[cfg(test)]
mod relative_eq_box_tests {
    use approx_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
        AssertRelativeEq,
    };
    use std::boxed::Box;


    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = Box::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Box::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ];
        let max_relative = [
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ];

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne1() {
        let lhs = Box::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Box::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ];
        let max_relative = [
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ];

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne2() {
        let lhs = Box::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Box::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ];
        let max_relative = [eps; 8];

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = Box::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Box::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 4.0_f32 * f32::EPSILON;
        let max_relative = 4.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = Box::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Box::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff1() {
        let lhs = Box::new([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let abs_diff = [0.0000000_f32; 8];

        assert_eq!(lhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff2() {
        let lhs = Box::new([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let rhs = Box::new([
            1.10_f32, 1.15_f32, 1.70_f32, 1.80_f32,
            2.80_f32, 2.70_f32, 4.40_f32, 4.60_f32,
        ]);
        let abs_diff = [
            0.100000024_f32, 0.100000024_f32, 0.20000005_f32, 0.20000005_f32,
            0.299999950_f32, 0.299999950_f32, 0.40000010_f32, 0.40000010_f32,
        ];

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff3() {
        let lhs = Box::new([
            0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
            4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
        ]);
        let abs_diff = [0.00000000000000_f32; 8];

        assert_eq!(lhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff4() {
        let lhs = Box::new([
            0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
            4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
        ]);
        let rhs = Box::new([
            1.0000000_f32, 1.9999500_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000000_f32, 7.0000000_f32, 8.0000000_f32,
        ]);
        let abs_diff = [
            0.00005000829700_f32, 0.00004994869200_f32, 0.00005006790000_f32, 0.00000047683716_f32,
            0.00005006790000_f32, 0.00000000000000_f32, 0.00005006790000_f32, 0.00000000000000_f32,
        ];

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = Box::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Box::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = Box::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Box::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff_all = 0.20_f32;
        let max_abs_diff = [max_abs_diff_all; 4];

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = Box::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Box::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
    }

    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = Box::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Box::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative_all = 0.20_f32;
        let max_relative = [max_relative_all; 4];

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), max_relative);
    }
}


#[cfg(test)]
mod relative_eq_rc_tests {
    use approx_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
        AssertRelativeEq,
    };
    use std::rc::Rc;


    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = Rc::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Rc::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ];
        let max_relative = [
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ];

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne1() {
        let lhs = Rc::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Rc::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ];
        let max_relative = [
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ];

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne2() {
        let lhs = Rc::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Rc::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ];
        let max_relative = [eps; 8];

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = Rc::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Rc::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 4.0_f32 * f32::EPSILON;
        let max_relative = 4.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = Rc::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Rc::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff1() {
        let lhs = Rc::new([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let abs_diff = [0.0000000_f32; 8];

        assert_eq!(lhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff2() {
        let lhs = Rc::new([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let rhs = Rc::new([
            1.10_f32, 1.15_f32, 1.70_f32, 1.80_f32,
            2.80_f32, 2.70_f32, 4.40_f32, 4.60_f32,
        ]);
        let abs_diff = [
            0.100000024_f32, 0.100000024_f32, 0.20000005_f32, 0.20000005_f32,
            0.299999950_f32, 0.299999950_f32, 0.40000010_f32, 0.40000010_f32,
        ];

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff3() {
        let lhs = Rc::new([
            0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
            4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
        ]);
        let abs_diff = [0.00000000000000_f32; 8];

        assert_eq!(lhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff4() {
        let lhs = Rc::new([
            0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
            4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
        ]);
        let rhs = Rc::new([
            1.0000000_f32, 1.9999500_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000000_f32, 7.0000000_f32, 8.0000000_f32,
        ]);
        let abs_diff = [
            0.00005000829700_f32, 0.00004994869200_f32, 0.00005006790000_f32, 0.00000047683716_f32,
            0.00005006790000_f32, 0.00000000000000_f32, 0.00005006790000_f32, 0.00000000000000_f32,
        ];

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = Rc::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Rc::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = Rc::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Rc::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff_all = 0.20_f32;
        let max_abs_diff = [max_abs_diff_all; 4];

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = Rc::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Rc::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
    }

    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = Rc::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Rc::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative_all = 0.20_f32;
        let max_relative = [max_relative_all; 4];

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), max_relative);
    }
}


#[cfg(test)]
mod relative_eq_arc_tests {
    use approx_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
        AssertRelativeEq,
    };
    use std::sync::Arc;


    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = Arc::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Arc::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ];
        let max_relative = [
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ];

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne1() {
        let lhs = Arc::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Arc::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ];
        let max_relative = [
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ];

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne2() {
        let lhs = Arc::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Arc::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = [
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ];
        let max_relative = [eps; 8];

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = Arc::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Arc::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 4.0_f32 * f32::EPSILON;
        let max_relative = 4.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = Arc::new([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Arc::new([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff1() {
        let lhs = Arc::new([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let abs_diff = [0.0000000_f32; 8];

        assert_eq!(lhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff2() {
        let lhs = Arc::new([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let rhs = Arc::new([
            1.10_f32, 1.15_f32, 1.70_f32, 1.80_f32,
            2.80_f32, 2.70_f32, 4.40_f32, 4.60_f32,
        ]);
        let abs_diff = [
            0.100000024_f32, 0.100000024_f32, 0.20000005_f32, 0.20000005_f32,
            0.299999950_f32, 0.299999950_f32, 0.40000010_f32, 0.40000010_f32,
        ];

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff3() {
        let lhs = Arc::new([
            0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
            4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
        ]);
        let abs_diff = [0.00000000000000_f32; 8];

        assert_eq!(lhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff4() {
        let lhs = Arc::new([
            0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
            4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
        ]);
        let rhs = Arc::new([
            1.0000000_f32, 1.9999500_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000000_f32, 7.0000000_f32, 8.0000000_f32,
        ]);
        let abs_diff = [
            0.00005000829700_f32, 0.00004994869200_f32, 0.00005006790000_f32, 0.00000047683716_f32,
            0.00005006790000_f32, 0.00000000000000_f32, 0.00005006790000_f32, 0.00000000000000_f32,
        ];

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = Arc::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Arc::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = Arc::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Arc::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff_all = 0.20_f32;
        let max_abs_diff = [max_abs_diff_all; 4];

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = Arc::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Arc::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative = [0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32];

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
    }

    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = Arc::new([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Arc::new([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative_all = 0.20_f32;
        let max_relative = [max_relative_all; 4];

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), max_relative);
    }
}


#[cfg(test)]
mod relative_eq_vec_tests {
    use approx_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
        AssertRelativeEq,
    };


    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = Vec::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Vec::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = Vec::from([
            1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = Vec::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne1() {
        let lhs = Vec::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Vec::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = Vec::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = Vec::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne2() {
        let lhs = Vec::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Vec::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = Vec::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);
        let max_relative = Vec::from([eps; 8]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = Vec::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Vec::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 4.0_f32 * f32::EPSILON;
        let max_relative = 4.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = Vec::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Vec::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff1() {
        let lhs = Vec::from([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let abs_diff = Vec::from([0.0000000_f32; 8]);

        assert_eq!(lhs.debug_abs_diff(&lhs), Some(abs_diff));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff2() {
        let lhs = Vec::from([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let rhs = Vec::from([
            1.10_f32, 1.15_f32, 1.70_f32, 1.80_f32,
            2.80_f32, 2.70_f32, 4.40_f32, 4.60_f32,
        ]);
        let abs_diff = Vec::from([
            0.100000024_f32, 0.100000024_f32, 0.20000005_f32, 0.20000005_f32,
            0.299999950_f32, 0.299999950_f32, 0.40000010_f32, 0.40000010_f32,
        ]);

        assert_eq!(lhs.debug_abs_diff(&rhs), Some(abs_diff.clone()));
        assert_eq!(rhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff3() {
        let lhs = Vec::from([
            0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
            4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
        ]);
        let abs_diff = Vec::from([0.00000000000000_f32; 8]);

        assert_eq!(lhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff4() {
        let lhs = Vec::from([
            0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
            4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
        ]);
        let rhs = Vec::from([
            1.0000000_f32, 1.9999500_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000000_f32, 7.0000000_f32, 8.0000000_f32,
        ]);
        let abs_diff = Vec::from([
            0.00005000829700_f32, 0.00004994869200_f32, 0.00005006790000_f32, 0.00000047683716_f32,
            0.00005006790000_f32, 0.00000000000000_f32, 0.00005006790000_f32, 0.00000000000000_f32,
        ]);

        assert_eq!(lhs.debug_abs_diff(&rhs), Some(abs_diff.clone()));
        assert_eq!(rhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
    }

    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = Vec::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Vec::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff = Vec::from([0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32]);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), Some(max_abs_diff));
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = Vec::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Vec::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff_all = 0.20_f32;
        let max_abs_diff = Vec::from([max_abs_diff_all; 4]);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), Some(max_abs_diff));
    }

    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = Vec::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Vec::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative = Vec::from([0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32]);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), Some(max_relative));
    }

    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = Vec::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Vec::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative_all = 0.20_f32;
        let max_relative = Vec::from([max_relative_all; 4]);

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), Some(max_relative));
    }

    #[test]
    fn test_eq_empty1() {
        let lhs: Vec<f32> = Vec::new();
        let rhs: Vec<f32> = Vec::new();
        let max_abs_diff = Vec::new();
        let max_relative = Vec::new();

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty1() {
        let lhs: Vec<f32> = Vec::new();
        let rhs: Vec<f32> = Vec::new();
        let eps = f32::EPSILON;
        let max_abs_diff = Vec::from([
            1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = Vec::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty2() {
        let lhs: Vec<f32> = Vec::new();
        let rhs: Vec<f32> = Vec::new();
        let eps = f32::EPSILON;
        let max_abs_diff = Vec::from([
            1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = Vec::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty3() {
        let lhs: Vec<f32> = Vec::new();
        let rhs: Vec<f32> = Vec::new();
        let eps = f32::EPSILON;
        let max_abs_diff = Vec::new();
        let max_relative = Vec::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty4() {
        let lhs = Vec::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Vec::new();
        let eps = f32::EPSILON;
        let max_abs_diff = Vec::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = Vec::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty5() {
        let lhs = Vec::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Vec::new();
        let eps = f32::EPSILON;
        let max_abs_diff = Vec::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = Vec::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty6() {
        let lhs = Vec::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Vec::new();
        let eps = f32::EPSILON;
        let max_abs_diff = Vec::new();
        let max_relative = Vec::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty7() {
        let lhs = Vec::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Vec::new();
        let max_abs_diff = Vec::new();
        let max_relative = Vec::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty8() {
        let lhs: Vec<f32> = Vec::new();
        let rhs = Vec::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = Vec::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);
        let max_relative = Vec::from([eps; 8]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty9() {
        let lhs: Vec<f32> = Vec::new();
        let rhs = Vec::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = Vec::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);
        let max_relative = Vec::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty10() {
        let lhs: Vec<f32> = Vec::new();
        let rhs = Vec::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = Vec::new();
        let max_relative = Vec::from([eps; 8]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty11() {
        let lhs: Vec<f32> = Vec::new();
        let rhs = Vec::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = Vec::new();
        let max_relative = Vec::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty12() {
        let lhs = Vec::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Vec::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = Vec::new();
        let max_relative = Vec::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty13() {
        let lhs = Vec::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Vec::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = Vec::from([
            1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = Vec::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty14() {
        let lhs = Vec::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Vec::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = Vec::new();
        let max_relative = Vec::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq_empty() {
        let lhs: Vec<f32> = Vec::new();
        let rhs: Vec<f32> = Vec::new();
        let max_abs_diff = 4.0_f32 * f32::EPSILON;
        let max_relative = 4.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne_empty1() {
        let lhs = Vec::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = Vec::new();
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne_empty2() {
        let lhs: Vec<f32> = Vec::new();
        let rhs = Vec::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_empty1() {
        let lhs = Vec::from([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let rhs = Vec::new();

        assert_eq!(lhs.debug_abs_diff(&rhs), None);
        assert_eq!(rhs.debug_abs_diff(&lhs), None);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_empty2() {
        let lhs = Vec::new();
        let rhs = Vec::from([
            1.0000000_f32, 1.9999500_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000000_f32, 7.0000000_f32, 8.0000000_f32,
        ]);

        assert_eq!(lhs.debug_abs_diff(&rhs), None);
        assert_eq!(rhs.debug_abs_diff(&lhs), None);
    }

    #[test]
    fn test_debug_abs_diff_tolerance_empty() {
        let lhs = Vec::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Vec::new();
        let max_abs_diff = Vec::from([0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32]);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), None);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), None);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance_empty() {
        let lhs: Vec<f32> = Vec::new();
        let rhs = Vec::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff_all = 0.20_f32;

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), None);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), None);
    }

    #[test]
    fn test_debug_relative_tolerance_empty() {
        let lhs = Vec::new();
        let rhs = Vec::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative = Vec::from([0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32]);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), None);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), None);
    }

    #[test]
    fn test_debug_relative_all_tolerance_empty() {
        let lhs = Vec::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = Vec::new();
        let max_relative_all = 0.20_f32;

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), None);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative_all), None);
    }
}


#[cfg(test)]
mod relative_eq_vecdeque_tests {
    use approx_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
        AssertRelativeEq,
    };
    use std::collections::VecDeque;


    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = VecDeque::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = VecDeque::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = VecDeque::from([
            1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = VecDeque::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne1() {
        let lhs = VecDeque::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = VecDeque::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = VecDeque::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = VecDeque::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne2() {
        let lhs = VecDeque::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = VecDeque::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = VecDeque::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);
        let max_relative = VecDeque::from([eps; 8]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = VecDeque::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = VecDeque::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 4.0_f32 * f32::EPSILON;
        let max_relative = 4.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = VecDeque::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = VecDeque::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff1() {
        let lhs = VecDeque::from([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let abs_diff = VecDeque::from([0.0000000_f32; 8]);

        assert_eq!(lhs.debug_abs_diff(&lhs), Some(abs_diff));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff2() {
        let lhs = VecDeque::from([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let rhs = VecDeque::from([
            1.10_f32, 1.15_f32, 1.70_f32, 1.80_f32,
            2.80_f32, 2.70_f32, 4.40_f32, 4.60_f32,
        ]);
        let abs_diff = VecDeque::from([
            0.100000024_f32, 0.100000024_f32, 0.20000005_f32, 0.20000005_f32,
            0.299999950_f32, 0.299999950_f32, 0.40000010_f32, 0.40000010_f32,
        ]);

        assert_eq!(lhs.debug_abs_diff(&rhs), Some(abs_diff.clone()));
        assert_eq!(rhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff3() {
        let lhs = VecDeque::from([
            0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
            4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
        ]);
        let abs_diff = VecDeque::from([0.00000000000000_f32; 8]);

        assert_eq!(lhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff4() {
        let lhs = VecDeque::from([
            0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
            4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
        ]);
        let rhs = VecDeque::from([
            1.0000000_f32, 1.9999500_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000000_f32, 7.0000000_f32, 8.0000000_f32,
        ]);
        let abs_diff = VecDeque::from([
            0.00005000829700_f32, 0.00004994869200_f32, 0.00005006790000_f32, 0.00000047683716_f32,
            0.00005006790000_f32, 0.00000000000000_f32, 0.00005006790000_f32, 0.00000000000000_f32,
        ]);

        assert_eq!(lhs.debug_abs_diff(&rhs), Some(abs_diff.clone()));
        assert_eq!(rhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
    }

    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = VecDeque::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = VecDeque::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff = VecDeque::from([0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32]);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), Some(max_abs_diff));
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = VecDeque::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = VecDeque::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff_all = 0.20_f32;
        let max_abs_diff = VecDeque::from([max_abs_diff_all; 4]);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), Some(max_abs_diff));
    }

    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = VecDeque::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = VecDeque::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative = VecDeque::from([0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32]);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), Some(max_relative));
    }

    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = VecDeque::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = VecDeque::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative_all = 0.20_f32;
        let max_relative = VecDeque::from([max_relative_all; 4]);

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), Some(max_relative));
    }

    #[test]
    fn test_eq_empty() {
        let lhs: VecDeque<f32> = VecDeque::new();
        let rhs: VecDeque<f32> = VecDeque::new();
        let max_abs_diff = VecDeque::new();
        let max_relative = VecDeque::new();

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_empty1() {
        let lhs: VecDeque<f32> = VecDeque::new();
        let rhs: VecDeque<f32> = VecDeque::new();
        let max_abs_diff = VecDeque::new();
        let max_relative = VecDeque::new();

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty1() {
        let lhs: VecDeque<f32> = VecDeque::new();
        let rhs: VecDeque<f32> = VecDeque::new();
        let eps = f32::EPSILON;
        let max_abs_diff = VecDeque::from([
            1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = VecDeque::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty2() {
        let lhs: VecDeque<f32> = VecDeque::new();
        let rhs: VecDeque<f32> = VecDeque::new();
        let eps = f32::EPSILON;
        let max_abs_diff = VecDeque::from([
            1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = VecDeque::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty3() {
        let lhs: VecDeque<f32> = VecDeque::new();
        let rhs: VecDeque<f32> = VecDeque::new();
        let eps = f32::EPSILON;
        let max_abs_diff = VecDeque::new();
        let max_relative = VecDeque::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty4() {
        let lhs = VecDeque::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = VecDeque::new();
        let eps = f32::EPSILON;
        let max_abs_diff = VecDeque::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = VecDeque::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty5() {
        let lhs = VecDeque::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = VecDeque::new();
        let eps = f32::EPSILON;
        let max_abs_diff = VecDeque::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = VecDeque::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty6() {
        let lhs = VecDeque::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = VecDeque::new();
        let eps = f32::EPSILON;
        let max_abs_diff = VecDeque::new();
        let max_relative = VecDeque::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty7() {
        let lhs = VecDeque::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = VecDeque::new();
        let max_abs_diff = VecDeque::new();
        let max_relative = VecDeque::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty8() {
        let lhs: VecDeque<f32> = VecDeque::new();
        let rhs = VecDeque::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = VecDeque::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);
        let max_relative = VecDeque::from([eps; 8]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty9() {
        let lhs: VecDeque<f32> = VecDeque::new();
        let rhs = VecDeque::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = VecDeque::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);
        let max_relative = VecDeque::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty10() {
        let lhs: VecDeque<f32> = VecDeque::new();
        let rhs = VecDeque::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = VecDeque::new();
        let max_relative = VecDeque::from([eps; 8]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty11() {
        let lhs: VecDeque<f32> = VecDeque::new();
        let rhs = VecDeque::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = VecDeque::new();
        let max_relative = VecDeque::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty12() {
        let lhs = VecDeque::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = VecDeque::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = VecDeque::new();
        let max_relative = VecDeque::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty13() {
        let lhs = VecDeque::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = VecDeque::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = VecDeque::from([
            1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = VecDeque::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty14() {
        let lhs = VecDeque::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = VecDeque::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = VecDeque::new();
        let max_relative = VecDeque::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq_empty() {
        let lhs: VecDeque<f32> = VecDeque::new();
        let rhs: VecDeque<f32> = VecDeque::new();
        let max_abs_diff = 4.0_f32 * f32::EPSILON;
        let max_relative = 4.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne_empty1() {
        let lhs = VecDeque::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = VecDeque::new();
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne_empty2() {
        let lhs: VecDeque<f32> = VecDeque::new();
        let rhs = VecDeque::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_empty1() {
        let lhs = VecDeque::from([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let rhs = VecDeque::new();

        assert_eq!(lhs.debug_abs_diff(&rhs), None);
        assert_eq!(rhs.debug_abs_diff(&lhs), None);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_empty2() {
        let lhs = VecDeque::new();
        let rhs = VecDeque::from([
            1.0000000_f32, 1.9999500_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000000_f32, 7.0000000_f32, 8.0000000_f32,
        ]);

        assert_eq!(lhs.debug_abs_diff(&rhs), None);
        assert_eq!(rhs.debug_abs_diff(&lhs), None);
    }

    #[test]
    fn test_debug_abs_diff_tolerance_empty() {
        let lhs = VecDeque::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = VecDeque::new();
        let max_abs_diff = VecDeque::from([0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32]);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), None);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), None);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance_empty() {
        let lhs: VecDeque<f32> = VecDeque::new();
        let rhs = VecDeque::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff_all = 0.20_f32;

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), None);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), None);
    }

    #[test]
    fn test_debug_relative_tolerance_empty() {
        let lhs = VecDeque::new();
        let rhs = VecDeque::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative = VecDeque::from([0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32]);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), None);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), None);
    }

    #[test]
    fn test_debug_relative_all_tolerance_empty() {
        let lhs = VecDeque::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = VecDeque::new();
        let max_relative_all = 0.20_f32;

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), None);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative_all), None);
    }
}


#[cfg(test)]
mod relative_eq_linked_list_tests {
    use approx_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
        AssertRelativeEq,
    };
    use std::collections::LinkedList;


    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = LinkedList::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = LinkedList::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = LinkedList::from([
            1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = LinkedList::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne1() {
        let lhs = LinkedList::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = LinkedList::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = LinkedList::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = LinkedList::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne2() {
        let lhs = LinkedList::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = LinkedList::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = LinkedList::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);
        let max_relative = LinkedList::from([eps; 8]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = LinkedList::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = LinkedList::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 4.0_f32 * f32::EPSILON;
        let max_relative = 4.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = LinkedList::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = LinkedList::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff1() {
        let lhs = LinkedList::from([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let abs_diff = LinkedList::from([0.0000000_f32; 8]);

        assert_eq!(lhs.debug_abs_diff(&lhs), Some(abs_diff));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff2() {
        let lhs = LinkedList::from([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let rhs = LinkedList::from([
            1.10_f32, 1.15_f32, 1.70_f32, 1.80_f32,
            2.80_f32, 2.70_f32, 4.40_f32, 4.60_f32,
        ]);
        let abs_diff = LinkedList::from([
            0.100000024_f32, 0.100000024_f32, 0.20000005_f32, 0.20000005_f32,
            0.299999950_f32, 0.299999950_f32, 0.40000010_f32, 0.40000010_f32,
        ]);

        assert_eq!(lhs.debug_abs_diff(&rhs), Some(abs_diff.clone()));
        assert_eq!(rhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff3() {
        let lhs = LinkedList::from([
            0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
            4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
        ]);
        let abs_diff = LinkedList::from([0.00000000000000_f32; 8]);

        assert_eq!(lhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff4() {
        let lhs = LinkedList::from([
            0.9999500_f32, 2.0000000_f32, 2.9999500_f32, 4.0000000_f32,
            4.9999500_f32, 6.0000000_f32, 6.9999500_f32, 8.0000000_f32,
        ]);
        let rhs = LinkedList::from([
            1.0000000_f32, 1.9999500_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000000_f32, 7.0000000_f32, 8.0000000_f32,
        ]);
        let abs_diff = LinkedList::from([
            0.00005000829700_f32, 0.00004994869200_f32, 0.00005006790000_f32, 0.00000047683716_f32,
            0.00005006790000_f32, 0.00000000000000_f32, 0.00005006790000_f32, 0.00000000000000_f32,
        ]);

        assert_eq!(lhs.debug_abs_diff(&rhs), Some(abs_diff.clone()));
        assert_eq!(rhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
    }

    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = LinkedList::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = LinkedList::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff = LinkedList::from([0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32]);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), Some(max_abs_diff));
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = LinkedList::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = LinkedList::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff_all = 0.20_f32;
        let max_abs_diff = LinkedList::from([max_abs_diff_all; 4]);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), Some(max_abs_diff));
    }

    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = LinkedList::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = LinkedList::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative = LinkedList::from([0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32]);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), Some(max_relative));
    }

    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = LinkedList::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = LinkedList::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative_all = 0.20_f32;
        let max_relative = LinkedList::from([max_relative_all; 4]);

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), Some(max_relative));
    }

    #[rustfmt::skip]
    #[test]
    fn test_eq_empty1() {
        let lhs: LinkedList<f32> = LinkedList::new();
        let rhs: LinkedList<f32> = LinkedList::new();
        let max_abs_diff = LinkedList::new();
        let max_relative = LinkedList::new();

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty1() {
        let lhs: LinkedList<f32> = LinkedList::new();
        let rhs: LinkedList<f32> = LinkedList::new();
        let eps = f32::EPSILON;
        let max_abs_diff = LinkedList::from([
            1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = LinkedList::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty2() {
        let lhs: LinkedList<f32> = LinkedList::new();
        let rhs: LinkedList<f32> = LinkedList::new();
        let eps = f32::EPSILON;
        let max_abs_diff = LinkedList::from([
            1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = LinkedList::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty3() {
        let lhs: LinkedList<f32> = LinkedList::new();
        let rhs: LinkedList<f32> = LinkedList::new();
        let eps = f32::EPSILON;
        let max_abs_diff = LinkedList::new();
        let max_relative = LinkedList::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty4() {
        let lhs = LinkedList::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = LinkedList::new();
        let eps = f32::EPSILON;
        let max_abs_diff = LinkedList::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = LinkedList::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty5() {
        let lhs = LinkedList::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = LinkedList::new();
        let eps = f32::EPSILON;
        let max_abs_diff = LinkedList::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = LinkedList::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty6() {
        let lhs = LinkedList::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = LinkedList::new();
        let eps = f32::EPSILON;
        let max_abs_diff = LinkedList::new();
        let max_relative = LinkedList::from([
            0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            0.5_f32 * eps, 0.5_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty7() {
        let lhs = LinkedList::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = LinkedList::new();
        let max_abs_diff = LinkedList::new();
        let max_relative = LinkedList::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty8() {
        let lhs: LinkedList<f32> = LinkedList::new();
        let rhs = LinkedList::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = LinkedList::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);
        let max_relative = LinkedList::from([eps; 8]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty9() {
        let lhs: LinkedList<f32> = LinkedList::new();
        let rhs = LinkedList::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = LinkedList::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);
        let max_relative = LinkedList::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty10() {
        let lhs: LinkedList<f32> = LinkedList::new();
        let rhs = LinkedList::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = LinkedList::new();
        let max_relative = LinkedList::from([eps; 8]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty11() {
        let lhs: LinkedList<f32> = LinkedList::new();
        let rhs = LinkedList::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = LinkedList::new();
        let max_relative = LinkedList::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty12() {
        let lhs = LinkedList::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = LinkedList::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = LinkedList::new();
        let max_relative = LinkedList::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty13() {
        let lhs = LinkedList::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = LinkedList::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = LinkedList::from([
            1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 4.0_f32 * eps, 4.0_f32 * eps,
        ]);
        let max_relative = LinkedList::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty14() {
        let lhs = LinkedList::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = LinkedList::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let eps = f32::EPSILON;
        let max_abs_diff = LinkedList::new();
        let max_relative = LinkedList::from([
            1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
            1.0_f32 * eps, 1.0_f32 * eps, 2.0_f32 * eps, 2.0_f32 * eps,
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq_empty() {
        let lhs: LinkedList<f32> = LinkedList::new();
        let rhs: LinkedList<f32> = LinkedList::new();
        let max_abs_diff = 4.0_f32 * f32::EPSILON;
        let max_relative = 4.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne_empty1() {
        let lhs = LinkedList::from([
            0.9999999_f32, 2.0000000_f32, 2.9999995_f32, 4.0000000_f32,
            4.9999999_f32, 6.0000000_f32, 6.9999995_f32, 8.0000000_f32,
        ]);
        let rhs = LinkedList::new();
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne_empty2() {
        let lhs: LinkedList<f32> = LinkedList::new();
        let rhs = LinkedList::from([
            1.0000000_f32, 1.9999995_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000001_f32, 7.0000000_f32, 7.9999995_f32,
        ]);
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_empty1() {
        let lhs = LinkedList::from([
            1.00_f32, 1.25_f32, 1.50_f32, 2.00_f32,
            2.50_f32, 3.00_f32, 4.00_f32, 5.00_f32,
        ]);
        let rhs = LinkedList::new();

        assert_eq!(lhs.debug_abs_diff(&rhs), None);
        assert_eq!(rhs.debug_abs_diff(&lhs), None);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_empty2() {
        let lhs = LinkedList::new();
        let rhs = LinkedList::from([
            1.0000000_f32, 1.9999500_f32, 3.0000000_f32, 4.0000005_f32,
            5.0000000_f32, 6.0000000_f32, 7.0000000_f32, 8.0000000_f32,
        ]);

        assert_eq!(lhs.debug_abs_diff(&rhs), None);
        assert_eq!(rhs.debug_abs_diff(&lhs), None);
    }

    #[test]
    fn test_debug_abs_diff_tolerance_empty() {
        let lhs = LinkedList::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = LinkedList::new();
        let max_abs_diff = LinkedList::from([0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32]);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), None);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), None);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance_empty() {
        let lhs: LinkedList<f32> = LinkedList::new();
        let rhs = LinkedList::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_abs_diff_all = 0.20_f32;

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), None);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), None);
    }

    #[test]
    fn test_debug_relative_tolerance_empty() {
        let lhs = LinkedList::new();
        let rhs = LinkedList::from([2.50_f32, 3.00_f32, 4.00_f32, 6.00_f32]);
        let max_relative = LinkedList::from([0.10_f32, 0.20_f32, 0.30_f32, 0.40_f32]);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), None);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), None);
    }

    #[test]
    fn test_debug_relative_all_tolerance_empty() {
        let lhs = LinkedList::from([2.00_f32, 3.25_f32, 4.50_f32, 5.75_f32]);
        let rhs = LinkedList::new();
        let max_relative_all = 0.20_f32;

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), None);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative_all), None);
    }
}


#[cfg(test)]
mod relative_eq_hash_map_tests {
    use approx_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
        AssertRelativeEq,
    };
    use std::collections::HashMap;


    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = HashMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = HashMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = HashMap::from([
            ("0", 2.0_f32 * f32::EPSILON),
            ("1", 1.0_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 1.0_f32 * f32::EPSILON),
            ("6", 1.0_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);
        let max_relative = HashMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 1.0_f32 * f32::EPSILON),
            ("2", 2.0_f32 * f32::EPSILON),
            ("3", 2.0_f32 * f32::EPSILON),
            ("4", 2.0_f32 * f32::EPSILON),
            ("5", 1.0_f32 * f32::EPSILON),
            ("6", 1.0_f32 * f32::EPSILON),
            ("7", 2.0_f32 * f32::EPSILON),
            ("8", 2.0_f32 * f32::EPSILON),
        ]);

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne1() {
        let lhs = HashMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = HashMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = HashMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);
        let max_relative = HashMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne2() {
        let lhs = HashMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = HashMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = HashMap::from([
            ("0", 2.0_f32 * f32::EPSILON),
            ("1", 1.0_f32 * f32::EPSILON),
            ("2", 2.0_f32 * f32::EPSILON),
            ("3", 2.0_f32 * f32::EPSILON),
            ("4", 2.0_f32 * f32::EPSILON),
            ("5", 1.0_f32 * f32::EPSILON),
            ("6", 1.0_f32 * f32::EPSILON),
            ("7", 2.0_f32 * f32::EPSILON),
            ("8", 2.0_f32 * f32::EPSILON),
        ]);
        let max_relative = HashMap::from([
            ("0", f32::EPSILON),
            ("1", f32::EPSILON),
            ("2", f32::EPSILON),
            ("3", f32::EPSILON),
            ("4", f32::EPSILON),
            ("5", f32::EPSILON),
            ("6", f32::EPSILON),
            ("7", f32::EPSILON),
            ("8", f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = HashMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = HashMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = 4.0_f32 * f32::EPSILON;
        let max_relative = 4.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = HashMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = HashMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff1() {
        let lhs = HashMap::from([
            ("0", 0.00_f32),
            ("1", 1.00_f32),
            ("2", 1.25_f32),
            ("3", 1.50_f32),
            ("4", 2.00_f32),
            ("5", 2.50_f32),
            ("6", 3.00_f32),
            ("7", 4.00_f32),
            ("8", 5.00_f32),
        ]);
        let abs_diff = HashMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.0000000_f32),
            ("2", 0.0000000_f32),
            ("3", 0.0000000_f32),
            ("4", 0.0000000_f32),
            ("5", 0.0000000_f32),
            ("6", 0.0000000_f32),
            ("7", 0.0000000_f32),
            ("8", 0.0000000_f32),
        ]);

        assert_eq!(lhs.debug_abs_diff(&lhs), Some(abs_diff));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff2() {
        let lhs = HashMap::from([
            ("0", 0.00_f32),
            ("1", 1.00_f32),
            ("2", 1.25_f32),
            ("3", 1.50_f32),
            ("4", 2.00_f32),
            ("5", 2.50_f32),
            ("6", 3.00_f32),
            ("7", 4.00_f32),
            ("8", 5.00_f32),
        ]);
        let rhs = HashMap::from([
            ("0", 0.10_f32),
            ("1", 1.10_f32),
            ("2", 1.15_f32),
            ("3", 1.70_f32),
            ("4", 1.80_f32),
            ("5", 2.80_f32),
            ("6", 2.70_f32),
            ("7", 4.40_f32),
            ("8", 4.60_f32),
        ]);
        let abs_diff = HashMap::from([
            ("0", 0.100000000_f32),
            ("1", 0.100000024_f32),
            ("2", 0.100000024_f32),
            ("3", 0.200000050_f32),
            ("4", 0.200000050_f32),
            ("5", 0.299999950_f32),
            ("6", 0.299999950_f32),
            ("7", 0.400000100_f32),
            ("8", 0.400000100_f32),
        ]);

        assert_eq!(lhs.debug_abs_diff(&rhs), Some(abs_diff.clone()));
        assert_eq!(rhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff3() {
        let lhs = HashMap::from([
            ("0", 0.0000500_f32),
            ("1", 0.9999500_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999500_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999500_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999500_f32),
            ("8", 8.0000000_f32),
        ]);
        let abs_diff = HashMap::from([
            ("0", 0.00000000000000_f32),
            ("1", 0.00000000000000_f32),
            ("2", 0.00000000000000_f32),
            ("3", 0.00000000000000_f32),
            ("4", 0.00000000000000_f32),
            ("5", 0.00000000000000_f32),
            ("6", 0.00000000000000_f32),
            ("7", 0.00000000000000_f32),
            ("8", 0.00000000000000_f32),
        ]);

        assert_eq!(lhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff4() {
        let lhs = HashMap::from([
            ("0", 0.0000500_f32),
            ("1", 0.9999500_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999500_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999500_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999500_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = HashMap::from([
            ("0", 0.0000000_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999500_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000000_f32),
            ("7", 7.0000000_f32),
            ("8", 8.0000000_f32),
        ]);
        let abs_diff = HashMap::from([
            ("0", 0.00005000000000_f32),
            ("1", 0.00005000829700_f32),
            ("2", 0.00004994869200_f32),
            ("3", 0.00005006790000_f32),
            ("4", 0.00000047683716_f32),
            ("5", 0.00005006790000_f32),
            ("6", 0.00000000000000_f32),
            ("7", 0.00005006790000_f32),
            ("8", 0.00000000000000_f32),
        ]);

        assert_eq!(lhs.debug_abs_diff(&rhs), Some(abs_diff.clone()));
        assert_eq!(rhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = HashMap::from([
            ("0", 2.00_f32),
            ("1", 3.25_f32),
            ("2", 4.50_f32),
            ("3", 5.75_f32),
        ]);
        let rhs = HashMap::from([
            ("0", 2.50_f32),
            ("1", 3.00_f32),
            ("2", 4.00_f32),
            ("3", 6.00_f32),
        ]);
        let max_abs_diff = HashMap::from([
            ("0", 0.10_f32),
            ("1", 0.20_f32),
            ("2", 0.30_f32),
            ("3", 0.40_f32),
        ]);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), Some(max_abs_diff));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = HashMap::from([
            ("0", 2.00_f32),
            ("1", 3.25_f32),
            ("2", 4.50_f32),
            ("3", 5.75_f32),
        ]);
        let rhs = HashMap::from([
            ("0", 2.50_f32),
            ("1", 3.00_f32),
            ("2", 4.00_f32),
            ("3", 6.00_f32),
        ]);
        let max_abs_diff_all = 0.20_f32;
        let max_abs_diff = HashMap::from([
            ("0", max_abs_diff_all),
            ("1", max_abs_diff_all),
            ("2", max_abs_diff_all),
            ("3", max_abs_diff_all),
        ]);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), Some(max_abs_diff));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = HashMap::from([
            ("0", 2.00_f32),
            ("1", 3.25_f32),
            ("2", 4.50_f32),
            ("3", 5.75_f32),
        ]);
        let rhs = HashMap::from([
            ("0", 2.50_f32),
            ("1", 3.00_f32),
            ("2", 4.00_f32),
            ("3", 6.00_f32),
        ]);
        let max_relative = HashMap::from([
            ("0", 0.10_f32),
            ("1", 0.20_f32),
            ("2", 0.30_f32),
            ("3", 0.40_f32),
        ]);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), Some(max_relative));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = HashMap::from([
            ("0", 2.00_f32),
            ("1", 3.25_f32),
            ("2", 4.50_f32),
            ("3", 5.75_f32),
        ]);
        let rhs = HashMap::from([
            ("0", 2.50_f32),
            ("1", 3.00_f32),
            ("2", 4.00_f32),
            ("3", 6.00_f32),
        ]);
        let max_relative_all = 0.20_f32;
        let max_relative = HashMap::from([
            ("0", max_relative_all),
            ("1", max_relative_all),
            ("2", max_relative_all),
            ("3", max_relative_all),
        ]);

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), Some(max_relative));
    }

    #[test]
    fn test_eq_empty1() {
        let lhs: HashMap<&str, f32> = HashMap::new();
        let rhs: HashMap<&str, f32> = HashMap::new();
        let max_abs_diff = HashMap::new();
        let max_relative = HashMap::new();

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty1() {
        let lhs: HashMap<&str, f32> = HashMap::new();
        let rhs: HashMap<&str, f32> = HashMap::new();
        let max_abs_diff = HashMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);
        let max_relative = HashMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty2() {
        let lhs: HashMap<&str, f32> = HashMap::new();
        let rhs: HashMap<&str, f32> = HashMap::new();
        let max_abs_diff = HashMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);
        let max_relative = HashMap::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty3() {
        let lhs: HashMap<&str, f32> = HashMap::new();
        let rhs: HashMap<&str, f32> = HashMap::new();
        let max_abs_diff = HashMap::new();
        let max_relative = HashMap::from([
            ("0", 2.0_f32 * f32::EPSILON),
            ("1", 1.0_f32 * f32::EPSILON),
            ("2", 2.0_f32 * f32::EPSILON),
            ("3", 2.0_f32 * f32::EPSILON),
            ("4", 2.0_f32 * f32::EPSILON),
            ("5", 1.0_f32 * f32::EPSILON),
            ("6", 1.0_f32 * f32::EPSILON),
            ("7", 2.0_f32 * f32::EPSILON),
            ("8", 2.0_f32 * f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty4() {
        let lhs = HashMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = HashMap::new();
        let max_abs_diff = HashMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);
        let max_relative = HashMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty5() {
        let lhs = HashMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = HashMap::new();
        let max_abs_diff = HashMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);
        let max_relative = HashMap::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty6() {
        let lhs = HashMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = HashMap::new();
        let max_abs_diff = HashMap::new();
        let max_relative = HashMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty7() {
        let lhs = HashMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = HashMap::new();
        let max_abs_diff = HashMap::new();
        let max_relative = HashMap::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty8() {
        let lhs: HashMap<&str, f32> = HashMap::new();
        let rhs = HashMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = HashMap::from([
            ("0", 2.0_f32 * f32::EPSILON),
            ("1", 1.0_f32 * f32::EPSILON),
            ("2", 2.0_f32 * f32::EPSILON),
            ("3", 2.0_f32 * f32::EPSILON),
            ("4", 2.0_f32 * f32::EPSILON),
            ("5", 1.0_f32 * f32::EPSILON),
            ("6", 1.0_f32 * f32::EPSILON),
            ("7", 2.0_f32 * f32::EPSILON),
            ("8", 2.0_f32 * f32::EPSILON),
        ]);
        let max_relative = HashMap::from([
            ("0", f32::EPSILON),
            ("1", f32::EPSILON),
            ("2", f32::EPSILON),
            ("3", f32::EPSILON),
            ("4", f32::EPSILON),
            ("5", f32::EPSILON),
            ("6", f32::EPSILON),
            ("7", f32::EPSILON),
            ("8", f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty9() {
        let lhs: HashMap<&str, f32> = HashMap::new();
        let rhs = HashMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = HashMap::from([
            ("0", 2.0_f32 * f32::EPSILON),
            ("1", 1.0_f32 * f32::EPSILON),
            ("2", 2.0_f32 * f32::EPSILON),
            ("3", 2.0_f32 * f32::EPSILON),
            ("4", 2.0_f32 * f32::EPSILON),
            ("5", 1.0_f32 * f32::EPSILON),
            ("6", 1.0_f32 * f32::EPSILON),
            ("7", 2.0_f32 * f32::EPSILON),
            ("8", 2.0_f32 * f32::EPSILON),
        ]);
        let max_relative = HashMap::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty10() {
        let lhs: HashMap<&str, f32> = HashMap::new();
        let rhs = HashMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = HashMap::new();
        let max_relative = HashMap::from([
            ("0", f32::EPSILON),
            ("1", f32::EPSILON),
            ("2", f32::EPSILON),
            ("3", f32::EPSILON),
            ("4", f32::EPSILON),
            ("5", f32::EPSILON),
            ("6", f32::EPSILON),
            ("7", f32::EPSILON),
            ("8", f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty11() {
        let lhs: HashMap<&str, f32> = HashMap::new();
        let rhs = HashMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = HashMap::new();
        let max_relative = HashMap::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty12() {
        let lhs = HashMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = HashMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = HashMap::new();
        let max_relative = HashMap::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty13() {
        let lhs = HashMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = HashMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = HashMap::from([
            ("0", 2.0_f32 * f32::EPSILON),
            ("1", 1.0_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 1.0_f32 * f32::EPSILON),
            ("6", 1.0_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);
        let max_relative = HashMap::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty14() {
        let lhs = HashMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = HashMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = HashMap::new();
        let max_relative = HashMap::from([
            ("0", 2.0_f32 * f32::EPSILON),
            ("1", 1.0_f32 * f32::EPSILON),
            ("2", 2.0_f32 * f32::EPSILON),
            ("3", 2.0_f32 * f32::EPSILON),
            ("4", 2.0_f32 * f32::EPSILON),
            ("5", 1.0_f32 * f32::EPSILON),
            ("6", 1.0_f32 * f32::EPSILON),
            ("7", 2.0_f32 * f32::EPSILON),
            ("8", 2.0_f32 * f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_all_eq_empty() {
        let lhs: HashMap<&str, f32> = HashMap::new();
        let rhs: HashMap<&str, f32> = HashMap::new();
        let max_abs_diff = 4.0_f32 * f32::EPSILON;
        let max_relative = 4.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne_empty1() {
        let lhs = HashMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = HashMap::new();
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne_empty2() {
        let lhs: HashMap<&str, f32> = HashMap::new();
        let rhs = HashMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_empty1() {
        let lhs = HashMap::from([
            ("0", 0.00_f32),
            ("1", 1.00_f32),
            ("2", 1.25_f32),
            ("3", 1.50_f32),
            ("4", 2.00_f32),
            ("5", 2.50_f32),
            ("6", 3.00_f32),
            ("7", 4.00_f32),
            ("8", 5.00_f32),
        ]);
        let rhs = HashMap::new();

        assert_eq!(lhs.debug_abs_diff(&rhs), None);
        assert_eq!(rhs.debug_abs_diff(&lhs), None);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_empty2() {
        let lhs = HashMap::new();
        let rhs = HashMap::from([
            ("0", 0.0000000_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999500_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000000_f32),
            ("7", 7.0000000_f32),
            ("8", 8.0000000_f32),
        ]);

        assert_eq!(lhs.debug_abs_diff(&rhs), None);
        assert_eq!(rhs.debug_abs_diff(&lhs), None);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_tolerance_empty() {
        let lhs = HashMap::from([
            ("0", 2.00_f32),
            ("1", 3.25_f32),
            ("2", 4.50_f32),
            ("3", 5.75_f32),
        ]);
        let rhs = HashMap::new();
        let max_abs_diff = HashMap::from([
            ("0", 0.10_f32),
            ("1", 0.20_f32),
            ("2", 0.30_f32),
            ("3", 0.40_f32),
        ]);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), None);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), None);
    }

    #[rustfmt::skip]    
    #[test]
    fn test_debug_abs_diff_all_tolerance_empty() {
        let lhs: HashMap<&str, f32> = HashMap::new();
        let rhs = HashMap::from([
            ("0", 2.50_f32),
            ("1", 3.00_f32),
            ("2", 4.00_f32),
            ("3", 6.00_f32),
        ]);
        let max_abs_diff_all = 0.20_f32;

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), None);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), None);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_tolerance_empty() {
        let lhs = HashMap::new();
        let rhs = HashMap::from([
            ("0", 2.50_f32),
            ("1", 3.00_f32),
            ("2", 4.00_f32),
            ("3", 6.00_f32),
        ]);
        let max_relative = HashMap::from([
            ("0", 0.10_f32),
            ("1", 0.20_f32),
            ("2", 0.30_f32),
            ("3", 0.40_f32),
        ]);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), None);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), None);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_all_tolerance_empty() {
        let lhs = HashMap::from([
            ("0", 2.00_f32),
            ("1", 3.25_f32),
            ("2", 4.50_f32),
            ("3", 5.75_f32),
        ]);
        let rhs = HashMap::new();
        let max_relative_all = 0.20_f32;

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), None);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative_all), None);
    }
}


#[cfg(test)]
mod relative_eq_btree_map_tests {
    use approx_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
        AssertRelativeEq,
    };
    use std::collections::BTreeMap;


    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = BTreeMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = BTreeMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = BTreeMap::from([
            ("0", 2.0_f32 * f32::EPSILON),
            ("1", 1.0_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 1.0_f32 * f32::EPSILON),
            ("6", 1.0_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);
        let max_relative = BTreeMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 1.0_f32 * f32::EPSILON),
            ("2", 2.0_f32 * f32::EPSILON),
            ("3", 2.0_f32 * f32::EPSILON),
            ("4", 2.0_f32 * f32::EPSILON),
            ("5", 1.0_f32 * f32::EPSILON),
            ("6", 1.0_f32 * f32::EPSILON),
            ("7", 2.0_f32 * f32::EPSILON),
            ("8", 2.0_f32 * f32::EPSILON),
        ]);

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne1() {
        let lhs = BTreeMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = BTreeMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = BTreeMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);
        let max_relative = BTreeMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne2() {
        let lhs = BTreeMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = BTreeMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = BTreeMap::from([
            ("0", 2.0_f32 * f32::EPSILON),
            ("1", 1.0_f32 * f32::EPSILON),
            ("2", 2.0_f32 * f32::EPSILON),
            ("3", 2.0_f32 * f32::EPSILON),
            ("4", 2.0_f32 * f32::EPSILON),
            ("5", 1.0_f32 * f32::EPSILON),
            ("6", 1.0_f32 * f32::EPSILON),
            ("7", 2.0_f32 * f32::EPSILON),
            ("8", 2.0_f32 * f32::EPSILON),
        ]);
        let max_relative = BTreeMap::from([
            ("0", f32::EPSILON),
            ("1", f32::EPSILON),
            ("2", f32::EPSILON),
            ("3", f32::EPSILON),
            ("4", f32::EPSILON),
            ("5", f32::EPSILON),
            ("6", f32::EPSILON),
            ("7", f32::EPSILON),
            ("8", f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = BTreeMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = BTreeMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = 4.0_f32 * f32::EPSILON;
        let max_relative = 4.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = BTreeMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = BTreeMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff1() {
        let lhs = BTreeMap::from([
            ("0", 0.00_f32),
            ("1", 1.00_f32),
            ("2", 1.25_f32),
            ("3", 1.50_f32),
            ("4", 2.00_f32),
            ("5", 2.50_f32),
            ("6", 3.00_f32),
            ("7", 4.00_f32),
            ("8", 5.00_f32),
        ]);
        let abs_diff = BTreeMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.0000000_f32),
            ("2", 0.0000000_f32),
            ("3", 0.0000000_f32),
            ("4", 0.0000000_f32),
            ("5", 0.0000000_f32),
            ("6", 0.0000000_f32),
            ("7", 0.0000000_f32),
            ("8", 0.0000000_f32),
        ]);

        assert_eq!(lhs.debug_abs_diff(&lhs), Some(abs_diff));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff2() {
        let lhs = BTreeMap::from([
            ("0", 0.00_f32),
            ("1", 1.00_f32),
            ("2", 1.25_f32),
            ("3", 1.50_f32),
            ("4", 2.00_f32),
            ("5", 2.50_f32),
            ("6", 3.00_f32),
            ("7", 4.00_f32),
            ("8", 5.00_f32),
        ]);
        let rhs = BTreeMap::from([
            ("0", 0.10_f32),
            ("1", 1.10_f32),
            ("2", 1.15_f32),
            ("3", 1.70_f32),
            ("4", 1.80_f32),
            ("5", 2.80_f32),
            ("6", 2.70_f32),
            ("7", 4.40_f32),
            ("8", 4.60_f32),
        ]);
        let abs_diff = BTreeMap::from([
            ("0", 0.100000000_f32),
            ("1", 0.100000024_f32),
            ("2", 0.100000024_f32),
            ("3", 0.200000050_f32),
            ("4", 0.200000050_f32),
            ("5", 0.299999950_f32),
            ("6", 0.299999950_f32),
            ("7", 0.400000100_f32),
            ("8", 0.400000100_f32),
        ]);

        assert_eq!(lhs.debug_abs_diff(&rhs), Some(abs_diff.clone()));
        assert_eq!(rhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff3() {
        let lhs = BTreeMap::from([
            ("0", 0.0000500_f32),
            ("1", 0.9999500_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999500_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999500_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999500_f32),
            ("8", 8.0000000_f32),
        ]);
        let abs_diff = BTreeMap::from([
            ("0", 0.00000000000000_f32),
            ("1", 0.00000000000000_f32),
            ("2", 0.00000000000000_f32),
            ("3", 0.00000000000000_f32),
            ("4", 0.00000000000000_f32),
            ("5", 0.00000000000000_f32),
            ("6", 0.00000000000000_f32),
            ("7", 0.00000000000000_f32),
            ("8", 0.00000000000000_f32),
        ]);

        assert_eq!(lhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff4() {
        let lhs = BTreeMap::from([
            ("0", 0.0000500_f32),
            ("1", 0.9999500_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999500_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999500_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999500_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = BTreeMap::from([
            ("0", 0.0000000_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999500_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000000_f32),
            ("7", 7.0000000_f32),
            ("8", 8.0000000_f32),
        ]);
        let abs_diff = BTreeMap::from([
            ("0", 0.00005000000000_f32),
            ("1", 0.00005000829700_f32),
            ("2", 0.00004994869200_f32),
            ("3", 0.00005006790000_f32),
            ("4", 0.00000047683716_f32),
            ("5", 0.00005006790000_f32),
            ("6", 0.00000000000000_f32),
            ("7", 0.00005006790000_f32),
            ("8", 0.00000000000000_f32),
        ]);

        assert_eq!(lhs.debug_abs_diff(&rhs), Some(abs_diff.clone()));
        assert_eq!(rhs.debug_abs_diff(&lhs), Some(abs_diff.clone()));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = BTreeMap::from([
            ("0", 2.00_f32),
            ("1", 3.25_f32),
            ("2", 4.50_f32),
            ("3", 5.75_f32),
        ]);
        let rhs = BTreeMap::from([
            ("0", 2.50_f32),
            ("1", 3.00_f32),
            ("2", 4.00_f32),
            ("3", 6.00_f32),
        ]);
        let max_abs_diff = BTreeMap::from([
            ("0", 0.10_f32),
            ("1", 0.20_f32),
            ("2", 0.30_f32),
            ("3", 0.40_f32),
        ]);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), Some(max_abs_diff));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = BTreeMap::from([
            ("0", 2.00_f32),
            ("1", 3.25_f32),
            ("2", 4.50_f32),
            ("3", 5.75_f32),
        ]);
        let rhs = BTreeMap::from([
            ("0", 2.50_f32),
            ("1", 3.00_f32),
            ("2", 4.00_f32),
            ("3", 6.00_f32),
        ]);
        let max_abs_diff_all = 0.20_f32;
        let max_abs_diff = BTreeMap::from([
            ("0", max_abs_diff_all),
            ("1", max_abs_diff_all),
            ("2", max_abs_diff_all),
            ("3", max_abs_diff_all),
        ]);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), Some(max_abs_diff));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = BTreeMap::from([
            ("0", 2.00_f32),
            ("1", 3.25_f32),
            ("2", 4.50_f32),
            ("3", 5.75_f32),
        ]);
        let rhs = BTreeMap::from([
            ("0", 2.50_f32),
            ("1", 3.00_f32),
            ("2", 4.00_f32),
            ("3", 6.00_f32),
        ]);
        let max_relative = BTreeMap::from([
            ("0", 0.10_f32),
            ("1", 0.20_f32),
            ("2", 0.30_f32),
            ("3", 0.40_f32),
        ]);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), Some(max_relative));
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = BTreeMap::from([
            ("0", 2.00_f32),
            ("1", 3.25_f32),
            ("2", 4.50_f32),
            ("3", 5.75_f32),
        ]);
        let rhs = BTreeMap::from([
            ("0", 2.50_f32),
            ("1", 3.00_f32),
            ("2", 4.00_f32),
            ("3", 6.00_f32),
        ]);
        let max_relative_all = 0.20_f32;
        let max_relative = BTreeMap::from([
            ("0", max_relative_all),
            ("1", max_relative_all),
            ("2", max_relative_all),
            ("3", max_relative_all),
        ]);

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), Some(max_relative));
    }

    #[test]
    fn test_eq_empty1() {
        let lhs: BTreeMap<&str, f32> = BTreeMap::new();
        let rhs: BTreeMap<&str, f32> = BTreeMap::new();
        let max_abs_diff = BTreeMap::new();
        let max_relative = BTreeMap::new();

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty1() {
        let lhs: BTreeMap<&str, f32> = BTreeMap::new();
        let rhs: BTreeMap<&str, f32> = BTreeMap::new();
        let max_abs_diff = BTreeMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);
        let max_relative = BTreeMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty2() {
        let lhs: BTreeMap<&str, f32> = BTreeMap::new();
        let rhs: BTreeMap<&str, f32> = BTreeMap::new();
        let max_abs_diff = BTreeMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);
        let max_relative = BTreeMap::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty3() {
        let lhs: BTreeMap<&str, f32> = BTreeMap::new();
        let rhs: BTreeMap<&str, f32> = BTreeMap::new();
        let max_abs_diff = BTreeMap::new();
        let max_relative = BTreeMap::from([
            ("0", 2.0_f32 * f32::EPSILON),
            ("1", 1.0_f32 * f32::EPSILON),
            ("2", 2.0_f32 * f32::EPSILON),
            ("3", 2.0_f32 * f32::EPSILON),
            ("4", 2.0_f32 * f32::EPSILON),
            ("5", 1.0_f32 * f32::EPSILON),
            ("6", 1.0_f32 * f32::EPSILON),
            ("7", 2.0_f32 * f32::EPSILON),
            ("8", 2.0_f32 * f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty4() {
        let lhs = BTreeMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = BTreeMap::new();
        let max_abs_diff = BTreeMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);
        let max_relative = BTreeMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty5() {
        let lhs = BTreeMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = BTreeMap::new();
        let max_abs_diff = BTreeMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);
        let max_relative = BTreeMap::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty6() {
        let lhs = BTreeMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = BTreeMap::new();
        let max_abs_diff = BTreeMap::new();
        let max_relative = BTreeMap::from([
            ("0", 1.0_f32 * f32::EPSILON),
            ("1", 0.5_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 0.5_f32 * f32::EPSILON),
            ("6", 0.5_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty7() {
        let lhs = BTreeMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = BTreeMap::new();
        let max_abs_diff = BTreeMap::new();
        let max_relative = BTreeMap::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty8() {
        let lhs: BTreeMap<&str, f32> = BTreeMap::new();
        let rhs = BTreeMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = BTreeMap::from([
            ("0", 2.0_f32 * f32::EPSILON),
            ("1", 1.0_f32 * f32::EPSILON),
            ("2", 2.0_f32 * f32::EPSILON),
            ("3", 2.0_f32 * f32::EPSILON),
            ("4", 2.0_f32 * f32::EPSILON),
            ("5", 1.0_f32 * f32::EPSILON),
            ("6", 1.0_f32 * f32::EPSILON),
            ("7", 2.0_f32 * f32::EPSILON),
            ("8", 2.0_f32 * f32::EPSILON),
        ]);
        let max_relative = BTreeMap::from([
            ("0", f32::EPSILON),
            ("1", f32::EPSILON),
            ("2", f32::EPSILON),
            ("3", f32::EPSILON),
            ("4", f32::EPSILON),
            ("5", f32::EPSILON),
            ("6", f32::EPSILON),
            ("7", f32::EPSILON),
            ("8", f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty9() {
        let lhs: BTreeMap<&str, f32> = BTreeMap::new();
        let rhs = BTreeMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = BTreeMap::from([
            ("0", 2.0_f32 * f32::EPSILON),
            ("1", 1.0_f32 * f32::EPSILON),
            ("2", 2.0_f32 * f32::EPSILON),
            ("3", 2.0_f32 * f32::EPSILON),
            ("4", 2.0_f32 * f32::EPSILON),
            ("5", 1.0_f32 * f32::EPSILON),
            ("6", 1.0_f32 * f32::EPSILON),
            ("7", 2.0_f32 * f32::EPSILON),
            ("8", 2.0_f32 * f32::EPSILON),
        ]);
        let max_relative = BTreeMap::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty10() {
        let lhs: BTreeMap<&str, f32> = BTreeMap::new();
        let rhs = BTreeMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = BTreeMap::new();
        let max_relative = BTreeMap::from([
            ("0", f32::EPSILON),
            ("1", f32::EPSILON),
            ("2", f32::EPSILON),
            ("3", f32::EPSILON),
            ("4", f32::EPSILON),
            ("5", f32::EPSILON),
            ("6", f32::EPSILON),
            ("7", f32::EPSILON),
            ("8", f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty11() {
        let lhs: BTreeMap<&str, f32> = BTreeMap::new();
        let rhs = BTreeMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = BTreeMap::new();
        let max_relative = BTreeMap::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty12() {
        let lhs = BTreeMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = BTreeMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = BTreeMap::new();
        let max_relative = BTreeMap::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty13() {
        let lhs = BTreeMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = BTreeMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = BTreeMap::from([
            ("0", 2.0_f32 * f32::EPSILON),
            ("1", 1.0_f32 * f32::EPSILON),
            ("2", 4.0_f32 * f32::EPSILON),
            ("3", 4.0_f32 * f32::EPSILON),
            ("4", 4.0_f32 * f32::EPSILON),
            ("5", 1.0_f32 * f32::EPSILON),
            ("6", 1.0_f32 * f32::EPSILON),
            ("7", 4.0_f32 * f32::EPSILON),
            ("8", 4.0_f32 * f32::EPSILON),
        ]);
        let max_relative = BTreeMap::new();

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne_empty14() {
        let lhs = BTreeMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = BTreeMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = BTreeMap::new();
        let max_relative = BTreeMap::from([
            ("0", 2.0_f32 * f32::EPSILON),
            ("1", 1.0_f32 * f32::EPSILON),
            ("2", 2.0_f32 * f32::EPSILON),
            ("3", 2.0_f32 * f32::EPSILON),
            ("4", 2.0_f32 * f32::EPSILON),
            ("5", 1.0_f32 * f32::EPSILON),
            ("6", 1.0_f32 * f32::EPSILON),
            ("7", 2.0_f32 * f32::EPSILON),
            ("8", 2.0_f32 * f32::EPSILON),
        ]);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_all_eq_empty() {
        let lhs: BTreeMap<&str, f32> = BTreeMap::new();
        let rhs: BTreeMap<&str, f32> = BTreeMap::new();
        let max_abs_diff = 4.0_f32 * f32::EPSILON;
        let max_relative = 4.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne_empty1() {
        let lhs = BTreeMap::from([
            ("0", 0.0000000_f32),
            ("1", 0.9999999_f32),
            ("2", 2.0000000_f32),
            ("3", 2.9999995_f32),
            ("4", 4.0000000_f32),
            ("5", 4.9999999_f32),
            ("6", 6.0000000_f32),
            ("7", 6.9999995_f32),
            ("8", 8.0000000_f32),
        ]);
        let rhs = BTreeMap::new();
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne_empty2() {
        let lhs: BTreeMap<&str, f32> = BTreeMap::new();
        let rhs = BTreeMap::from([
            ("0", 0.0000002_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999995_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000001_f32),
            ("7", 7.0000000_f32),
            ("8", 7.9999995_f32),
        ]);
        let max_abs_diff = 2.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_empty1() {
        let lhs = BTreeMap::from([
            ("0", 0.00_f32),
            ("1", 1.00_f32),
            ("2", 1.25_f32),
            ("3", 1.50_f32),
            ("4", 2.00_f32),
            ("5", 2.50_f32),
            ("6", 3.00_f32),
            ("7", 4.00_f32),
            ("8", 5.00_f32),
        ]);
        let rhs = BTreeMap::new();

        assert_eq!(lhs.debug_abs_diff(&rhs), None);
        assert_eq!(rhs.debug_abs_diff(&lhs), None);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_empty2() {
        let lhs = BTreeMap::new();
        let rhs = BTreeMap::from([
            ("0", 0.0000000_f32),
            ("1", 1.0000000_f32),
            ("2", 1.9999500_f32),
            ("3", 3.0000000_f32),
            ("4", 4.0000005_f32),
            ("5", 5.0000000_f32),
            ("6", 6.0000000_f32),
            ("7", 7.0000000_f32),
            ("8", 8.0000000_f32),
        ]);

        assert_eq!(lhs.debug_abs_diff(&rhs), None);
        assert_eq!(rhs.debug_abs_diff(&lhs), None);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_tolerance_empty() {
        let lhs = BTreeMap::from([
            ("0", 2.00_f32),
            ("1", 3.25_f32),
            ("2", 4.50_f32),
            ("3", 5.75_f32),
        ]);
        let rhs = BTreeMap::new();
        let max_abs_diff = BTreeMap::from([
            ("0", 0.10_f32),
            ("1", 0.20_f32),
            ("2", 0.30_f32),
            ("3", 0.40_f32),
        ]);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), None);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), None);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_all_tolerance_empty() {
        let lhs: BTreeMap<&str, f32> = BTreeMap::new();
        let rhs = BTreeMap::from([
            ("0", 2.50_f32),
            ("1", 3.00_f32),
            ("2", 4.00_f32),
            ("3", 6.00_f32),
        ]);
        let max_abs_diff_all = 0.20_f32;

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), None);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), None);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_tolerance_empty() {
        let lhs = BTreeMap::new();
        let rhs = BTreeMap::from([
            ("0", 2.50_f32),
            ("1", 3.00_f32),
            ("2", 4.00_f32),
            ("3", 6.00_f32),
        ]);
        let max_relative = BTreeMap::from([
            ("0", 0.10_f32),
            ("1", 0.20_f32),
            ("2", 0.30_f32),
            ("3", 0.40_f32),
        ]);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), None);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), None);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_all_tolerance_empty() {
        let lhs = BTreeMap::from([
            ("0", 2.00_f32),
            ("1", 3.25_f32),
            ("2", 4.50_f32),
            ("3", 5.75_f32),
        ]);
        let rhs = BTreeMap::new();
        let max_relative_all = 0.20_f32;

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), None);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative_all), None);
    }
}
