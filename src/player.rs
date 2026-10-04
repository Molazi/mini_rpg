use crate::combat::Damageable;

pub struct Player {
    name: String,
    hp: u32,
    max_hp: u32,
    potions: u32,
    damage: u32,
}

impl Player {
    pub fn new(name: String, hp: u32, max_hp: u32, potions: u32, damage: u32) -> Player {
        Player {
            name,
            hp,
            max_hp,
            potions,
            damage,
        }
    }
    pub fn health(&self) -> u32 {
        self.hp
    }
    pub fn max_health(&self) -> u32 {
        self.max_hp
    }
    pub fn potions(&self) -> u32 {
        self.potions
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn attack_damage(&self) -> u32 {
        self.damage
    }
    pub fn take_damage(&mut self, damage: u32) {
        if damage >= self.hp {
            self.hp = 0;
        } else {
            self.hp -= damage;
        }
    }
    fn heal(&mut self, amount: u32) -> u32 {
        if self.hp + amount >= self.max_hp {
            let healed = self.max_hp - self.hp;
            self.hp = self.max_hp;
            healed
        } else {
            self.hp += amount;
            amount
        }
    }
    pub fn use_potion(&mut self) -> Option<u32> {
        if self.potions == 0 {
            None
        } else {
            self.potions -= 1;
            Some(self.heal(20))
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
        let mut player = super::Player::new(String::from("Arthur"), 100, 100, 3, 10);
        player.take_damage(20);
        assert_eq!(player.hp, 80);
    }
    #[test]
    fn test_take_damage_exceeding_health() {
        let mut player = super::Player::new(String::from("Mordred"), 150, 150, 3, 15);
        player.take_damage(200);
        assert_eq!(player.hp, 0);
    }
    #[test]
    fn test_take_damage_equal_to_health() {
        let mut player = super::Player::new(String::from("Mordred"), 150, 150, 3, 15);
        player.take_damage(150);
        assert_eq!(player.hp, 0);
    }
    #[test]
    fn test_use_potion() {
        let mut player = super::Player::new(String::from("Arthur"), 70, 100, 3, 10);
        let result = player.use_potion();
        assert_eq!(result, Some(20));
        assert_eq!(player.health(), 90);
        assert_eq!(player.potions, 2);
    }
    #[test]
    fn test_no_potions() {
        let mut player = super::Player::new(String::from("Arthur"), 70, 100, 0, 10);
        let result = player.use_potion();
        assert_eq!(result, None);
        assert_eq!(player.health(), 70);
        assert_eq!(player.potions, 0);
    }
    #[test]
    fn test_potion_hp_cap() {
        let mut player = super::Player::new(String::from("Arthur"), 90, 100, 3, 10);
        let result = player.use_potion();
        assert_eq!(result, Some(10));
        assert_eq!(player.health(), 100);
        assert_eq!(player.potions, 2);
    }
}
