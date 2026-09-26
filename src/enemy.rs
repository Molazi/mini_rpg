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
    pub fn heal(&mut self, amount: u32) {
        if self.hp + amount >= self.max_hp {
            self.hp = self.max_hp;
        } else {
            self.hp += amount;
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
        let mut enemy_1 = super::Enemy::new(String::from("Goblin"), 20, 20, 10);
        enemy_1.take_damage(10);
        assert_eq!(enemy_1.health(), 10);
    }
    #[test]
    fn test_heal() {
        let mut enemy_1 = super::Enemy::new(String::from("Goblin"), 10, 20, 10);
        enemy_1.heal(20);
        assert_eq!(enemy_1.health(), enemy_1.max_hp);
    }
    #[test]
    fn test_is_alive() {
        let enemy = super::Enemy::new(String::from("Goblin"), 100, 100, 10);
        assert!(enemy.is_alive());
    }
    #[test]
    fn test_is_alive_2() {
        let enemy = super::Enemy::new(String::from("Goblin"), 0, 100, 10);
        assert!(!enemy.is_alive());
    }
    #[test]
    fn test_attack_damage() {
        let enemy = super::Enemy::new(String::from("Goblin"), 20, 20, 5);
        assert_eq!(enemy.attack_damage(), 5);
    }
}
