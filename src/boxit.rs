pub mod cli {
    use std::io;
    use super::*;

    pub fn main() {
        let padding = 10;
        let stdin = io::stdin();
        let n = get_n(stdin.lock());
        let mut lines: Vec<String> = Vec::new();
        for (i, line) in stdin.lines().enumerate() {
            if i == n {
                break;
            }
            match line {
                Ok(l) => lines.push(l),
                Err(_) => panic!("error reading line")
            }
        }
        let longest: usize = lines.iter()
            .map(|line| line.len())
            .max()
            .unwrap();
        println!("longest line: {}", longest);
        lines
            .iter()
            .for_each(|l| {
                println!("{}", frame_line(l, longest + padding));
            });
    }
    fn get_n<T: io::BufRead>(mut input: T) -> usize {
        let mut buffer = String::new();
        input.read_line(&mut buffer).unwrap();
        let n: usize = buffer.trim().parse().unwrap();
        n
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_get_n() {
            assert_eq!(get_n(io::BufReader::new(String::from("5\nhello").as_bytes())), 5);
            assert_eq!(get_n(io::BufReader::new(String::from("4 \nhello").as_bytes())), 4);
        }

        #[test]
        #[should_panic]
        fn test_get_n_panics() {
            get_n(io::BufReader::new(String::from("t \nhello").as_bytes()));
        }
    }

}

fn frame_line(line: &str, pad_to: usize) -> String {
    let remaining = pad_to - line.len();
    let mut ret: String = line.to_string();
    if ret.len() > pad_to {
        panic!("line too long. len: {}, framing: {}", ret.len(), pad_to);
    }
    let suffix = " ".repeat(remaining - 2) + "+";
    let prefix = "+ ";
    ret.insert_str(0, prefix);
    ret.push_str(&suffix);
    ret
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_init() {
        assert_eq!(frame_line("And now", 14), String::from("+ And now    +"));
    }
}
