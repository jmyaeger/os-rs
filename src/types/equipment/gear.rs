use crate::error::GearError;
use crate::types::equipment::armor::Armor;
use crate::types::equipment::weapon::Weapon;
use serde::Deserialize;
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
            || matches(&self.second_ammo)
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
        self.cape.as_ref().is_some_and(|cape| {
            cape.name == "Dizana's quiver"
                && cape.matches_version("Charged")
                && self.weapon.uses_bolts_or_arrows()
                && (self.ammo.as_ref().is_some_and(Armor::is_bolt_or_arrow)
                    || self
                        .second_ammo
                        .as_ref()
                        .is_some_and(Armor::is_bolt_or_arrow))
        })
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
            gear.head = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.neck {
            gear.neck = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.cape {
            gear.cape = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.ammo {
            gear.ammo = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.second_ammo {
            gear.second_ammo = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.weapon {
            gear.weapon = Weapon::new(&name, version.as_deref())?;
        }

        if let Some((name, version)) = self.shield {
            gear.shield = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.body {
            gear.body = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.legs {
            gear.legs = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.hands {
            gear.hands = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.feet {
            gear.feet = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.ring {
            gear.ring = Some(Armor::new(&name, version.as_deref())?);
        }

        Ok(gear)
    }
}

// Slots in which a player can equip gear
#[derive(Debug, PartialEq, Eq, Hash, Default, Deserialize, Clone, Display, Copy)]
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
    Weapon,
    Shield,
    Cape,
}
