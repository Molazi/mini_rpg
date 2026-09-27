pub trait Damageable {
    fn take_damage(&mut self, damage: u32);
    fn name(&self) -> &str;
}

pub fn attack(attacker: &impl Damageable, target: &mut impl Damageable, damage: u32) {
    target.take_damage(damage);
    println!(
        "{} dealt {} damage to {}",
        attacker.name(),
        damage,
        target.name()
    );
}

#[cfg(test)]
mod tests {
    use crate::player::Player;
    #[test]
    fn test_attack() {
        let character_1 = Player::new(String::from("Arthur"), 100, 100, 3, 10);
        let mut enemy_1 = crate::enemy::Enemy::new(String::from("Goblin"), 20, 20, 10);
        super::attack(&character_1, &mut enemy_1, 20);
        assert_eq!(enemy_1.health(), 0);
    }
    #[test]
    fn test_enemy_attack() {
        let mut character_1 = Player::new(String::from("Arthur"), 100, 100, 3, 10);
        let enemy_1 = crate::enemy::Enemy::new(String::from("Goblin"), 20, 20, 10);
        super::attack(&enemy_1, &mut character_1, 20);
        assert_eq!(character_1.health(), 80);
    }
}
