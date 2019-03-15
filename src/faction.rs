use crate::user_group::*;

pub struct Faction {
    pub users: UserGroup,
    pub power: f32,
    pub claims: Vec<(i32, i32, i32)>,
    pub power_boost: f32,
}

impl Faction {
    pub fn claim_from(&mut self, other: &mut Faction, settings: &FactionSettings) -> FactionResult {
        Ok(())
    }
}

pub struct FactionSettings {
    pub user_settings: UserGroupSettings,
    pub maximum_player_power: f32,
    pub flags: FactionFlags,
}

pub struct FactionFlags {
    pub claimable: bool,
    pub pvp_enabled: bool,
    pub power_loss_in_territory: bool,
    pub power_gain_in_territory: bool,
    /// If true, will not destroy the faction once all players leaved.
    pub permanent: bool,
}

pub type FactionRepository = Vec<Faction>;

pub type FactionResult = std::result::Result<(), FactionError>;

#[derive(Debug)]
pub enum FactionError {
    NotEnoughPower,
    Unclaimable,
    PvpDenied,
    UseDenied,
}

pub struct LandClaimSettings {
    pub claim_size: [f32; 3],
}

impl LandClaimSettings {
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
