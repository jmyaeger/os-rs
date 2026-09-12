use crate::types::equipment::{Armor, GearSlot, Weapon};

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
pub enum AmmoApplicability {
    // Ammo is compatible and ranged bonuses are applied
    Included,
    // Ammo is compatible, but ranged bonuses are ignored
    Allowed,
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
    pub fn applicability(&self, ammo_type: AmmoType) -> AmmoApplicability {
        match (self, ammo_type) {
            (Self::OwnAmmo | &Self::None, _) => AmmoApplicability::Allowed,
            (Self::Unsupported, _) | (_, AmmoType::Unsupported) => AmmoApplicability::Unsupported,
            (_, AmmoType::NotAmmo) => AmmoApplicability::Incompatible,
            (Self::ArrowsUpTo(max_tier), AmmoType::Arrow(tier)) => {
                if tier <= *max_tier {
                    AmmoApplicability::Included
                } else {
                    AmmoApplicability::Incompatible
                }
            }
            (Self::BoltsUpTo(max_tier), AmmoType::Bolt(tier) | AmmoType::GemTippedBolt(tier)) => {
                if tier <= *max_tier {
                    AmmoApplicability::Included
                } else {
                    AmmoApplicability::Incompatible
                }
            }
            (Self::BoltsUpTo(max_tier), AmmoType::SilverBolt) => {
                if *max_tier < BoltTier::Iron {
                    AmmoApplicability::Incompatible
                } else {
                    AmmoApplicability::Included
                }
            }
            (Self::Exact(exact), _) => {
                if *exact == ammo_type {
                    AmmoApplicability::Included
                } else {
                    AmmoApplicability::Incompatible
                }
            }
            (Self::OgreAmmoUpTo(max_tier), AmmoType::OgreAmmo(tier)) => {
                if tier <= *max_tier {
                    AmmoApplicability::Included
                } else {
                    AmmoApplicability::Incompatible
                }
            }
            (Self::OneOf(options), _) => {
                if options.contains(&ammo_type) {
                    AmmoApplicability::Included
                } else {
                    AmmoApplicability::Incompatible
                }
            }
            _ => AmmoApplicability::Incompatible,
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
            877 | 878 | 6061 | 6062 => AmmoType::Bolt(BoltTier::Bronze),
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
}

impl Weapon {
    pub fn ammo_requirement(&self) -> AmmoRequirement {
        match self.id {
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
            | 25865 => AmmoRequirement::OwnAmmo,
            10149 => AmmoRequirement::Exact(AmmoType::HerbTar(TarType::Guam)),
            10146 => AmmoRequirement::Exact(AmmoType::HerbTar(TarType::Marrentill)),
            10147 => AmmoRequirement::Exact(AmmoType::HerbTar(TarType::Tarromin)),
            10148 => AmmoRequirement::Exact(AmmoType::HerbTar(TarType::Harralander)),
            28834 => AmmoRequirement::Exact(AmmoType::HerbTar(TarType::Irit)),
            28869 => AmmoRequirement::Exact(AmmoType::AntlerBolt),
            29000 => AmmoRequirement::Exact(AmmoType::AtlatlDart),
            _ if self.is_ranged_weapon() => AmmoRequirement::Unsupported,
            _ => AmmoRequirement::None,
        }
    }

    pub fn ammo_applicability(&self, ammo: &Armor) -> AmmoApplicability {
        if let Some(ammo_type) = ammo.ammo_type() {
            self.ammo_requirement().applicability(ammo_type)
        } else {
            AmmoApplicability::Allowed
        }
    }
}
