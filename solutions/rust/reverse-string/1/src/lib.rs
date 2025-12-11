pub fn reverse(input: &str) -> String {
     let mut chars: Vec<char> = input.chars().collect();
    chars.reverse();
    let result: String = chars.iter().collect();
    result
}
