//! Types and methods for deserializing equipment info from the JSON database.
use serde::Deserialize;
use std::sync::LazyLock;

use crate::{
    constants,
    error::GearError,
    types::equipment::{
        armor::Armor,
        bonuses::EquipmentBonuses,
        gear::GearSlot,
        weapon::{Weapon, WeaponCategory, WeaponSpeed},
    },
};

const EQUIPMENT_JSON_STR: &str = include_str!(concat!(env!("OUT_DIR"), "/equipment.json"));

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
    pub speed: Option<u8>,
    pub category: Option<WeaponCategory>,
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

        let Some(category) = self.category else {
            return Err(GearError::MissingWeaponCategory(self.name));
        };

        let combat_styles = Weapon::get_styles_from_weapon_category(category);

        let speed = self
            .speed
            .ok_or(GearError::MissingWeaponSpeed(self.name.clone()))?;
        let attack_range = self
            .attack_range
            .ok_or(GearError::MissingAttackRange(self.name.clone()))?;
        let is_two_handed = self
            .is_two_handed
            .ok_or(GearError::MissingTwoHandedField(self.name.clone()))?;
        let is_staff = matches!(
            category,
            WeaponCategory::BladedStaff | WeaponCategory::Staff
        );
        let spec_cost = constants::SPEC_COSTS
            .iter()
            .find(|w| w.0 == self.name)
            .map(|c| c.1);

        let weapon = Weapon {
            name: self.name,
            version: self.version,
            id: self.id,
            bonuses: self.bonuses,
            slot: GearSlot::Weapon,
            speed: WeaponSpeed::new(speed),
            attack_range,
            is_two_handed,
            spec_cost,
            poison_severity: 0,
            combat_styles,
            is_staff,
            image: self.image,
            category,
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
