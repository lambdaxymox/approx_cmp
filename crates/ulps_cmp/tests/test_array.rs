extern crate ulps_cmp;


#[cfg(test)]
mod ulps_eq_array_f32_tests {
    use ulps_cmp::{
        assert_ulps_eq,
        assert_ulps_ne,
        ulps_eq,
        ulps_ne,
        UlpsAllEq,
        UlpsEq,
    };

    fn array_uniform<T, const N: usize>(value: T) -> [T; N]
    where
        T: Copy,
    {
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
        let lhs = array_uniform::<f32, N>(value);
        let rhs = array_uniform::<f32, N>(value);
        let max_abs_diff = array_uniform::<f32, N>(f32::EPSILON);
        let max_ulps = array_uniform(4_u32);

        assert!(lhs.ulps_eq(&rhs, &max_abs_diff, &max_ulps));
        assert!(ulps_eq!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps));
        assert_ulps_eq!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    }

    fn check_ne_array<const N: usize>(value: f32, min_value: f32) {
        let lhs = array_uniform::<f32, N>(value);
        let rhs = array_range::<N>(min_value);
        let max_abs_diff = array_uniform::<f32, N>(f32::EPSILON);
        let max_ulps = array_uniform(4_u32);

        assert!(lhs.ulps_ne(&rhs, &max_abs_diff, &max_ulps));
        assert!(ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps));
        assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    }

    fn check_all_eq_array<const N: usize>(value: f32) {
        let lhs = array_uniform::<f32, N>(value);
        let rhs = array_uniform::<f32, N>(value);
        let max_ulps = 4_u32;

        assert!(lhs.ulps_all_eq(&rhs, &f32::EPSILON, &max_ulps));
        assert!(ulps_eq!(lhs, rhs, abs_diff_all <= f32::EPSILON, ulps_all <= max_ulps));
        assert_ulps_eq!(lhs, rhs, abs_diff_all <= f32::EPSILON, ulps_all <= max_ulps);
    }

    fn check_all_ne_array<const N: usize>(value: f32, min_value: f32) {
        let lhs = array_uniform::<f32, N>(value);
        let rhs = array_range::<N>(min_value);
        let max_ulps = 4_u32;

        assert!(lhs.ulps_all_ne(&rhs, &f32::EPSILON, &max_ulps));
        assert!(ulps_ne!(lhs, rhs, abs_diff_all <= f32::EPSILON, ulps_all <= max_ulps));
        assert_ulps_ne!(lhs, rhs, abs_diff_all <= f32::EPSILON, ulps_all <= max_ulps);
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
mod ulps_eq_array_f64_tests {
    use ulps_cmp::{
        assert_ulps_eq,
        assert_ulps_ne,
        ulps_eq,
        ulps_ne,
        UlpsAllEq,
        UlpsEq,
    };

    fn array_uniform<T, const N: usize>(value: T) -> [T; N]
    where
        T: Copy,
    {
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
        let lhs = array_uniform::<f64, N>(value);
        let rhs = array_uniform::<f64, N>(value);
        let max_abs_diff = array_uniform::<f64, N>(f64::EPSILON);
        let max_ulps = array_uniform(4_u64);

        assert!(lhs.ulps_eq(&rhs, &max_abs_diff, &max_ulps));
        assert!(ulps_eq!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps));
        assert_ulps_eq!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    }

    fn check_ne_array<const N: usize>(value: f64, min_value: f64) {
        let lhs = array_uniform::<f64, N>(value);
        let rhs = array_range(min_value);
        let max_abs_diff = array_uniform(f64::EPSILON);
        let max_ulps = array_uniform(4_u64);

        assert!(lhs.ulps_ne(&rhs, &max_abs_diff, &max_ulps));
        assert!(ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps));
        assert_ulps_ne!(lhs, rhs, abs_diff <= max_abs_diff, ulps <= max_ulps);
    }

    fn check_all_eq_array<const N: usize>(value: f64) {
        let lhs = array_uniform::<f64, N>(value);
        let rhs = array_uniform::<f64, N>(value);
        let max_ulps = 4_u64;

        assert!(lhs.ulps_all_eq(&rhs, &f64::EPSILON, &max_ulps));
        assert!(ulps_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON, ulps_all <= max_ulps));
        assert_ulps_eq!(lhs, rhs, abs_diff_all <= f64::EPSILON, ulps_all <= max_ulps);
    }

    fn check_all_ne_array<const N: usize>(value: f64, min_value: f64) {
        let lhs = array_uniform::<f64, N>(value);
        let rhs = array_range::<N>(min_value);
        let max_ulps = 4_u64;

        assert!(lhs.ulps_all_ne(&rhs, &f64::EPSILON, &max_ulps));
        assert!(ulps_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON, ulps_all <= max_ulps));
        assert_ulps_ne!(lhs, rhs, abs_diff_all <= f64::EPSILON, ulps_all <= max_ulps);
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
mod ulps_eq_array_f32_debug_tests {
    use ulps_cmp::{
        AssertUlpsAllEq,
        AssertUlpsEq,
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
    fn test_debug_ulps_diff1() {
        let lhs = array_uniform::<32>(1.0_f32);
        let rhs = array_uniform::<32>(1.0_f32);
        let ulps_diff = [Some(0_u32); 32];

        assert_eq!(lhs.debug_ulps_diff(&rhs), ulps_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_ulps_diff2() {
        let lhs = array_uniform::<32>(1.0_f32);
        let rhs = array_range::<32>(2.0_f32);
        let ulps_diff: [Option<u32>; 32] = [
            Some(8388608_u32),  Some(12582912_u32), Some(16777216_u32), Some(18874368_u32),
            Some(20971520_u32), Some(23068672_u32), Some(25165824_u32), Some(26214400_u32),
            Some(27262976_u32), Some(28311552_u32), Some(29360128_u32), Some(30408704_u32),
            Some(31457280_u32), Some(32505856_u32), Some(33554432_u32), Some(34078720_u32),
            Some(34603008_u32), Some(35127296_u32), Some(35651584_u32), Some(36175872_u32),
            Some(36700160_u32), Some(37224448_u32), Some(37748736_u32), Some(38273024_u32),
            Some(38797312_u32), Some(39321600_u32), Some(39845888_u32), Some(40370176_u32),
            Some(40894464_u32), Some(41418752_u32), Some(41943040_u32), Some(42205184_u32),
        ];

        assert_eq!(lhs.debug_ulps_diff(&rhs), ulps_diff);
        assert_eq!(rhs.debug_ulps_diff(&lhs), ulps_diff);
    }

    #[test]
    fn test_debug_ulps_tolerance1() {
        let lhs = array_uniform::<32>(1.0_f32);
        let rhs = array_uniform::<32>(1.0_f32);
        let max_ulps_all = 4_u32;
        let max_ulps = [max_ulps_all; 32];
        let tolerance = max_ulps;

        assert_eq!(lhs.debug_ulps_tolerance(&rhs, &max_ulps), tolerance);
    }

    #[test]
    fn test_debug_ulps_tolerance2() {
        let lhs = array_uniform::<32>(1.0_f32);
        let rhs = array_range::<32>(2.0_f32);
        let max_ulps_all = 4_u32;
        let max_ulps = [max_ulps_all; 32];
        let tolerance = [max_ulps_all; 32];

        assert_eq!(lhs.debug_ulps_tolerance(&rhs, &max_ulps), tolerance);
        assert_eq!(rhs.debug_ulps_tolerance(&lhs, &max_ulps), tolerance);
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
        let tolerance = 4.0_f32 * f32::EPSILON;
        let max_abs_diff = array_uniform::<32>(tolerance);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &tolerance), max_abs_diff);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &tolerance), max_abs_diff);
    }

    #[test]
    fn test_debug_ulps_all_tolerance1() {
        let lhs = array_uniform::<32>(1.0_f32);
        let rhs = array_uniform::<32>(1.0_f32);
        let max_ulps = 16_u32;
        let tolerance = [max_ulps; 32];

        assert_eq!(lhs.debug_ulps_all_tolerance(&rhs, &max_ulps), tolerance);
    }

    #[test]
    fn test_debug_ulps_all_tolerance2() {
        let lhs = array_uniform::<32>(1.0_f32);
        let rhs = array_range::<32>(2.0_f32);
        let max_ulps = 16_u32;
        let tolerance = [max_ulps; 32];

        assert_eq!(lhs.debug_ulps_all_tolerance(&rhs, &max_ulps), tolerance);
        assert_eq!(rhs.debug_ulps_all_tolerance(&lhs, &max_ulps), tolerance);
    }
}


#[cfg(test)]
mod ulps_eq_array_f64_debug_tests {
    use ulps_cmp::{
        AssertUlpsAllEq,
        AssertUlpsEq,
    };

    fn array_uniform<T, const N: usize>(value: T) -> [T; N]
    where
        T: Copy,
    {
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
        let lhs = array_uniform(1.0_f64);
        let rhs = array_uniform(1.0_f64);
        let abs_diff = array_uniform::<f64, 32>(0.0_f64);

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff2() {
        let lhs = array_uniform(1.0_f64);
        let rhs = array_range(2.0_f64);
        let abs_diff = array_range::<32>(1.0_f64);

        assert_eq!(lhs.debug_abs_diff(&rhs), abs_diff);
        assert_eq!(rhs.debug_abs_diff(&lhs), abs_diff);
    }

    #[test]
    fn test_debug_abs_diff_tolerance1() {
        let lhs = array_uniform::<f64, 32>(1.0_f64);
        let rhs = array_uniform::<f64, 32>(1.0_f64);
        let max_abs_diff_all = 4.0_f64 * f64::EPSILON;
        let max_abs_diff = array_uniform::<f64, 32>(max_abs_diff_all);
        let tolerance = max_abs_diff;

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), tolerance);
    }

    #[test]
    fn test_debug_abs_diff_tolerance2() {
        let lhs = array_uniform::<f64, 32>(1.0_f64);
        let rhs = array_range::<32>(2.0_f64);
        let max_abs_diff_all = 4.0_f64 * f64::EPSILON;
        let max_abs_diff = array_uniform::<f64, 32>(max_abs_diff_all);
        let tolerance = max_abs_diff;

        assert_eq!(lhs.debug_abs_diff_tolerance(&rhs, &max_abs_diff), tolerance);
        assert_eq!(rhs.debug_abs_diff_tolerance(&lhs, &max_abs_diff), tolerance);
    }

    #[test]
    fn test_debug_ulps_diff1() {
        let lhs = array_uniform(1.0_f64);
        let rhs = array_uniform(1.0_f64);
        let ulps_diff = [Some(0_u64); 32];

        assert_eq!(lhs.debug_ulps_diff(&rhs), ulps_diff);
    }

    #[rustfmt::skip]
    #[test]
    fn test_debug_ulps_diff2() {
        let lhs = array_uniform(1.0_f64);
        let rhs = array_range(2.0_f64);
        let ulps_diff = [
            Some(4503599627370496_u64),  Some(6755399441055744_u64),  Some(9007199254740992_u64),  Some(10133099161583616_u64),
            Some(11258999068426240_u64), Some(12384898975268864_u64), Some(13510798882111488_u64), Some(14073748835532800_u64),
            Some(14636698788954112_u64), Some(15199648742375424_u64), Some(15762598695796736_u64), Some(16325548649218048_u64),
            Some(16888498602639360_u64), Some(17451448556060672_u64), Some(18014398509481984_u64), Some(18295873486192640_u64),
            Some(18577348462903296_u64), Some(18858823439613952_u64), Some(19140298416324608_u64), Some(19421773393035264_u64),
            Some(19703248369745920_u64), Some(19984723346456576_u64), Some(20266198323167232_u64), Some(20547673299877888_u64),
            Some(20829148276588544_u64), Some(21110623253299200_u64), Some(21392098230009856_u64), Some(21673573206720512_u64),
            Some(21955048183431168_u64), Some(22236523160141824_u64), Some(22517998136852480_u64), Some(22658735625207808_u64),
        ];

        assert_eq!(lhs.debug_ulps_diff(&rhs), ulps_diff);
        assert_eq!(rhs.debug_ulps_diff(&lhs), ulps_diff);
    }

    #[test]
    fn test_debug_ulps_tolerance1() {
        let lhs = array_uniform::<f64, 32>(1.0_f64);
        let rhs = array_uniform::<f64, 32>(1.0_f64);
        let max_ulps = array_uniform::<u64, 32>(4_u64);
        let tolerance = max_ulps;

        assert_eq!(lhs.debug_ulps_tolerance(&rhs, &max_ulps), tolerance);
    }

    #[test]
    fn test_debug_ulps_tolerance2() {
        let lhs = array_uniform::<f64, 32>(1.0_f64);
        let rhs = array_range::<32>(2.0_f64);
        let max_ulps = array_uniform::<u64, 32>(4_u64);
        let tolerance = max_ulps;

        assert_eq!(lhs.debug_ulps_tolerance(&rhs, &max_ulps), tolerance);
        assert_eq!(rhs.debug_ulps_tolerance(&lhs, &max_ulps), tolerance);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance1() {
        let lhs = array_uniform::<f64, 32>(1.0_f64);
        let rhs = array_uniform::<f64, 32>(1.0_f64);
        let max_abs_diff = 4.0_f64 * f64::EPSILON;
        let tolerance = array_uniform::<f64, 32>(max_abs_diff);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff), tolerance);
    }

    #[test]
    fn test_debug_abs_diff_all_tolerance2() {
        let lhs = array_uniform::<f64, 32>(1.0_f64);
        let rhs = array_range::<32>(2.0_f64);
        let max_abs_diff = 4.0_f64 * f64::EPSILON;
        let tolerance = array_uniform::<f64, 32>(max_abs_diff);

        assert_eq!(lhs.debug_abs_diff_all_tolerance(&rhs, &max_abs_diff), tolerance);
        assert_eq!(rhs.debug_abs_diff_all_tolerance(&lhs, &max_abs_diff), tolerance);
    }

    #[test]
    fn test_debug_ulps_all_tolerance1() {
        let lhs = array_uniform::<f64, 32>(1.0_f64);
        let rhs = array_uniform::<f64, 32>(1.0_f64);
        let max_ulps = 16_u64;
        let tolerance = array_uniform::<u64, 32>(max_ulps);

        assert_eq!(lhs.debug_ulps_all_tolerance(&rhs, &max_ulps), tolerance);
    }

    #[test]
    fn test_debug_ulps_all_tolerance2() {
        let lhs = array_uniform::<f64, 32>(1.0_f64);
        let rhs = array_range::<32>(2.0_f64);
        let max_ulps = 16_u64;
        let tolerance = array_uniform::<u64, 32>(max_ulps);

        assert_eq!(lhs.debug_ulps_all_tolerance(&rhs, &max_ulps), tolerance);
        assert_eq!(rhs.debug_ulps_all_tolerance(&lhs, &max_ulps), tolerance);
    }
}
