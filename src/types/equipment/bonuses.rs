use serde::Deserialize;

use crate::types::equipment::styles::CombatType;

// Equipment stat bonuses for each combat style (generally used for accuracy/defense bonuses)
#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Deserialize)]
pub struct StyleBonus {
    pub stab: i32,
    pub slash: i32,
    pub crush: i32,
    pub ranged: i32,
    pub magic: i32,
}

impl StyleBonus {
    pub fn add_bonuses(&mut self, other: &StyleBonus) {
        // Add another set of bonuses to the totals
        self.stab += other.stab;
        self.slash += other.slash;
        self.crush += other.crush;
        self.ranged += other.ranged;
        self.magic += other.magic;
    }

    pub fn highest_style(&self) -> CombatType {
        [
            (self.stab, CombatType::Stab),
            (self.slash, CombatType::Slash),
            (self.crush, CombatType::Crush),
            (self.ranged, CombatType::Ranged),
            (self.magic, CombatType::Magic),
        ]
        .into_iter()
        .max_by_key(|&(bonus, _)| bonus)
        .map(|(_, combat_type)| combat_type)
        .expect("array should be non-empty")
    }
}

// Equipment strength bonuses for each primary style
#[derive(Debug, PartialEq, Default, Deserialize, Clone)]
pub struct StrengthBonus {
    pub melee: i32,
    pub ranged: i32,
    pub magic: f32,
}

impl StrengthBonus {
    pub fn add_bonuses(&mut self, other: &StrengthBonus) {
        // Add another set of bonuses to the totals
        self.melee += other.melee;
        self.ranged += other.ranged;
        self.magic += other.magic;
    }
}

// Collection of all equipment bonuses for an item
#[derive(Debug, Default, PartialEq, Deserialize, Clone)]
pub struct EquipmentBonuses {
    pub attack: StyleBonus,
    pub defence: StyleBonus,
    pub strength: StrengthBonus,
    pub prayer: i32,
}

impl EquipmentBonuses {
    pub fn add_bonuses(&mut self, other: &EquipmentBonuses) {
        // Add another set of bonuses to the totals of each type of bonus
        self.attack.add_bonuses(&other.attack);
        self.defence.add_bonuses(&other.defence);
        self.strength.add_bonuses(&other.strength);
        self.prayer += other.prayer;
    }
}
