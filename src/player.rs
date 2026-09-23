use crate::combat::Damageable;

pub struct Player {
    name: String,
    hp: u32,
    max_hp: u32,
    potions: u32,
}

impl Player {
    pub fn new(name: String, hp: u32, max_hp: u32, potions: u32) -> Player {
        Player {
            name,
            hp,
            max_hp,
            potions,
        }
    }
    pub fn health(&self) -> u32 {
        self.hp
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn take_damage(&mut self, damage: u32) {
        if damage >= self.hp {
            self.hp = 0;
        } else {
            self.hp -= damage;
        }
    }
    fn heal(&mut self, amount: u32) {
        if self.hp + amount >= self.max_hp {
            self.hp = self.max_hp;
        } else {
            self.hp += amount;
        }
    }
    pub fn use_potion(&mut self) {
        if self.potions == 0 {
            println!("No potions left!");
        } else {
            self.potions -= 1;
            println!("{} used healing potion!", self.name());
            self.heal(20);
        }
    }
    pub fn is_alive(&self) -> bool {
        self.hp != 0
    }
}

impl Damageable for Player {
    fn take_damage(&mut self, damage: u32) {
        Player::take_damage(self, damage);
    }

    fn name(&self) -> &str {
        Player::name(self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_take_damage() {
        let mut player = super::Player::new(String::from("Arthur"), 100, 100, 3);

        player.take_damage(20);

        assert_eq!(player.hp, 80);

        let mut player_2 = super::Player::new(String::from("Mordred"), 150, 150, 3);

        player_2.take_damage(200);

        assert_eq!(player_2.hp, 0);
    }
    #[test]
    fn test_health() {
        let player = super::Player::new(String::from("Arthur"), 100, 100, 3);

        assert_eq!(player.health(), 100);
    }
    #[test]
    fn test_heal() {
        let mut player = super::Player::new(String::from("Arthur"), 80, 100, 3);

        player.heal(20);

        assert_eq!(player.hp, 100);
    }
    #[test]
    fn test_is_alive() {
        let player = super::Player::new(String::from("Arthur"), 100, 100, 3);
        assert!(player.is_alive());
    }
    #[test]
    fn test_is_alive_2() {
        let player = super::Player::new(String::from("Arthur"), 0, 100, 3);
        assert!(!player.is_alive());
    }
    #[test]
    fn test_potions_amount() {
        let player = super::Player::new(String::from("Arthur"), 100, 100, 3);
        assert_eq!(player.potions, 3);
    }
    #[test]
    fn test_use_potion() {
        let mut player = super::Player::new(String::from("Arthur"), 70, 100, 3);
        player.use_potion();
        assert_eq!(player.health(), 90);
        assert_eq!(player.potions, 2);
    }
    #[test]
    fn test_no_potions() {
        let mut player = super::Player::new(String::from("Arthur"), 70, 100, 0);
        player.use_potion();
        assert_eq!(player.health(), 70);
        assert_eq!(player.potions, 0);
    }
    #[test]
    fn test_potion_hp_cap() {
        let mut player = super::Player::new(String::from("Arthur"), 90, 100, 3);
        player.use_potion();
        assert_eq!(player.health(), 100);
        assert_eq!(player.potions, 2);
    }
}
