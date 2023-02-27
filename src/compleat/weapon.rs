pub struct Weapon {
    name: String,
    pub damage: i32,
    pub cost: i32,
    pub ultimate: i32,
}
impl Weapon {
    pub fn new(name: &str, damage: i32, cost: i32, ultimate: i32) -> Self {
        Weapon {
            name: name.to_string(),
            damage,
            cost,
            ultimate,
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_init() {
        let w = Weapon::new("sword", 5, 1, 25);
        assert_eq!(w.damage, 5);
    }
}
