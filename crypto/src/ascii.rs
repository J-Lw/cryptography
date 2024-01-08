// p:
// - Using the following integer array, convert the numbers to their corresponding ascii characters to obtain a flag.
// - [99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98, 108, 51, 125].
pub fn to_ascii(numbers: Vec<i32>) -> String {
    let mut flag = String::new();
     
    for number in numbers.iter() {
        if *number <= 57 {
            let number = std::char::from_u32((number - 48) as u32); 
            flag.push(number); 
        }
    }

    flag
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("Product: {}", to_ascii(vec![99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98, 108, 51, 125]));
    }
}
