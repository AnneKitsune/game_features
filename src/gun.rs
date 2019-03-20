
pub enum WeaponMode {
    Manual,
    Semi,
    Burst{burst_count: u32, burst_fire_per_second: f64},
    Auto,
}

pub struct WeaponDefinition<K> {
    pub key: K,
    pub base_damage: f64,
    pub weapon_mode: WeaponMode,
    /// In case of burst, the number of times per second you can start a new burst sequence.
    pub fire_per_second: f64,
    pub clip_size: u32,
    pub ammo_consume_per_shot: u32,
    pub fire_speed_multiplier_ramp: PartialFunction<u32, f64>,
    pub reload_time: f64,
    pub projectile_count: u32,
    pub recoil_pattern: RecoilPattern,
    pub spread: SpreadPattern,
    pub spread_reduction_per_second: f64,
    pub bullet_penetration_damage_loss_percent: f64,
    pub distance_damage_multiplier: PartialFunction<f64, f64>,
    pub show_crosshair: bool,
}

pub struct WeaponInstance<K> {
    pub key: K,
    pub last_shot_time: f64,
    pub reloading: bool,
    pub burst_firing: bool,
    pub burst_shots_left: u32,
    pub spread_value: f64,
}

pub enum RecoilPattern {
    Random{hangle: f64, vangle: f64},
    Fixed{points: Vec<(f64, f64)>},
}

pub enum SpreadPattern {
    Constant{max_angle: f64},
    /// Current spread -> new spread
    Ramped{func: PartialFunction<f64, f64>},
}

