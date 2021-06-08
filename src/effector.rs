use crate::*;
use std::collections::HashMap;
use std::hash::Hash;

/// Holds the definitions of the stat effectors.
#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct EffectorDefinitions<K, E: Hash + Eq> {
    /// The definitions.
    pub defs: HashMap<E, EffectorDefinition<K, E>>,
}

impl<K, E: Hash + Eq> Default for EffectorDefinitions<K, E> {
    fn default() -> Self {
        Self {
            defs: HashMap::default(),
        }
    }
}

impl<K: Hash + Eq + Clone, E: Hash + Eq + Clone> From<Vec<EffectorDefinition<K, E>>>
    for EffectorDefinitions<K, E>
{
    fn from(t: Vec<EffectorDefinition<K, E>>) -> Self {
        let defs = t
            .into_iter()
            .map(|s| (s.key.clone(), s))
            .collect::<HashMap<_, _>>();
        Self::new(defs)
    }
}

/// A collection of currently active effectors.
#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct EffectorSet<E> {
    /// The active effectors.
    pub effectors: Vec<EffectorInstance<E>>,
}

impl<E> Default for EffectorSet<E> {
    fn default() -> Self {
        Self { effectors: vec![] }
    }
}

impl<E: Hash + Eq> EffectorSet<E> {
    /// Applies the effects of this effector to the provided `StatSet`.
    /// The delta time is used when using effectors that apply directly to
    /// the base stat value. (WIP)
    pub fn apply_to<K: Eq + Hash>(
        self: &Self,
        effector_defs: &EffectorDefinitions<K, E>,
        stat_set: &mut StatSet<K>,
        _delta_time: f32,
    ) {
        for mut s in stat_set.stats.values_mut() {
            let mut new_value = s.value;
            let mut multiplicative_multiplier = 1.0;
            let mut additive_multiplier = 0.0;
            let mut additive = 0.0;
            // find effectors affecting this stat
            for e in self.effectors.iter() {
                let def = effector_defs
                    .defs
                    .get(&e.effector_key)
                    .expect("Tried to get unknown stat key.");

                // Algo:
                // - Apply all multiplicative multipliers
                // - Apply all additive multipliers
                // - Apply all additives

                // look into the effect of each effector
                for (key, ty) in def.effects.iter() {
                    // if any matches
                    if *key == s.key {
                        // Apply Effector
                        match ty {
                            EffectorType::Additive(v) => additive += v,
                            EffectorType::AdditiveMultiplier(v) => additive_multiplier += v,
                            EffectorType::MultiplicativeMultiplier(v) => {
                                multiplicative_multiplier *= v
                            }
                        }
                    }
                }
            }
            let multiplier = multiplicative_multiplier + additive_multiplier;
            new_value += additive;
            new_value *= multiplier;
            s.value_with_effectors = new_value;
        }
    }
}

/// The definition of a stat effector.
/// This modifies temporarily the value of a stat.
#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct EffectorDefinition<K, E> {
    /// The key of the effector.
    pub key: E,
    /// The duration of the effector.
    /// None means that it does not expire.
    /// Some(0) means that it is applied only once.
    /// Some(n) means that it is applied for n seconds.
    pub duration: Option<f64>,
    /// The effects that cause this effector.
    /// Note that effectors can only cause effects on a single stat.
    /// To affect multiple stats, create multiple effectors.
    // TODO consider using only a single element here? It almost never happens that
    // we want to apply multiple changes to the same stat.
    pub effects: Vec<(K, EffectorType)>,
}

/// The way this effector modifies the stat.
#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub enum EffectorType {
    /// Adds a value to the base value of the stat.
    Additive(f64),
    /// Multiplies the stat by a value.
    /// Stacks additively with other multipliers affecting this same stat.
    AdditiveMultiplier(f64),
    /// Multiplies the stat by a value.
    /// Stacks multiplicatively with other multipliers affecting this same stat.
    MultiplicativeMultiplier(f64),
}

/// An active instance of an effector.
#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct EffectorInstance<E> {
    /// The key of the effector.
    pub effector_key: E,
    /// The time before this effector expires.
    pub disable_in: Option<f64>,
}
