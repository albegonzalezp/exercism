pub fn square(s: u32) -> u128 {
    if !(1..=64).contains(&s) {
        panic!("should be positive number, max 64")
    }

    2u128.pow(s-1)
}

pub fn total() -> u128 {
    2u128.pow(64) - 1
}