pub enum BattleEvent {
    EnemyAppeared {
        name: String,
    },
    Attack {
        attacker: String,
        target: String,
        damage: u32,
    },
    PotionUsed {
        name: String,
        healed: u32,
    },

    NoPotions,

    Defeated {
        name: String,
    },
    PlayerRan {
        name: String,
    },
    Victory {
        name: String,
    },
}
