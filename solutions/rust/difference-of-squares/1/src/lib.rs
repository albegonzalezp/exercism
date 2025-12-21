pub fn square_of_sum(n: u32) -> u32 {
    if n < 1 {
        return 0;
    }
    let mut result:u32 = 0;
    (1..=n).for_each(|x| result += x);

    result * result
 
}

pub fn sum_of_squares(n: u32) -> u32 {
    if n < 1 {
        return 0;
    }
    let mut result:u32 = 0;
    (1..=n).for_each(|x| result += x * x);

    result
}

pub fn difference(n: u32) -> u32 {
    //todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
    square_of_sum(n) - sum_of_squares(n)
}
