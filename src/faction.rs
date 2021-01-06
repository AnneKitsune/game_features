use crate::user_group::*;

/// A team with the ability to claim ownership over terrain.
/// WIP
pub struct Faction {
    /// The group of users that this faction is composed of.
    pub users: UserGroup,
    /// The claiming power of this faction. Limits the number of claims it can have and maintain.
    // TODO calculate power from users.
    pub power: f32,
    /// All the claims owned by this faction.
    pub claims: Vec<(i32, i32, i32)>,
    /// A value added to the calculated power value.
    pub power_boost: f32,
}

impl Faction {
    /// Claim terrain from another faction.
    pub fn claim_from(&mut self, other: &mut Faction, settings: &FactionSettings) -> FactionResult {
        Ok(())
    }
}

/// Settings of the faction module.
pub struct FactionSettings {
    /// The settings related to users.
    pub user_settings: UserGroupSettings,
    /// The maximum player-generated claim power.
    pub maximum_player_power: f32,
    /// The flags that apply to claimed terrain and faction behavior.
    pub flags: FactionFlags,
}

/// Fags that modify how a faction behaves and how the claimed terrain behaves.
pub struct FactionFlags {
    /// You can steal terrain from this faction.
    pub claimable: bool,
    /// Player attacks are enabled in claimed terrain.
    pub pvp_enabled: bool,
    /// You lose power on death in claimed terrain.
    pub power_loss_in_territory: bool,
    /// You can gain power in claimed terrain.
    pub power_gain_in_territory: bool,
    /// If true, will not destroy the faction once all players leaved the faction.
    pub permanent: bool,
}

/// Alias type. List of all known factions.
pub type FactionRepository = Vec<Faction>;

/// Alias type. Result of faction methods that can fail.
pub type FactionResult = std::result::Result<(), FactionError>;

/// Errors that can occur while using factions.
#[derive(Debug)]
pub enum FactionError {
    /// You don't have enough power to claim terrain.
    NotEnoughPower,
    /// You cannot claim this terrain.
    Unclaimable,
    /// The pvp is not allowed in this terrain.
    PvpDenied,
    /// You cannot use an item in this terrain.
    UseDenied,
}

/// The settings related to terrain claiming and how the world is divided into claimable chunks.
pub struct LandClaimSettings {
    /// The size of the claimable chunks.
    pub claim_size: [f32; 3],
}

impl LandClaimSettings {
    /// Get the three dimensional ID of this claim area.
    pub fn claim_id_from_position(&self, pos: &[f32; 3]) -> (i32, i32, i32) {
        let x = pos[0] / self.claim_size[0];
        let y = pos[1] / self.claim_size[1];
        let z = if self.claim_size[2] != 0.0 {
            pos[2] / self.claim_size[2]
        } else {
            0.0
        };
        (x as i32, y as i32, z as i32)
    }
}
