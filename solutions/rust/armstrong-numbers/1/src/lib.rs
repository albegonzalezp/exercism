use num_traits::{ToPrimitive};
pub fn is_armstrong_number(num: u32) -> bool {
    let mut result = 0;
    let mut n = num;
    while n > 0 {
        result += (n % 10).pow(num.to_string().len().to_u32().unwrap());
        n /= 10
    }

    result == num
}
