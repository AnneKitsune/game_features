pub struct Tiered<T> {
    pub tier: u32,
    pub element: T,
}

pub struct Leveled<T: LevelFor> {
    pub level: u32,
    pub accumulated_xp: u32,
    pub element: T,
}

// Will usually use PartialFunction.
pub trait LevelFor {
    fn level_for_xp(&self, xp: u32) -> u32;
}
