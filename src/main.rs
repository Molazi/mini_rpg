mod combat;
mod player;

fn main() {
    let character_1 = player::Player::new(String::from("Arthur"), 100, 100);
    let mut character_2 = player::Player::new(String::from("Mordred"), 150, 150);

    combat::attack(&character_1, &mut character_2, 20);
    println!("Mordred HP: {}", character_2.health());
}
