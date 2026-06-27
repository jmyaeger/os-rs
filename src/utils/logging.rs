use crate::types::monster::{AttackType, Monster};
use crate::types::player::{Player, SwitchType};

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    PlayerAttack {
        player_id: i32,
        success: bool,
        damage: u32,
    },
    PlayerSpec {
        player_id: i32,
        success: bool,
        damage: u32,
        switch_type: SwitchType,
    },
    PlayerDamaged {
        player_id: i32,
        damage: u32,
    },
    ThrallAttack {
        player_id: i32,
        damage: u32,
    },
    MonsterAttack {
        monster_id: i32,
        success: bool,
        damage: u32,
        style: Option<AttackType>,
    },
    MonsterDamaged {
        monster_id: i32,
        damage: u32,
    },
    GearSwitch {
        player_id: i32,
        switch_type: SwitchType,
    },
    FoodEaten {
        player_id: i32,
        heal_amount: u32,
    },
    PlayerHpRegen {
        player_id: i32,
        amount: u32,
    },
    MonsterHpRegen {
        monster_id: i32,
        amount: u32,
    },
    PlayerStatsRegen {
        player_id: i32,
        amount: u32,
    },
    MonsterStatsRegen {
        monster_id: i32,
        amount: u32,
    },
    MonsterDeath {
        monster_id: i32,
    },
    PlayerDeath {
        player_id: i32,
    },
    MonsterEffectDamage {
        monster_id: i32,
        damage: u32,
    },
    MonsterFrozen {
        monster_id: i32,
    },
    MonsterFreezeEnded {
        monster_id: i32,
    },
    RedemptionProc {
        player_id: i32,
        heal_amount: u32,
    },
    BloodFuryHeal {
        player_id: i32,
        heal_amount: u32,
    },
    MonsterRecoilDamage {
        monster_id: i32,
        damage: u32,
    },
    PlayerRegenSpecEnergy {
        player_id: i32,
        amount: u8,
    },
    MonsterHeal {
        monster_id: i32,
        amount: u32,
    },
    Custom {
        message: String,
    },
}

#[derive(Debug, Clone)]
pub struct Event {
    pub tick: i32,
    pub event_type: EventType,
    pub player_states: Vec<Player>,
    pub monster_states: Vec<Monster>,
}

impl Event {
    pub fn new(
        tick: i32,
        event_type: EventType,
        player_states: Vec<Player>,
        monster_states: Vec<Monster>,
    ) -> Self {
        Self {
            tick,
            event_type,
            player_states,
            monster_states,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FightLog {
    pub initial_player_states: Vec<Player>,
    pub initial_monster_states: Vec<Monster>,
    pub events: Vec<Event>,
}

impl FightLog {
    pub fn new(initial_player_states: Vec<Player>, initial_monster_states: Vec<Monster>) -> Self {
        Self {
            initial_player_states,
            initial_monster_states,
            events: vec![],
        }
    }

    pub fn empty() -> Self {
        Self {
            initial_player_states: vec![],
            initial_monster_states: vec![],
            events: vec![],
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }
}

#[derive(Debug, Clone, Default)]
pub struct FightLogs {
    pub logs: Vec<FightLog>,
}
