extern crate approx_cmp;


#[cfg(test)]
mod abs_diff_eq_unit_tests {
    use approx_cmp::{
        AbsDiffEq,
        AbsDiffAllEq,
        abs_diff_eq,
        assert_abs_diff_eq,
    };

    #[test]
    fn test_eq() {
        assert!(AbsDiffEq::abs_diff_eq(&(), &(), &()));
        assert!(abs_diff_eq!((), (), abs_diff <= ()));
        assert_abs_diff_eq!((), (), abs_diff <= ());
    }

    #[test]
    fn test_all_eq() {
        assert!(AbsDiffAllEq::abs_diff_all_eq(&(), &(), &()));
        assert!(abs_diff_eq!((), (), abs_diff <= ()));
        assert_abs_diff_eq!((), (), abs_diff <= ());
    }
}

#[cfg(test)]
mod abs_diff_eq_tuple1_tests {
    use approx_cmp::{
        assert_abs_diff_eq,
        assert_abs_diff_ne,
    };

    #[test]
    fn test_eq() {
        let lhs = (4.0_f32,);
        let rhs = (4.0000005_f32,);
        let max_abs_diff = (5.0_f32 * f32::EPSILON,);

        assert_abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff);
    }

    #[test]
    fn test_ne() {
        let lhs = (4.0_f32,);
        let rhs = (2.0_f32,);
        let max_abs_diff = (1.0_f32 * f32::EPSILON,);

        assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff);
    }

    #[test]
    fn test_all_eq() {
        let lhs = (4.0_f32,);
        let rhs = (4.0000005_f32,);
        let max_abs_diff_all = 5.0_f32 * f32::EPSILON;

        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= max_abs_diff_all);
    }

    #[test]
    fn test_all_ne() {
        let lhs = (4.0_f32,);
        let rhs = (2.0_f32,);
        let max_abs_diff_all = 1.0_f32 * f32::EPSILON;

        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= max_abs_diff_all);
    }
}

#[cfg(test)]
mod abs_diff_eq_tuple2_tests {
    use approx_cmp::{
        assert_abs_diff_eq,
        assert_abs_diff_ne,
    };

    #[test]
    fn test_eq() {
        let lhs = (2.9999999_f32, 3.0000004_f32);
        let rhs = (3.0_f32, 3.0_f32);
        let max_abs_diff = (1.0_f32 * f32::EPSILON, 5.0_f32 * f32::EPSILON);

        assert_abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff);
    }

    #[test]
    fn test_ne() {
        let lhs = (2.9999995_f32, 3.0000004_f32);
        let rhs = (3.0_f32, 3.0_f32);
        let max_abs_diff = (1.0_f32 * f32::EPSILON, 3.0_f32 * f32::EPSILON);

        assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff);
    }

    #[test]
    fn test_all_eq() {
        let lhs = (2.9999999_f32, 3.0000004_f32);
        let rhs = (3.0_f32, 3.0_f32);
        let max_abs_diff = 5.0_f32 * f32::EPSILON;

        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }

    #[test]
    fn test_all_ne() {
        let lhs = (2.9999995_f32, 3.0000004_f32);
        let rhs = (3.0_f32, 3.0_f32);
        let max_abs_diff = 3.0_f32 * f32::EPSILON;

        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }
}

#[cfg(test)]
mod abs_diff_eq_tuple3_tests {
    use approx_cmp::{
        assert_abs_diff_eq,
        assert_abs_diff_ne,
    };

    #[test]
    fn test_eq() {
        let lhs = (-1.0000001_f32, 2.0000002_f32, 3.0000003_f32);
        let rhs = (-1.0_f32, 2.0_f32, 3.0_f32);
        let max_abs_diff = (2.0_f32 * f32::EPSILON, 3.0_f32 * f32::EPSILON, 4.0_f32 * f32::EPSILON);

        assert_abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff);
    }

    #[test]
    fn test_ne() {
        let lhs = (-1.0000001_f32, 2.0000002_f32, 3.0000003_f32);
        let rhs = (-1.0_f32, 2.0_f32, 3.0_f32);
        let max_abs_diff = (0.5_f32 * f32::EPSILON, 1.5_f32 * f32::EPSILON, 2.5_f32 * f32::EPSILON);

        assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff);
    }

    #[test]
    fn test_all_eq() {
        let lhs = (-1.0000001_f32, 2.0000002_f32, 3.0000003_f32);
        let rhs = (-1.0_f32, 2.0_f32, 3.0_f32);
        let max_abs_diff = 4.0_f32 * f32::EPSILON;

        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }

    #[test]
    fn test_all_ne() {
        let lhs = (-1.0000001_f32, 2.0000002_f32, 3.0000003_f32);
        let rhs = (-1.0_f32, 2.0_f32, 3.0_f32);
        let max_abs_diff = 1.0_f32 * f32::EPSILON;

        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }
}

#[cfg(test)]
mod abs_diff_eq_tuple4_tests {
    use approx_cmp::{
        assert_abs_diff_eq,
        assert_abs_diff_ne,
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

        assert_abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff);
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

        assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff);
    }

    #[test]
    fn test_all_eq() {
        let lhs = (-1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32);
        let rhs = (-1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32);
        let max_abs_diff = 5.0_f32 * f32::EPSILON;

        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }

    #[test]
    fn test_all_ne() {
        let lhs = (-1.0000001_f32, 2.0000002_f32, 3.0000003_f32, -4.0000004_f32);
        let rhs = (-1.0_f32, 2.0_f32, 3.0_f32, -4.0_f32);
        let max_abs_diff = 1.0_f32 * f32::EPSILON;

        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }
}

#[cfg(test)]
mod abs_diff_eq_tuple5_tests {
    use approx_cmp::{
        assert_abs_diff_eq,
        assert_abs_diff_ne,
    };

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

        assert_abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff);
    }

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

        assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff);
    }

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
        let max_abs_diff = 6.0_f32 * f32::EPSILON;

        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }

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
        let max_abs_diff = 1.0_f32 * f32::EPSILON;

        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }
}

#[cfg(test)]
mod abs_diff_eq_tuple6_tests {
    use approx_cmp::{
        assert_abs_diff_eq,
        assert_abs_diff_ne,
    };

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

        assert_abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff);
    }

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

        assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff);
    }

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
        let max_abs_diff = 7.0_f32 * f32::EPSILON;

        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }

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
        let max_abs_diff = 1.0_f32 * f32::EPSILON;

        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }
}

#[cfg(test)]
mod abs_diff_eq_tuple7_tests {
    use approx_cmp::{
        assert_abs_diff_eq,
        assert_abs_diff_ne,
    };

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

        assert_abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff);
    }

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

        assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff);
    }

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
        let max_abs_diff = 8.0_f32 * f32::EPSILON;

        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }

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
        let max_abs_diff = 1.0_f32 * f32::EPSILON;

        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }
}

#[cfg(test)]
mod abs_diff_eq_tuple8_tests {
    use approx_cmp::{
        assert_abs_diff_eq,
        assert_abs_diff_ne,
    };

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

        assert_abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff);
    }

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

        assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff);
    }

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
        let max_abs_diff = 9.0_f32 * f32::EPSILON;

        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }

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
        let max_abs_diff = 1.0_f32 * f32::EPSILON;

        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }
}

#[cfg(test)]
mod abs_diff_eq_tuple9_tests {
    use approx_cmp::{
        assert_abs_diff_eq,
        assert_abs_diff_ne,
    };

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

        assert_abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff);
    }

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

        assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff);
    }

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
        let max_abs_diff = 10.0_f32 * f32::EPSILON;

        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }

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
        let max_abs_diff = 1.0_f32 * f32::EPSILON;

        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }
}

#[cfg(test)]
mod abs_diff_eq_tuple10_tests {
    use approx_cmp::{
        assert_abs_diff_eq,
        assert_abs_diff_ne,
    };

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

        assert_abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff);
    }

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

        assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff);
    }

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
        let max_abs_diff = 10.0_f32 * f32::EPSILON;

        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }

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
        let max_abs_diff = 1.0_f32 * f32::EPSILON;

        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }
}

#[cfg(test)]
mod abs_diff_eq_tuple11_tests {
    use approx_cmp::{
        assert_abs_diff_eq,
        assert_abs_diff_ne,
    };

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

        assert_abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff);
    }

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

        assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff);
    }

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
        let max_abs_diff = 10.0_f32 * f32::EPSILON;

        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }

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
        let max_abs_diff = 1.0_f32 * f32::EPSILON;

        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }
}

#[cfg(test)]
mod abs_diff_eq_tuple12_tests {
    use approx_cmp::{
        assert_abs_diff_eq,
        assert_abs_diff_ne,
    };

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

        assert_abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff);
    }

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

        assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff);
    }

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
        let max_abs_diff = 10.0_f32 * f32::EPSILON;

        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }

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
        let max_abs_diff = 1.0_f32 * f32::EPSILON;

        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= max_abs_diff);
    }
}

