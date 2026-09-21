use crate::player::Player;

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
    #[test]
    fn test_attack() {
        let character_1 = super::Player::new(String::from("Arthur"), 100, 100);
        let mut character_2 = super::Player::new(String::from("Mordred"), 150, 150);
        super::attack(&character_1, &mut character_2, 20);
        assert_eq!(character_2.health(), 130);
    }
}
