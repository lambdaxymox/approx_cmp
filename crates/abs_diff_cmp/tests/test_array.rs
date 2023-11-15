#[cfg(test)]
mod abs_diff_eq_array_f32_tests {
    use abs_diff_cmp::{
        abs_diff_eq,
        abs_diff_ne,
        assert_abs_diff_eq,
        assert_abs_diff_ne,
        AbsDiffAllEq,
        AbsDiffEq,
    };

    fn array_uniform<const N: usize>(value: f32) -> [f32; N] {
        [value; N]
    }

    fn array_range<const N: usize>(min_value: f32) -> [f32; N] {
        let mut array = [0.0_f32; N];
        for i in 0..N {
            array[i] = (min_value + (i as f32)) as f32;
        }

        array
    }

    fn check_eq_array<const N: usize>(value: f32) {
        let lhs = array_uniform::<N>(value);
        let rhs = array_uniform::<N>(value);
        let max_abs_diff = array_uniform::<N>(f32::EPSILON);

        assert!(lhs.abs_diff_eq(&rhs, &max_abs_diff));
        assert!(abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff));
        assert_abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff);
    }

    fn check_ne_array<const N: usize>(value: f32, min_value: f32) {
        let lhs = array_uniform::<N>(value);
        let rhs = array_range::<N>(min_value);
        let max_abs_diff = array_uniform::<N>(f32::EPSILON);

        assert!(lhs.abs_diff_ne(&rhs, &max_abs_diff));
        assert!(abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff));
        assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff);
    }

    fn check_all_eq_array<const N: usize>(value: f32) {
        let lhs = array_uniform::<N>(value);
        let rhs = array_uniform::<N>(value);

        assert!(lhs.abs_diff_all_eq(&rhs, &f32::EPSILON));
        assert!(abs_diff_eq!(lhs, rhs, abs_diff_all <= f32::EPSILON));
        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= f32::EPSILON);
    }

    fn check_all_ne_array<const N: usize>(value: f32, min_value: f32) {
        let lhs = array_uniform::<N>(value);
        let rhs = array_range::<N>(min_value);

        assert!(lhs.abs_diff_all_ne(&rhs, &f32::EPSILON));
        assert!(abs_diff_ne!(lhs, rhs, abs_diff_all <= f32::EPSILON));
        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= f32::EPSILON);
    }

    #[test]
    fn test_eq_array_empty() {
        check_eq_array::<0>(1.0_f32);
        check_all_eq_array::<0>(1.0_f32);
    }

    #[test]
    fn test_eq_array() {
        check_eq_array::<1>(1.0_f32);
        check_eq_array::<2>(1.0_f32);
        check_eq_array::<3>(1.0_f32);
        check_eq_array::<4>(1.0_f32);
        check_eq_array::<8>(1.0_f32);
        check_eq_array::<16>(1.0_f32);
        check_eq_array::<32>(1.0_f32);
        check_eq_array::<64>(1.0_f32);
    }

    #[test]
    fn test_ne_array() {
        check_ne_array::<1>(1.0_f32, 2.0_f32);
        check_ne_array::<2>(1.0_f32, 2.0_f32);
        check_ne_array::<3>(1.0_f32, 2.0_f32);
        check_ne_array::<4>(1.0_f32, 2.0_f32);
        check_ne_array::<8>(1.0_f32, 2.0_f32);
        check_ne_array::<16>(1.0_f32, 2.0_f32);
        check_ne_array::<32>(1.0_f32, 2.0_f32);
        check_ne_array::<64>(1.0_f32, 2.0_f32);
    }

    #[test]
    fn test_all_eq_array() {
        check_all_eq_array::<1>(1.0_f32);
        check_all_eq_array::<2>(1.0_f32);
        check_all_eq_array::<3>(1.0_f32);
        check_all_eq_array::<4>(1.0_f32);
        check_all_eq_array::<8>(1.0_f32);
        check_all_eq_array::<16>(1.0_f32);
        check_all_eq_array::<32>(1.0_f32);
        check_all_eq_array::<64>(1.0_f32);
    }

    #[test]
    fn test_all_ne_array() {
        check_all_ne_array::<1>(1.0_f32, 2.0_f32);
        check_all_ne_array::<2>(1.0_f32, 2.0_f32);
        check_all_ne_array::<3>(1.0_f32, 2.0_f32);
        check_all_ne_array::<4>(1.0_f32, 2.0_f32);
        check_all_ne_array::<8>(1.0_f32, 2.0_f32);
        check_all_ne_array::<16>(1.0_f32, 2.0_f32);
        check_all_ne_array::<32>(1.0_f32, 2.0_f32);
        check_all_ne_array::<64>(1.0_f32, 2.0_f32);
    }
}


#[cfg(test)]
mod abs_diff_eq_array_f64_tests {
    use abs_diff_cmp::{
        abs_diff_eq,
        abs_diff_ne,
        assert_abs_diff_eq,
        assert_abs_diff_ne,
        AbsDiffAllEq,
        AbsDiffEq,
    };

    fn array_uniform<const N: usize>(value: f64) -> [f64; N] {
        [value; N]
    }

    fn array_range<const N: usize>(min_value: f64) -> [f64; N] {
        let mut array = [0.0_f64; N];
        for i in 0..N {
            array[i] = (min_value + (i as f64)) as f64;
        }

        array
    }

    fn check_eq_array<const N: usize>(value: f64) {
        let lhs = array_uniform::<N>(value);
        let rhs = array_uniform::<N>(value);
        let max_abs_diff = array_uniform::<N>(f64::EPSILON);

        assert!(lhs.abs_diff_eq(&rhs, &max_abs_diff));
        assert!(abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff));
        assert_abs_diff_eq!(lhs, rhs, abs_diff <= max_abs_diff);
    }

    fn check_ne_array<const N: usize>(value: f64, min_value: f64) {
        let lhs = array_uniform::<N>(value);
        let rhs = array_range::<N>(min_value);
        let max_abs_diff = array_uniform::<N>(f64::EPSILON);

        assert!(lhs.abs_diff_ne(&rhs, &max_abs_diff));
        assert!(abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff));
        assert_abs_diff_ne!(lhs, rhs, abs_diff <= max_abs_diff);
    }

    fn check_all_eq_array<const N: usize>(value: f64) {
        let lhs = array_uniform::<N>(value);
        let rhs = array_uniform::<N>(value);

        assert!(lhs.abs_diff_all_eq(&rhs, &f64::EPSILON));
        assert!(abs_diff_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON));
        assert_abs_diff_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON);
    }

    fn check_all_ne_array<const N: usize>(value: f64, min_value: f64) {
        let lhs = array_uniform::<N>(value);
        let rhs = array_range::<N>(min_value);

        assert!(lhs.abs_diff_all_ne(&rhs, &f64::EPSILON));
        assert!(abs_diff_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON));
        assert_abs_diff_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON);
    }


    #[test]
    fn test_eq_array_empty() {
        check_eq_array::<0>(1.0_f64);
        check_all_eq_array::<0>(1.0_f64);
    }

    #[test]
    fn test_eq_array() {
        check_eq_array::<1>(1.0_f64);
        check_eq_array::<2>(1.0_f64);
        check_eq_array::<3>(1.0_f64);
        check_eq_array::<4>(1.0_f64);
        check_eq_array::<8>(1.0_f64);
        check_eq_array::<16>(1.0_f64);
        check_eq_array::<32>(1.0_f64);
        check_eq_array::<64>(1.0_f64);
    }

    #[test]
    fn test_ne_array() {
        check_ne_array::<1>(1.0_f64, 2.0_f64);
        check_ne_array::<2>(1.0_f64, 2.0_f64);
        check_ne_array::<3>(1.0_f64, 2.0_f64);
        check_ne_array::<4>(1.0_f64, 2.0_f64);
        check_ne_array::<8>(1.0_f64, 2.0_f64);
        check_ne_array::<16>(1.0_f64, 2.0_f64);
        check_ne_array::<32>(1.0_f64, 2.0_f64);
        check_ne_array::<64>(1.0_f64, 2.0_f64);
    }

    #[test]
    fn test_all_eq_array() {
        check_all_eq_array::<1>(1.0_f64);
        check_all_eq_array::<2>(1.0_f64);
        check_all_eq_array::<3>(1.0_f64);
        check_all_eq_array::<4>(1.0_f64);
        check_all_eq_array::<8>(1.0_f64);
        check_all_eq_array::<16>(1.0_f64);
        check_all_eq_array::<32>(1.0_f64);
        check_all_eq_array::<64>(1.0_f64);
    }

    #[test]
    fn test_all_ne_array() {
        check_all_ne_array::<1>(1.0_f64, 2.0_f64);
        check_all_ne_array::<2>(1.0_f64, 2.0_f64);
        check_all_ne_array::<3>(1.0_f64, 2.0_f64);
        check_all_ne_array::<4>(1.0_f64, 2.0_f64);
        check_all_ne_array::<8>(1.0_f64, 2.0_f64);
        check_all_ne_array::<16>(1.0_f64, 2.0_f64);
        check_all_ne_array::<32>(1.0_f64, 2.0_f64);
        check_all_ne_array::<64>(1.0_f64, 2.0_f64);
    }
}

#[cfg(test)]
mod abs_diff_eq_array_f32_debug_tests {
    use abs_diff_cmp::{
        AssertAbsDiffAllEq,
        AssertAbsDiffEq,
    };

    fn array_uniform<const N: usize>(value: f32) -> [f32; N] {
        [value; N]
    }

    fn array_range<const N: usize>(min_value: f32) -> [f32; N] {
        let mut array = [0.0_f32; N];
        for i in 0..N {
            array[i] = (min_value + (i as f32)) as f32;
        }

        array
    }


    #[test]
    fn test_debug_abs_diff1() {
        let lhs = array_uniform::<32>(1.0_f32);
        let rhs = array_uniform::<32>(1.0_f32);
        let abs_diff = [0.0_f32; 32];

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff2() {
        let lhs = array_uniform::<32>(1.0_f32);
        let rhs = array_range::<32>(2.0_f32);
        let abs_diff = array_range::<32>(1.0_f32);

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_tolerance1() {
        let lhs = array_uniform::<32>(1.0_f32);
        let rhs = array_uniform::<32>(1.0_f32);
        let max_abs_diff = 4.0_f32 * f32::EPSILON;
        let tolerance = array_uniform::<32>(max_abs_diff);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &tolerance), tolerance);
    }

    #[test]
    fn test_debug_abs_diff_tolerance2() {
        let lhs = array_uniform::<32>(1.0_f32);
        let rhs = array_range::<32>(2.0_f32);
        let max_abs_diff = 4.0_f32 * f32::EPSILON;
        let tolerance = array_uniform::<32>(max_abs_diff);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &tolerance), tolerance);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &tolerance), tolerance);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance1() {
        let lhs = array_uniform::<32>(1.0_f32);
        let rhs = array_uniform::<32>(1.0_f32);
        let max_abs_diff = 4.0_f32 * f32::EPSILON;
        let tolerance = array_uniform::<32>(max_abs_diff);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff), tolerance);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance2() {
        let lhs = array_uniform::<32>(1.0_f32);
        let rhs = array_range::<32>(2.0_f32);
        let max_abs_diff = 4.0_f32 * f32::EPSILON;
        let tolerance = array_uniform::<32>(max_abs_diff);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff), tolerance);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff), tolerance);
    }
}


#[cfg(test)]
mod abs_diff_eq_array_f64_debug_tests {
    use abs_diff_cmp::{
        AssertAbsDiffAllEq,
        AssertAbsDiffEq,
    };

    fn array_uniform<const N: usize>(value: f64) -> [f64; N] {
        [value; N]
    }

    fn array_range<const N: usize>(min_value: f64) -> [f64; N] {
        let mut array = [0.0_f64; N];
        for i in 0..N {
            array[i] = (min_value + (i as f64)) as f64;
        }

        array
    }


    #[test]
    fn test_debug_abs_diff1() {
        let lhs = array_uniform::<32>(1.0_f64);
        let rhs = array_uniform::<32>(1.0_f64);
        let abs_diff = [0.0_f64; 32];

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff2() {
        let lhs = array_uniform::<32>(1.0_f64);
        let rhs = array_range::<32>(2.0_f64);
        let abs_diff = array_range::<32>(1.0_f64);

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_tolerance1() {
        let lhs = array_uniform::<32>(1.0_f64);
        let rhs = array_uniform::<32>(1.0_f64);
        let max_abs_diff = 4.0_f64 * f64::EPSILON;
        let tolerance = array_uniform::<32>(max_abs_diff);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &tolerance), tolerance);
    }

    #[test]
    fn test_debug_abs_diff_tolerance2() {
        let lhs = array_uniform::<32>(1.0_f64);
        let rhs = array_range::<32>(2.0_f64);
        let max_abs_diff = 4.0_f64 * f64::EPSILON;
        let tolerance = array_uniform::<32>(max_abs_diff);

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &tolerance), tolerance);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &tolerance), tolerance);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance1() {
        let lhs = array_uniform::<32>(1.0_f64);
        let rhs = array_uniform::<32>(1.0_f64);
        let max_abs_diff = 4.0_f64 * f64::EPSILON;
        let tolerance = array_uniform::<32>(max_abs_diff);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff), tolerance);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance2() {
        let lhs = array_uniform::<32>(1.0_f64);
        let rhs = array_range::<32>(2.0_f64);
        let max_abs_diff = 4.0_f64 * f64::EPSILON;
        let tolerance = array_uniform::<32>(max_abs_diff);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff), tolerance);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff), tolerance);
    }
}
