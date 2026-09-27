// Struct for holding sunfire rune min hit value
#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct SunfireBoost {
    pub active: bool,
    pub min_hit: u32,
}

// Collection of effects that provide a damage boost in some cases
#[derive(Debug, PartialEq, Clone)]
pub struct StatusBoosts {
    pub on_task: bool,
    pub in_wilderness: bool,
    pub in_multi: bool,
    pub forinthry_surge: bool,
    pub charge_active: bool,
    pub kandarin_diary: bool,
    pub mark_of_darkness: bool,
    pub acb_spec: bool,
    pub zcb_spec: bool,
    pub sunfire: SunfireBoost,
    pub soulreaper_stacks: u32,
}

impl Default for StatusBoosts {
    fn default() -> Self {
        Self {
            // Assume on task, not in wildy, and Kandarin Hard Diary unlocked
            on_task: true,
            in_wilderness: false,
            in_multi: false,
            forinthry_surge: false,
            charge_active: false,
            kandarin_diary: true,
            mark_of_darkness: false,
            acb_spec: false,
            zcb_spec: false,
            sunfire: SunfireBoost::default(),
            soulreaper_stacks: 0,
        }
    }
}

// Poison and venom effects - will likely rework this in the future
#[derive(Default, Debug, PartialEq, Clone)]
pub struct StatusEffects {
    pub poisoned: bool,
    pub venomed: bool,
    pub immune_poison: bool,
    pub immune_venom: bool,
    pub poison_severity: u8,
}

// Holds set effect data to avoid iterating through gear many times
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct SetEffects {
    pub full_void: bool,
    pub full_elite_void: bool,
    pub full_justiciar: bool,
    pub full_inquisitor: bool,
    pub full_dharoks: bool,
    pub full_torags: bool,
    pub full_guthans: bool,
    pub full_veracs: bool,
    pub full_karils: bool,
    pub full_ahrims: bool,
    pub full_obsidian: bool,
    pub full_blood_moon: bool,
    pub full_blue_moon: bool,
    pub full_eclipse_moon: bool,
    pub bloodbark_pieces: usize,
}
