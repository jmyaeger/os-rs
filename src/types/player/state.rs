//! `Player` methods for manipulating `Player` stats and effects during combat.
use super::Player;
use crate::combat::attacks::effects::CombatEffect;
use crate::types::prayers::Prayer;

impl Player {
    pub fn heal(&mut self, amount: u32, overheal_hp: Option<u32>) {
        // Heals the player by the specified amount (with optional maximum overheal)
        self.stats.hitpoints.restore(amount, overheal_hp);
    }

    pub fn regen_all_stats(&mut self) {
        if self.stats.hitpoints.current < self.stats.hitpoints.base {
            let wearing_hp_cape =
                self.is_wearing_any(vec![("Hitpoints cape", None), ("Hitpoints cape(t)", None)]);
            let wearing_regen_bracelet = self.is_wearing("Regen bracelet", None);
            let using_rapid_heal = self.prayers.contains_prayer(Prayer::RapidHeal);
            let heal_amount = if using_rapid_heal {
                if wearing_regen_bracelet { 2 } else { 1 }
            } else if wearing_hp_cape && wearing_regen_bracelet {
                4
            } else if wearing_hp_cape || wearing_regen_bracelet {
                2
            } else {
                1
            };
            self.stats.hitpoints.restore(heal_amount, None);
        }

        if self.stats.attack.current < self.stats.attack.base {
            self.stats.attack.restore(1, None);
        }

        if self.stats.strength.current < self.stats.strength.base {
            self.stats.strength.restore(1, None);
        }

        if self.stats.defence.current < self.stats.defence.base {
            self.stats.defence.restore(1, None);
        }

        if self.stats.ranged.current < self.stats.ranged.base {
            self.stats.ranged.restore(1, None);
        }

        if self.stats.magic.current < self.stats.magic.base {
            self.stats.magic.restore(1, None);
        }
    }

    pub fn take_damage(&mut self, amount: u32) {
        // Takes damage, capping at 0 HP
        self.stats.hitpoints.drain(amount);
    }

    pub fn clear_inactive_effects(&mut self) {
        self.active_effects.retain(|event| match event {
            CombatEffect::Poison { tick_counter, .. }
            | CombatEffect::Venom { tick_counter, .. }
            | CombatEffect::Burn { tick_counter, .. }
            | CombatEffect::DelayedHeal { tick_counter, .. }
            | CombatEffect::DamageOverTime { tick_counter, .. } => tick_counter.is_some(),
            CombatEffect::DelayedAttack { tick_delay, .. }
            | CombatEffect::DelayedBurn {
                tick_delay,
                burn_ticks: _,
            } => tick_delay.is_some(),
        });
    }

    pub fn restore_prayer(&mut self, amount: u32, max_level: Option<u32>) {
        let cap = max_level.unwrap_or(self.stats.prayer.base);
        self.stats
            .prayer
            .restore(amount, Some(cap - self.stats.prayer.base));
    }
}
