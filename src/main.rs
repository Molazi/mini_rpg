mod combat;
mod enemy;
mod player;

fn main() {
    let character_1 = player::Player::new(String::from("Arthur"), 100, 100);
    let mut enemy_1 = enemy::Enemy::new(String::from("Goblin"), 20, 20);

    combat::attack(&character_1, &mut enemy_1, 20);
    println!("{} HP: {}", enemy_1.name(), enemy_1.health());
}
