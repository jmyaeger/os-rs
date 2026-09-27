//! Methods for fetching player data from the OSRS Hiscores. This module is optional
//! and requires the `hiscores` feature to be enabled.
use super::Player;
use crate::error::PlayerError;
use crate::types::stats::{PlayerStats, SpecEnergy, Stat};
use reqwest;
use std::collections::HashMap;

impl Player {
    pub async fn lookup_stats(&mut self, rsn: &str) -> Result<(), PlayerError> {
        // Fetch stats from OSRS hiscores and set the corresponding fields
        let stats = fetch_player_data(rsn).await?;
        self.stats = parse_player_data(stats)?;
        self.attrs.name = Some(rsn.to_string());

        Ok(())
    }
}

pub async fn fetch_player_data(rsn: &str) -> Result<String, PlayerError> {
    // Fetches player data from the OSRS hiscores
    let url = "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws";
    let params = [("player", rsn)];
    let client = reqwest::Client::new();
    let response = client.get(url).query(&params).send().await?; // Use .await
    let data = response.text().await?; // Use .await
    Ok(data)
}

pub fn parse_player_data(data: String) -> Result<PlayerStats, PlayerError> {
    // Parses player data and creates a PlayerStats struct from it
    let skills = [
        "attack",
        "defence",
        "strength",
        "hitpoints",
        "ranged",
        "prayer",
        "magic",
    ];
    let data_lines: Vec<&str> = data.lines().collect();
    let mut skill_map = HashMap::new();

    for (i, skill) in skills.iter().enumerate() {
        let line_parts: Vec<&str> = data_lines[i + 1].split(',').collect();
        let level = line_parts[1].parse::<u32>()?;
        skill_map.insert(*skill, level);
    }

    let mining_lvl = data_lines[15].split(',').collect::<Vec<&str>>()[1];
    skill_map.insert("mining", mining_lvl.parse::<u32>()?);
    let herblore_lvl = data_lines[16].split(',').collect::<Vec<&str>>()[1];
    skill_map.insert("herblore", herblore_lvl.parse::<u32>()?);

    Ok(PlayerStats {
        hitpoints: Stat::new(skill_map["hitpoints"], None),
        attack: Stat::new(skill_map["attack"], None),
        strength: Stat::new(skill_map["strength"], None),
        defence: Stat::new(skill_map["defence"], None),
        ranged: Stat::new(skill_map["ranged"], None),
        magic: Stat::new(skill_map["magic"], None),
        prayer: Stat::new(skill_map["prayer"], None),
        mining: Stat::new(skill_map["mining"], None),
        herblore: Stat::new(skill_map["herblore"], None),
        spec: SpecEnergy::default(),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_lookup_stats() {
        let mut player = Player::new();
        player.lookup_stats("Lynx Titan").await.unwrap();
        assert_eq!(player.stats.attack.base, 99);
        assert_eq!(player.stats.defence.base, 99);
        assert_eq!(player.stats.strength.base, 99);
        assert_eq!(player.stats.hitpoints.base, 99);
        assert_eq!(player.stats.ranged.base, 99);
        assert_eq!(player.stats.magic.base, 99);
        assert_eq!(player.stats.prayer.base, 99);
    }
}
