/*
bit calculator util functions
*/
pub mod cli {
    use std::io;
    use super::*;
    fn prompt_for_number(prompt: &str, stdin: &mut io::Stdin) -> String {
        let mut buffer = String::new();
        let mut parsed: bool = false;
        while ! parsed {
            println!("{}", prompt);
            let ret = stdin.read_line(&mut buffer);
            match ret {
                Ok(n) => {
                    if n != 4 {
                        println!("input should be three bits, found {}", n-1);
                        continue;
                    }
                    parsed = true;
                },
                Err(e) => {
                    println!("input error: {}", e); 
                    continue;
                }

            };
        }
        return buffer.trim().to_owned();
    }
    pub fn main() {
        let mut buffer = String::new();
        let mut stdin = io::stdin();
        loop {
            println!("input operator (NOT|OR|AND):");
            stdin.read_line(&mut buffer).unwrap();
            match buffer.trim() {
                "NOT" => {
                    let number = prompt_for_number("enter a number:",&mut stdin);
                    buffer = three_bit_not(&number);
                },
                "OR" => {
                    let first = prompt_for_number("enter first number:",&mut stdin);
                    let second = prompt_for_number("enter another number:",&mut stdin);
                    buffer = three_bit_or(&first, &second);
                },
                "AND" => {
                    let first = prompt_for_number("enter first number:",&mut stdin);
                    let second = prompt_for_number("enter another number:",&mut stdin);
                    buffer = three_bit_and(&first, &second);
                },
                _ => { 
                    println!("invalid operator: {}", buffer);
                    continue;
                }
            };
            break;
        }
        println!("{}", buffer);
    }
}

pub fn one_bit_not(bit: &str) -> String {
    match parse_bit(bit) {
        0 => String::from("1"),
        1 => String::from("0"),
        _ => panic!("\"{}\" is not a bit", bit),
    }
}
pub fn three_bit_not(bits: &str) -> String {
    let chars: String = bits.chars().map(|b| one_bit_not(&b.to_string())).collect();
    assert_eq!(chars.len(), 3);
    chars
}

pub fn one_bit_or(first: &str, second: &str) -> String {
    (parse_bit(first) | parse_bit(second)).to_string()
}
pub fn three_bit_or(first: &str, second: &str) -> String {
    Iterator::zip(first.chars(), second.chars())
        .map(|pair| one_bit_or(&pair.0.to_string(), &pair.1.to_string()))
        .collect()
}

pub fn one_bit_and(first: &str, second: &str) -> String {
    (parse_bit(first) & parse_bit(second)).to_string()
}
pub fn three_bit_and(first: &str, second: &str) -> String {
    Iterator::zip(first.chars(), second.chars())
        .map(|pair| one_bit_and(&pair.0.to_string(), &pair.1.to_string()))
        .collect()
}

fn parse_bit(from: &str) -> u8 {
    match from {
        "0" => 0,
        "1" => 1,
        _ => panic!("\"{}\" is not a bit", from),
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one_bit_not() {
        assert_eq!(one_bit_not("0"), "1");
        assert_eq!(one_bit_not("1"), "0");
    }

    #[test]
    #[should_panic]
    fn test_one_bit_not_panics() {
        one_bit_not("2");
    }

    #[test]
    fn test_three_bit_not() {
        assert_eq!(three_bit_not("000"), "111");
        assert_eq!(three_bit_not("010"), "101");
    }

    #[test]
    #[should_panic]
    fn test_three_bit_not_panics() {
        three_bit_not("00");
        three_bit_not("0000");
    }
    #[test]
    fn test_one_bit_or() {
        assert_eq!(one_bit_or("0", "0"), "0");
        assert_eq!(one_bit_or("0", "1"), "1");
        assert_eq!(one_bit_or("1", "1"), "1");
    }
    #[test]
    #[should_panic]
    fn test_one_bit_or_panics() {
        one_bit_or("2", "0");
    }
    #[test]
    fn test_three_bit_or() {
        assert_eq!(three_bit_or("000", "001"), "001");
        assert_eq!(three_bit_or("010", "001"), "011");
        assert_eq!(three_bit_or("000", "111"), "111");
        assert_eq!(three_bit_or("000", "000"), "000");
    }
    #[test]
    fn test_one_bit_and() {
        assert_eq!(one_bit_and("0", "1"), "0");
        assert_eq!(one_bit_and("1", "1"), "1");
        assert_eq!(one_bit_and("0", "0"), "0");
    }
    #[test]
    #[should_panic]
    fn test_one_bit_and_panics() {
        one_bit_and("1", "10");
    }
    #[test]
    fn test_three_bit_and() {
        assert_eq!(three_bit_and("000", "001"), "000");
        assert_eq!(three_bit_and("001", "001"), "001");
        assert_eq!(three_bit_and("101", "001"), "001");
    }
}
