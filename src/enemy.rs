use crate::combat::Damageable;

pub struct Enemy {
    name: String,
    hp: u32,
    max_hp: u32,
    damage: u32,
}

impl Enemy {
    pub fn new(name: String, hp: u32, max_hp: u32, damage: u32) -> Enemy {
        Enemy {
            name,
            hp,
            max_hp,
            damage,
        }
    }
    pub fn health(&self) -> u32 {
        self.hp
    }
    pub fn max_health(&self) -> u32 {
        self.max_hp
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
    pub fn is_alive(&self) -> bool {
        self.hp != 0
    }
}

impl Damageable for Enemy {
    fn take_damage(&mut self, damage: u32) {
        Enemy::take_damage(self, damage);
    }

    fn name(&self) -> &str {
        Enemy::name(self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_take_damage() {
        let mut enemy = super::Enemy::new(String::from("Goblin"), 20, 20, 10);
        enemy.take_damage(10);
        assert_eq!(enemy.hp, 10);
    }
    #[test]
    fn test_take_damage_exceeding_health() {
        let mut enemy = super::Enemy::new(String::from("Goblin"), 20, 20, 10);
        enemy.take_damage(30);
        assert_eq!(enemy.hp, 0);
    }
    #[test]
    fn test_take_damage_equal_to_health() {
        let mut enemy = super::Enemy::new(String::from("Goblin"), 20, 20, 10);
        enemy.take_damage(20);
        assert_eq!(enemy.hp, 0);
    }
}
