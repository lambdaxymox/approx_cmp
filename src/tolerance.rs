pub trait ApproxCompareTolerance {
    fn default_tolerance() -> Self;
}

pub trait UlpsCompareTolerance {
    fn default_ulps_tolerance() -> Self;
}

macro_rules! approx_compare_tolerance {
    ($(($T:ident, $default_tolerance:expr)),* $(,)?) => {$(
        impl ApproxCompareTolerance for $T {
            #[inline]
            fn default_tolerance() -> Self {
                $default_tolerance
            }
        }
    )*};
}

approx_compare_tolerance!(
    (u8, 0),
    (u16, 0),
    (u32, 0),
    (u64, 0),
    (u128, 0),
    (usize, 0),
    (i8, 0),
    (i16, 0),
    (i32, 0),
    (i64, 0),
    (i128, 0),
    (isize, 0),
    (f32, f32::EPSILON),
    (f64, f64::EPSILON),
);

macro_rules! ulps_compare_tolerance {
    ($(($T:ident, $default_ulps_tolerance:expr)),* $(,)?) => {$(
        impl UlpsCompareTolerance for $T {
            #[inline]
            fn default_ulps_tolerance() -> Self {
                $default_ulps_tolerance
            }
        }
    )*};
}

ulps_compare_tolerance!(
    (u32, 4),
    (u64, 4),
);

