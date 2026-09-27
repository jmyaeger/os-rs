use crate::{
    constants,
    error::AmmoError,
    types::{
        equipment::{Armor, Gear, GearSlot, Weapon, canonical_item_id, weapon::WeaponCategory},
        player::Player,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AmmoType {
    // Standard ammo types
    Arrow(ArrowTier),
    Bolt(BoltTier),
    Javelin,

    // Separate because the dorg crossbow can't shoot these but can shoot
    // bolts in the same tier
    SilverBolt,
    GemTippedBolt(BoltTier),

    // Special ammo types
    AtlatlDart,
    HerbTar(TarType),
    TrainingArrow,
    OgreAmmo(OgreTier),
    AntlerBolt,
    BoneBolt,
    BoltRack,
    KebbitBolt,
    NotAmmo,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArrowTier {
    BronzeAndIron,
    Steel,
    Mithril,
    Adamant,
    Rune,
    AmethystAndBroad,
    Dragon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BoltTier {
    Bronze,
    Blurite,
    Iron,
    Steel,
    Mithril,
    Adamant,
    RuniteAndBroad,
    Dragon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnchantedBoltType {
    Opal,
    Jade,
    Pearl,
    Topaz,
    Sapphire,
    Emerald,
    Ruby,
    Diamond,
    Dragonstone,
    Onyx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OgreTier {
    Bronze,
    Iron,
    Steel,
    Black,
    Mithril,
    Adamant,
    Rune,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TarType {
    Guam,
    Marrentill,
    Tarromin,
    Harralander,
    Irit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmmoCompatibility {
    // Ammo is used by the weapon, and ranged bonuses are applied
    Compatible,
    // Weapon can still be used with this ammo equipped, but the bonuses
    // are ignored
    Ignored,
    Incompatible,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmmoRequirement {
    None, // non-ranged weapons
    OwnAmmo,
    ArrowsUpTo(ArrowTier),
    BoltsUpTo(BoltTier),
    OgreAmmoUpTo(OgreTier),
    Exact(AmmoType),
    OneOf(&'static [AmmoType]),
    Unsupported,
}

impl AmmoRequirement {
    pub fn compatibility(&self, ammo_type: AmmoType) -> AmmoCompatibility {
        match (self, ammo_type) {
            (Self::OwnAmmo | &Self::None, _) => AmmoCompatibility::Ignored,
            (Self::Unsupported, _) | (_, AmmoType::Unsupported) => AmmoCompatibility::Unsupported,
            (_, AmmoType::NotAmmo) => AmmoCompatibility::Incompatible,
            (Self::ArrowsUpTo(max_tier), AmmoType::Arrow(tier)) => {
                if tier <= *max_tier {
                    AmmoCompatibility::Compatible
                } else {
                    AmmoCompatibility::Incompatible
                }
            }
            (Self::BoltsUpTo(max_tier), AmmoType::Bolt(tier) | AmmoType::GemTippedBolt(tier)) => {
                if tier <= *max_tier {
                    AmmoCompatibility::Compatible
                } else {
                    AmmoCompatibility::Incompatible
                }
            }
            (Self::BoltsUpTo(max_tier), AmmoType::SilverBolt) => {
                if *max_tier < BoltTier::Iron {
                    AmmoCompatibility::Incompatible
                } else {
                    AmmoCompatibility::Compatible
                }
            }
            (Self::Exact(exact), _) => {
                if *exact == ammo_type {
                    AmmoCompatibility::Compatible
                } else {
                    AmmoCompatibility::Incompatible
                }
            }
            (Self::OgreAmmoUpTo(max_tier), AmmoType::OgreAmmo(tier)) => {
                if tier <= *max_tier {
                    AmmoCompatibility::Compatible
                } else {
                    AmmoCompatibility::Incompatible
                }
            }
            (Self::OneOf(options), _) => {
                if options.contains(&ammo_type) {
                    AmmoCompatibility::Compatible
                } else {
                    AmmoCompatibility::Incompatible
                }
            }
            _ => AmmoCompatibility::Incompatible,
        }
    }
}

impl Armor {
    pub fn ammo_type(&self) -> Option<AmmoType> {
        if self.slot != GearSlot::Ammo {
            return None;
        }

        let ammo_type = match self.id {
            // Arrows
            882 | 883 | 5616 | 5622 | 598 | 942 | 33553 | 884 | 885 | 5617 | 5623 | 2532 | 2533
            | 33559 | 22227 | 22228 | 22229 | 22230 => AmmoType::Arrow(ArrowTier::BronzeAndIron),
            886 | 887 | 5618 | 5624 | 2534 | 2535 | 33565 => AmmoType::Arrow(ArrowTier::Steel),
            888 | 889 | 5619 | 5625 | 2536 | 2537 | 33571 => AmmoType::Arrow(ArrowTier::Mithril),
            890 | 891 | 5620 | 5626 | 2538 | 2539 | 33577 => AmmoType::Arrow(ArrowTier::Adamant),
            892 | 893 | 5621 | 5627 | 78 | 2540 | 2541 | 33583 => AmmoType::Arrow(ArrowTier::Rune),
            21326 | 21332 | 21334 | 21336 | 4160 | 21328 | 21330 | 33589 | 33601 => {
                AmmoType::Arrow(ArrowTier::AmethystAndBroad)
            }
            11212 | 11227 | 11228 | 11229 | 11217 | 11222 | 33595 => {
                AmmoType::Arrow(ArrowTier::Dragon)
            }

            // Bolts
            877 | 878 | 881 | 6061 | 6062 => AmmoType::Bolt(BoltTier::Bronze),
            879 | 9236 => AmmoType::GemTippedBolt(BoltTier::Bronze),
            9139 | 9286 | 9293 | 9300 => AmmoType::Bolt(BoltTier::Blurite),
            9335 | 9237 => AmmoType::GemTippedBolt(BoltTier::Blurite),
            9140 | 9287 | 9294 | 9301 => AmmoType::Bolt(BoltTier::Iron),
            880 | 9238 => AmmoType::GemTippedBolt(BoltTier::Iron),
            9145 | 9292 | 9299 | 9306 => AmmoType::SilverBolt,
            9141 | 9288 | 9295 | 9302 => AmmoType::Bolt(BoltTier::Steel),
            9336 | 9239 => AmmoType::GemTippedBolt(BoltTier::Steel),
            9142 | 9289 | 9296 | 9303 => AmmoType::Bolt(BoltTier::Mithril),
            9337 | 9240 | 9338 | 9241 => AmmoType::GemTippedBolt(BoltTier::Mithril),
            9143 | 9290 | 9297 | 9304 => AmmoType::Bolt(BoltTier::Adamant),
            9339 | 9242 | 9340 | 9243 => AmmoType::GemTippedBolt(BoltTier::Adamant),
            9144 | 9291 | 9298 | 9305 | 11875 | 21316 => AmmoType::Bolt(BoltTier::RuniteAndBroad),
            9341 | 9244 | 9342 | 9245 => AmmoType::GemTippedBolt(BoltTier::RuniteAndBroad),
            21905 | 21924 | 21926 | 21928 => AmmoType::Bolt(BoltTier::Dragon),
            21955 | 21932 | 21957 | 21934 | 21959 | 21936 | 21961 | 21938 | 21963 | 21940
            | 21965 | 21942 | 21967 | 21944 | 21969 | 21946 | 21971 | 21948 | 21973 | 21950 => {
                AmmoType::GemTippedBolt(BoltTier::Dragon)
            }

            // Javelins
            825 | 831 | 5642 | 5648 | 826 | 832 | 5643 | 5649 | 827 | 833 | 5644 | 5650 | 828
            | 834 | 5645 | 5651 | 829 | 835 | 5646 | 5652 | 830 | 836 | 5647 | 5653 | 21318
            | 21320 | 21322 | 21324 | 19484 | 19486 | 19488 | 19490 => AmmoType::Javelin,

            // Training arrows
            9706 => AmmoType::TrainingArrow,

            // Ogre and brutal arrows
            2866 | 4773 => AmmoType::OgreAmmo(OgreTier::Bronze),
            4778 => AmmoType::OgreAmmo(OgreTier::Iron),
            4783 => AmmoType::OgreAmmo(OgreTier::Steel),
            4788 => AmmoType::OgreAmmo(OgreTier::Black),
            4793 => AmmoType::OgreAmmo(OgreTier::Mithril),
            4798 => AmmoType::OgreAmmo(OgreTier::Adamant),
            4803 => AmmoType::OgreAmmo(OgreTier::Rune),

            // Bone bolts
            8882 => AmmoType::BoneBolt,

            // Kebbit bolts
            10158 | 10159 => AmmoType::KebbitBolt,

            // Antler bolts
            28872 | 28878 => AmmoType::AntlerBolt,

            // Bolt racks
            4740 => AmmoType::BoltRack,

            // Herb tars
            10142 => AmmoType::HerbTar(TarType::Guam),
            10143 => AmmoType::HerbTar(TarType::Marrentill),
            10144 => AmmoType::HerbTar(TarType::Tarromin),
            10145 => AmmoType::HerbTar(TarType::Harralander),
            28837 => AmmoType::HerbTar(TarType::Irit),

            // Atlatl darts
            28991 => AmmoType::AtlatlDart,

            // Blessings
            20220 | 20223 | 20226 | 20229 | 20232 | 20235 | 22941 | 22943 | 22945 | 22947 => {
                AmmoType::NotAmmo
            }

            // Grapples
            9419 | 24721 => AmmoType::NotAmmo,

            _ => AmmoType::Unsupported,
        };
        Some(ammo_type)
    }

    pub fn is_arrow(&self) -> bool {
        // TODO: verify in-game that all of these actually work with the quiver
        matches!(
            self.ammo_type(),
            Some(AmmoType::Arrow(_)) | Some(AmmoType::OgreAmmo(_)) | Some(AmmoType::TrainingArrow)
        )
    }

    pub fn is_bolt(&self) -> bool {
        // TODO: verify in-game that all of these actually work with the quiver
        matches!(
            self.ammo_type(),
            Some(AmmoType::Bolt(_))
                | Some(AmmoType::GemTippedBolt(_))
                | Some(AmmoType::KebbitBolt)
                | Some(AmmoType::AntlerBolt)
                | Some(AmmoType::BoneBolt)
                | Some(AmmoType::SilverBolt)
                | Some(AmmoType::BoltRack)
        )
    }

    pub fn is_bolt_or_arrow(&self) -> bool {
        self.is_arrow() || self.is_bolt()
    }
}

impl Weapon {
    /// Determine the type of ammo required for a ranged weapon
    pub fn ammo_requirement(&self) -> AmmoRequirement {
        match canonical_item_id(self.id) {
            11708 | 23357 | 841 | 839 => AmmoRequirement::ArrowsUpTo(ArrowTier::BronzeAndIron),
            9705 => AmmoRequirement::Exact(AmmoType::TrainingArrow),
            843 | 845 | 4236 => AmmoRequirement::ArrowsUpTo(ArrowTier::Steel),
            849 | 847 | 10280 => AmmoRequirement::ArrowsUpTo(ArrowTier::Mithril),
            853 | 851 => AmmoRequirement::ArrowsUpTo(ArrowTier::Adamant),
            2883 => AmmoRequirement::OgreAmmoUpTo(OgreTier::Mithril),
            4827 => AmmoRequirement::OgreAmmoUpTo(OgreTier::Rune),
            857 | 855 | 10282 => AmmoRequirement::ArrowsUpTo(ArrowTier::Rune),
            28794 | 6724 | 861 | 12788 | 859 | 10284 => {
                AmmoRequirement::ArrowsUpTo(ArrowTier::AmethystAndBroad)
            }
            11235 | 27853 | 12424 | 27610 | 27612 | 20997 | 29591 | 33245 => {
                AmmoRequirement::ArrowsUpTo(ArrowTier::Dragon)
            }
            837 | 767 | 9174 => AmmoRequirement::BoltsUpTo(BoltTier::Bronze),
            9176 => AmmoRequirement::BoltsUpTo(BoltTier::Blurite),
            9177 => AmmoRequirement::BoltsUpTo(BoltTier::Iron),
            9179 => AmmoRequirement::BoltsUpTo(BoltTier::Steel),
            9181 => AmmoRequirement::BoltsUpTo(BoltTier::Mithril),
            9183 => AmmoRequirement::BoltsUpTo(BoltTier::Adamant),
            9185 => AmmoRequirement::BoltsUpTo(BoltTier::RuniteAndBroad),
            21902 | 21012 | 11785 | 26374 | 33251 => AmmoRequirement::BoltsUpTo(BoltTier::Dragon),
            19478 | 19481 => AmmoRequirement::Exact(AmmoType::Javelin),
            8880 => AmmoRequirement::OneOf(&[
                AmmoType::Bolt(BoltTier::Bronze),
                AmmoType::Bolt(BoltTier::Blurite),
                AmmoType::Bolt(BoltTier::Iron),
                AmmoType::BoneBolt,
            ]),
            10156 => AmmoRequirement::Exact(AmmoType::KebbitBolt),
            4734 => AmmoRequirement::Exact(AmmoType::BoltRack),
            12924 | 12926 | 22547 | 22550 | 23983 | 23985 | 24123 | 27652 | 27655 | 25862
            | 25865 | 25867 | 23901 | 23902 | 23903 | 23855 | 23856 | 23857 => {
                AmmoRequirement::OwnAmmo
            }
            10149 => AmmoRequirement::Exact(AmmoType::HerbTar(TarType::Guam)),
            10146 => AmmoRequirement::Exact(AmmoType::HerbTar(TarType::Marrentill)),
            10147 => AmmoRequirement::Exact(AmmoType::HerbTar(TarType::Tarromin)),
            10148 => AmmoRequirement::Exact(AmmoType::HerbTar(TarType::Harralander)),
            28834 => AmmoRequirement::Exact(AmmoType::HerbTar(TarType::Irit)),
            28869 => AmmoRequirement::Exact(AmmoType::AntlerBolt),
            29000 => AmmoRequirement::Exact(AmmoType::AtlatlDart),
            _ if matches!(
                self.category,
                WeaponCategory::Thrown | WeaponCategory::Chinchompas | WeaponCategory::Blaster
            ) =>
            {
                AmmoRequirement::OwnAmmo
            }
            _ if self.is_ranged_weapon() => AmmoRequirement::Unsupported,
            _ => AmmoRequirement::None,
        }
    }

    /// Find the compatibility of the weapon with a specific piece of ammo
    pub fn ammo_compatibility(&self, ammo: Option<&Armor>) -> AmmoCompatibility {
        if let Some(ammo) = ammo {
            if let Some(ammo_type) = ammo.ammo_type() {
                self.ammo_requirement().compatibility(ammo_type)
            } else {
                AmmoCompatibility::Ignored
            }
        } else if [AmmoRequirement::None, AmmoRequirement::OwnAmmo]
            .contains(&self.ammo_requirement())
        {
            AmmoCompatibility::Ignored
        } else {
            AmmoCompatibility::Incompatible
        }
    }

    /// Convenience method for determining whether the weapon is not incompatible with the ammo
    pub fn is_ammo_allowed(&self, ammo: Option<&Armor>) -> bool {
        [AmmoCompatibility::Compatible, AmmoCompatibility::Ignored]
            .contains(&self.ammo_compatibility(ammo))
    }
}

impl Gear {
    pub fn choose_compatible_ammo(&self) -> Result<Option<&Armor>, AmmoError> {
        // Check if the weapon needs ammo
        if matches!(
            self.weapon.ammo_requirement(),
            AmmoRequirement::None | AmmoRequirement::OwnAmmo
        ) {
            return Ok(None);
        }

        // Check to see if ammo is equipped at all
        if (self.ammo.is_none() && self.second_ammo.is_none())
            || (self.ammo.is_none() && !self.is_wearing_any(constants::QUIVER_VARIANTS))
        {
            // Prioritize reporting an unsupported weapon over empty ammo slots
            if self.weapon.ammo_requirement() == AmmoRequirement::Unsupported {
                return Err(AmmoError::Unsupported {
                    ammo1: "empty".to_string(),
                    ammo2: "empty".to_string(),
                    weapon: self.weapon.name.clone(),
                });
            }
            return Err(AmmoError::NoAmmoEquipped(self.weapon.name.clone()));
        }

        let wearing_quiver = self.is_wearing_any(constants::QUIVER_VARIANTS);
        let main = self.weapon.ammo_compatibility(self.ammo.as_ref());
        let second = self.weapon.ammo_compatibility(self.second_ammo.as_ref());

        if main == AmmoCompatibility::Compatible {
            return Ok(self.ammo.as_ref());
        }

        if wearing_quiver && second == AmmoCompatibility::Compatible {
            return Ok(self.second_ammo.as_ref());
        }

        let main_name = self
            .ammo
            .as_ref()
            .map_or("empty", |ammo| ammo.name.as_str())
            .to_owned();
        let second_name = self
            .second_ammo
            .as_ref()
            .map_or("empty", |ammo| ammo.name.as_str())
            .to_owned();
        let weapon_name = self.weapon.name.clone();

        match (
            main,
            second,
            self.is_wearing_any(constants::QUIVER_VARIANTS),
        ) {
            // First two branches return before reaching this match statement
            (AmmoCompatibility::Compatible, _, _) => unreachable!(),
            (_, AmmoCompatibility::Compatible, true) => unreachable!(),
            (AmmoCompatibility::Ignored, AmmoCompatibility::Ignored, true)
            | (AmmoCompatibility::Ignored, _, false) => Ok(None),
            (AmmoCompatibility::Incompatible, AmmoCompatibility::Incompatible, true) => {
                Err(AmmoError::InvalidAmmoBothSlots {
                    ammo1: main_name,
                    ammo2: second_name,
                    weapon: weapon_name,
                })
            }
            (AmmoCompatibility::Incompatible, _, false) => Err(AmmoError::InvalidAmmo {
                ammo: main_name,
                weapon: weapon_name,
            }),
            (AmmoCompatibility::Unsupported, AmmoCompatibility::Unsupported, true)
            | (AmmoCompatibility::Unsupported, _, false) => Err(AmmoError::Unsupported {
                ammo1: main_name,
                ammo2: second_name,
                weapon: weapon_name,
            }),
            // Not possible for only one ammo type to be ignored, unless one ammo type is unsupported
            // (which is covered in the previous case)
            (AmmoCompatibility::Ignored, _, _) | (_, AmmoCompatibility::Ignored, _) => {
                unreachable!()
            }
            // This is probably never going to be reached, so I don't think it's worth making a special
            // error variant for having one incompatible ammo type and one unsupported ammo type with a quiver
            _ => Err(AmmoError::Unsupported {
                ammo1: main_name,
                ammo2: second_name,
                weapon: weapon_name,
            }),
        }
    }
}

impl Player {
    /// Evaluate whether the player has valid ammo equipped for their current weapon
    pub fn validate_ammo(&self) -> Result<(), AmmoError> {
        let _ = self.gear.choose_compatible_ammo()?;
        Ok(())
    }

    /// Determine whether the player is actively using a particular enchanted bolt type
    pub fn is_using_enchanted_bolt(&self, bolt_type: EnchantedBoltType) -> bool {
        let active_ammo = self.gear.choose_compatible_ammo();

        if let Ok(opt) = active_ammo
            && let Some(ammo) = opt
        {
            let ammo_label = (
                ammo.name.as_str(),
                ammo.version.as_deref(),
            );
            match bolt_type {
                EnchantedBoltType::Diamond => constants::DIAMOND_BOLTS.contains(&ammo_label),
                EnchantedBoltType::Dragonstone => {
                    constants::DRAGONSTONE_BOLTS.contains(&ammo_label)
                }
                EnchantedBoltType::Jade => constants::JADE_BOLTS.contains(&ammo_label),
                EnchantedBoltType::Emerald => constants::EMERALD_BOLTS.contains(&ammo_label),
                EnchantedBoltType::Onyx => constants::ONYX_BOLTS.contains(&ammo_label),
                EnchantedBoltType::Opal => constants::OPAL_BOLTS.contains(&ammo_label),
                EnchantedBoltType::Pearl => constants::PEARL_BOLTS.contains(&ammo_label),
                EnchantedBoltType::Ruby => constants::RUBY_BOLTS.contains(&ammo_label),
                EnchantedBoltType::Sapphire => constants::SAPPHIRE_BOLTS.contains(&ammo_label),
                EnchantedBoltType::Topaz => constants::TOPAZ_BOLTS.contains(&ammo_label),
            }
        } else {
            false
        }
    }

    pub fn is_firing_ammo(&self, name: &str) -> bool {
        self.gear
            .choose_compatible_ammo()
            .is_ok_and(|opt| opt.is_some_and(|ammo| ammo.name == name))
    }
}
