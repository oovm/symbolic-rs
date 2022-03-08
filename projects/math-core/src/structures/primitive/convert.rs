use super::*;

macro_rules! into_primitive {
    ($($t:ty => $i:ident),*) => {
        $(
            impl From<$t> for Primitive {
                fn from(value: $t) -> Self {
                    Primitive::$i(value)
                }
            }
        )*
    };
}

into_primitive![
    bool => Boolean,
    char => Character
];

macro_rules! into_integer {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Primitive {
                fn from(value: $t) -> Self {
                    Primitive::Integer(BigInt::from(value))
                }
            }
        )*
    };
}

into_integer![i8, i16, i32, i64, i128, isize, BigInt];
into_integer![u8, u16, u32, u64, u128, usize, BigUint];

macro_rules! into_string {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Primitive {
                fn from(value: $t) -> Self {
                    Primitive::String(value.to_string())
                }
            }
        )*
    };
}

into_string!(String, &String, &str);
