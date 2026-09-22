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

fn main() {
    let mut player = Player::new(String::from("Arthur"), 100, 100);
    let mut enemy_1 = Enemy::new(String::from("Goblin"), 20, 20);

    loop {
        let action = choose_action();
        match action {
            Action::Attack => combat::attack(&player, &mut enemy_1, 10),
            Action::Heal => Player::heal(&mut player, 20),
            Action::Run => {
                println!("Player runs away!");
                break;
            }
        }

        if enemy_1.health() == 0 {
            println!("{} is dead!", enemy_1.name());
            println!("You win!");
            break;
        }

        combat::attack(&enemy_1, &mut player, 5);

        if player.health() == 0 {
            println!("{} is dead!", player.name());
            println!("You lost!");
            break;
        }
    }

    println!("{} HP: {}", enemy_1.name(), enemy_1.health());
    println!("{} HP: {}", player.name(), player.health());
}
