use std::{fmt::Display, str::FromStr};

pub fn string_to_u64(value: &str) -> u64 {
    let num = u64::from_str(value).unwrap();

    // calls from_str internally
    let num: u64 = value.parse().unwrap();

    num
}

pub fn convert_traits<T, U>(src: T) -> U
where
    T: Into<U> + Display,
    U: From<T>,
{
    U::from(src)
}

pub fn defaultValue<T>(value: T) -> T
where
    T: Default,
{
    T::default()
}
