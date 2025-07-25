use crate::{Array, ArraySize};

pub trait IntoArraySize<T>: generic_array::ArrayLength<T> {
    type ArraySize: ArraySize;
}

pub trait IntoGenericArrayLength: ArraySize {
    type ArrayLength<T>: generic_array::ArrayLength<T>;
}


impl<T, U> From<generic_array::GenericArray<T, U>> for Array<T, <U as IntoArraySize<T>>::ArraySize>
where
    U: generic_array::ArrayLength<T> + IntoArraySize<T>
{
    fn from(value: generic_array::GenericArray<T, U>) -> Self {
        value.into()
    }
}

impl<'r, T, U> From<&'r generic_array::GenericArray<T, U>> for &'r Array<T, <U as IntoArraySize<T>>::ArraySize>
where
    U: generic_array::ArrayLength<T> + IntoArraySize<T>
{
    fn from(value: &'r generic_array::GenericArray<T, U>) -> Self {
        value.into()
    }
}

impl<'r, T, U> From<&'r mut generic_array::GenericArray<T, U>> for &'r mut Array<T, <U as IntoArraySize<T>>::ArraySize>
where
    U: generic_array::ArrayLength<T> + IntoArraySize<T>
{
    fn from(value: &'r mut generic_array::GenericArray<T, U>) -> Self {
        value.into()
    }
}

impl<T, U> From<Array<T, U>> for generic_array::GenericArray<T, <U as IntoGenericArrayLength>::ArrayLength::<T>>
where
    U: ArraySize + IntoGenericArrayLength
{
    fn from(value: Array<T, U>) -> Self {
        value.into()
    }
}

impl<'r, T, U> From<&'r Array<T, U>> for &'r generic_array::GenericArray<T, <U as IntoGenericArrayLength>::ArrayLength::<T>>
where
    U: ArraySize + IntoGenericArrayLength
{
    fn from(value: &'r Array<T, U>) -> Self {
        value.into()
    }
}

impl<'r, T, U> From<&'r mut Array<T, U>> for &'r mut generic_array::GenericArray<T, <U as IntoGenericArrayLength>::ArrayLength::<T>>
where
    U: ArraySize + IntoGenericArrayLength
{
    fn from(value: &'r mut Array<T, U>) -> Self {
        value.into()
    }
}



macro_rules! impl_consts {
    ($($len:expr => $ty:ident),+ $(,)?) => {
        $(
            impl IntoGenericArrayLength for $ty {
                type ArrayLength<T> = $ty;
            }

            impl<T> FromGenericArrayLength<T> for $ty {
                type ArraySize = $ty;
            }
        )+
     };
}

macro_rules! impl_consts_with_import {
    ($($len:expr => $ty:ident),+ $(,)?) => {
        $(
            pub use typenum::consts::$ty;
            impl_consts!($len => $ty);
        )+
     };
}

impl_consts_with_import! {
    1  => U1,
    2  => U2,
    3  => U3,
    4  => U4,
    5  => U5,
    6  => U6,
    7  => U7,
    8  => U8,
    9  => U9,
    10 => U10,
    11 => U11,
    12 => U12,
    13 => U13,
    14 => U14,
    15 => U15,
    16 => U16,
    17 => U17,
    18 => U18,
    19 => U19,
    20 => U20,
    21 => U21,
    22 => U22,
    23 => U23,
    24 => U24,
    25 => U25,
    26 => U26,
    27 => U27,
    28 => U28,
    29 => U29,
    30 => U30,
    31 => U31,
    32 => U32,

    33 => U33,
    34 => U34,
    35 => U35,
    36 => U36,
    37 => U37,
    38 => U38,
    39 => U39,
    40 => U40,
    41 => U41,
    42 => U42,
    43 => U43,
    44 => U44,
    45 => U45,
    46 => U46,
    47 => U47,
    48 => U48,
    49 => U49,
    50 => U50,
    51 => U51,
    52 => U52,
    53 => U53,
    54 => U54,
    55 => U55,
    56 => U56,
    57 => U57,
    58 => U58,
    59 => U59,
    60 => U60,
    61 => U61,
    62 => U62,
    63 => U63,
    64 => U64,

    70 => U70,
    80 => U80,
    90 => U90,

    100 => U100,
    200 => U200,
    300 => U300,
    400 => U400,
    500 => U500,

    128 => U128,
    256 => U256,
    512 => U512,

    // 1000 => U1000,
    1024 => U1024
}


#[cfg(feature = "extra-sizes")]
#[allow(missing_docs)]
mod extra_sizes {
    use super::{IntoGenericArrayLength, IntoArraySize};
    use typenum::{
        UInt, UTerm,
        consts::{B0, B1},
    };

    // This macro constructs a UInt type from a sequence of bits.  The bits are interpreted as the
    // little-endian representation of the integer in question.  For example, uint!(1 1 0 1 0 0 1) is
    // U75 (not U105).
    macro_rules! uint {
        () => { UTerm };
        (0 $($bs:tt)*) => { UInt< uint!($($bs)*), B0 > };
        (1 $($bs:tt)*) => { UInt< uint!($($bs)*), B1 > };
    }

    pub type U1040 = uint!(0 0 0 0 1 0 0 0 0 0 1);
    pub type U1056 = uint!(0 0 0 0 0 1 0 0 0 0 1);
    pub type U1072 = uint!(0 0 0 0 1 1 0 0 0 0 1);
    pub type U1088 = uint!(0 0 0 0 0 0 1 0 0 0 1);
    pub type U1104 = uint!(0 0 0 0 1 0 1 0 0 0 1);
    pub type U1120 = uint!(0 0 0 0 0 1 1 0 0 0 1);
    pub type U1136 = uint!(0 0 0 0 1 1 1 0 0 0 1);
    pub type U1152 = uint!(0 0 0 0 0 0 0 1 0 0 1);
    pub type U1168 = uint!(0 0 0 0 1 0 0 1 0 0 1);
    pub type U1184 = uint!(0 0 0 0 0 1 0 1 0 0 1);
    pub type U1200 = uint!(0 0 0 0 1 1 0 1 0 0 1);
    pub type U1216 = uint!(0 0 0 0 0 0 1 1 0 0 1);
    pub type U1232 = uint!(0 0 0 0 1 0 1 1 0 0 1);
    pub type U1248 = uint!(0 0 0 0 0 1 1 1 0 0 1);
    pub type U1264 = uint!(0 0 0 0 1 1 1 1 0 0 1);
    pub type U1280 = uint!(0 0 0 0 0 0 0 0 1 0 1);
    pub type U1296 = uint!(0 0 0 0 1 0 0 0 1 0 1);
    pub type U1312 = uint!(0 0 0 0 0 1 0 0 1 0 1);
    pub type U1328 = uint!(0 0 0 0 1 1 0 0 1 0 1);
    pub type U1344 = uint!(0 0 0 0 0 0 1 0 1 0 1);
    pub type U1360 = uint!(0 0 0 0 1 0 1 0 1 0 1);
    pub type U1376 = uint!(0 0 0 0 0 1 1 0 1 0 1);
    pub type U1392 = uint!(0 0 0 0 1 1 1 0 1 0 1);
    pub type U1408 = uint!(0 0 0 0 0 0 0 1 1 0 1);
    pub type U1424 = uint!(0 0 0 0 1 0 0 1 1 0 1);
    pub type U1440 = uint!(0 0 0 0 0 1 0 1 1 0 1);
    pub type U1456 = uint!(0 0 0 0 1 1 0 1 1 0 1);
    pub type U1472 = uint!(0 0 0 0 0 0 1 1 1 0 1);
    pub type U1488 = uint!(0 0 0 0 1 0 1 1 1 0 1);
    pub type U1504 = uint!(0 0 0 0 0 1 1 1 1 0 1);
    pub type U1520 = uint!(0 0 0 0 1 1 1 1 1 0 1);
    pub type U1536 = uint!(0 0 0 0 0 0 0 0 0 1 1);
    pub type U1552 = uint!(0 0 0 0 1 0 0 0 0 1 1);
    pub type U1568 = uint!(0 0 0 0 0 1 0 0 0 1 1);
    pub type U1584 = uint!(0 0 0 0 1 1 0 0 0 1 1);
    pub type U1600 = uint!(0 0 0 0 0 0 1 0 0 1 1);
    pub type U1616 = uint!(0 0 0 0 1 0 1 0 0 1 1);
    pub type U1632 = uint!(0 0 0 0 0 1 1 0 0 1 1);
    pub type U1648 = uint!(0 0 0 0 1 1 1 0 0 1 1);
    pub type U1664 = uint!(0 0 0 0 0 0 0 1 0 1 1);
    pub type U1680 = uint!(0 0 0 0 1 0 0 1 0 1 1);
    pub type U1696 = uint!(0 0 0 0 0 1 0 1 0 1 1);
    pub type U1712 = uint!(0 0 0 0 1 1 0 1 0 1 1);
    pub type U1728 = uint!(0 0 0 0 0 0 1 1 0 1 1);
    pub type U1744 = uint!(0 0 0 0 1 0 1 1 0 1 1);
    pub type U1760 = uint!(0 0 0 0 0 1 1 1 0 1 1);
    pub type U1776 = uint!(0 0 0 0 1 1 1 1 0 1 1);
    pub type U1792 = uint!(0 0 0 0 0 0 0 0 1 1 1);
    pub type U1808 = uint!(0 0 0 0 1 0 0 0 1 1 1);
    pub type U1824 = uint!(0 0 0 0 0 1 0 0 1 1 1);
    pub type U1840 = uint!(0 0 0 0 1 1 0 0 1 1 1);
    pub type U1856 = uint!(0 0 0 0 0 0 1 0 1 1 1);
    pub type U1872 = uint!(0 0 0 0 1 0 1 0 1 1 1);
    pub type U1888 = uint!(0 0 0 0 0 1 1 0 1 1 1);
    pub type U1904 = uint!(0 0 0 0 1 1 1 0 1 1 1);
    pub type U1920 = uint!(0 0 0 0 0 0 0 1 1 1 1);
    pub type U1936 = uint!(0 0 0 0 1 0 0 1 1 1 1);
    pub type U1952 = uint!(0 0 0 0 0 1 0 1 1 1 1);
    pub type U1968 = uint!(0 0 0 0 1 1 0 1 1 1 1);
    pub type U1984 = uint!(0 0 0 0 0 0 1 1 1 1 1);
    pub type U2000 = uint!(0 0 0 0 1 0 1 1 1 1 1);
    pub type U2016 = uint!(0 0 0 0 0 1 1 1 1 1 1);
    pub type U2032 = uint!(0 0 0 0 1 1 1 1 1 1 1);
    pub type U2064 = uint!(0 0 0 0 1 0 0 0 0 0 0 1);
    pub type U2080 = uint!(0 0 0 0 0 1 0 0 0 0 0 1);
    pub type U2096 = uint!(0 0 0 0 1 1 0 0 0 0 0 1);
    pub type U2112 = uint!(0 0 0 0 0 0 1 0 0 0 0 1);
    pub type U2128 = uint!(0 0 0 0 1 0 1 0 0 0 0 1);
    pub type U2144 = uint!(0 0 0 0 0 1 1 0 0 0 0 1);
    pub type U2160 = uint!(0 0 0 0 1 1 1 0 0 0 0 1);
    pub type U2176 = uint!(0 0 0 0 0 0 0 1 0 0 0 1);
    pub type U2192 = uint!(0 0 0 0 1 0 0 1 0 0 0 1);
    pub type U2208 = uint!(0 0 0 0 0 1 0 1 0 0 0 1);
    pub type U2224 = uint!(0 0 0 0 1 1 0 1 0 0 0 1);
    pub type U2240 = uint!(0 0 0 0 0 0 1 1 0 0 0 1);
    pub type U2256 = uint!(0 0 0 0 1 0 1 1 0 0 0 1);
    pub type U2272 = uint!(0 0 0 0 0 1 1 1 0 0 0 1);
    pub type U2288 = uint!(0 0 0 0 1 1 1 1 0 0 0 1);
    pub type U2304 = uint!(0 0 0 0 0 0 0 0 1 0 0 1);
    pub type U2320 = uint!(0 0 0 0 1 0 0 0 1 0 0 1);
    pub type U2336 = uint!(0 0 0 0 0 1 0 0 1 0 0 1);
    pub type U2352 = uint!(0 0 0 0 1 1 0 0 1 0 0 1);
    pub type U2368 = uint!(0 0 0 0 0 0 1 0 1 0 0 1);
    pub type U2384 = uint!(0 0 0 0 1 0 1 0 1 0 0 1);
    pub type U2400 = uint!(0 0 0 0 0 1 1 0 1 0 0 1);
    pub type U2416 = uint!(0 0 0 0 1 1 1 0 1 0 0 1);
    pub type U2432 = uint!(0 0 0 0 0 0 0 1 1 0 0 1);
    pub type U2448 = uint!(0 0 0 0 1 0 0 1 1 0 0 1);
    pub type U2464 = uint!(0 0 0 0 0 1 0 1 1 0 0 1);
    pub type U2480 = uint!(0 0 0 0 1 1 0 1 1 0 0 1);
    pub type U2496 = uint!(0 0 0 0 0 0 1 1 1 0 0 1);
    pub type U2512 = uint!(0 0 0 0 1 0 1 1 1 0 0 1);
    pub type U2528 = uint!(0 0 0 0 0 1 1 1 1 0 0 1);
    pub type U2544 = uint!(0 0 0 0 1 1 1 1 1 0 0 1);
    pub type U2560 = uint!(0 0 0 0 0 0 0 0 0 1 0 1);
    pub type U2576 = uint!(0 0 0 0 1 0 0 0 0 1 0 1);
    pub type U2592 = uint!(0 0 0 0 0 1 0 0 0 1 0 1);
    pub type U2608 = uint!(0 0 0 0 1 1 0 0 0 1 0 1);
    pub type U2624 = uint!(0 0 0 0 0 0 1 0 0 1 0 1);
    pub type U2640 = uint!(0 0 0 0 1 0 1 0 0 1 0 1);
    pub type U2656 = uint!(0 0 0 0 0 1 1 0 0 1 0 1);
    pub type U2672 = uint!(0 0 0 0 1 1 1 0 0 1 0 1);
    pub type U2688 = uint!(0 0 0 0 0 0 0 1 0 1 0 1);
    pub type U2704 = uint!(0 0 0 0 1 0 0 1 0 1 0 1);
    pub type U2720 = uint!(0 0 0 0 0 1 0 1 0 1 0 1);
    pub type U2736 = uint!(0 0 0 0 1 1 0 1 0 1 0 1);
    pub type U2752 = uint!(0 0 0 0 0 0 1 1 0 1 0 1);
    pub type U2768 = uint!(0 0 0 0 1 0 1 1 0 1 0 1);
    pub type U2784 = uint!(0 0 0 0 0 1 1 1 0 1 0 1);
    pub type U2800 = uint!(0 0 0 0 1 1 1 1 0 1 0 1);
    pub type U2816 = uint!(0 0 0 0 0 0 0 0 1 1 0 1);
    pub type U2832 = uint!(0 0 0 0 1 0 0 0 1 1 0 1);
    pub type U2848 = uint!(0 0 0 0 0 1 0 0 1 1 0 1);
    pub type U2864 = uint!(0 0 0 0 1 1 0 0 1 1 0 1);
    pub type U2880 = uint!(0 0 0 0 0 0 1 0 1 1 0 1);
    pub type U2896 = uint!(0 0 0 0 1 0 1 0 1 1 0 1);
    pub type U2912 = uint!(0 0 0 0 0 1 1 0 1 1 0 1);
    pub type U2928 = uint!(0 0 0 0 1 1 1 0 1 1 0 1);
    pub type U2944 = uint!(0 0 0 0 0 0 0 1 1 1 0 1);
    pub type U2960 = uint!(0 0 0 0 1 0 0 1 1 1 0 1);
    pub type U2976 = uint!(0 0 0 0 0 1 0 1 1 1 0 1);
    pub type U2992 = uint!(0 0 0 0 1 1 0 1 1 1 0 1);
    pub type U3008 = uint!(0 0 0 0 0 0 1 1 1 1 0 1);
    pub type U3024 = uint!(0 0 0 0 1 0 1 1 1 1 0 1);
    pub type U3040 = uint!(0 0 0 0 0 1 1 1 1 1 0 1);
    pub type U3056 = uint!(0 0 0 0 1 1 1 1 1 1 0 1);
    pub type U3072 = uint!(0 0 0 0 0 0 0 0 0 0 1 1);
    pub type U3088 = uint!(0 0 0 0 1 0 0 0 0 0 1 1);
    pub type U3104 = uint!(0 0 0 0 0 1 0 0 0 0 1 1);
    pub type U3120 = uint!(0 0 0 0 1 1 0 0 0 0 1 1);
    pub type U3136 = uint!(0 0 0 0 0 0 1 0 0 0 1 1);
    pub type U3152 = uint!(0 0 0 0 1 0 1 0 0 0 1 1);
    pub type U3168 = uint!(0 0 0 0 0 1 1 0 0 0 1 1);
    pub type U3184 = uint!(0 0 0 0 1 1 1 0 0 0 1 1);
    pub type U3200 = uint!(0 0 0 0 0 0 0 1 0 0 1 1);
    pub type U3216 = uint!(0 0 0 0 1 0 0 1 0 0 1 1);
    pub type U3232 = uint!(0 0 0 0 0 1 0 1 0 0 1 1);
    pub type U3248 = uint!(0 0 0 0 1 1 0 1 0 0 1 1);
    pub type U3264 = uint!(0 0 0 0 0 0 1 1 0 0 1 1);
    pub type U3280 = uint!(0 0 0 0 1 0 1 1 0 0 1 1);
    pub type U3296 = uint!(0 0 0 0 0 1 1 1 0 0 1 1);
    pub type U3312 = uint!(0 0 0 0 1 1 1 1 0 0 1 1);
    pub type U3328 = uint!(0 0 0 0 0 0 0 0 1 0 1 1);
    pub type U3344 = uint!(0 0 0 0 1 0 0 0 1 0 1 1);
    pub type U3360 = uint!(0 0 0 0 0 1 0 0 1 0 1 1);
    pub type U3376 = uint!(0 0 0 0 1 1 0 0 1 0 1 1);
    pub type U3392 = uint!(0 0 0 0 0 0 1 0 1 0 1 1);
    pub type U3408 = uint!(0 0 0 0 1 0 1 0 1 0 1 1);
    pub type U3424 = uint!(0 0 0 0 0 1 1 0 1 0 1 1);
    pub type U3440 = uint!(0 0 0 0 1 1 1 0 1 0 1 1);
    pub type U3456 = uint!(0 0 0 0 0 0 0 1 1 0 1 1);
    pub type U3472 = uint!(0 0 0 0 1 0 0 1 1 0 1 1);
    pub type U3488 = uint!(0 0 0 0 0 1 0 1 1 0 1 1);
    pub type U3504 = uint!(0 0 0 0 1 1 0 1 1 0 1 1);
    pub type U3520 = uint!(0 0 0 0 0 0 1 1 1 0 1 1);
    pub type U3536 = uint!(0 0 0 0 1 0 1 1 1 0 1 1);
    pub type U3552 = uint!(0 0 0 0 0 1 1 1 1 0 1 1);
    pub type U3568 = uint!(0 0 0 0 1 1 1 1 1 0 1 1);
    pub type U3584 = uint!(0 0 0 0 0 0 0 0 0 1 1 1);
    pub type U3600 = uint!(0 0 0 0 1 0 0 0 0 1 1 1);
    pub type U3616 = uint!(0 0 0 0 0 1 0 0 0 1 1 1);
    pub type U3632 = uint!(0 0 0 0 1 1 0 0 0 1 1 1);
    pub type U3648 = uint!(0 0 0 0 0 0 1 0 0 1 1 1);
    pub type U3664 = uint!(0 0 0 0 1 0 1 0 0 1 1 1);
    pub type U3680 = uint!(0 0 0 0 0 1 1 0 0 1 1 1);
    pub type U3696 = uint!(0 0 0 0 1 1 1 0 0 1 1 1);
    pub type U3712 = uint!(0 0 0 0 0 0 0 1 0 1 1 1);
    pub type U3728 = uint!(0 0 0 0 1 0 0 1 0 1 1 1);
    pub type U3744 = uint!(0 0 0 0 0 1 0 1 0 1 1 1);
    pub type U3760 = uint!(0 0 0 0 1 1 0 1 0 1 1 1);
    pub type U3776 = uint!(0 0 0 0 0 0 1 1 0 1 1 1);
    pub type U3792 = uint!(0 0 0 0 1 0 1 1 0 1 1 1);
    pub type U3808 = uint!(0 0 0 0 0 1 1 1 0 1 1 1);
    pub type U3824 = uint!(0 0 0 0 1 1 1 1 0 1 1 1);
    pub type U3840 = uint!(0 0 0 0 0 0 0 0 1 1 1 1);
    pub type U3856 = uint!(0 0 0 0 1 0 0 0 1 1 1 1);
    pub type U3872 = uint!(0 0 0 0 0 1 0 0 1 1 1 1);
    pub type U3888 = uint!(0 0 0 0 1 1 0 0 1 1 1 1);
    pub type U3904 = uint!(0 0 0 0 0 0 1 0 1 1 1 1);
    pub type U3920 = uint!(0 0 0 0 1 0 1 0 1 1 1 1);
    pub type U3936 = uint!(0 0 0 0 0 1 1 0 1 1 1 1);
    pub type U3952 = uint!(0 0 0 0 1 1 1 0 1 1 1 1);
    pub type U3968 = uint!(0 0 0 0 0 0 0 1 1 1 1 1);
    pub type U3984 = uint!(0 0 0 0 1 0 0 1 1 1 1 1);
    pub type U4000 = uint!(0 0 0 0 0 1 0 1 1 1 1 1);
    pub type U4016 = uint!(0 0 0 0 1 1 0 1 1 1 1 1);
    pub type U4032 = uint!(0 0 0 0 0 0 1 1 1 1 1 1);
    pub type U4048 = uint!(0 0 0 0 1 0 1 1 1 1 1 1);
    pub type U4064 = uint!(0 0 0 0 0 1 1 1 1 1 1 1);
    pub type U4080 = uint!(0 0 0 0 1 1 1 1 1 1 1 1);

    // ML-DSA sizes
    //
    // Includes the public key, private key, and signature sizes not covered elsewhere, as well as
    // some intermediate value sizes.
    pub type U2420 = uint!(0 0 1 0 1 1 1 0 1 0 0 1);
    pub type U3309 = uint!(1 0 1 1 0 1 1 1 0 0 1 1);
    pub type U4480 = uint!(0 0 0 0 0 0 0 1 1 0 0 0 1);
    pub type U4544 = uint!(0 0 0 0 0 0 1 1 1 0 0 0 1);
    pub type U4595 = uint!(1 1 0 0 1 1 1 1 1 0 0 0 1);
    pub type U4627 = uint!(1 1 0 0 1 0 0 0 0 1 0 0 1);
    pub type U4896 = uint!(0 0 0 0 0 1 0 0 1 1 0 0 1);

    // SLH-DSA sizes
    pub type U7856 = uint!(0 0 0 0 1 1 0 1 0 1 1 1 1);
    pub type U16224 = uint!(0 0 0 0 0 1 1 0 1 1 1 1 1 1);
    pub type U17088 = uint!(0 0 0 0 0 0 1 1 0 1 0 0 0 0 1);
    pub type U29792 = uint!(0 0 0 0 0 1 1 0 0 0 1 0 1 1 1);
    pub type U35664 = uint!(0 0 0 0 1 0 1 0 1 1 0 1 0 0 0 1);
    pub type U49856 = uint!(0 0 0 0 0 0 1 1 0 1 0 0 0 0 1 1);

    // Kemeleon ML-KEM Encoding sizes
    pub type U749 = uint!(1 0 1 1 0 1 1 1 0 1);
    pub type U781 = uint!(1 0 1 1 0 0 0 0 1 1);
    pub type U877 = uint!(1 0 1 1 0 1 1 0 1 1);
    pub type U1124 = uint!(0 0 1 0 0 1 1 0 0 0 1);
    pub type U1156 = uint!(0 0 1 0 0 0 0 1 0 0 1);
    pub type U1252 = uint!(0 0 1 0 0 1 1 1 0 0 1);
    pub type U1498 = uint!(0 1 0 1 1 0 1 1 1 0 1);
    pub type U1530 = uint!(0 1 0 1 1 1 1 1 1 0 1);
    pub type U1658 = uint!(0 1 0 1 1 1 1 0 0 1 1);

    // LMS sizes
    pub type U2047 = uint!(1 1 1 1 1 1 1 1 1 1 1);
    pub type U2180 = uint!(0 0 1 0 0 0 0 1 0 0 0 1);
    pub type U4292 = uint!(0 0 1 0 0 0 1 1 0 0 0 0 1);
    pub type U8516 = uint!(0 0 1 0 0 0 1 0 1 0 0 0 0 1);

    // FrodoKEM640 sizes

    pub type U9616 = uint!(0 0 0 0 1 0 0 1 1 0 1 0 0 1);
    pub type U19888 = uint!(0 0 0 0 1 1 0 1 1 0 1 1 0 0 1);
    pub type U9720 = uint!(0 0 0 1 1 1 1 1 1 0 1 0 0 1);
    pub type U9752 = uint!(0 0 0 1 1 0 0 0 0 1 1 0 0 1);

    // FrodoKEM976 sizes
    pub type U15632 = uint!(0 0 0 0 1 0 0 0 1 0 1 1 1 1);
    pub type U31296 = uint!(0 0 0 0 0 0 1 0 0 1 0 1 1 1 1);
    pub type U15744 = uint!(0 0 0 0 0 0 0 1 1 0 1 1 1 1);
    pub type U15792 = uint!(0 0 0 0 1 1 0 1 1 0 1 1 1 1);

    // FrodoKEM1344 sizes
    pub type U21520 = uint!(0 0 0 0 1 0 0 0 0 0 1 0 1 0 1);
    pub type U43088 = uint!(0 0 0 0 1 0 1 0 0 0 0 1 0 1 0 1);
    pub type U21632 = uint!(0 0 0 0 0 0 0 1 0 0 1 0 1 0 1);
    pub type U21696 = uint!(0 0 0 0 0 0 1 1 0 0 1 0 1 0 1);

    impl_consts! {
        1040 => U1040,
        1056 => U1056,
        1072 => U1072,
        1088 => U1088,
        1104 => U1104,
        1120 => U1120,
        1136 => U1136,
        1152 => U1152,
        1168 => U1168,
        1184 => U1184,
        1200 => U1200,
        1216 => U1216,
        1232 => U1232,
        1248 => U1248,
        1264 => U1264,
        1280 => U1280,
        1296 => U1296,
        1312 => U1312,
        1328 => U1328,
        1344 => U1344,
        1360 => U1360,
        1376 => U1376,
        1392 => U1392,
        1408 => U1408,
        1424 => U1424,
        1440 => U1440,
        1456 => U1456,
        1472 => U1472,
        1488 => U1488,
        1504 => U1504,
        1520 => U1520,
        1536 => U1536,
        1552 => U1552,
        1568 => U1568,
        1584 => U1584,
        1600 => U1600,
        1616 => U1616,
        1632 => U1632,
        1648 => U1648,
        1664 => U1664,
        1680 => U1680,
        1696 => U1696,
        1712 => U1712,
        1728 => U1728,
        1744 => U1744,
        1760 => U1760,
        1776 => U1776,
        1792 => U1792,
        1808 => U1808,
        1824 => U1824,
        1840 => U1840,
        1856 => U1856,
        1872 => U1872,
        1888 => U1888,
        1904 => U1904,
        1920 => U1920,
        1936 => U1936,
        1952 => U1952,
        1968 => U1968,
        1984 => U1984,
        2000 => U2000,
        2016 => U2016,
        2032 => U2032,
        2064 => U2064,
        2080 => U2080,
        2096 => U2096,
        2112 => U2112,
        2128 => U2128,
        2144 => U2144,
        2160 => U2160,
        2176 => U2176,
        2192 => U2192,
        2208 => U2208,
        2224 => U2224,
        2240 => U2240,
        2256 => U2256,
        2272 => U2272,
        2288 => U2288,
        2304 => U2304,
        2320 => U2320,
        2336 => U2336,
        2352 => U2352,
        2368 => U2368,
        2384 => U2384,
        2400 => U2400,
        2416 => U2416,
        2432 => U2432,
        2448 => U2448,
        2464 => U2464,
        2480 => U2480,
        2496 => U2496,
        2512 => U2512,
        2528 => U2528,
        2544 => U2544,
        2560 => U2560,
        2576 => U2576,
        2592 => U2592,
        2608 => U2608,
        2624 => U2624,
        2640 => U2640,
        2656 => U2656,
        2672 => U2672,
        2688 => U2688,
        2704 => U2704,
        2720 => U2720,
        2736 => U2736,
        2752 => U2752,
        2768 => U2768,
        2784 => U2784,
        2800 => U2800,
        2816 => U2816,
        2832 => U2832,
        2848 => U2848,
        2864 => U2864,
        2880 => U2880,
        2896 => U2896,
        2912 => U2912,
        2928 => U2928,
        2944 => U2944,
        2960 => U2960,
        2976 => U2976,
        2992 => U2992,
        3008 => U3008,
        3024 => U3024,
        3040 => U3040,
        3056 => U3056,
        3072 => U3072,
        3088 => U3088,
        3104 => U3104,
        3120 => U3120,
        3136 => U3136,
        3152 => U3152,
        3168 => U3168,
        3184 => U3184,
        3200 => U3200,
        3216 => U3216,
        3232 => U3232,
        3248 => U3248,
        3264 => U3264,
        3280 => U3280,
        3296 => U3296,
        3312 => U3312,
        3328 => U3328,
        3344 => U3344,
        3360 => U3360,
        3376 => U3376,
        3392 => U3392,
        3408 => U3408,
        3424 => U3424,
        3440 => U3440,
        3456 => U3456,
        3472 => U3472,
        3488 => U3488,
        3504 => U3504,
        3520 => U3520,
        3536 => U3536,
        3552 => U3552,
        3568 => U3568,
        3584 => U3584,
        3600 => U3600,
        3616 => U3616,
        3632 => U3632,
        3648 => U3648,
        3664 => U3664,
        3680 => U3680,
        3696 => U3696,
        3712 => U3712,
        3728 => U3728,
        3744 => U3744,
        3760 => U3760,
        3776 => U3776,
        3792 => U3792,
        3808 => U3808,
        3824 => U3824,
        3840 => U3840,
        3856 => U3856,
        3872 => U3872,
        3888 => U3888,
        3904 => U3904,
        3920 => U3920,
        3936 => U3936,
        3952 => U3952,
        3968 => U3968,
        3984 => U3984,
        4000 => U4000,
        4016 => U4016,
        4032 => U4032,
        4048 => U4048,
        4064 => U4064,
        4080 => U4080,
    }

    // ML-DSA sizes
    impl_consts! {
        2420 => U2420,
        3309 => U3309,
        4480 => U4480,
        4544 => U4544,
        4595 => U4595,
        4627 => U4627,
        4896 => U4896,
    }

    // SLH-DSA sizes
    impl_consts! {
        7856 => U7856,
        16224 => U16224,
        17088 => U17088,
        29792 => U29792,
        35664 => U35664,
        49856 => U49856,
    }

    // Kemeleon ML-KEM Encoding sizes
    impl_consts! {
        749 => U749,
        781 => U781,
        877 => U877,
        1124 => U1124,
        1156 => U1156,
        1252 => U1252,
        1498 => U1498,
        1530 => U1530,
        1658 => U1658,
    }

    // LMS sizes
    impl_consts! {
        2047 => U2047,
        2180 => U2180,
        4292 => U4292,
        8516 => U8516,
    }

    // Frodo sizes
    impl_consts! {
        9616 => U9616,
        19888 => U19888,
        9720 => U9720,
        9752 => U9752,
        15632 => U15632,
        31296 => U31296,
        15744 => U15744,
        15792 => U15792,
        21520 => U21520,
        43088 => U43088,
        21632 => U21632,
        21696 => U21696,
    }
}
