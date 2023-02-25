mod bits;
mod config;

use std::io;
use bits::*;

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    let mut answer = String::new();
    loop {
        println!("input operator (NOT|OR|AND):");
        stdin.read_line(&mut buffer).unwrap();
        match buffer.trim() {
            "NOT" => {
                let number = prompt_for_number("enter a number:",&mut stdin);
                answer = three_bit_not(&number);
                break;
            },
            "OR" => {
                let first = prompt_for_number("enter first number:",&mut stdin);
                let second = prompt_for_number("enter another number:",&mut stdin);
                answer = three_bit_or(&first, &second);
                break;
            },
            "AND" => {
                let first = prompt_for_number("enter first number:",&mut stdin);
                let second = prompt_for_number("enter another number:",&mut stdin);
                answer = three_bit_and(&first, &second);
                break;
            },
            _ => { 
                println!("invalid operator: {}", buffer);
                continue;
            }
        };
    }
    println!("{}", answer);
}
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
