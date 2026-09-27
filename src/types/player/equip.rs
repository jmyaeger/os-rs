use super::Player;
use crate::constants;
use crate::error::GearError;
use crate::types::equipment::{Armor, CombatStyle, Equipment, EquipmentBonuses, GearSlot, Weapon};
use std::rc::Rc;

impl Player {
    pub fn equip(&mut self, item_name: &str, version: Option<&str>) -> Result<(), GearError> {
        if let Ok(armor) = Armor::new(item_name, version) {
            self.equip_item(Box::new(armor))?;
        } else if let Ok(weapon) = Weapon::new(item_name, version) {
            self.equip_item(Box::new(weapon))?;
        }

        self.update_bonuses();
        self.update_set_effects();

        Ok(())
    }

    pub fn unequip_slot(&mut self, slot: &GearSlot) {
        let gear = Rc::make_mut(&mut self.gear);
        match slot {
            GearSlot::Ammo => gear.ammo = None,
            GearSlot::Body => gear.body = None,
            GearSlot::Cape => gear.cape = None,
            GearSlot::Feet => gear.feet = None,
            GearSlot::Hands => gear.hands = None,
            GearSlot::Head => gear.head = None,
            GearSlot::Legs => gear.legs = None,
            GearSlot::Neck => gear.neck = None,
            GearSlot::Ring => gear.ring = None,
            GearSlot::SecondAmmo => gear.second_ammo = None,
            GearSlot::Shield => gear.shield = None,
            GearSlot::Weapon => {
                gear.weapon = Weapon::default();
                self.set_active_style(CombatStyle::Kick);
            }
            GearSlot::None => {}
        }
        self.update_bonuses();
        self.update_set_effects();
        self.combat_type = self.gear.weapon.combat_styles[&self.attrs.active_style].combat_type;
    }

    pub fn equip_item(&mut self, item: Box<dyn Equipment>) -> Result<(), GearError> {
        let mut weapon_changed = false;
        let gear = Rc::make_mut(&mut self.gear);
        let slot = item.slot();
        match slot {
            GearSlot::Weapon => {
                if let Some(weapon) = item.as_any().downcast_ref::<Weapon>() {
                    gear.weapon = weapon.clone();
                    weapon_changed = true;

                    // Unequip shield if weapon is two handed
                    if gear.weapon.is_two_handed {
                        gear.shield = None;
                    }

                    // Modify attack speed if weapon is on rapid
                    if self.attrs.active_style == CombatStyle::Rapid
                        && gear.weapon.combat_styles.contains_key(&CombatStyle::Rapid)
                    {
                        gear.weapon.speed = gear.weapon.base_speed - 1;
                    }
                } else {
                    return Err(GearError::NotAWeapon {
                        item_name: item.name().to_string(),
                        slot: item.slot().to_string(),
                    });
                }
            }
            GearSlot::Ammo => {
                let item = item.as_any().downcast_ref::<Armor>().cloned();
                if gear.is_wearing_any(constants::QUIVER_VARIANTS)
                    && let Some(ammo) = &gear.ammo
                    && let Some(ref item) = item
                {
                    if item.is_bolt_or_arrow() {
                        // Equip the new ammo to the quiver slot if it's a bolt or arrow
                        gear.second_ammo = Some(item.clone());
                    } else if ammo.is_bolt_or_arrow() {
                        // If it's not a bolt or arrow but the first slot is, move the
                        // first slot to the second slot and replace it with this ammo
                        gear.second_ammo = Some(ammo.clone());
                        gear.ammo = Some(item.clone());
                    } else {
                        // Otherwise, just replace the first ammo slot
                        gear.ammo = Some(item.clone());
                    }
                } else {
                    gear.ammo = item;
                }
            }
            GearSlot::Cape => {
                gear.cape = item.as_any().downcast_ref::<Armor>().cloned();
            }
            GearSlot::Shield => {
                gear.shield = item.as_any().downcast_ref::<Armor>().cloned();
                if gear.weapon.is_two_handed {
                    gear.weapon = Weapon::default();
                    weapon_changed = true;
                }
            }
            GearSlot::Body => gear.body = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Feet => gear.feet = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Hands => gear.hands = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Head => gear.head = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Legs => gear.legs = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Neck => gear.neck = item.as_any().downcast_ref::<Armor>().cloned(),
            GearSlot::Ring => gear.ring = item.as_any().downcast_ref::<Armor>().cloned(),
            // Items cannot have a "second ammo" specific slot
            GearSlot::SecondAmmo => unreachable!(),
            GearSlot::None => {
                return Err(GearError::NoneSlot(item.name().to_string()));
            }
        }

        if weapon_changed {
            self.set_active_style(self.gear.weapon.category.default_style());
        }

        self.update_bonuses();
        self.update_set_effects();
        Ok(())
    }

    pub fn equip_ammo_in_slot(&mut self, ammo: Armor, slot: GearSlot) -> Result<(), GearError> {
        let gear = Rc::make_mut(&mut self.gear);

        if ammo.slot != GearSlot::Ammo {
            return Err(GearError::WrongSlot {
                item: ammo.name.clone(),
                wrong: slot,
                right: ammo.slot,
            });
        }

        if slot == GearSlot::SecondAmmo && !ammo.is_bolt_or_arrow() {
            return Err(GearError::WrongQuiverAmmo(ammo.name.clone()));
        }

        if slot == GearSlot::Ammo {
            gear.ammo = Some(ammo);
        } else if slot == GearSlot::SecondAmmo {
            gear.second_ammo = Some(ammo);
        } else {
            return Err(GearError::WrongSlot {
                item: ammo.name.clone(),
                wrong: slot,
                right: GearSlot::Ammo,
            });
        }

        self.update_bonuses();
        self.update_set_effects();
        Ok(())
    }

    pub fn get_slot(&self, slot: &GearSlot) -> Option<Box<dyn Equipment>> {
        match slot {
            GearSlot::Weapon => Some(Box::new(self.gear.weapon.clone())),
            GearSlot::Ammo => self
                .gear
                .ammo
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Body => self
                .gear
                .body
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Cape => self
                .gear
                .cape
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Feet => self
                .gear
                .feet
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Hands => self
                .gear
                .hands
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Head => self
                .gear
                .head
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Legs => self
                .gear
                .legs
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Neck => self
                .gear
                .neck
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Ring => self
                .gear
                .ring
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::SecondAmmo => self
                .gear
                .second_ammo
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::Shield => self
                .gear
                .shield
                .as_ref()
                .map(|item| Box::new(item.clone()) as Box<dyn Equipment>),
            GearSlot::None => None,
        }
    }

    pub fn update_bonuses(&mut self) {
        // Update equipment bonuses based on the equipped items
        self.bonuses = EquipmentBonuses::default();

        for item in [
            &self.gear.head,
            &self.gear.neck,
            &self.gear.cape,
            &self.gear.shield,
            &self.gear.body,
            &self.gear.legs,
            &self.gear.hands,
            &self.gear.feet,
            &self.gear.ring,
        ]
        .into_iter()
        .flatten()
        {
            self.bonuses.add_bonuses(&item.bonuses);
        }

        // Add ammo ranged bonuses if applicable
        let included_ammo = self.gear.choose_compatible_ammo();
        if let Ok(included_ammo) = included_ammo
            && let Some(ammo) = included_ammo
        {
            self.bonuses.attack.ranged += ammo.bonuses.attack.ranged;
            self.bonuses.strength.ranged += ammo.bonuses.strength.ranged;
        }

        // Add prayer bonuses even if ammo is not compatible
        if let Some(main_ammo) = &self.gear.ammo {
            self.bonuses.prayer += main_ammo.bonuses.prayer;
        }
        if let Some(second_ammo) = &self.gear.second_ammo
            && self.is_wearing_quiver()
        {
            self.bonuses.prayer += second_ammo.bonuses.prayer;
        }

        // Add charged quiver bonuses if applicable
        if self.gear.is_quiver_bonus_valid() {
            self.bonuses.attack.ranged += 10;
            self.bonuses.strength.ranged += 1;
        }

        // Dinh's bulwark bonus is applied directly to gear strength bonus
        if self.is_wearing("Dinh's bulwark", None) && self.attrs.active_style == CombatStyle::Pummel
        {
            self.bonuses.strength.melee += self.bulwark_bonus();
        }

        self.bonuses.add_bonuses(&self.gear.weapon.bonuses);
    }

    pub fn update_set_effects(&mut self) {
        // Update status of all set effects at once
        self.set_effects.full_ahrims = self.is_wearing_all(constants::FULL_AHRIMS);
        self.set_effects.full_blood_moon = self.is_wearing_all(constants::FULL_BLOOD_MOON);
        self.set_effects.full_blue_moon = self.is_wearing_all(constants::FULL_BLUE_MOON);
        self.set_effects.full_dharoks = self.is_wearing_all(constants::FULL_DHAROKS);
        self.set_effects.full_guthans = self.is_wearing_all(constants::FULL_GUTHANS);
        self.set_effects.full_eclipse_moon = self.is_wearing_all(constants::FULL_ECLIPSE_MOON);
        self.set_effects.full_inquisitor = self.is_wearing_all(constants::FULL_INQUISITOR);
        self.set_effects.full_justiciar = self.is_wearing_all(constants::FULL_JUSTICIAR);
        self.set_effects.full_karils = self.is_wearing_all(constants::FULL_KARILS);
        self.set_effects.full_obsidian = self.is_wearing_all(constants::FULL_OBSIDIAN);
        self.set_effects.full_torags = self.is_wearing_all(constants::FULL_TORAGS);
        self.set_effects.full_void = self.is_wearing_full_void();
        self.set_effects.full_elite_void = self.is_wearing_full_elite_void();
        self.set_effects.bloodbark_pieces = constants::BLOODBARK_ARMOR
            .iter()
            .filter(|armor| self.is_wearing(armor.0, armor.1))
            .count();
    }
}
