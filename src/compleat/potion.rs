pub struct Potion {
    name: String,
    benefit: i32,
}
impl Potion {
    pub fn new(name: &str, benefit: Option<i32>) -> Self {
        let default = match benefit {
            Some(b) => b,
            None => 10,
        };
        Potion {
            name: name.to_string(),
            benefit: default,
        }
    }
    pub fn drink(&mut self) -> i32 {
        let ret = self.benefit;
        self.benefit = 0;
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new() {
        let p = Potion::new("ale", None);
        assert_eq!(p.benefit, 10);
        let p = Potion::new("ale", Some(50));
        assert_eq!(p.benefit, 50);
    }
    #[test]
    fn test_drink() {
        let mut p = Potion::new("ale", None);
        assert_eq!(p.benefit, 10);
        let benefit = p.drink();
        assert_eq!(benefit, 10);
        assert_eq!(p.benefit, 0);
    }
}
