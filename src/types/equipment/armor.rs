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
    fn set_fields_from_json(
        &mut self,
        json: &str,
        item_name: &str,
        version: Option<&str>,
    ) -> Result<(), GearError> {
        let all_items: Vec<EquipmentJson> = serde_json::from_str(json)?;
        let version_string = version.map(ToString::to_string);
        let matched_item = all_items
            .into_iter()
            .find(|a| a.name == item_name && a.version == version_string)
            .ok_or(GearError::EquipmentNotFound {
                name: item_name.to_string(),
                version: version_string,
            })?;

        *self = matched_item.into_armor()?;

        Ok(())
    }

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

    pub fn is_valid_ranged_ammo(&self) -> bool {
        // Check if an ammo slot item can be used as ranged ammo
        !self.name.contains("blessing")
            && !["Ghommal's lucky penny", "Mith grapple", "Hallowed grapple"]
                .contains(&self.name.as_str())
            && self.slot == GearSlot::Ammo
    }

    pub fn is_bolt_or_arrow(&self) -> bool {
        self.is_bolt() || self.is_arrow()
    }

    pub fn is_bolt(&self) -> bool {
        self.name.to_lowercase().contains("bolt")
    }

    pub fn is_arrow(&self) -> bool {
        self.name.contains("arrow")
    }

    pub fn matches_version(&self, version: &str) -> bool {
        self.version.as_ref().is_some_and(|v| v.contains(version))
    }
}
