//! The `Armor` type, which represents any non-weapon piece of equipment.
use std::{any::Any, fmt};

use serde::Deserialize;

use super::{Equipment, bonuses::EquipmentBonuses, gear::GearSlot, json::EquipmentJson};
use crate::error::GearError;

// Any equippable item that is not a weapon
#[derive(Debug, PartialEq, Default, Deserialize, Clone)]
pub struct Armor {
    pub name: String,
    pub version: Option<String>,
    pub id: i32,
    pub bonuses: EquipmentBonuses,
    pub slot: GearSlot,
    pub image: String,
}

impl Equipment for Armor {
    fn set_from_entry(&mut self, entry: EquipmentJson) -> Result<(), GearError> {
        *self = entry.into_armor()?;
        Ok(())
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_image_path(&self) -> &str {
        self.image.as_str()
    }

    fn slot(&self) -> GearSlot {
        self.slot
    }
}

impl fmt::Display for Armor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(version) = &self.version {
            write!(f, "{} ({})", self.name, version)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

impl Armor {
    pub fn new(name: &str, version: Option<&str>) -> Result<Self, GearError> {
        // Create a new Armor struct from item name and version (optional)
        let mut armor = Armor::default();
        armor.set_info(name, version)?;
        Ok(armor)
    }

    pub fn matches_version(&self, version: &str) -> bool {
        self.version.as_ref().is_some_and(|v| v.contains(version))
    }
}
