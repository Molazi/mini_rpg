use crate::enemy::Enemy;
use crate::event::BattleEvent;
use crate::player::Player;

pub fn render(player: &Player, enemy: Option<&Enemy>, log: &[BattleEvent]) {
    println!("PLAYER");
    println!("{}", player.name());
    println!("{} / {}", player.health(), player.max_health());
    println!("Potions: {}", player.potions());

    println!("ENEMY");
    match enemy {
        Some(enemy) => {
            println!("{}", enemy.name());
            println!("{} / {}", enemy.health(), enemy.max_health());
        }
        None => {}
    }

    println!("BATTLE LOG");
    for event in log {
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

    println!("MENU");
    println!("1. Attack");
    println!("2. Heal");
    println!("3. Run");
}
