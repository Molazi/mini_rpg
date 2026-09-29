mod combat;
mod enemy;
mod event;
mod game;
mod player;
mod ui;
use enemy::Enemy;
use player::Player;

fn main() {
    let mut player = Player::new(String::from("Arthur"), 100, 100, 3, 10);
    let mut enemies = vec![
        Enemy::new(String::from("Goblin"), 20, 20, 5),
        Enemy::new(String::from("Bandit"), 30, 30, 10),
        Enemy::new(String::from("Mutant"), 50, 50, 15),
    ];

    game::game_loop(&mut player, &mut enemies);
}
