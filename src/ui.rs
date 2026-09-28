use crate::enemy::Enemy;
use crate::event::BattleEvent;
use crate::player::Player;
use crossterm::cursor::MoveTo;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;

pub fn render(player: &Player, enemy: Option<&Enemy>, log: &[BattleEvent]) {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    println!("========================================");
    println!("                MINI RPG");
    println!("========================================");
    println!();

    println!("PLAYER");
    println!("{}", player.name());
    println!("HP: {} / {}", player.health(), player.max_health());
    println!("Potions: {}", player.potions());
    println!();

    println!("ENEMY");
    match enemy {
        Some(enemy) => {
            println!("{}", enemy.name());
            println!("HP: {} / {}", enemy.health(), enemy.max_health());
        }
        None => {}
    }
    println!();

    println!("---------------BATTLE LOG---------------");
    let start = if log.len() > 5 { log.len() - 5 } else { 0 };
    let recent_log = &log[start..];
    for event in recent_log {
        match event {
            BattleEvent::EnemyAppeared { name } => println!("> {} appeared!", name),
            BattleEvent::Attack {
                attacker,
                target,
                damage,
            } => println!("> {} dealt {} damage to {}", attacker, damage, target),
            BattleEvent::PotionUsed { name, healed } => println!("> {} healed {} HP", name, healed),
            BattleEvent::NoPotions => println!("> No potions left!"),
            BattleEvent::Defeated { name } => println!("> {} is dead!", name),
            BattleEvent::PlayerRan { name } => println!("> {} runs away!", name),
            BattleEvent::Victory { name } => println!("> {} won!", name),
        }
    }

    println!("----------------------------------------");
    println!("1. Attack");
    println!("2. Heal");
    println!("3. Run");
}
