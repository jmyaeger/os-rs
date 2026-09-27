use super::Player;
use crate::types::potions::{Potion, PotionBoost, PotionStat};
use crate::types::prayers::Prayer;
use std::rc::Rc;

impl Player {
    pub fn reset_current_stats(&mut self, include_spec: bool) {
        // Restore to base stats, full spec energy, and reapply potion boosts
        if include_spec {
            self.stats.reset_all();
        } else {
            let current_spec = self.stats.spec;
            self.stats.reset_all();
            self.stats.spec = current_spec;
        }
        if let Some(hp) = self.state.current_hp {
            self.stats.hitpoints.current = hp;
        }
        self.apply_potion_boosts();
    }

    pub fn calc_potion_boosts(&mut self) {
        // Calculate all of the selected potion boosts
        if let Some(potions) = &mut self.potions.attack {
            for potion in potions {
                if potion.potion_type == Potion::Moonlight {
                    potion.calc_moonlight_boost(
                        self.stats.attack,
                        self.stats.herblore,
                        &PotionStat::Attack,
                    );
                } else if potion.potion_type == Potion::ZamorakBrew {
                    potion.calc_zamorak_brew_boost(self.stats.attack, &PotionStat::Attack);
                } else {
                    potion.calc_boost(self.stats.attack);
                }
            }
        }
        if let Some(potions) = &mut self.potions.strength {
            for potion in potions {
                if potion.potion_type == Potion::Moonlight {
                    potion.calc_moonlight_boost(
                        self.stats.strength,
                        self.stats.herblore,
                        &PotionStat::Strength,
                    );
                } else if potion.potion_type == Potion::DragonBattleaxe {
                    potion.calc_dragon_battleaxe_boost(
                        self.stats.attack,
                        self.stats.defence,
                        self.stats.ranged,
                        self.stats.magic,
                    );
                } else if potion.potion_type == Potion::ZamorakBrew {
                    potion.calc_zamorak_brew_boost(self.stats.strength, &PotionStat::Strength);
                } else {
                    potion.calc_boost(self.stats.strength);
                }
            }
        }
        if let Some(potions) = &mut self.potions.defence {
            for potion in potions {
                if potion.potion_type == Potion::Moonlight {
                    potion.calc_moonlight_boost(
                        self.stats.defence,
                        self.stats.herblore,
                        &PotionStat::Defence,
                    );
                } else {
                    potion.calc_boost(self.stats.defence);
                }
            }
        }
        if let Some(potions) = &mut self.potions.ranged {
            for potion in potions {
                potion.calc_boost(self.stats.ranged);
            }
        }
        if let Some(potions) = &mut self.potions.magic {
            for potion in potions {
                potion.calc_boost(self.stats.magic);
            }
        }
    }

    fn apply_potion_boosts(&mut self) {
        // Apply all of the selected potion boosts to the player's live stats
        if let Some(potions) = &self.potions.attack {
            let max_boost = potions.iter().map(|p| p.boost).max().unwrap_or_default();
            self.stats.attack.boost(max_boost);
        }
        if let Some(potions) = &self.potions.strength {
            let max_boost = potions.iter().map(|p| p.boost).max().unwrap_or_default();
            self.stats.strength.boost(max_boost);
        }
        if let Some(potions) = &self.potions.defence {
            let max_boost = potions.iter().map(|p| p.boost).max().unwrap_or_default();
            self.stats.defence.boost(max_boost);
        }
        if let Some(potions) = &self.potions.ranged {
            let max_boost = potions.iter().map(|p| p.boost).max().unwrap_or_default();
            self.stats.ranged.boost(max_boost);
        }
        if let Some(potions) = &self.potions.magic {
            let max_boost = potions.iter().map(|p| p.boost).max().unwrap_or_default();
            self.stats.magic.boost(max_boost);
        }
    }

    pub fn add_potion(&mut self, potion: Potion) {
        // Add a potion to the correct slot, calc boosts, and reset live stats
        if potion.boosts_attack() {
            self.potions
                .attack
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
        } else if potion.boosts_strength() {
            self.potions
                .strength
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
        } else if potion.boosts_defence() {
            self.potions
                .defence
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
        } else if potion.boosts_ranged() {
            self.potions
                .ranged
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
        } else if potion.boosts_magic() {
            self.potions
                .magic
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
        } else if potion.boosts_all_melee() {
            self.potions
                .attack
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
            self.potions
                .strength
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
            self.potions
                .defence
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
        } else if potion.boosts_all() {
            self.potions
                .attack
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
            self.potions
                .strength
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
            self.potions
                .defence
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
            self.potions
                .ranged
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
            self.potions
                .magic
                .get_or_insert_with(Vec::new)
                .push(PotionBoost::new(&potion));
        }

        self.calc_potion_boosts();
        self.reset_current_stats(false);
    }

    pub fn remove_potion(&mut self, potion: Potion) {
        self.potions.remove_potion(potion);
        self.calc_potion_boosts();
        self.reset_current_stats(false);
    }

    pub fn add_prayer(&mut self, prayer: Prayer) {
        Rc::make_mut(&mut self.prayers).add(prayer);
    }

    pub fn remove_prayer(&mut self, prayer: Prayer) {
        Rc::make_mut(&mut self.prayers).remove(prayer);
    }
}
