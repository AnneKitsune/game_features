// TODO consider if the tier stuff is useful at all.
/// Tiered element.
/// Simply adds a numerical value to any element.
pub struct Tiered<T> {
    /// The numerical tier.
    pub tier: u32,
    /// The element that has a tier.
    pub element: T,
}

/// A levelable element.
/// It can be anything: an item, a player, a monster, a skill.
pub struct Leveled<T: LevelFor> {
    /// The experience that this has accumulated.
    pub accumulated_xp: u32,
    /// The inner element that is leveled.
    pub element: T,
}

impl<T: LevelFor> Leveled<T> {
    /// The current level.
    pub fn level(&self) -> u32 {
        self.element.level_for_xp(self.accumulated_xp)
    }
}

/// A trait that can calculate the level for something that can accumulate experience.
/// We suggest using `PartialFunction` when implementing this trait.
pub trait LevelFor {
    /// Returns the level that you have using the amount of experience.
    fn level_for_xp(&self, xp: u32) -> u32;
}
