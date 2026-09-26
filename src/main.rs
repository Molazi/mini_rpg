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

enum ActionResult {
    Continue,
    EnemyDefeated,
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

fn player_turn(player: &mut Player, enemy: &mut Enemy) -> ActionResult {
    let action = choose_action();

    match action {
        Action::Attack => {
            combat::attack(player, enemy, 10);
            if !enemy.is_alive() {
                ActionResult::EnemyDefeated
            } else {
                ActionResult::Continue
            }
        }
        Action::Heal => {
            player.use_potion();
            ActionResult::Continue
        }
        Action::Run => ActionResult::Run,
    }
}

fn enemy_turn(enemy: &Enemy, player: &mut Player) {
    combat::attack(enemy, player, enemy.attack_damage());
}

fn game_loop(player: &mut Player, enemies: &mut Vec<Enemy>) {
    for foe in enemies {
        if !foe.is_alive() {
            continue;
        }
        println!("{} appeared!", foe.name());
        loop {
            match player_turn(player, foe) {
                ActionResult::Continue => {
                    enemy_turn(foe, player);
                    if !player.is_alive() {
                        println!("{} is dead!", player.name());
                        println!("You lost!");
                        return;
                    }
                }
                ActionResult::EnemyDefeated => {
                    println!("{} is dead!", foe.name());
                    break;
                }
                ActionResult::Run => {
                    println!("{} runs away!", player.name());
                    return;
                }
            }
        }
    }
    println!("You win!");
}

fn main() {
    let mut player = Player::new(String::from("Arthur"), 100, 100, 3);
    let mut enemies = vec![
        Enemy::new(String::from("Goblin"), 20, 20, 5),
        Enemy::new(String::from("Bandit"), 30, 30, 10),
        Enemy::new(String::from("Mutant"), 50, 50, 15),
    ];

    game_loop(&mut player, &mut enemies);
}
