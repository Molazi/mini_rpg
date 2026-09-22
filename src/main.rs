mod combat;
mod enemy;
mod player;
use enemy::Enemy;
use player::Player;

enum Action {
    Attack,
    Heal,
    Run,
}

fn choose_action() -> Action {
    loop {
        println!("Choose action: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => break Action::Attack,
            "2" => break Action::Heal,
            "3" => break Action::Run,
            _ => {
                println!("Unknown Action");
            }
        }
    }
}

fn player_turn(player: &mut Player, enemy: &mut Enemy) -> bool {
    let action = choose_action();

    match action {
        Action::Attack => {
            combat::attack(player, enemy, 10);
            true
        }
        Action::Heal => {
            Player::heal(player, 20);
            true
        }
        Action::Run => false,
    }
}

fn enemy_turn(enemy: &Enemy, player: &mut Player) {
    combat::attack(enemy, player, 10);
}

fn game_loop(player: &mut Player, enemy: &mut Enemy) {
    loop {
        if !player_turn(player, enemy) {
            break;
        }

        if !enemy.is_alive() {
            println!("{} is dead!", enemy.name());
            println!("You win!");
            break;
        }

        enemy_turn(enemy, player);

        if !player.is_alive() {
            println!("{} is dead!", player.name());
            println!("You lost!");
            break;
        }
    }
}

fn main() {
    let mut player = Player::new(String::from("Arthur"), 100, 100);
    let mut enemy_1 = Enemy::new(String::from("Goblin"), 20, 20);

    game_loop(&mut player, &mut enemy_1);

    println!("{} HP: {}", enemy_1.name(), enemy_1.health());
    println!("{} HP: {}", player.name(), player.health());
}
