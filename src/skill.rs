
// World interaction
// or
// Stat buff
pub struct SkillDefinition<K, S> {
    pub key: K,
    pub name: String,
    pub description: String,
    pub cooldown: f64,
    // stat usage
    pub stat_transition: Option<S>,
}

pub struct SkillInstance<K> {
    pub skill_key: K,
    pub current_cooldown: f64,
}