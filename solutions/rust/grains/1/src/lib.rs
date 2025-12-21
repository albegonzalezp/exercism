pub fn square(s: u32) -> u128 {
    
    if s < 1 || s > 64 {
        panic!("should be positive number, max 64")
    }

    2u128.pow(s-1)
}

pub fn total() -> u128 {
    2u128.pow(64) - 1
}