pub mod player;
pub mod potion;
pub mod weapon;
pub mod cli {
    use super::*;

    use player::Player;
    use potion::Potion;
    use weapon::Weapon;
    pub fn main() {
        println!("not implemented");
        // Create two players
        let mut frodo = Player::new("Frodo");
        let mut sauron = Player::new("Sauron");

        // Create a magic elixir
        let mut elixir = Potion::new("Magic Elixir", Some(20));

        // Create some weapons
        let sting = Weapon::new("Sting", 5, 5, 50);
        let blaster = Weapon::new("Blaster", 10, 2, 15);

        // Play the game! -- the players take turns attacking each other
        sauron.attack(&mut frodo, &blaster, None); // Sauron attacks Frodo with a blaster
        frodo.attack(&mut sauron, &sting, None); // Frodo attacks Sauron with Sting
        sauron.attack(&mut frodo, &blaster, None); //  Sauron blasts Frodo again
        sauron.attack(&mut frodo, &blaster, Some(true)); // Sauron ultimates
        sauron.attack(&mut frodo, &blaster, None); // desperation move

        // Frodo drinks a potion
        frodo.drink(&mut elixir);

        // Let's examine who's healthier
        println!("{frodo}");
        println!("{sauron}");
    }
}
