use serde::Deserialize;
use std::sync::LazyLock;

use crate::{
    error::GearError,
    types::equipment::{armor::Armor, bonuses::EquipmentBonuses, gear::GearSlot, weapon::Weapon},
};

const EQUIPMENT_JSON_STR: &str = include_str!("../../databases/equipment.json");

static EQUIPMENT: LazyLock<Vec<EquipmentJson>> = LazyLock::new(|| {
    serde_json::from_str(EQUIPMENT_JSON_STR).expect("Bundled equipment JSON is invalid.")
});

pub fn all_equipment() -> &'static [EquipmentJson] {
    &EQUIPMENT
}

// Intermediate struct for JSON deserialization
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct EquipmentJson {
    pub name: String,
    pub version: Option<String>,
    pub id: i32,
    pub slot: String,
    pub image: String,
    pub speed: Option<i32>,
    pub category: Option<String>,
    pub bonuses: EquipmentBonuses,
    pub is_two_handed: Option<bool>,
    pub attack_range: Option<i8>,
}

impl EquipmentJson {
    pub fn into_weapon(self) -> Result<Weapon, GearError> {
        if self.slot != "weapon" {
            return Err(GearError::NotAWeapon {
                item_name: self.name,
                slot: self.slot,
            });
        }

        let combat_styles = match self.category {
            Some(category) => Weapon::get_styles_from_weapon_type(&category),
            None => return Err(GearError::MissingWeaponCategory(self.name)),
        };

        let speed = self
            .speed
            .ok_or(GearError::MissingWeaponSpeed(self.name.clone()))?;
        let attack_range = self
            .attack_range
            .ok_or(GearError::MissingAttackRange(self.name.clone()))?;
        let is_two_handed = self
            .is_two_handed
            .ok_or(GearError::MissingTwoHandedField(self.name.clone()))?;

        let weapon = Weapon {
            name: self.name,
            version: self.version,
            id: self.id,
            bonuses: self.bonuses,
            slot: GearSlot::Weapon,
            speed,
            base_speed: speed,
            attack_range,
            is_two_handed,
            spec_cost: None,
            poison_severity: 0,
            combat_styles,
            is_staff: false,
            image: self.image,
        };

        Ok(weapon)
    }

    pub fn into_armor(self) -> Result<Armor, GearError> {
        if self.slot == "weapon" {
            return Err(GearError::NotArmor(self.name));
        }

        Ok(Armor {
            name: self.name,
            version: self.version,
            id: self.id,
            bonuses: self.bonuses,
            slot: parse_gear_slot(self.slot)?,
            image: self.image,
        })
    }
}

fn parse_gear_slot(slot: String) -> Result<GearSlot, GearError> {
    // Translate a gear slot string into an enum

    let trimmed = slot.replace('\"', "");

    match trimmed.as_str() {
        "head" => Ok(GearSlot::Head),
        "neck" => Ok(GearSlot::Neck),
        "cape" => Ok(GearSlot::Cape),
        "body" => Ok(GearSlot::Body),
        "legs" => Ok(GearSlot::Legs),
        "shield" => Ok(GearSlot::Shield),
        "feet" => Ok(GearSlot::Feet),
        "hands" => Ok(GearSlot::Hands),
        "ring" => Ok(GearSlot::Ring),
        "ammo" => Ok(GearSlot::Ammo),
        "weapon" => unreachable!(),
        _ => Err(GearError::UnknownSlot(slot)),
    }
}
