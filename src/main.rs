mod combat;
mod enemy;
mod player;

fn main() {
    let mut character_1 = player::Player::new(String::from("Arthur"), 100, 100);
    let mut enemy_1 = enemy::Enemy::new(String::from("Goblin"), 20, 20);

    combat::attack(&enemy_1, &mut character_1, 10);
    println!("{} HP: {}", character_1.name(), character_1.health());

    combat::attack(&character_1, &mut enemy_1, 20);
    println!("{} HP: {}", enemy_1.name(), enemy_1.health());
}
