mod combat;
mod enemy;
mod event;
mod player;
mod ui;
use enemy::Enemy;
use player::Player;

use crate::event::BattleEvent;

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

fn player_turn(player: &mut Player, enemy: &mut Enemy, log: &mut Vec<BattleEvent>) -> ActionResult {
    let action = choose_action();

    match action {
        Action::Attack => {
            let damage = player.attack_damage();
            combat::attack(player, enemy, damage);
            log.push(BattleEvent::Attack {
                attacker: String::from(player.name()),
                target: String::from(enemy.name()),
                damage,
            });
            if !enemy.is_alive() {
                ActionResult::EnemyDefeated
            } else {
                ActionResult::Continue
            }
        }
        Action::Heal => {
            let healed = player.use_potion();
            match healed {
                Some(healed) => log.push(BattleEvent::PotionUsed {
                    name: String::from(player.name()),
                    healed,
                }),
                None => log.push(BattleEvent::NoPotions),
            }
            ActionResult::Continue
        }
        Action::Run => ActionResult::Run,
    }
}

fn enemy_turn(enemy: &Enemy, player: &mut Player, log: &mut Vec<BattleEvent>) {
    let damage = enemy.attack_damage();
    combat::attack(enemy, player, damage);
    log.push(BattleEvent::Attack {
        attacker: String::from(enemy.name()),
        target: String::from(player.name()),
        damage,
    })
}

fn game_loop(player: &mut Player, enemies: &mut Vec<Enemy>) {
    let mut log: Vec<BattleEvent> = Vec::new();
    for foe in enemies {
        if !foe.is_alive() {
            continue;
        }
        log.push(BattleEvent::EnemyAppeared {
            name: String::from(foe.name()),
        });
        loop {
            ui::render(player, Some(foe), &log);
            match player_turn(player, foe, &mut log) {
                ActionResult::Continue => {
                    enemy_turn(foe, player, &mut log);
                    if !player.is_alive() {
                        log.push(BattleEvent::Defeated {
                            name: String::from(player.name()),
                        });
                        ui::render(player, Some(foe), &log);
                        return;
                    }
                }
                ActionResult::EnemyDefeated => {
                    log.push(BattleEvent::Defeated {
                        name: String::from(foe.name()),
                    });
                    ui::render(player, Some(foe), &log);
                    break;
                }
                ActionResult::Run => {
                    log.push(BattleEvent::PlayerRan {
                        name: String::from(player.name()),
                    });
                    ui::render(player, Some(foe), &log);
                    return;
                }
            }
        }
    }
    log.push(BattleEvent::Victory {
        name: String::from(player.name()),
    });
    ui::render(player, None, &log);
}

fn main() {
    let mut player = Player::new(String::from("Arthur"), 100, 100, 3, 10);
    let mut enemies = vec![
        Enemy::new(String::from("Goblin"), 20, 20, 5),
        Enemy::new(String::from("Bandit"), 30, 30, 10),
        Enemy::new(String::from("Mutant"), 50, 50, 15),
    ];

    game_loop(&mut player, &mut enemies);
}
