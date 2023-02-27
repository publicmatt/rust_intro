use crate::compleat::potion::Potion;
use crate::compleat::weapon::Weapon;
use std::fmt;

pub struct Player {
    name: String,
    health: i32,
    mana: i32,
}
impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            health: 100,
            mana: 100,
        }
    }
    fn suffer_damage(&mut self, damage: i32) -> () {
        if self.health <= damage {
            self.health = 0;
        } else {
            self.health -= damage;
        }
    }
    pub fn attack(
        &mut self,
        opponent: &mut Player,
        weapon: &Weapon,
        is_ultimate: Option<bool>,
    ) -> () {
        let (damage, cost, ultimate) = weapon.stats();
        if self.mana <= cost {
            return ();
        }
        let ult = match is_ultimate {
            Some(v) => v,
            None => false,
        };
        let d: i32;
        if ult {
            d = ultimate;
            self.mana = 0;
        } else {
            d = damage;
            self.mana -= cost;
        }
        opponent.suffer_damage(d);
    }
    pub fn drink(&mut self, potion: &mut Potion) -> () {
        let benefit = potion.drink();
        if benefit + self.health >= 100 {
            self.health = 100;
        } else {
            self.health += benefit;
        }
    }
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}(health={}, mana={})",
            self.name, self.health, self.mana
        )
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
    #[test]
    fn test_suffer_damage() {
        let mut p = Player::new("Samewise");
        p.suffer_damage(10);
        assert_eq!(p.health, 90);
        p.suffer_damage(100);
        assert_eq!(p.health, 0);
    }
    #[test]
    fn test_attack() {
        let mut first = Player::new("Samewise");
        let mut second = Player::new("Orc");
        let w = Weapon::new("sword", 5, 1, 25);
        first.attack(&mut second, &w, None);
        assert_eq!(first.mana, 99);
        assert_eq!(second.health, 95);
        first.attack(&mut second, &w, Some(false));
        assert_eq!(first.mana, 98);
        assert_eq!(second.health, 90);
        first.attack(&mut second, &w, Some(true));
        assert_eq!(first.mana, 0);
        assert_eq!(second.health, 65);
    }
    #[test]
    fn test_drink() {
        let mut first = Player::new("Samewise");
        let mut p = Potion::new("elixir", Some(10));
        first.drink(&mut p);
        assert_eq!(first.health, 100);
        first.health -= 5;
        first.drink(&mut p);
        assert_eq!(first.health, 95);
        p = Potion::new("elixir", Some(10));
        first.drink(&mut p);
        assert_eq!(first.health, 100);
        p = Potion::new("elixir", Some(10));
        first.health -= 50;
        first.drink(&mut p);
        assert_eq!(first.health, 60);
    }
}
