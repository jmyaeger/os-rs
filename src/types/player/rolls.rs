//! Types for storing and tracking a `Player`'s combat rolls (maximum attack rolls,
//! maximum defence rolls, and maximum hit).
use crate::error::PlayerError;
use crate::types::equipment::CombatType;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PlayerAttRolls {
    stab: i32,
    slash: i32,
    crush: i32,
    light: i32,
    standard: i32,
    heavy: i32,
    magic: i32,
}

impl PlayerAttRolls {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, combat_type: CombatType) -> Result<i32, PlayerError> {
        match combat_type {
            CombatType::Stab => Ok(self.stab),
            CombatType::Slash => Ok(self.slash),
            CombatType::Crush => Ok(self.crush),
            CombatType::Light => Ok(self.light),
            CombatType::Standard => Ok(self.standard),
            CombatType::Heavy => Ok(self.heavy),
            CombatType::Ranged => Err(PlayerError::NoGenericRangedStyle),
            CombatType::Magic => Ok(self.magic),
            CombatType::None => Ok(0),
        }
    }

    pub fn set(&mut self, combat_type: CombatType, value: i32) -> Result<(), PlayerError> {
        match combat_type {
            CombatType::Stab => self.stab = value,
            CombatType::Slash => self.slash = value,
            CombatType::Crush => self.crush = value,
            CombatType::Light => self.light = value,
            CombatType::Standard => self.standard = value,
            CombatType::Heavy => self.heavy = value,
            CombatType::Ranged => return Err(PlayerError::NoGenericRangedStyle),
            CombatType::Magic => self.magic = value,
            CombatType::None => {}
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PlayerDefRolls {
    stab: i32,
    slash: i32,
    crush: i32,
    ranged: i32,
    magic: i32,
}

impl PlayerDefRolls {
    pub fn get(&self, combat_type: CombatType) -> i32 {
        match combat_type {
            CombatType::Stab => self.stab,
            CombatType::Slash => self.slash,
            CombatType::Crush => self.crush,
            CombatType::Ranged | CombatType::Light | CombatType::Standard | CombatType::Heavy => {
                self.ranged
            }
            CombatType::Magic => self.magic,
            CombatType::None => 0,
        }
    }

    pub fn set(&mut self, combat_type: CombatType, value: i32) {
        match combat_type {
            CombatType::Stab => self.stab = value,
            CombatType::Slash => self.slash = value,
            CombatType::Crush => self.crush = value,
            CombatType::Ranged | CombatType::Light | CombatType::Standard | CombatType::Heavy => {
                self.ranged = value;
            }
            CombatType::Magic => self.magic = value,
            CombatType::None => {}
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PlayerMaxHits {
    stab: u32,
    slash: u32,
    crush: u32,
    light: u32,
    standard: u32,
    heavy: u32,
    magic: u32,
}

impl PlayerMaxHits {
    pub fn get(&self, combat_type: CombatType) -> u32 {
        match combat_type {
            CombatType::Stab => self.stab,
            CombatType::Slash => self.slash,
            CombatType::Crush => self.crush,
            CombatType::Light => self.light,
            CombatType::Standard => self.standard,
            CombatType::Heavy => self.heavy,
            CombatType::Ranged => self.standard, // All ranged max hits are the same
            CombatType::Magic => self.magic,
            CombatType::None => 0,
        }
    }

    pub fn set(&mut self, combat_type: CombatType, value: u32) {
        match combat_type {
            CombatType::Stab => self.stab = value,
            CombatType::Slash => self.slash = value,
            CombatType::Crush => self.crush = value,
            CombatType::Light => self.light = value,
            CombatType::Standard => self.standard = value,
            CombatType::Heavy => self.heavy = value,
            CombatType::Ranged => {
                self.standard = value;
                self.light = value;
                self.heavy = value;
            }
            CombatType::Magic => self.magic = value,
            CombatType::None => {}
        }
    }
}
