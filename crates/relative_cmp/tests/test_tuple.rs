extern crate relative_cmp;


#[cfg(test)]
mod relative_eq_tuple1_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
        AssertRelativeEq,
    };

    #[test]
    fn test_eq() {
        let lhs = (4.0_f32,);
        let rhs = (4.0000005_f32,);
        let max_abs_diff = (0.5_f32 * f32::EPSILON,);
        let max_relative = (6.0_f32 * f32::EPSILON,);

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne() {
        let lhs = (4.0_f32,);
        let rhs = (2.0_f32,);
        let max_abs_diff = (1.0_f32 * f32::EPSILON,);
        let max_relative = (1.0_f32 * f32::EPSILON,);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_all_eq() {
        let lhs = (4.0_f32,);
        let rhs = (4.0000005_f32,);
        let max_abs_diff = 0.5_f32 * f32::EPSILON;
        let max_relative = 6.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[test]
    fn test_all_ne() {
        let lhs = (4.0_f32,);
        let rhs = (2.0_f32,);
        let max_abs_diff = 1.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[test]
    fn test_debug_abs_diff() {
        let lhs = (1.0_f32,);
        let rhs = (1.0000011_f32,);
        let abs_diff = (0.0000010728836_f32,);

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = (1.0_f32,);
        let rhs = (1.0000011_f32,);
        let max_abs_diff = (0.2_f32,);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = (1.0_f32,);
        let rhs = (1.0000011_f32,);
        let max_relative = (0.2_f32,);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), max_relative);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = (1.0_f32,);
        let rhs = (1.0000011_f32,);
        let max_abs_diff_all = 0.2_f32;
        let max_abs_diff = (0.2_f32,);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = (1.0_f32,);
        let rhs = (1.0000011_f32,);
        let max_relative_all = 0.2_f32;
        let max_relative = (0.2_f32,);

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), max_relative);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative_all), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple2_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
    };

    #[test]
    fn test_eq() {
        let lhs = (2.9999999_f32, 3.0000004_f32);
        let rhs = (3.0_f32, 3.0_f32);
        let max_abs_diff = (1.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON);
        let max_relative = (1.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON);

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne() {
        let lhs = (2.9999995_f32, 3.0000004_f32);
        let rhs = (3.0_f32, 3.0_f32);
        let max_abs_diff = (1.0_f32 * f32::EPSILON, 3.0_f32 * f32::EPSILON);
        let max_relative = (1.0_f32 * f32::EPSILON, 3.0_f32 * f32::EPSILON);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_all_eq() {
        let lhs = (2.9999999_f32, 3.0000004_f32);
        let rhs = (3.0_f32, 3.0_f32);
        let max_abs_diff = 5.0_f32 * f32::EPSILON;
        let max_relative = 5.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[test]
    fn test_all_ne() {
        let lhs = (2.9999995_f32, 3.0000004_f32);
        let rhs = (3.0_f32, 3.0_f32);
        let max_abs_diff = 1.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = (1.0_f32, 2.0_f32);
        let rhs = (1.0000011_f32, 2.0000022_f32);
        let max_abs_diff_all = 0.2_f32;
        let max_abs_diff = (0.2_f32, 0.2_f32);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = (1.0_f32, 2.0_f32);
        let rhs = (1.0000011_f32, 2.0000022_f32);
        let max_relative_all = 0.2_f32;
        let max_relative = (0.2_f32, 0.2_f32);

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), max_relative);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative_all), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple2_heterogenous_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeEq,
    };

    #[test]
    fn test_eq() {
        let lhs = (2.9999999_f32, 3.0000004_f64);
        let rhs = (3.0_f32, 3.0_f64);
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (1.0_f32 * f32::EPSILON, 5.0_f64 * epsilon);
        let max_relative = (1.0_f32 * f32::EPSILON, 5.0_f64 * epsilon);

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne() {
        let lhs = (2.9999995_f32, 3.0000004_f64);
        let rhs = (3.0_f32, 3.0_f64);
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (1.0_f32 * f32::EPSILON, 3.0_f64 * epsilon);
        let max_relative = (1.0_f32 * f32::EPSILON, 3.0_f64 * epsilon);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_debug_abs_diff() {
        let lhs = (1.0_f32, 2.0_f64);
        let rhs = (1.0000011_f32, 2.0000022_f64);
        let abs_diff = (0.0000010728836_f32, 0.00000219999999995224_f64);

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = (1.0_f32, 2.0_f64);
        let rhs = (1.0000011_f32, 2.000022_f64);
        let max_abs_diff = (0.2_f32, 0.3_f64);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = (1.0_f32, 2.0_f64);
        let rhs = (1.0000011_f32, 2.000022_f64);
        let max_relative = (0.2_f32, 0.3_f64);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple3_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
    };

    #[test]
    fn test_eq() {
        let lhs = (-1.0000001_f32, 2.0000002_f32, 3.0000003_f32);
        let rhs = (-1.0_f32, 2.0_f32, 3.0_f32);
        let max_abs_diff = (2.0_f32 * f32::EPSILON, 3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON);
        let max_relative = (2.0_f32 * f32::EPSILON, 3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON);

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne() {
        let lhs = (-1.0000001_f32, 2.0000002_f32, 3.0000003_f32);
        let rhs = (-1.0_f32, 2.0_f32, 3.0_f32);
        let max_abs_diff = (0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON);
        let max_relative = (0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_all_eq() {
        let lhs = (-1.0000001_f32, 2.0000002_f32, 3.0000003_f32);
        let rhs = (-1.0_f32, 2.0_f32, 3.0_f32);
        let max_abs_diff = 1.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[test]
    fn test_all_ne() {
        let lhs = (-1.0000001_f32, 2.0000002_f32, 3.0000003_f32);
        let rhs = (-1.0_f32, 2.0_f32, 3.0_f32);
        let max_abs_diff = 0.5_f32 * f32::EPSILON;
        let max_relative = 0.5_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = (1.0_f32, 2.0_f32, 4.0_f32);
        let rhs = (1.0000011_f32, 2.0000022_f32, 4.0000044_f32);
        let max_abs_diff_all = 0.2_f32;
        let max_abs_diff = (0.2_f32, 0.2_f32, 0.2_f32);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = (1.0_f32, 2.0_f32, 4.0_f32);
        let rhs = (1.0000011_f32, 2.0000022_f32, 4.0000044_f32);
        let max_relative_all = 0.2_f32;
        let max_relative = (0.2_f32, 0.2_f32, 0.2_f32);

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), max_relative);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative_all), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple3_heterogenous_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeEq,
    };

    #[test]
    fn test_eq() {
        let lhs = (-1.0000001_f32, 2.0000002_f64, 3.0000003_f32);
        let rhs = (-1.0_f32, 2.0_f64, 3.0_f32);
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (2.0_f32 * f32::EPSILON, 3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON);
        let max_relative = (2.0_f32 * f32::EPSILON, 3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON);

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne() {
        let lhs = (-1.0000001_f32, 2.0000002_f64, 3.0000003_f32);
        let rhs = (-1.0_f32, 2.0_f64, 3.0_f32);
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON);
        let max_relative = (0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_debug_abs_diff() {
        let lhs = (1.0_f32, 2.0_f64, 4.0_f32);
        let rhs = (1.0000011_f32, 2.0000022_f64, 4.0000044_f32);
        let abs_diff = (0.0000010728836_f32, 0.00000219999999995224_f64, 0.0000042915344_f32);

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = (1.0_f32, 2.0_f64, 4.0_f32);
        let rhs = (1.0000011_f32, 2.0000022_f64, 4.0000044_f32);
        let max_abs_diff = (0.2_f32, 0.3_f64, 0.4_f32);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = (1.0_f32, 2.0_f64, 4.0_f32);
        let rhs = (1.0000011_f32, 2.0000022_f64, 4.0000044_f32);
        let max_relative = (0.2_f32, 0.3_f64, 0.4_f32);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple4_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
    };

    #[test]
    fn test_eq() {
        let lhs = (-1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32);
        let rhs = (-1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32);
        let max_abs_diff = (
            2.0_f32 * f32::EPSILON,
            3.0_f32 * f32::EPSILON,
            4.0_f32 * f32::EPSILON,
            5.0_f32 * f32::EPSILON,
        );
        let max_relative = (
            2.0_f32 * f32::EPSILON,
            3.0_f32 * f32::EPSILON,
            4.0_f32 * f32::EPSILON,
            5.0_f32 * f32::EPSILON,
        );

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne() {
        let lhs = (-1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32);
        let rhs = (-1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32);
        let max_abs_diff = (
            0.5_f32 * f32::EPSILON,
            1.5_f32 * f32::EPSILON,
            2.5_f32 * f32::EPSILON,
            3.5_f32 * f32::EPSILON,
        );
        let max_relative = (
            0.5_f32 * f32::EPSILON,
            1.5_f32 * f32::EPSILON,
            2.5_f32 * f32::EPSILON,
            3.5_f32 * f32::EPSILON,
        );

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_all_eq() {
        let lhs = (-1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32);
        let rhs = (-1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32);
        let max_abs_diff = 1.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[test]
    fn test_all_ne() {
        let lhs = (-1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32);
        let rhs = (-1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32);
        let max_abs_diff = 0.5_f32 * f32::EPSILON;
        let max_relative = 0.5_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = (1.0_f32, 2.0_f32, 4.0_f32, 8.0_f32);
        let rhs = (1.0000011_f32, 2.0000022_f32, 4.0000044_f32, 8.0000088_f32);
        let max_abs_diff_all = 0.2_f32;
        let max_abs_diff = (0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = (1.0_f32, 2.0_f32, 4.0_f32, 8.0_f32);
        let rhs = (1.0000011_f32, 2.0000022_f32, 4.0000044_f32, 8.0000088_f32);
        let max_relative_all = 0.2_f32;
        let max_relative = (0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32);

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), max_relative);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative_all), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple4_heterogenous_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeEq,
    };

    #[test]
    fn test_eq() {
        let lhs = (-1.0000001_f32, 2.0000002_f64, 3.0000003_f32, -4.0000004_f64);
        let rhs = (-1.0_f32, 2.0_f64, 3.0_f32, -4.0_f64);
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (2.0_f32 * f32::EPSILON, 3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon);
        let max_relative = (2.0_f32 * f32::EPSILON, 3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon);

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_ne() {
        let lhs = (-1.0000001_f32, 2.0000002_f64, 3.0000003_f32, -4.0000004_f64);
        let rhs = (-1.0_f32, 2.0_f64, 3.0_f32, -4.0_f64);
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon);
        let max_relative = (0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon);

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[test]
    fn test_debug_abs_diff() {
        let lhs = (1.0_f32, 2.0_f64, 4.0_f32, 8.0_f64);
        let rhs = (1.0000011_f32, 2.0000022_f64, 4.0000044_f32, 8.0000088_f64);
        let abs_diff = (
            0.0000010728836_f32,
            0.00000219999999995224_f64,
            0.0000042915344_f32,
            0.00000879999999980896_f64,
        );

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = (1.0_f32, 2.0_f64, 4.0_f32, 8.0_f64);
        let rhs = (1.0000011_f32, 2.0000022_f64, 4.0000044_f32, 8.0000088_f64);
        let max_abs_diff = (0.2_f32, 0.3_f64, 0.4_f32, 0.5_f64);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), max_abs_diff);
    }

    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = (1.0_f32, 2.0_f64, 4.0_f32, 8.0_f64);
        let rhs = (1.0000011_f32, 2.0000022_f64, 4.0000044_f32, 8.0000088_f64);
        let max_relative = (0.2_f32, 0.3_f64, 0.4_f32, 0.5_f64);

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple5_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
    };

    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32,
        );
        let max_abs_diff = (
            2.0_f32 * f32::EPSILON, 3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON,
            6.0_f32 * f32::EPSILON,
        );
        let max_relative = (
            2.0_f32 * f32::EPSILON, 3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON,
            6.0_f32 * f32::EPSILON,
        );

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32,
        );
        let max_abs_diff = (
            0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON, 3.5_f32 * f32::EPSILON,
            4.5_f32 * f32::EPSILON,
        );
        let max_relative = (
            0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON, 3.5_f32 * f32::EPSILON,
            4.5_f32 * f32::EPSILON,
        );

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32,
        );
        let max_abs_diff = 1.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32,
        );
        let max_abs_diff = 0.5_f32 * f32::EPSILON;
        let max_relative = 0.5_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = (
            1.0_f32,  2.0_f32, 4.0_f32, 8.0_f32,
            16.0_f32,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f32, 4.0000044_f32, 8.0000088_f32,
            16.0000176_f32,
        );
        let max_abs_diff_all = 0.2_f32;
        let max_abs_diff = (
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32,
        );

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), max_abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = (
            1.0_f32,  2.0_f32, 4.0_f32, 8.0_f32,
            16.0_f32,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f32, 4.0000044_f32, 8.0000088_f32,
            16.0000176_f32,
        );
        let max_relative_all = 0.2_f32;
        let max_relative = (
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32,
        );

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), max_relative);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative_all), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple5_heterogenous_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeEq,
    };

    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f64, 3.0000003_f32, -4.0000004_f64,
            -5.0000005_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f64, 3.0_f32, -4.0_f64,
            -5.0_f32,
        );
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (
            2.0_f32 * f32::EPSILON, 3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon,
            6.0_f32 * f32::EPSILON,
        );
        let max_relative = (
            2.0_f32 * f32::EPSILON, 3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon,
            6.0_f32 * f32::EPSILON,
        );

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f64, 3.0000003_f32, -4.0000004_f64,
            -5.0000005_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f64, 3.0_f32, -4.0_f64,
            -5.0_f32,
        );
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (
            0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon,
            4.5_f32 * f32::EPSILON,
        );
        let max_relative = (
            0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon,
            4.5_f32 * f32::EPSILON,
        );

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff() {
        let lhs = (
            1.0_f32,  2.0_f64, 4.0_f32, 8.0_f64,
            16.0_f32,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f64, 4.0000044_f32, 8.0000088_f64,
            16.0000176_f32,
        );
        let abs_diff = (
            0.0000010728836_f32, 0.00000219999999995224_f64, 0.0000042915344_f32, 0.00000879999999980896_f64,
            0.000017166138_f32,
        );

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = (
            1.0_f32,  2.0_f64, 4.0_f32, 8.0_f64,
            16.0_f32,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f64, 4.0000044_f32, 8.0000088_f64,
            16.0000176_f32,
        );
        let max_abs_diff = (
            0.2_f32, 0.3_f64, 0.4_f32, 0.5_f64,
            0.6_f32,
        );

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), max_abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = (
            1.0_f32,  2.0_f64, 4.0_f32, 8.0_f64,
            16.0_f32,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f64, 4.0000044_f32, 8.0000088_f64,
            16.0000176_f32,
        );
        let max_relative = (
            0.2_f32, 0.3_f64, 0.4_f32, 0.5_f64,
            0.6_f32,
        );

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple6_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
    };

    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32, 6.0000006_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32, 6.0_f32,
        );
        let max_abs_diff = (
            2.0_f32 * f32::EPSILON, 3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON,
            6.0_f32 * f32::EPSILON, 7.0_f32 * f32::EPSILON,
        );
        let max_relative = (
            2.0_f32 * f32::EPSILON, 3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON,
            6.0_f32 * f32::EPSILON, 7.0_f32 * f32::EPSILON,
        );

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32, 6.0000006_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32, 6.0_f32,
        );
        let max_abs_diff = (
            0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON, 3.5_f32 * f32::EPSILON,
            4.5_f32 * f32::EPSILON, 5.5_f32 * f32::EPSILON,
        );
        let max_relative = (
            0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON, 3.5_f32 * f32::EPSILON,
            4.5_f32 * f32::EPSILON, 5.5_f32 * f32::EPSILON,
        );

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32, 6.0000006_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32, 6.0_f32,
        );
        let max_abs_diff = 1.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32, 6.0000006_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32, 6.0_f32,
        );
        let max_abs_diff = 0.5_f32 * f32::EPSILON;
        let max_relative = 0.5_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = (
            1.0_f32,  2.0_f32,  4.0_f32, 8.0_f32,
            16.0_f32, 32.0_f32,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f32,  4.0000044_f32, 8.0000088_f32,
            16.0000176_f32, 32.0000352_f32,
        );
        let max_abs_diff_all = 0.2_f32;
        let max_abs_diff = (
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32,
        );

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), max_abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = (
            1.0_f32,  2.0_f32,  4.0_f32, 8.0_f32,
            16.0_f32, 32.0_f32,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f32,  4.0000044_f32, 8.0000088_f32,
            16.0000176_f32, 32.0000352_f32,
        );
        let max_relative_all = 0.2_f32;
        let max_relative = (
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32,
        );

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), max_relative);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative_all), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple6_heterogenous_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeEq,
    };

    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f64, 3.0000003_f32, -4.0000004_f64,
            -5.0000005_f32, 6.0000006_f64,
        );
        let rhs = (
            -1.0_f32, 2.0_f64, 3.0_f32, -4.0_f64,
            -5.0_f32, 6.0_f64,
        );
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (
            2.0_f32 * f32::EPSILON, 3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon,
            6.0_f32 * f32::EPSILON, 7.0_f64 * epsilon,
        );
        let max_relative = (
            2.0_f32 * f32::EPSILON, 3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon,
            6.0_f32 * f32::EPSILON, 7.0_f64 * epsilon,
        );

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f64, 3.0000003_f32, -4.0000004_f64,
            -5.0000005_f32, 6.0000006_f64,
        );
        let rhs = (
            -1.0_f32, 2.0_f64, 3.0_f32, -4.0_f64,
            -5.0_f32, 6.0_f64,
        );
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (
            0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon,
            4.5_f32 * f32::EPSILON, 5.5_f64 * epsilon,
        );
        let max_relative = (
            0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon,
            4.5_f32 * f32::EPSILON, 5.5_f64 * epsilon,
        );

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff() {
        let lhs = (
            1.0_f32,  2.0_f64,  4.0_f32, 8.0_f64,
            16.0_f32, 32.0_f64,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f64,  4.0000044_f32, 8.0000088_f64,
            16.0000176_f32, 32.0000352_f64,
        );
        let abs_diff = (
            0.0000010728836_f32, 0.00000219999999995224_f64, 0.0000042915344_f32, 0.00000879999999980896_f64,
            0.000017166138_f32,  0.00003519999999923584_f64, 
        );

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = (
            1.0_f32,  2.0_f64, 4.0_f32, 8.0_f64,
            16.0_f32, 32.0_f64,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f64,  4.0000044_f32, 8.0000088_f64,
            16.0000176_f32, 32.0000352_f64,
        );
        let max_abs_diff = (
            0.2_f32, 0.3_f64, 0.4_f32, 0.5_f64,
            0.6_f32, 0.7_f64,
        );

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), max_abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = (
            1.0_f32,  2.0_f64, 4.0_f32, 8.0_f64,
            16.0_f32, 32.0_f64,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f64,  4.0000044_f32, 8.0000088_f64,
            16.0000176_f32, 32.0000352_f64,
        );
        let max_relative = (
            0.2_f32, 0.3_f64, 0.4_f32, 0.5_f64,
            0.6_f32, 0.7_f64,
        );

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple7_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
    };

    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32, 6.0000006_f32, 7.0000007_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32, 6.0_f32, 7.0_f32,
        );
        let max_abs_diff = (
            2.0_f32 * f32::EPSILON, 3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON,
            6.0_f32 * f32::EPSILON, 7.0_f32 * f32::EPSILON, 8.0_f32 * f32::EPSILON,
        );
        let max_relative = (
            2.0_f32 * f32::EPSILON, 3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON,
            6.0_f32 * f32::EPSILON, 7.0_f32 * f32::EPSILON, 8.0_f32 * f32::EPSILON,
        );

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32, 6.0000006_f32, 7.0000007_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32, 6.0_f32, 7.0_f32,
        );
        let max_abs_diff = (
            0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON, 3.5_f32 * f32::EPSILON,
            4.5_f32 * f32::EPSILON, 5.5_f32 * f32::EPSILON, 6.5_f32 * f32::EPSILON,
        );
        let max_relative = (
            0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON, 3.5_f32 * f32::EPSILON,
            4.5_f32 * f32::EPSILON, 5.5_f32 * f32::EPSILON, 6.5_f32 * f32::EPSILON,
        );

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32, 6.0000006_f32, 7.0000007_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32, 6.0_f32, 7.0_f32,
        );
        let max_abs_diff = 1.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32, 6.0000006_f32, 7.0000007_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32, 6.0_f32, 7.0_f32,
        );
        let max_abs_diff = 0.5_f32 * f32::EPSILON;
        let max_relative = 0.5_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = (
            1.0_f32,  2.0_f32,  4.0_f32,  8.0_f32,
            16.0_f32, 32.0_f32, 64.0_f32,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f32,  4.0000044_f32,  8.0000088_f32,
            16.0000176_f32, 32.0000352_f32, 64.0000704_f32,
        );
        let max_abs_diff_all = 0.2_f32;
        let max_abs_diff = (
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32, 0.2_f32,
        );

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), max_abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = (
            1.0_f32,  2.0_f32,  4.0_f32,  8.0_f32,
            16.0_f32, 32.0_f32, 64.0_f32,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f32,  4.0000044_f32,  8.0000088_f32,
            16.0000176_f32, 32.0000352_f32, 64.0000704_f32,
        );
        let max_relative_all = 0.2_f32;
        let max_relative = (
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32, 0.2_f32,
        );

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), max_relative);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative_all), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple7_heterogenous_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeEq,
    };

    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f64, 3.0000003_f32, -4.0000004_f64,
            -5.0000005_f32, 6.0000006_f64, 7.0000007_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f64, 3.0_f32, -4.0_f64,
            -5.0_f32, 6.0_f64, 7.0_f32,
        );
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (
            2.0_f32 * f32::EPSILON, 3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon,
            6.0_f32 * f32::EPSILON, 7.0_f64 * epsilon, 8.0_f32 * f32::EPSILON,
        );
        let max_relative = (
            2.0_f32 * f32::EPSILON, 3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon,
            6.0_f32 * f32::EPSILON, 7.0_f64 * epsilon, 8.0_f32 * f32::EPSILON,
        );

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f64, 3.0000003_f32, -4.0000004_f64,
            -5.0000005_f32, 6.0000006_f64, 7.0000007_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f64, 3.0_f32, -4.0_f64,
            -5.0_f32, 6.0_f64, 7.0_f32,
        );
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (
            0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon,
            4.5_f32 * f32::EPSILON, 5.5_f64 * epsilon, 6.5_f32 * f32::EPSILON,
        );
        let max_relative = (
            0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon,
            4.5_f32 * f32::EPSILON, 5.5_f64 * epsilon, 6.5_f32 * f32::EPSILON,
        );

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff() {
        let lhs = (
            1.0_f32,  2.0_f64,  4.0_f32,  8.0_f64,
            16.0_f32, 32.0_f64, 64.0_f32,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f64,  4.0000044_f32,  8.0000088_f64,
            16.0000176_f32, 32.0000352_f64, 64.0000704_f32,
        );
        let abs_diff = (
            0.0000010728836_f32, 0.00000219999999995224_f64, 0.0000042915344_f32, 0.00000879999999980896_f64,
            0.000017166138_f32,  0.00003519999999923584_f64, 0.00006866455_f32,
        );

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = (
            1.0_f32,  2.0_f64,  4.0_f32,  8.0_f64,
            16.0_f32, 32.0_f64, 64.0_f32,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f64,  4.0000044_f32,  8.0000088_f64,
            16.0000176_f32, 32.0000352_f64, 64.0000704_f32,
        );
        let max_abs_diff = (
            0.2_f32, 0.3_f64, 0.4_f32, 0.5_f64,
            0.6_f32, 0.7_f64, 0.8_f32,
        );

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), max_abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = (
            1.0_f32,  2.0_f64,  4.0_f32,  8.0_f64,
            16.0_f32, 32.0_f64, 64.0_f32,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f64,  4.0000044_f32,  8.0000088_f64,
            16.0000176_f32, 32.0000352_f64, 64.0000704_f32,
        );
        let max_relative = (
            0.2_f32, 0.3_f64, 0.4_f32, 0.5_f64,
            0.6_f32, 0.7_f64, 0.8_f32,
        );

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple8_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
    };

    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32, 6.0000006_f32, 7.0000007_f32, -8.0000008_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32, 6.0_f32, 7.0_f32, -8.0_f32,
        );
        let max_abs_diff = (
            2.0_f32 * f32::EPSILON, 3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON,
            6.0_f32 * f32::EPSILON, 7.0_f32 * f32::EPSILON, 8.0_f32 * f32::EPSILON, 9.0_f32 * f32::EPSILON,
        );
        let max_relative = (
            2.0_f32 * f32::EPSILON, 3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON,
            6.0_f32 * f32::EPSILON, 7.0_f32 * f32::EPSILON, 8.0_f32 * f32::EPSILON, 9.0_f32 * f32::EPSILON,
        );

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32, 6.0000006_f32, 7.0000007_f32, -8.0000008_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32, 6.0_f32, 7.0_f32, -8.0_f32,
        );
        let max_abs_diff = (
            0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON, 3.5_f32 * f32::EPSILON,
            4.5_f32 * f32::EPSILON, 5.5_f32 * f32::EPSILON, 6.5_f32 * f32::EPSILON, 7.5_f32 * f32::EPSILON,
        );
        let max_relative = (
            0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON, 3.5_f32 * f32::EPSILON,
            4.5_f32 * f32::EPSILON, 5.5_f32 * f32::EPSILON, 6.5_f32 * f32::EPSILON, 7.5_f32 * f32::EPSILON,
        );

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32, 6.0000006_f32, 7.0000007_f32, -8.0000008_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32, 6.0_f32, 7.0_f32, -8.0_f32,
        );
        let max_abs_diff = 1.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32, 6.0000006_f32, 7.0000007_f32, -8.0000008_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32, 6.0_f32, 7.0_f32, -8.0_f32,
        );
        let max_abs_diff = 0.5_f32 * f32::EPSILON;
        let max_relative = 0.5_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = (
            1.0_f32,  2.0_f32,  4.0_f32,  8.0_f32,
            16.0_f32, 32.0_f32, 64.0_f32, 128.0_f32,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f32,  4.0000044_f32,  8.0000088_f32,
            16.0000176_f32, 32.0000352_f32, 64.0000704_f32, 128.0001408_f32,
        );
        let max_abs_diff_all = 0.2_f32;
        let max_abs_diff = (
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
        );

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), max_abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = (
            1.0_f32,  2.0_f32,  4.0_f32,  8.0_f32,
            16.0_f32, 32.0_f32, 64.0_f32, 128.0_f32,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f32,  4.0000044_f32,  8.0000088_f32,
            16.0000176_f32, 32.0000352_f32, 64.0000704_f32, 128.0001408_f32,
        );
        let max_relative_all = 0.2_f32;
        let max_relative = (
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
        );

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), max_relative);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative_all), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple8_heterogenous_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeEq,
    };

    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f64, 3.0000003_f32, -4.0000004_f64,
            -5.0000005_f32, 6.0000006_f64, 7.0000007_f32, -8.0000008_f64,
        );
        let rhs = (
            -1.0_f32, 2.0_f64, 3.0_f32, -4.0_f64,
            -5.0_f32, 6.0_f64, 7.0_f32, -8.0_f64,
        );
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (
            2.0_f32 * f32::EPSILON, 3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon,
            6.0_f32 * f32::EPSILON, 7.0_f64 * epsilon, 8.0_f32 * f32::EPSILON, 9.0_f64 * epsilon,
        );
        let max_relative = (
            2.0_f32 * f32::EPSILON, 3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon,
            6.0_f32 * f32::EPSILON, 7.0_f64 * epsilon, 8.0_f32 * f32::EPSILON, 9.0_f64 * epsilon,
        );

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f64, 3.0000003_f32, -4.0000004_f64,
            -5.0000005_f32, 6.0000006_f64, 7.0000007_f32, -8.0000008_f64,
        );
        let rhs = (
            -1.0_f32, 2.0_f64, 3.0_f32, -4.0_f64,
            -5.0_f32, 6.0_f64, 7.0_f32, -8.0_f64,
        );
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (
            0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon,
            4.5_f32 * f32::EPSILON, 5.5_f64 * epsilon, 6.5_f32 * f32::EPSILON, 7.5_f64 * epsilon,
        );
        let max_relative = (
            0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon,
            4.5_f32 * f32::EPSILON, 5.5_f64 * epsilon, 6.5_f32 * f32::EPSILON, 7.5_f64 * epsilon,
        );

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff() {
        let lhs = (
            1.0_f32,  2.0_f64,  4.0_f32,  8.0_f64,
            16.0_f32, 32.0_f64, 64.0_f32, 128.0_f64,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f64,  4.0000044_f32,  8.0000088_f64,
            16.0000176_f32, 32.0000352_f64, 64.0000704_f32, 128.0001408_f64,
        );
        let abs_diff = (
            0.0000010728836_f32, 0.00000219999999995224_f64, 0.0000042915344_f32, 0.00000879999999980896_f64,
            0.000017166138_f32,  0.00003519999999923584_f64, 0.00006866455_f32,   0.00014079999999694337_f64,
        );

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = (
            1.0_f32,  2.0_f64,  4.0_f32,  8.0_f64,
            16.0_f32, 32.0_f64, 64.0_f32, 128.0_f64,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f64,  4.0000044_f32,  8.0000088_f64,
            16.0000176_f32, 32.0000352_f64, 64.0000704_f32, 128.0001408_f64,
        );
        let max_abs_diff = (
            0.2_f32, 0.3_f64, 0.4_f32, 0.5_f64,
            0.6_f32, 0.7_f64, 0.8_f32, 1.6_f64,
        );

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), max_abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = (
            1.0_f32,  2.0_f64,  4.0_f32,  8.0_f64,
            16.0_f32, 32.0_f64, 64.0_f32, 128.0_f64,
        );
        let rhs = (
            1.0000011_f32,  2.0000022_f64,  4.0000044_f32,  8.0000088_f64,
            16.0000176_f32, 32.0000352_f64, 64.0000704_f32, 128.0001408_f64,
        );
        let max_relative = (
            0.2_f32, 0.3_f64, 0.4_f32, 0.5_f64,
            0.6_f32, 0.7_f64, 0.8_f32, 1.6_f64,
        );

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple9_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
    };

    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32, 6.0000006_f32, 7.0000007_f32, -8.0000008_f32,
             9.0000009_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32, 6.0_f32, 7.0_f32, -8.0_f32,
             9.0_f32,
        );
        let max_abs_diff = (
            2.0_f32 * f32::EPSILON,  3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON,
            6.0_f32 * f32::EPSILON,  7.0_f32 * f32::EPSILON, 8.0_f32 * f32::EPSILON, 9.0_f32 * f32::EPSILON,
            10.0_f32 * f32::EPSILON,
        );
        let max_relative = (
            2.0_f32 * f32::EPSILON,  3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON,
            6.0_f32 * f32::EPSILON,  7.0_f32 * f32::EPSILON, 8.0_f32 * f32::EPSILON, 9.0_f32 * f32::EPSILON,
            10.0_f32 * f32::EPSILON,
        );

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32, 6.0000006_f32, 7.0000007_f32, -8.0000008_f32,
             9.0000009_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32, 6.0_f32, 7.0_f32, -8.0_f32,
             9.0_f32,
        );
        let max_abs_diff = (
            0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON, 3.5_f32 * f32::EPSILON,
            4.5_f32 * f32::EPSILON, 5.5_f32 * f32::EPSILON, 6.5_f32 * f32::EPSILON, 7.5_f32 * f32::EPSILON,
            8.5_f32 * f32::EPSILON,
        );
        let max_relative = (
            0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON, 3.5_f32 * f32::EPSILON,
            4.5_f32 * f32::EPSILON, 5.5_f32 * f32::EPSILON, 6.5_f32 * f32::EPSILON, 7.5_f32 * f32::EPSILON,
            8.5_f32 * f32::EPSILON,
        );

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32, 6.0000006_f32, 7.0000007_f32, -8.0000008_f32,
             9.0000009_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32, 6.0_f32, 7.0_f32, -8.0_f32,
             9.0_f32,
        );
        let max_abs_diff = 1.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32, 6.0000006_f32, 7.0000007_f32, -8.0000008_f32,
             9.0000009_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32,
            -5.0_f32, 6.0_f32, 7.0_f32, -8.0_f32,
             9.0_f32,
        );
        let max_abs_diff = 0.5_f32 * f32::EPSILON;
        let max_relative = 0.5_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = (
            1.0_f32,   2.0_f32,  4.0_f32,  8.0_f32,
            16.0_f32,  32.0_f32, 64.0_f32, 128.0_f32,
            256.0_f32,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f32,  4.0000044_f32,  8.0000088_f32,
            16.0000176_f32,  32.0000352_f32, 64.0000704_f32, 128.0001408_f32,
            256.0002816_f32,
        );
        let max_abs_diff_all = 0.2_f32;
        let max_abs_diff = (
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32,
        );

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), max_abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = (
            1.0_f32,   2.0_f32,  4.0_f32,  8.0_f32,
            16.0_f32,  32.0_f32, 64.0_f32, 128.0_f32,
            256.0_f32,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f32,  4.0000044_f32,  8.0000088_f32,
            16.0000176_f32,  32.0000352_f32, 64.0000704_f32, 128.0001408_f32,
            256.0002816_f32,
        );
        let max_relative_all = 0.2_f32;
        let max_relative = (
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32,
        );

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), max_relative);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative_all), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple9_heterogenous_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeEq,
    };

    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f64, 3.0000003_f32, -4.0000004_f64,
            -5.0000005_f32, 6.0000006_f64, 7.0000007_f32, -8.0000008_f64,
             9.0000009_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f64, 3.0_f32, -4.0_f64,
            -5.0_f32, 6.0_f64, 7.0_f32, -8.0_f64,
             9.0_f32,
        );
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (
            2.0_f32 * f32::EPSILON,  3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon,
            6.0_f32 * f32::EPSILON,  7.0_f64 * epsilon, 8.0_f32 * f32::EPSILON, 9.0_f64 * epsilon,
            10.0_f32 * f32::EPSILON,
        );
        let max_relative = (
            2.0_f32 * f32::EPSILON,  3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon,
            6.0_f32 * f32::EPSILON,  7.0_f64 * epsilon, 8.0_f32 * f32::EPSILON, 9.0_f64 * epsilon,
            10.0_f32 * f32::EPSILON,
        );

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        let lhs = (
            -1.0000001_f32, 2.0000002_f64, 3.0000003_f32, -4.0000004_f64,
            -5.0000005_f32, 6.0000006_f64, 7.0000007_f32, -8.0000008_f64,
             9.0000009_f32,
        );
        let rhs = (
            -1.0_f32, 2.0_f64, 3.0_f32, -4.0_f64,
            -5.0_f32, 6.0_f64, 7.0_f32, -8.0_f64,
             9.0_f32,
        );
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (
            0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon,
            4.5_f32 * f32::EPSILON, 5.5_f64 * epsilon, 6.5_f32 * f32::EPSILON, 7.5_f64 * epsilon,
            8.5_f32 * f32::EPSILON,
        );
        let max_relative = (
            0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon,
            4.5_f32 * f32::EPSILON, 5.5_f64 * epsilon, 6.5_f32 * f32::EPSILON, 7.5_f64 * epsilon,
            8.5_f32 * f32::EPSILON,
        );

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff() {
        let lhs = (
            1.0_f32,   2.0_f64,  4.0_f32,  8.0_f64,
            16.0_f32,  32.0_f64, 64.0_f32, 128.0_f64,
            256.0_f32,
        );
        let rhs = (
            1.0000011_f32, 2.0000022_f64, 4.0000044_f32, 8.0000088_f64,
            16.0000176_f32, 32.0000352_f64, 64.0000704_f32, 128.0001408_f64,
            256.0002816_f32,
        );
        let abs_diff = (
            0.0000010728836_f32, 0.00000219999999995224_f64, 0.0000042915344_f32, 0.00000879999999980896_f64,
            0.000017166138_f32, 0.00003519999999923584_f64, 0.00006866455_f32, 0.00014079999999694337_f64,
            0.0002746582_f32,
        );

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = (
            1.0_f32,   2.0_f64,  4.0_f32,  8.0_f64,
            16.0_f32,  32.0_f64, 64.0_f32, 128.0_f64,
            256.0_f32,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f64,  4.0000044_f32,  8.0000088_f64,
            16.0000176_f32,  32.0000352_f64, 64.0000704_f32, 128.0001408_f64,
            256.0002816_f32,
        );
        let max_abs_diff = (
            0.2_f32, 0.3_f64, 0.4_f32, 0.5_f64,
            0.6_f32, 0.7_f64, 0.8_f32, 1.6_f64,
            3.2_f32,
        );

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), max_abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = (
            1.0_f32,   2.0_f64,  4.0_f32,  8.0_f64,
            16.0_f32,  32.0_f64, 64.0_f32, 128.0_f64,
            256.0_f32,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f64,  4.0000044_f32,  8.0000088_f64,
            16.0000176_f32,  32.0000352_f64, 64.0000704_f32, 128.0001408_f64,
            256.0002816_f32,
        );
        let max_relative = (
            0.2_f32, 0.3_f64, 0.4_f32, 0.5_f64,
            0.6_f32, 0.7_f64, 0.8_f32, 1.6_f64,
            3.2_f32,
        );

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple10_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
    };

    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f32,   3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32,  6.0000006_f32,   7.0000007_f32, -8.0000008_f32,
             9.0000009_f32, -10.00000001_f32,
        );
        let rhs = (
            -1.0_f32,  2.0_f32,  3.0_f32, -4.0_f32,
            -5.0_f32,  6.0_f32,  7.0_f32, -8.0_f32,
             9.0_f32, -10.0_f32,
        );
        let max_abs_diff = (
            2.0_f32 * f32::EPSILON,  3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON,
            6.0_f32 * f32::EPSILON,  7.0_f32 * f32::EPSILON, 8.0_f32 * f32::EPSILON, 9.0_f32 * f32::EPSILON,
            10.0_f32 * f32::EPSILON, 1.0_f32 * f32::EPSILON,
        );
        let max_relative = (
            2.0_f32 * f32::EPSILON,  3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON,
            6.0_f32 * f32::EPSILON,  7.0_f32 * f32::EPSILON, 8.0_f32 * f32::EPSILON, 9.0_f32 * f32::EPSILON,
            10.0_f32 * f32::EPSILON, 1.0_f32 * f32::EPSILON,
        );

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f32,   3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32,  6.0000006_f32,   7.0000007_f32, -8.0000008_f32,
             9.0000009_f32, -10.00000001_f32,
        );
        let rhs = (
            -1.0_f32,  2.0_f32,  3.0_f32, -4.0_f32,
            -5.0_f32,  6.0_f32,  7.0_f32, -8.0_f32,
             9.0_f32, -10.0_f32,
        );
        let max_abs_diff = (
            0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON, 3.5_f32 * f32::EPSILON,
            4.5_f32 * f32::EPSILON, 5.5_f32 * f32::EPSILON, 6.5_f32 * f32::EPSILON, 7.5_f32 * f32::EPSILON,
            8.5_f32 * f32::EPSILON, 0.5_f32 * f32::EPSILON,
        );
        let max_relative = (
            0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON, 3.5_f32 * f32::EPSILON,
            4.5_f32 * f32::EPSILON, 5.5_f32 * f32::EPSILON, 6.5_f32 * f32::EPSILON, 7.5_f32 * f32::EPSILON,
            8.5_f32 * f32::EPSILON, 0.5_f32 * f32::EPSILON,
        );

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f32,   3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32,  6.0000006_f32,   7.0000007_f32, -8.0000008_f32,
             9.0000009_f32, -10.00000001_f32,
        );
        let rhs = (
            -1.0_f32,  2.0_f32,  3.0_f32, -4.0_f32,
            -5.0_f32,  6.0_f32,  7.0_f32, -8.0_f32,
             9.0_f32, -10.0_f32,
        );
        let max_abs_diff = 1.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f32,   3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32,  6.0000006_f32,   7.0000007_f32, -8.0000008_f32,
             9.0000009_f32, -10.00000001_f32,
        );
        let rhs = (
            -1.0_f32,  2.0_f32,  3.0_f32, -4.0_f32,
            -5.0_f32,  6.0_f32,  7.0_f32, -8.0_f32,
             9.0_f32, -10.0_f32,
        );
        let max_abs_diff = 0.5_f32 * f32::EPSILON;
        let max_relative = 0.5_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = (
            1.0_f32,   2.0_f32,   4.0_f32,  8.0_f32,
            16.0_f32,  32.0_f32,  64.0_f32, 128.0_f32,
            256.0_f32, 512.0_f32,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f32,   4.0000044_f32,  8.0000088_f32,
            16.0000176_f32,  32.0000352_f32,  64.0000704_f32, 128.0001408_f32,
            256.0002816_f32, 512.0005632_f32,
        );
        let max_abs_diff_all = 0.2_f32;
        let max_abs_diff = (
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32,
        );

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), max_abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = (
            1.0_f32,   2.0_f32,   4.0_f32,  8.0_f32,
            16.0_f32,  32.0_f32,  64.0_f32, 128.0_f32,
            256.0_f32, 512.0_f32,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f32,   4.0000044_f32,  8.0000088_f32,
            16.0000176_f32,  32.0000352_f32,  64.0000704_f32, 128.0001408_f32,
            256.0002816_f32, 512.0005632_f32,
        );
        let max_relative_all = 0.2_f32;
        let max_relative = (
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32,
        );

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), max_relative);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative_all), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple10_heterogenous_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeEq,
    };

    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f64,   3.0000003_f32, -4.0000004_f64,
            -5.0000005_f32,  6.0000006_f64,   7.0000007_f32, -8.0000008_f64,
             9.0000009_f32, -10.00000001_f64,
        );
        let rhs = (
            -1.0_f32,  2.0_f64,  3.0_f32, -4.0_f64,
            -5.0_f32,  6.0_f64,  7.0_f32, -8.0_f64,
             9.0_f32, -10.0_f64,
        );
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (
            2.0_f32 * f32::EPSILON,  3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon,
            6.0_f32 * f32::EPSILON,  7.0_f64 * epsilon, 8.0_f32 * f32::EPSILON, 9.0_f64 * epsilon,
            10.0_f32 * f32::EPSILON, 1.0_f64 * epsilon,
        );
        let max_relative = (
            2.0_f32 * f32::EPSILON,  3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon,
            6.0_f32 * f32::EPSILON,  7.0_f64 * epsilon, 8.0_f32 * f32::EPSILON, 9.0_f64 * epsilon,
            10.0_f32 * f32::EPSILON, 1.0_f64 * epsilon,
        );

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f64,   3.0000003_f32, -4.0000004_f64,
            -5.0000005_f32,  6.0000006_f64,   7.0000007_f32, -8.0000008_f64,
             9.0000009_f32, -10.00000001_f64,
        );
        let rhs = (
            -1.0_f32,  2.0_f64,  3.0_f32, -4.0_f64,
            -5.0_f32,  6.0_f64,  7.0_f32, -8.0_f64,
             9.0_f32, -10.0_f64,
        );
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (
            0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon,
            4.5_f32 * f32::EPSILON, 5.5_f64 * epsilon, 6.5_f32 * f32::EPSILON, 7.5_f64 * epsilon,
            8.5_f32 * f32::EPSILON, 0.5_f64 * epsilon,
        );
        let max_relative = (
            0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon,
            4.5_f32 * f32::EPSILON, 5.5_f64 * epsilon, 6.5_f32 * f32::EPSILON, 7.5_f64 * epsilon,
            8.5_f32 * f32::EPSILON, 0.5_f64 * epsilon,
        );

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff() {
        let lhs = (
            1.0_f32,   2.0_f64,   4.0_f32,  8.0_f64,
            16.0_f32,  32.0_f64,  64.0_f32, 128.0_f64,
            256.0_f32, 512.0_f64,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f64,   4.0000044_f32,  8.0000088_f64,
            16.0000176_f32,  32.0000352_f64,  64.0000704_f32, 128.0001408_f64,
            256.0002816_f32, 512.0005632_f64,
        );
        let abs_diff = (
            0.0000010728836_f32, 0.00000219999999995224_f64, 0.0000042915344_f32, 0.00000879999999980896_f64,
            0.000017166138_f32,  0.00003519999999923584_f64, 0.00006866455_f32,   0.00014079999999694337_f64,
            0.0002746582_f32,    0.0005631999999877735_f64,
        );

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = (
            1.0_f32,   2.0_f64,   4.0_f32,  8.0_f64,
            16.0_f32,  32.0_f64,  64.0_f32, 128.0_f64,
            256.0_f32, 512.0_f64,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f64,   4.0000044_f32,  8.0000088_f64,
            16.0000176_f32,  32.0000352_f64,  64.0000704_f32, 128.0001408_f64,
            256.0002816_f32, 512.0005632_f64,
        );
        let max_abs_diff = (
            0.2_f32, 0.3_f64, 0.4_f32, 0.5_f64,
            0.6_f32, 0.7_f64, 0.8_f32, 1.6_f64,
            3.2_f32, 6.4_f64,
        );

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), max_abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = (
            1.0_f32,   2.0_f64,   4.0_f32,  8.0_f64,
            16.0_f32,  32.0_f64,  64.0_f32, 128.0_f64,
            256.0_f32, 512.0_f64,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f64,   4.0000044_f32,  8.0000088_f64,
            16.0000176_f32,  32.0000352_f64,  64.0000704_f32, 128.0001408_f64,
            256.0002816_f32, 512.0005632_f64,
        );
        let max_relative = (
            0.2_f32, 0.3_f64, 0.4_f32, 0.5_f64,
            0.6_f32, 0.7_f64, 0.8_f32, 1.6_f64,
            3.2_f32, 6.4_f64,
        );

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple11_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
    };

    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f32,  3.0000003_f32,  -4.0000004_f32,
            -5.0000005_f32,  6.0000006_f32,  7.0000007_f32,  -8.0000008_f32,
             9.0000009_f32, -10.0000001_f32, 11.0000002_f32,
        );
        let rhs = (
            -1.0_f32,  2.0_f32,  3.0_f32,  -4.0_f32,
            -5.0_f32,  6.0_f32,  7.0_f32,  -8.0_f32,
             9.0_f32, -10.0_f32, 11.0_f32,
        );
        let max_abs_diff = (
            2.0_f32 * f32::EPSILON,  3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON,
            6.0_f32 * f32::EPSILON,  7.0_f32 * f32::EPSILON, 8.0_f32 * f32::EPSILON, 9.0_f32 * f32::EPSILON,
            10.0_f32 * f32::EPSILON, 1.0_f32 * f32::EPSILON, 2.0_f32 * f32::EPSILON,
        );
        let max_relative = (
            2.0_f32 * f32::EPSILON,  3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON,
            6.0_f32 * f32::EPSILON,  7.0_f32 * f32::EPSILON, 8.0_f32 * f32::EPSILON, 9.0_f32 * f32::EPSILON,
            10.0_f32 * f32::EPSILON, 1.0_f32 * f32::EPSILON, 2.0_f32 * f32::EPSILON,
        );

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f32,  3.0000003_f32, -4.0000004_f32,
            -5.0000005_f32,  6.0000006_f32,  7.0000007_f32, -8.0000008_f32,
             9.0000009_f32, -10.0000001_f32, 11.0000002_f32,
        );
        let rhs = (
            -1.0_f32,  2.0_f32,  3.0_f32,  -4.0_f32,
            -5.0_f32,  6.0_f32,  7.0_f32,  -8.0_f32,
             9.0_f32, -10.0_f32, 11.0_f32,
        );
        let max_abs_diff = (
            0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON, 3.5_f32 * f32::EPSILON,
            4.5_f32 * f32::EPSILON, 5.5_f32 * f32::EPSILON, 6.5_f32 * f32::EPSILON, 7.5_f32 * f32::EPSILON,
            8.5_f32 * f32::EPSILON, 0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON,
        );
        let max_relative = (
            0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON, 3.5_f32 * f32::EPSILON,
            4.5_f32 * f32::EPSILON, 5.5_f32 * f32::EPSILON, 6.5_f32 * f32::EPSILON, 7.5_f32 * f32::EPSILON,
            8.5_f32 * f32::EPSILON, 0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON,
        );

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f32,  3.0000003_f32,  -4.0000004_f32,
            -5.0000005_f32,  6.0000006_f32,  7.0000007_f32,  -8.0000008_f32,
             9.0000009_f32, -10.0000001_f32, 11.0000002_f32,
        );
        let rhs = (
            -1.0_f32,  2.0_f32,  3.0_f32,  -4.0_f32,
            -5.0_f32,  6.0_f32,  7.0_f32,  -8.0_f32,
             9.0_f32, -10.0_f32, 11.0_f32,
        );
        let max_abs_diff = 1.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f32,  3.0000003_f32,  -4.0000004_f32,
            -5.0000005_f32,  6.0000006_f32,  7.0000007_f32,  -8.0000008_f32,
             9.0000009_f32, -10.0000001_f32, 11.0000002_f32,
        );
        let rhs = (
            -1.0_f32,  2.0_f32,  3.0_f32,  -4.0_f32,
            -5.0_f32,  6.0_f32,  7.0_f32,  -8.0_f32,
             9.0_f32, -10.0_f32, 11.0_f32,
        );
        let max_abs_diff = 0.5_f32 * f32::EPSILON;
        let max_relative = 0.5_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = (
            1.0_f32,   2.0_f32,   4.0_f32,    8.0_f32,
            16.0_f32,  32.0_f32,  64.0_f32,   128.0_f32,
            256.0_f32, 512.0_f32, 1024.0_f32,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f32,   4.0000044_f32,    8.0000088_f32,
            16.0000176_f32,  32.0000352_f32,  64.0000704_f32,   128.0001408_f32,
            256.0002816_f32, 512.0005632_f32, 1024.0011264_f32,
        );
        let max_abs_diff_all = 0.2_f32;
        let max_abs_diff = (
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32, 0.2_f32,
        );

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), max_abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = (
            1.0_f32,   2.0_f32,   4.0_f32,    8.0_f32,
            16.0_f32,  32.0_f32,  64.0_f32,   128.0_f32,
            256.0_f32, 512.0_f32, 1024.0_f32,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f32,   4.0000044_f32,    8.0000088_f32,
            16.0000176_f32,  32.0000352_f32,  64.0000704_f32,   128.0001408_f32,
            256.0002816_f32, 512.0005632_f32, 1024.0011264_f32,
        );
        let max_relative_all = 0.2_f32;
        let max_relative = (
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32, 0.2_f32,
        );

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), max_relative);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative_all), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple11_heterogenous_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeEq,
    };

    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f64,  3.0000003_f32,  -4.0000004_f64,
            -5.0000005_f32,  6.0000006_f64,  7.0000007_f32,  -8.0000008_f64,
             9.0000009_f32, -10.0000001_f64, 11.0000002_f32,
        );
        let rhs = (
            -1.0_f32,  2.0_f64,  3.0_f32,  -4.0_f64,
            -5.0_f32,  6.0_f64,  7.0_f32,  -8.0_f64,
             9.0_f32, -10.0_f64, 11.0_f32,
        );
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (
            2.0_f32 * f32::EPSILON,  3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon,
            6.0_f32 * f32::EPSILON,  7.0_f64 * epsilon, 8.0_f32 * f32::EPSILON, 9.0_f64 * epsilon,
            10.0_f32 * f32::EPSILON, 1.0_f64 * epsilon, 2.0_f32 * f32::EPSILON,
        );
        let max_relative = (
            2.0_f32 * f32::EPSILON,  3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon,
            6.0_f32 * f32::EPSILON,  7.0_f64 * epsilon, 8.0_f32 * f32::EPSILON, 9.0_f64 * epsilon,
            10.0_f32 * f32::EPSILON, 1.0_f64 * epsilon, 2.0_f32 * f32::EPSILON,
        );

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f64,  3.0000003_f32,  -4.0000004_f64,
            -5.0000005_f32,  6.0000006_f64,  7.0000007_f32,  -8.0000008_f64,
             9.0000009_f32, -10.0000001_f64, 11.0000002_f32,
        );
        let rhs = (
            -1.0_f32,  2.0_f64,  3.0_f32,  -4.0_f64,
            -5.0_f32,  6.0_f64,  7.0_f32,  -8.0_f64,
             9.0_f32, -10.0_f64, 11.0_f32,
        );
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (
            0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon,
            4.5_f32 * f32::EPSILON, 5.5_f64 * epsilon, 6.5_f32 * f32::EPSILON, 7.5_f64 * epsilon,
            8.5_f32 * f32::EPSILON, 0.5_f64 * epsilon, 1.5_f32 * f32::EPSILON,
        );
        let max_relative = (
            0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon,
            4.5_f32 * f32::EPSILON, 5.5_f64 * epsilon, 6.5_f32 * f32::EPSILON, 7.5_f64 * epsilon,
            8.5_f32 * f32::EPSILON, 0.5_f64 * epsilon, 1.5_f32 * f32::EPSILON,
        );

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff() {
        let lhs = (
            1.0_f32,   2.0_f64,    4.0_f32,   8.0_f64,
            16.0_f32,  32.0_f64,  64.0_f32,   128.0_f64,
            256.0_f32, 512.0_f64, 1024.0_f32,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f64,   4.0000044_f32,    8.0000088_f64,
            16.0000176_f32,  32.0000352_f64,  64.0000704_f32,   128.0001408_f64,
            256.0002816_f32, 512.0005632_f64, 1024.0011264_f32,
        );
        let abs_diff = (
            0.0000010728836_f32, 0.00000219999999995224_f64, 0.0000042915344_f32, 0.00000879999999980896_f64,
            0.000017166138_f32,  0.00003519999999923584_f64, 0.00006866455_f32,   0.00014079999999694337_f64,
            0.0002746582_f32,    0.0005631999999877735_f64,  0.0010986328_f32,
        );

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = (
            1.0_f32,   2.0_f64,   4.0_f32,    8.0_f64,
            16.0_f32,  32.0_f64,  64.0_f32,   128.0_f64,
            256.0_f32, 512.0_f64, 1024.0_f32,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f64,   4.0000044_f32,    8.0000088_f64,
            16.0000176_f32,  32.0000352_f64,  64.0000704_f32,   128.0001408_f64,
            256.0002816_f32, 512.0005632_f64, 1024.0011264_f32,
        );
        let max_abs_diff = (
            0.2_f32, 0.3_f64, 0.4_f32,  0.5_f64,
            0.6_f32, 0.7_f64, 0.8_f32,  1.6_f64,
            3.2_f32, 6.4_f64, 12.8_f32,
        );

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), max_abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = (
            1.0_f32,   2.0_f64,   4.0_f32,    8.0_f64,
            16.0_f32,  32.0_f64,  64.0_f32,   128.0_f64,
            256.0_f32, 512.0_f64, 1024.0_f32,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f64,   4.0000044_f32,    8.0000088_f64,
            16.0000176_f32,  32.0000352_f64,  64.0000704_f32,   128.0001408_f64,
            256.0002816_f32, 512.0005632_f64, 1024.0011264_f32,
        );
        let max_relative = (
            0.2_f32, 0.3_f64, 0.4_f32,  0.5_f64,
            0.6_f32, 0.7_f64, 0.8_f32,  1.6_f64,
            3.2_f32, 6.4_f64, 12.8_f32,
        );

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple12_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeAllEq,
    };

    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f32,  3.0000003_f32,  -4.0000004_f32,
            -5.0000005_f32,  6.0000006_f32,  7.0000007_f32,  -8.0000008_f32,
             9.0000009_f32, -10.0000001_f32, 11.0000002_f32,  12.0000003_f32,
        );
        let rhs = (
            -1.0_f32,  2.0_f32,  3.0_f32,  -4.0_f32,
            -5.0_f32,  6.0_f32,  7.0_f32,  -8.0_f32,
             9.0_f32, -10.0_f32, 11.0_f32,  12.0_f32,
        );
        let max_abs_diff = (
            2.0_f32 * f32::EPSILON,  3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON,
            6.0_f32 * f32::EPSILON,  7.0_f32 * f32::EPSILON, 8.0_f32 * f32::EPSILON, 9.0_f32 * f32::EPSILON,
            10.0_f32 * f32::EPSILON, 1.0_f32 * f32::EPSILON, 2.0_f32 * f32::EPSILON, 3.0_f32 * f32::EPSILON,
        );
        let max_relative = (
            2.0_f32 * f32::EPSILON,  3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON,
            6.0_f32 * f32::EPSILON,  7.0_f32 * f32::EPSILON, 8.0_f32 * f32::EPSILON, 9.0_f32 * f32::EPSILON,
            10.0_f32 * f32::EPSILON, 1.0_f32 * f32::EPSILON, 2.0_f32 * f32::EPSILON, 3.0_f32 * f32::EPSILON,
        );

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f32,  3.0000003_f32,  -4.0000004_f32,
            -5.0000005_f32,  6.0000006_f32,  7.0000007_f32,  -8.0000008_f32,
             9.0000009_f32, -10.0000001_f32, 11.0000002_f32,  12.0000003_f32,
        );
        let rhs = (
            -1.0_f32,  2.0_f32,  3.0_f32,  -4.0_f32,
            -5.0_f32,  6.0_f32,  7.0_f32,  -8.0_f32,
             9.0_f32, -10.0_f32, 11.0_f32,  12.0_f32,
        );
        let max_abs_diff = (
            0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON, 3.5_f32 * f32::EPSILON,
            4.5_f32 * f32::EPSILON, 5.5_f32 * f32::EPSILON, 6.5_f32 * f32::EPSILON, 7.5_f32 * f32::EPSILON,
            8.5_f32 * f32::EPSILON, 0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON,
        );
        let max_relative = (
            0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON, 3.5_f32 * f32::EPSILON,
            4.5_f32 * f32::EPSILON, 5.5_f32 * f32::EPSILON, 6.5_f32 * f32::EPSILON, 7.5_f32 * f32::EPSILON,
            8.5_f32 * f32::EPSILON, 0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON,
        );

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_eq() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f32,  3.0000003_f32,  -4.0000004_f32,
            -5.0000005_f32,  6.0000006_f32,  7.0000007_f32,  -8.0000008_f32,
             9.0000009_f32, -10.0000001_f32, 11.0000002_f32,  12.0000003_f32,
        );
        let rhs = (
            -1.0_f32,  2.0_f32,  3.0_f32,  -4.0_f32,
            -5.0_f32,  6.0_f32,  7.0_f32,  -8.0_f32,
             9.0_f32, -10.0_f32, 11.0_f32,  12.0_f32,
        );
        let max_abs_diff = 1.0_f32 * f32::EPSILON;
        let max_relative = 1.0_f32 * f32::EPSILON;

        assert_relative_eq!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_all_ne() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f32,  3.0000003_f32,  -4.0000004_f32,
            -5.0000005_f32,  6.0000006_f32,  7.0000007_f32,  -8.0000008_f32,
             9.0000009_f32, -10.0000001_f32, 11.0000002_f32,  12.0000003_f32,
        );
        let rhs = (
            -1.0_f32,  2.0_f32,  3.0_f32,  -4.0_f32,
            -5.0_f32,  6.0_f32,  7.0_f32,  -8.0_f32,
             9.0_f32, -10.0_f32, 11.0_f32,  12.0_f32,
        );
        let max_abs_diff = 0.5_f32 * f32::EPSILON;
        let max_relative = 0.5_f32 * f32::EPSILON;

        assert_relative_ne!(lhs, rhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff_all <= max_abs_diff, relative_all <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_all_tolerance() {
        let lhs = (
            1.0_f32,   2.0_f32,   4.0_f32,    8.0_f32,
            16.0_f32,  32.0_f32,  64.0_f32,   128.0_f32,
            256.0_f32, 512.0_f32, 1024.0_f32, 2048.0_f32,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f32,   4.0000044_f32,    8.0000088_f32,
            16.0000176_f32,  32.0000352_f32,  64.0000704_f32,   128.0001408_f32,
            256.0002816_f32, 512.0005632_f32, 1024.0011264_f32, 2048.0022528_f32,
        );
        let max_abs_diff_all = 0.2_f32;
        let max_abs_diff = (
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
        );

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff_all), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff_all), max_abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_all_tolerance() {
        let lhs = (
            1.0_f32,   2.0_f32,   4.0_f32,    8.0_f32,
            16.0_f32,  32.0_f32,  64.0_f32,   128.0_f32,
            256.0_f32, 512.0_f32, 1024.0_f32, 2048.0_f32,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f32,   4.0000044_f32,    8.0000088_f32,
            16.0000176_f32,  32.0000352_f32,  64.0000704_f32,   128.0001408_f32,
            256.0002816_f32, 512.0005632_f32, 1024.0011264_f32, 2048.0022528_f32,
        );
        let max_relative_all = 0.2_f32;
        let max_relative = (
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
            0.2_f32, 0.2_f32, 0.2_f32, 0.2_f32,
        );

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative_all), max_relative);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative_all), max_relative);
    }
}

#[cfg(test)]
mod relative_eq_tuple12_heterogenous_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        AssertRelativeEq,
    };

    #[rustfmt::skip]
    #[test]
    fn test_eq() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f64,  3.0000003_f32,  -4.0000004_f64,
            -5.0000005_f32,  6.0000006_f64,  7.0000007_f32,  -8.0000008_f64,
             9.0000009_f32, -10.0000001_f64, 11.0000002_f32,  12.0000003_f64,
        );
        let rhs = (
            -1.0_f32,  2.0_f64,  3.0_f32,  -4.0_f64,
            -5.0_f32,  6.0_f64,  7.0_f32,  -8.0_f64,
             9.0_f32, -10.0_f64, 11.0_f32,  12.0_f64,
        );
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (
            2.0_f32 * f32::EPSILON,  3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon,
            6.0_f32 * f32::EPSILON,  7.0_f64 * epsilon, 8.0_f32 * f32::EPSILON, 9.0_f64 * epsilon,
            10.0_f32 * f32::EPSILON, 1.0_f64 * epsilon, 2.0_f32 * f32::EPSILON, 3.0_f64 * epsilon,
        );
        let max_relative = (
            2.0_f32 * f32::EPSILON,  3.0_f64 * epsilon, 4.0_f32 * f32::EPSILON, 5.0_f64 * epsilon,
            6.0_f32 * f32::EPSILON,  7.0_f64 * epsilon, 8.0_f32 * f32::EPSILON, 9.0_f64 * epsilon,
            10.0_f32 * f32::EPSILON, 1.0_f64 * epsilon, 2.0_f32 * f32::EPSILON, 3.0_f64 * epsilon,
        );

        assert_relative_eq!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_eq!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ne() {
        let lhs = (
            -1.0000001_f32,  2.0000002_f64,  3.0000003_f32,  -4.0000004_f64,
            -5.0000005_f32,  6.0000006_f64,  7.0000007_f32,  -8.0000008_f64,
             9.0000009_f32, -10.0000001_f64, 11.0000002_f32,  12.0000003_f64,
        );
        let rhs = (
            -1.0_f32,  2.0_f64,  3.0_f32,  -4.0_f64,
            -5.0_f32,  6.0_f64,  7.0_f32,  -8.0_f64,
             9.0_f32, -10.0_f64, 11.0_f32,  12.0_f64,
        );
        let epsilon = f32::EPSILON as f64;
        let max_abs_diff = (
            0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon,
            4.5_f32 * f32::EPSILON, 5.5_f64 * epsilon, 6.5_f32 * f32::EPSILON, 7.5_f64 * epsilon,
            8.5_f32 * f32::EPSILON, 0.5_f64 * epsilon, 1.5_f32 * f32::EPSILON, 2.5_f64 * epsilon,
        );
        let max_relative = (
            0.5_f32 * f32::EPSILON, 1.5_f64 * epsilon, 2.5_f32 * f32::EPSILON, 3.5_f64 * epsilon,
            4.5_f32 * f32::EPSILON, 5.5_f64 * epsilon, 6.5_f32 * f32::EPSILON, 7.5_f64 * epsilon,
            8.5_f32 * f32::EPSILON, 0.5_f64 * epsilon, 1.5_f32 * f32::EPSILON, 2.5_f64 * epsilon,
        );

        assert_relative_ne!(lhs, rhs, abs_diff <= max_abs_diff, relative <= max_relative);
        assert_relative_ne!(rhs, lhs, abs_diff <= max_abs_diff, relative <= max_relative);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff() {
        let lhs = (
            1.0_f32,   2.0_f64,   4.0_f32,    8.0_f64,
            16.0_f32,  32.0_f64,  64.0_f32,   128.0_f64,
            256.0_f32, 512.0_f64, 1024.0_f32, 2048.0_f64,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f64,   4.0000044_f32,    8.0000088_f64,
            16.0000176_f32,  32.0000352_f64,  64.0000704_f32,   128.0001408_f64,
            256.0002816_f32, 512.0005632_f64, 1024.0011264_f32, 2048.0022528_f64,
        );
        let abs_diff = (
            0.0000010728836_f32, 0.00000219999999995224_f64, 0.0000042915344_f32, 0.00000879999999980896_f64,
            0.000017166138_f32,  0.00003519999999923584_f64, 0.00006866455_f32,   0.00014079999999694337_f64,
            0.0002746582_f32,    0.0005631999999877735_f64,  0.0010986328_f32,    0.002252799999951094_f64,
        );

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_abs_diff_tolerance() {
        let lhs = (
            1.0_f32,   2.0_f64,   4.0_f32,    8.0_f64,
            16.0_f32,  32.0_f64,  64.0_f32,   128.0_f64,
            256.0_f32, 512.0_f64, 1024.0_f32, 2048.0_f64,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f64,   4.0000044_f32,    8.0000088_f64,
            16.0000176_f32,  32.0000352_f64,  64.0000704_f32,   128.0001408_f64,
            256.0002816_f32, 512.0005632_f64, 1024.0011264_f32, 2048.0022528_f64,
        );
        let max_abs_diff = (
            0.2_f32, 0.3_f64, 0.4_f32,  0.5_f64,
            0.6_f32, 0.7_f64, 0.8_f32,  1.6_f64,
            3.2_f32, 6.4_f64, 12.8_f32, 25.6_f64,
        );

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), max_abs_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_tolerance() {
        let lhs = (
            1.0_f32,   2.0_f64,   4.0_f32,    8.0_f64,
            16.0_f32,  32.0_f64,  64.0_f32,   128.0_f64,
            256.0_f32, 512.0_f64, 1024.0_f32, 2048.0_f64,
        );
        let rhs = (
            1.0000011_f32,   2.0000022_f64,   4.0000044_f32,    8.0000088_f64,
            16.0000176_f32,  32.0000352_f64,  64.0000704_f32,   128.0001408_f64,
            256.0002816_f32, 512.0005632_f64, 1024.0011264_f32, 2048.0022528_f64,
        );
        let max_relative = (
            0.2_f32, 0.3_f64, 0.4_f32,  0.5_f64,
            0.6_f32, 0.7_f64, 0.8_f32,  1.6_f64,
            3.2_f32, 6.4_f64, 12.8_f32, 25.6_f64,
        );

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), max_relative);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), max_relative);
    }
}
