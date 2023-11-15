#[cfg(test)]
mod relative_eq_array_f32_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        relative_eq,
        relative_ne,
        RelativeAllEq,
        RelativeEq,
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
mod relative_eq_array_f64_tests {
    use relative_cmp::{
        assert_relative_eq,
        assert_relative_ne,
        relative_eq,
        relative_ne,
        RelativeAllEq,
        RelativeEq,
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
mod relative_eq_array_f32_debug_tests {
    use relative_cmp::{
        AssertRelativeAllEq,
        AssertRelativeEq,
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
        let max_abs_diff_all = 4.0_f32 * f32::EPSILON;
        let max_abs_diff = array_uniform::<32>(max_abs_diff_all);
        let tolerance = max_abs_diff;

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), tolerance);
    }

    #[test]
    fn test_debug_abs_diff_tolerance2() {
        let lhs = array_uniform::<32>(1.0_f32);
        let rhs = array_range::<32>(2.0_f32);
        let max_abs_diff_all = 4.0_f32 * f32::EPSILON;
        let max_abs_diff = array_uniform::<32>(max_abs_diff_all);
        let tolerance = max_abs_diff;

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), tolerance);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), tolerance);
    }

    #[test]
    fn test_debug_relative_tolerance1() {
        let lhs = array_uniform::<32>(1.0_f32);
        let rhs = array_uniform::<32>(1.0_f32);
        let max_relative_all = 4.0_f32 * f32::EPSILON;
        let max_relative = array_uniform::<32>(max_relative_all);
        let tolerance = max_relative;

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), tolerance);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_tolerance2() {
        let lhs = array_uniform::<32>(1.0_f32);
        let rhs = array_range::<32>(2.0_f32);
        let max_relative_all = 4.0_f32 * f32::EPSILON;
        let max_relative = array_uniform::<32>(max_relative_all);
        let tolerance = [
            9.536743e-7_f32,   1.4305115e-6_f32, 1.9073486e-6_f32, 2.3841858e-6_f32,
            2.861023e-6_f32,   3.33786e-6_f32,   3.8146973e-6_f32, 4.2915344e-6_f32,
            4.7683716e-6_f32,  5.2452087e-6_f32, 5.722046e-6_f32,  6.198883e-6_f32,
            6.67572e-6_f32,    7.1525574e-6_f32, 7.6293945e-6_f32, 8.106232e-6_f32,
            8.583069e-6_f32,   9.059906e-6_f32,  9.536743e-6_f32,  1.001358e-5_f32,
            1.04904175e-5_f32, 1.0967255e-5_f32, 1.1444092e-5_f32, 1.1920929e-5_f32,
            1.2397766e-5_f32,  1.2874603e-5_f32, 1.335144e-5_f32,  1.3828278e-5_f32,
            1.4305115e-5_f32,  1.4781952e-5_f32, 1.5258789e-5_f32, 1.5735626e-5_f32,
        ];

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), tolerance);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), tolerance);
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

    #[test]
    fn test_debug_relative_all_tolerance1() {
        let lhs = array_uniform::<32>(1.0_f32);
        let rhs = array_uniform::<32>(1.0_f32);
        let max_relative = 4.0_f32 * f32::EPSILON;
        let tolerance = array_uniform::<32>(max_relative);

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative), tolerance);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_all_tolerance2() {
        let lhs = array_uniform::<32>(1.0_f32);
        let rhs = array_range::<32>(2.0_f32);
        let max_relative = 4.0_f32 * f32::EPSILON;
        let tolerance = [
            9.536743e-7_f32,   1.4305115e-6_f32, 1.9073486e-6_f32, 2.3841858e-6_f32,
            2.861023e-6_f32,   3.33786e-6_f32,   3.8146973e-6_f32, 4.2915344e-6_f32,
            4.7683716e-6_f32,  5.2452087e-6_f32, 5.722046e-6_f32,  6.198883e-6_f32,
            6.67572e-6_f32,    7.1525574e-6_f32, 7.6293945e-6_f32, 8.106232e-6_f32,
            8.583069e-6_f32,   9.059906e-6_f32,  9.536743e-6_f32,  1.001358e-5_f32,
            1.04904175e-5_f32, 1.0967255e-5_f32, 1.1444092e-5_f32, 1.1920929e-5_f32,
            1.2397766e-5_f32,  1.2874603e-5_f32, 1.335144e-5_f32,  1.3828278e-5_f32,
            1.4305115e-5_f32,  1.4781952e-5_f32, 1.5258789e-5_f32, 1.5735626e-5_f32,
        ];

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative), tolerance);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative), tolerance);
    }
}


#[cfg(test)]
mod relative_eq_array_f64_debug_tests {
    use relative_cmp::{
        AssertRelativeAllEq,
        AssertRelativeEq,
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
        let max_abs_diff_all = 4.0_f64 * f64::EPSILON;
        let max_abs_diff = array_uniform::<32>(max_abs_diff_all);
        let tolerance = max_abs_diff;

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), tolerance);
    }

    #[test]
    fn test_debug_abs_diff_tolerance2() {
        let lhs = array_uniform::<32>(1.0_f64);
        let rhs = array_range::<32>(2.0_f64);
        let max_abs_diff_all = 4.0_f64 * f64::EPSILON;
        let max_abs_diff = array_uniform::<32>(max_abs_diff_all);
        let tolerance = max_abs_diff;

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), tolerance);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), tolerance);
    }

    #[test]
    fn test_debug_relative_tolerance1() {
        let lhs = array_uniform::<32>(1.0_f64);
        let rhs = array_uniform::<32>(1.0_f64);
        let max_relative_all = 4.0_f64 * f64::EPSILON;
        let max_relative = array_uniform::<32>(max_relative_all);
        let tolerance = max_relative;

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), tolerance);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_tolerance2() {
        let lhs = array_uniform::<32>(1.0_f64);
        let rhs = array_range::<32>(2.0_f64);
        let max_relative_all = 4.0_f64 * f64::EPSILON;
        let max_relative = array_uniform::<32>(max_relative_all);
        let tolerance = [
            1.7763568394002505e-15_f64, 2.6645352591003757e-15_f64, 3.552713678800501e-15_f64,  4.440892098500626e-15_f64,
            5.329070518200751e-15_f64,  6.217248937900877e-15_f64,  7.105427357601002e-15_f64,  7.993605777301127e-15_f64,
            8.881784197001252e-15_f64,  9.769962616701378e-15_f64,  1.0658141036401503e-14_f64, 1.1546319456101628e-14_f64,
            1.2434497875801753e-14_f64, 1.3322676295501878e-14_f64, 1.4210854715202004e-14_f64, 1.509903313490213e-14_f64,
            1.5987211554602254e-14_f64, 1.687538997430238e-14_f64,  1.7763568394002505e-14_f64, 1.865174681370263e-14_f64,
            1.9539925233402755e-14_f64, 2.042810365310288e-14_f64,  2.1316282072803006e-14_f64, 2.220446049250313e-14_f64,
            2.3092638912203256e-14_f64, 2.398081733190338e-14_f64,  2.4868995751603507e-14_f64, 2.5757174171303632e-14_f64,
            2.6645352591003757e-14_f64, 2.7533531010703882e-14_f64, 2.842170943040401e-14_f64,  2.930988785010413e-14_f64,
        ];

        assert_eq!(lhs.debug_relative_tolerance(&rhs, &max_relative), tolerance);
        assert_eq!(rhs.debug_relative_tolerance(&lhs, &max_relative), tolerance);
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

    #[test]
    fn test_debug_relative_all_tolerance1() {
        let lhs = array_uniform::<32>(1.0_f64);
        let rhs = array_uniform::<32>(1.0_f64);
        let max_relative = 4.0_f64 * f64::EPSILON;
        let tolerance = array_uniform::<32>(max_relative);

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative), tolerance);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_relative_all_tolerance2() {
        let lhs = array_uniform::<32>(1.0_f64);
        let rhs = array_range::<32>(2.0_f64);
        let max_relative = 4.0_f64 * f64::EPSILON;
        let tolerance = [
            1.7763568394002505e-15_f64, 2.6645352591003757e-15_f64, 3.552713678800501e-15_f64,  4.440892098500626e-15_f64,
            5.329070518200751e-15_f64,  6.217248937900877e-15_f64,  7.105427357601002e-15_f64,  7.993605777301127e-15_f64,
            8.881784197001252e-15_f64,  9.769962616701378e-15_f64,  1.0658141036401503e-14_f64, 1.1546319456101628e-14_f64,
            1.2434497875801753e-14_f64, 1.3322676295501878e-14_f64, 1.4210854715202004e-14_f64, 1.509903313490213e-14_f64,
            1.5987211554602254e-14_f64, 1.687538997430238e-14_f64,  1.7763568394002505e-14_f64, 1.865174681370263e-14_f64,
            1.9539925233402755e-14_f64, 2.042810365310288e-14_f64,  2.1316282072803006e-14_f64, 2.220446049250313e-14_f64,
            2.3092638912203256e-14_f64, 2.398081733190338e-14_f64,  2.4868995751603507e-14_f64, 2.5757174171303632e-14_f64,
            2.6645352591003757e-14_f64, 2.7533531010703882e-14_f64, 2.842170943040401e-14_f64,  2.930988785010413e-14_f64,
        ];

        assert_eq!(lhs.debug_relative_all_tolerance(&rhs, &max_relative), tolerance);
        assert_eq!(rhs.debug_relative_all_tolerance(&lhs, &max_relative), tolerance);
    }
}
