use std::fmt;
struct Player {
    name: String,
    health: i32,
    mana: i32,
}
impl Player {
    fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            health: 100,
            mana: 100,
        }
    }
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(health={}, mana={})", self.name, self.health, self.mana)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_display_player() {
        let p = Player::new("Samewise");
        assert_eq!(format!("{p}"), "Samewise(health=100, mana=100)");
    }
}
