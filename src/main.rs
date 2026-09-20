mod player;
use player::Player;

fn main() {
    let mut player_1 = Player::new(String::from("Arthur"), 100, 100);

    player_1.take_damage(20);
    let new_health = player_1.health();

    println!("HP: {}", new_health);
}
