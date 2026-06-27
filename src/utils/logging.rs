use serde::{Deserialize, Serialize};

use crate::types::monster::{AttackType, Monster};
use crate::types::player::{Player, SwitchType};

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    PlayerAttack {
        player_id: PlayerFightId,
        success: bool,
        damage: u32,
    },
    PlayerSpec {
        player_id: PlayerFightId,
        success: bool,
        damage: u32,
        switch_type: SwitchType,
    },
    PlayerDamaged {
        player_id: PlayerFightId,
        damage: u32,
    },
    ThrallAttack {
        player_id: PlayerFightId,
        damage: u32,
    },
    MonsterAttack {
        monster_id: MonsterFightId,
        success: bool,
        damage: u32,
        style: Option<AttackType>,
    },
    MonsterDamaged {
        monster_id: MonsterFightId,
        damage: u32,
    },
    GearSwitch {
        player_id: PlayerFightId,
        switch_type: SwitchType,
    },
    FoodEaten {
        player_id: PlayerFightId,
        heal_amount: u32,
    },
    PlayerHpRegen {
        player_id: PlayerFightId,
        amount: u32,
    },
    MonsterHpRegen {
        monster_id: MonsterFightId,
        amount: u32,
    },
    PlayerStatsRegen {
        player_id: PlayerFightId,
        amount: u32,
    },
    MonsterStatsRegen {
        monster_id: MonsterFightId,
        amount: u32,
    },
    MonsterDeath {
        monster_id: MonsterFightId,
    },
    PlayerDeath {
        player_id: PlayerFightId,
    },
    MonsterEffectDamage {
        monster_id: MonsterFightId,
        damage: u32,
    },
    MonsterFrozen {
        monster_id: MonsterFightId,
    },
    MonsterFreezeEnded {
        monster_id: MonsterFightId,
    },
    RedemptionProc {
        player_id: PlayerFightId,
        heal_amount: u32,
    },
    BloodFuryHeal {
        player_id: PlayerFightId,
        heal_amount: u32,
    },
    MonsterRecoilDamage {
        monster_id: MonsterFightId,
        damage: u32,
    },
    PlayerRegenSpecEnergy {
        player_id: PlayerFightId,
        amount: u8,
    },
    MonsterHeal {
        monster_id: MonsterFightId,
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
    pub player_states: Vec<PlayerSnapshot>,
    pub monster_states: Vec<MonsterSnapshot>,
}

impl Event {
    pub fn new(
        tick: i32,
        event_type: EventType,
        player_states: Vec<PlayerSnapshot>,
        monster_states: Vec<MonsterSnapshot>,
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
    pub initial_player_states: Vec<PlayerSnapshot>,
    pub initial_monster_states: Vec<MonsterSnapshot>,
    pub events: Vec<Event>,
}

impl FightLog {
    pub fn new(
        initial_player_states: Vec<PlayerSnapshot>,
        initial_monster_states: Vec<MonsterSnapshot>,
    ) -> Self {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct PlayerFightId(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct MonsterFightId(pub i32);

#[derive(Debug, Clone)]
pub struct PlayerSnapshot {
    pub id: PlayerFightId,
    pub state: Player,
}

impl PlayerSnapshot {
    pub fn new(player: &Player) -> Self {
        Self {
            id: player.fight_id(),
            state: player.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonsterSnapshot {
    pub id: MonsterFightId,
    pub state: Monster,
}

impl MonsterSnapshot {
    pub fn new(monster: &Monster) -> Self {
        Self {
            id: monster.fight_id(),
            state: monster.clone(),
        }
    }
}

#[derive(Debug)]
pub enum FightRecorder<'a> {
    Disabled,
    Enabled(&'a mut FightLog),
}

impl FightRecorder<'_> {
    pub fn record(
        &mut self,
        tick: i32,
        event_type: EventType,
        player_states: Vec<PlayerSnapshot>,
        monster_states: Vec<MonsterSnapshot>,
    ) {
        let Self::Enabled(log) = self else {
            return;
        };

        log.add_event(Event {
            tick,
            event_type,
            player_states,
            monster_states,
        });
    }
}

#[derive(Debug, Default)]
pub struct FightLogs {
    pub logs: Vec<FightLog>,
}
