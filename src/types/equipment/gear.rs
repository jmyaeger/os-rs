//! The `Gear` type, which contains all of a `Player`'s equipped items. This
//! module also includes a builder for `Gear`.
use crate::constants;
use crate::error::GearError;
use crate::types::equipment::Equipment;
use crate::types::equipment::armor::Armor;
use crate::types::equipment::weapon::Weapon;
use serde::{Deserialize, Serialize};
use std::string::ToString;
use strum_macros::Display;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Gear {
    pub head: Option<Armor>,
    pub neck: Option<Armor>,
    pub cape: Option<Armor>,
    pub ammo: Option<Armor>,
    pub second_ammo: Option<Armor>,
    pub weapon: Weapon, // Default to unarmed, which is still a weapon
    pub shield: Option<Armor>,
    pub body: Option<Armor>,
    pub legs: Option<Armor>,
    pub hands: Option<Armor>,
    pub feet: Option<Armor>,
    pub ring: Option<Armor>,
}

impl Gear {
    pub fn is_wearing(&self, gear_name: &str, version: Option<&str>) -> bool {
        // Check if the player is wearing the specified piece of gear

        let matches = |opt: &Option<Armor>| -> bool {
            opt.as_ref()
                .is_some_and(|a| a.name == gear_name && a.version.as_deref() == version)
        };

        self.weapon.name == gear_name && self.weapon.version.as_deref() == version
            || matches(&self.head)
            || matches(&self.neck)
            || matches(&self.cape)
            || matches(&self.ammo)
            || (matches(&self.second_ammo) && self.is_wearing_any(constants::QUIVER_VARIANTS))
            || matches(&self.shield)
            || matches(&self.body)
            || matches(&self.legs)
            || matches(&self.feet)
            || matches(&self.hands)
            || matches(&self.ring)
    }

    pub fn is_wearing_any_version(&self, gear_name: &str) -> bool {
        // Same as is_wearing() but allows for any version to match
        self.weapon.name == gear_name
            || [
                &self.head,
                &self.neck,
                &self.cape,
                &self.ammo,
                &self.shield,
                &self.body,
                &self.legs,
                &self.feet,
                &self.hands,
                &self.ring,
            ]
            .iter()
            .filter_map(|slot| slot.as_ref())
            .any(|armor| armor.name == gear_name)
            || (self.is_wearing_any(constants::QUIVER_VARIANTS)
                && self
                    .second_ammo
                    .as_ref()
                    .is_some_and(|a| a.name() == gear_name))
    }

    pub fn is_wearing_any<I>(&self, gear_names: I) -> bool
    where
        I: IntoIterator<Item = (&'static str, Option<&'static str>)>,
    {
        // Check if the player is wearing any item in the provided Vec
        gear_names
            .into_iter()
            .any(|gear_name| self.is_wearing(gear_name.0, gear_name.1))
    }

    pub fn is_wearing_all<I>(&self, gear_names: I) -> bool
    where
        I: IntoIterator<Item = (&'static str, Option<&'static str>)>,
    {
        // Check if the player is wearing all items in the provided Vec
        gear_names
            .into_iter()
            .all(|gear_name| self.is_wearing(gear_name.0, gear_name.1))
    }

    pub fn is_quiver_bonus_valid(&self) -> bool {
        // Check if the player is wearing a quiver and using a weapon with bolts or arrows
        self.is_wearing_any(constants::QUIVER_VARIANTS)
            && self
                .cape
                .as_ref()
                .is_some_and(|c| c.version != Some("Uncharged".to_string()))
            && self
                .choose_compatible_ammo()
                .is_ok_and(|opt| opt.is_some_and(|ammo| ammo.is_bolt_or_arrow()))
    }

    pub fn builder() -> GearBuilder {
        GearBuilder::default()
    }
}

/// Builder for constructing `Gear` instances by item name and version.
///
/// # Example
/// ```
/// use osrs::types::equipment::Gear;
///
/// let gear = Gear::builder()
///     .head("Torva full helm", None)
///     .body("Torva platebody", None)
///     .legs("Torva platelegs", None)
///     .weapon("Osmumten's fang", None)
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct GearBuilder {
    head: Option<(String, Option<String>)>,
    neck: Option<(String, Option<String>)>,
    cape: Option<(String, Option<String>)>,
    ammo: Option<(String, Option<String>)>,
    second_ammo: Option<(String, Option<String>)>,
    weapon: Option<(String, Option<String>)>,
    shield: Option<(String, Option<String>)>,
    body: Option<(String, Option<String>)>,
    legs: Option<(String, Option<String>)>,
    hands: Option<(String, Option<String>)>,
    feet: Option<(String, Option<String>)>,
    ring: Option<(String, Option<String>)>,
}

impl GearBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the head slot item.
    pub fn head(mut self, name: &str, version: Option<&str>) -> Self {
        self.head = Some((name.to_string(), version.map(ToString::to_string)));
        self
    }

    /// Set the neck slot item.
    pub fn neck(mut self, name: &str, version: Option<&str>) -> Self {
        self.neck = Some((name.to_string(), version.map(ToString::to_string)));
        self
    }

    /// Set the cape slot item.
    pub fn cape(mut self, name: &str, version: Option<&str>) -> Self {
        self.cape = Some((name.to_string(), version.map(ToString::to_string)));
        self
    }

    /// Set the ammo slot item.
    pub fn ammo(mut self, name: &str, version: Option<&str>) -> Self {
        self.ammo = Some((name.to_string(), version.map(ToString::to_string)));
        self
    }

    /// Set the second ammo slot item (for quiver).
    pub fn second_ammo(mut self, name: &str, version: Option<&str>) -> Self {
        self.second_ammo = Some((name.to_string(), version.map(ToString::to_string)));
        self
    }

    /// Set the weapon slot item.
    pub fn weapon(mut self, name: &str, version: Option<&str>) -> Self {
        self.weapon = Some((name.to_string(), version.map(ToString::to_string)));
        self
    }

    /// Set the shield slot item.
    pub fn shield(mut self, name: &str, version: Option<&str>) -> Self {
        self.shield = Some((name.to_string(), version.map(ToString::to_string)));
        self
    }

    /// Set the body slot item.
    pub fn body(mut self, name: &str, version: Option<&str>) -> Self {
        self.body = Some((name.to_string(), version.map(ToString::to_string)));
        self
    }

    /// Set the legs slot item.
    pub fn legs(mut self, name: &str, version: Option<&str>) -> Self {
        self.legs = Some((name.to_string(), version.map(ToString::to_string)));
        self
    }

    /// Set the hands slot item.
    pub fn hands(mut self, name: &str, version: Option<&str>) -> Self {
        self.hands = Some((name.to_string(), version.map(ToString::to_string)));
        self
    }

    /// Set the feet slot item.
    pub fn feet(mut self, name: &str, version: Option<&str>) -> Self {
        self.feet = Some((name.to_string(), version.map(ToString::to_string)));
        self
    }

    /// Set the ring slot item.
    pub fn ring(mut self, name: &str, version: Option<&str>) -> Self {
        self.ring = Some((name.to_string(), version.map(ToString::to_string)));
        self
    }

    /// Build the `Gear` instance.
    pub fn build(self) -> Result<Gear, GearError> {
        let mut gear = Gear::default();

        if let Some((name, version)) = self.head {
            let head = Armor::new(&name, version.as_deref())?;
            validate_slot(&head, GearSlot::Head)?;
            gear.head = Some(head);
        }

        if let Some((name, version)) = self.neck {
            let neck = Armor::new(&name, version.as_deref())?;
            validate_slot(&neck, GearSlot::Neck)?;
            gear.neck = Some(neck);
        }

        if let Some((name, version)) = self.cape {
            let cape = Armor::new(&name, version.as_deref())?;
            validate_slot(&cape, GearSlot::Cape)?;
            gear.cape = Some(cape);
        }

        if let Some((name, version)) = self.ammo {
            let ammo = Armor::new(&name, version.as_deref())?;
            validate_slot(&ammo, GearSlot::Ammo)?;
            gear.ammo = Some(ammo);
        }

        if let Some((name, version)) = self.second_ammo {
            // Since the builder allows the second slot to be individually addressed
            // (unlike `Player::equip_item()`), the bolt/arrow requirement is enforced here
            let second_ammo = Armor::new(&name, version.as_deref())?;
            validate_slot(&second_ammo, GearSlot::Ammo)?;
            if !second_ammo.is_bolt_or_arrow() {
                return Err(GearError::WrongQuiverAmmo(second_ammo.name.clone()));
            }
            gear.second_ammo = Some(second_ammo);
        }

        if let Some((name, version)) = self.weapon {
            let weapon = Weapon::new(&name, version.as_deref())?;
            validate_slot(&weapon, GearSlot::Weapon)?;
            gear.weapon = weapon;
        }

        if let Some((name, version)) = self.shield {
            let shield = Armor::new(&name, version.as_deref())?;
            validate_slot(&shield, GearSlot::Shield)?;
            gear.shield = Some(shield);
        }

        if let Some((name, version)) = self.body {
            let body = Armor::new(&name, version.as_deref())?;
            validate_slot(&body, GearSlot::Body)?;
            gear.body = Some(body);
        }

        if let Some((name, version)) = self.legs {
            let legs = Armor::new(&name, version.as_deref())?;
            validate_slot(&legs, GearSlot::Legs)?;
            gear.legs = Some(legs);
        }

        if let Some((name, version)) = self.hands {
            let hands = Armor::new(&name, version.as_deref())?;
            validate_slot(&hands, GearSlot::Hands)?;
            gear.hands = Some(hands);
        }

        if let Some((name, version)) = self.feet {
            let feet = Armor::new(&name, version.as_deref())?;
            validate_slot(&feet, GearSlot::Feet)?;
            gear.feet = Some(feet);
        }

        if let Some((name, version)) = self.ring {
            let ring = Armor::new(&name, version.as_deref())?;
            validate_slot(&ring, GearSlot::Ring)?;
            gear.ring = Some(ring);
        }

        Ok(gear)
    }
}

fn validate_slot(item: &dyn Equipment, slot: GearSlot) -> Result<(), GearError> {
    if item.slot() == slot {
        Ok(())
    } else {
        Err(GearError::WrongSlot {
            item: item.name().to_string(),
            wrong: slot,
            right: item.slot(),
        })
    }
}

// Slots in which a player can equip gear
#[derive(Debug, PartialEq, Eq, Hash, Default, Deserialize, Serialize, Clone, Display, Copy)]
pub enum GearSlot {
    #[default]
    None,
    Head,
    Neck,
    Body,
    Legs,
    Hands,
    Feet,
    Ring,
    Ammo,
    SecondAmmo,
    Weapon,
    Shield,
    Cape,
}
