fn main() {
    series("125443634234", 2);
}

pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut return_vector: Vec<String> = Vec::new();

    if digits.is_empty() || len > digits.len() {
        return return_vector;
    }

    for i in 0..=digits.len() - len {
        let substring = &digits[i..i + len];
        return_vector.push(substring.to_string());
    }

    println!("{:?}", return_vector);
    return_vector
}
