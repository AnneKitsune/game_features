use derivative::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
// Different properties of a player/item/entity

/// The definition of a stat.
/// A stat is a named float value optionally constrained between two other values and with a
/// default value. It is used to create effects, conditions and in general to hold state
/// for each entity.
/// For example, it can be used to contain the health or mana of an entity just as well as it
/// can be used to keep track of the number of enemies positioned around an entity.
#[derive(Debug, Clone, Serialize, Deserialize, new, Builder)]
pub struct StatDefinition<K> {
    /// The key.
    pub key: K,
    /// The name.
    pub name: String,
    /// The computer friendly name.
    pub friendly_name: String,
    /// The default value.
    pub default_value: f64,
    /// The minimum value.
    #[new(default)]
    pub min_value: Option<f64>,
    /// The maximum value.
    #[new(default)]
    pub max_value: Option<f64>,
    /// The icon of this stat.
    #[new(default)]
    pub icon_path: Option<String>,
}

impl<K: Clone> StatDefinition<K> {
    /// Creates the default StatInstance for this StatDefinition.
    pub fn default_instance(&self) -> StatInstance<K> {
        StatInstance::new(self.key.clone(), self.default_value)
    }
}

/// An instance of a stat.
/// Contains a base value as well as a value after applying the stat effectors.
#[derive(Debug, Clone, Serialize, Deserialize, new, Builder)]
pub struct StatInstance<K> {
    /// The key of the stat.
    pub key: K,
    /// The base value of the stat.
    pub value: f64,
    /// The value of this stat after applying the effectors.
    #[new(value = "value")]
    pub value_with_effectors: f64,
}

/// The definitions of all known stats.
#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct StatDefinitions<K: Hash + Eq> {
    /// The definitions.
    pub defs: HashMap<K, StatDefinition<K>>,
}

impl<K: Hash + Eq> Default for StatDefinitions<K> {
    fn default() -> Self {
        Self {
            defs: HashMap::default(),
        }
    }
}

impl<K: Hash + Eq + Clone> StatDefinitions<K> {
    /// Converts the `StatDefinitions` into a `StatSet` using the default stat values.
    pub fn to_statset(&self) -> StatSet<K> {
        let instances = self
            .defs
            .iter()
            .map(|(k, v)| (k.clone(), v.default_instance()))
            .collect::<HashMap<_, _>>();
        StatSet::new(instances)
    }
}

impl<K: Hash + Eq + Clone> From<Vec<StatDefinition<K>>> for StatDefinitions<K> {
    fn from(t: Vec<StatDefinition<K>>) -> Self {
        let defs = t
            .into_iter()
            .map(|s| (s.key.clone(), s))
            .collect::<HashMap<_, _>>();
        Self::new(defs)
    }
}

/// Holds the instances of all the stats an entity has.
#[derive(Debug, Clone, Default, Serialize, Deserialize, new)]
pub struct StatSet<K: Hash + Eq> {
    /// The stats.
    pub stats: HashMap<K, StatInstance<K>>,
}

//impl<K: Hash+Eq> StatSet<K> {
//    pub fn update(&mut self, delta_time: f64, stat_set: &mut StatSet<K>) {
//        let mut rm_idx = vec![];
//        for (idx, stat) in self.effectors.iter_mut().enumerate() {
//            // TODO: apply modifier rules and ordering.
//
//            if let Some(left) = stat.disable_in.as_mut() {
//                *left -= delta_time;
//                if *left <= 0.0 {
//                    rm_idx.push(idx);
//                }
//            }
//        }
//
//        rm_idx.reverse();
//        for idx in rm_idx {
//            self.effectors.swap_remove(idx);
//        }
//    }
//}

/// Condition based on a stat to activate something.
#[derive(Clone, Debug, Serialize, Deserialize, new)]
pub struct StatCondition<K> {
    /// The key of the stat.
    pub stat_key: K,
    /// The type of condition.
    pub condition: StatConditionType,
}

impl<K: Hash + Eq + Debug> StatCondition<K> {
    /// Checks if this stat condition is met using for the provided `StatSet` using the known
    /// `StatDefinitions`.
    pub fn check(&self, stats: &StatSet<K>, stat_defs: &StatDefinitions<K>) -> bool {
        let v = stats.stats.get(&self.stat_key).expect(&format!(
            "Requested stat key {:?} is not in provided StatSet.",
            self.stat_key
        ));
        let def = stat_defs.defs.get(&self.stat_key).expect(&format!(
            "Requested stat key {:?} is not in provided StatDefinitions.",
            self.stat_key
        ));
        self.condition
            .is_true(v.value, def.min_value, def.max_value)
    }
}

/// A condition based on a stat's value.
#[derive(Clone, Serialize, Deserialize, new, Derivative)]
#[derivative(Debug)]
pub enum StatConditionType {
    /// The stat value must be higher or equal to this value.
    MinValue(f64),
    /// The stat value must be between these values.
    BetweenValue(f64, f64),
    /// The stat value must be lower or equal to this value.
    MaxValue(f64),
    /// The minimum progress of the value between its minimum and maximum.
    /// This calculates the distance between the minimum and maximum values, then assigns
    /// a value between 0.0 and 1.0 that correspond to the absolute distance from the minimum.
    /// If the minimum value is 10 and the maximum is 20 and we have a value of 15, then this
    /// corresponds to a "distance" of 0.5 (50%!) of the way between 10 and 20.
    MinPercent(f64),
    /// The minimum progress of the value between its minimum and maximum.
    /// This calculates the distance between the minimum and maximum values, then assigns
    /// a value between 0.0 and 1.0 that correspond to the absolute distance from the minimum.
    /// If the minimum value is 10 and the maximum is 20 and we have a value of 15, then this
    /// corresponds to a "distance" of 0.5 (50%!) of the way between 10 and 20.
    BetweenPercent(f64, f64),
    /// The minimum progress of the value between its minimum and maximum.
    /// This calculates the distance between the minimum and maximum values, then assigns
    /// a value between 0.0 and 1.0 that correspond to the absolute distance from the minimum.
    /// If the minimum value is 10 and the maximum is 20 and we have a value of 15, then this
    /// corresponds to a "distance" of 0.5 (50%!) of the way between 10 and 20.
    MaxPercent(f64),
    /// The value is divisible by this value.
    /// DivisibleBy(2) is equivalent to (value % 2 == 0).
    DivisibleBy(i32),
    /// A custom function that takes the value and returns whether the condition passed or not.
    #[serde(skip)]
    //Custom(#[derivative(Debug = "ignore")] std::sync::Arc<Box<dyn Fn(f64) -> bool>>),
    Custom(#[derivative(Debug = "ignore")] fn(f64) -> bool),
}

impl StatConditionType {
    /// Checks if the condition is true using the actual value, as well as the minimum and maximum
    /// values of the stat (found in the `StatDefinition`).
    pub fn is_true(&self, value: f64, min_value: Option<f64>, max_value: Option<f64>) -> bool {
        let percent = if let (Some(min_value), Some(max_value)) = (min_value, max_value) {
            Some((value - min_value) / (max_value - min_value))
        } else {
            None
        };
        match &*self {
            StatConditionType::MinValue(v) => value >= *v,
            StatConditionType::BetweenValue(min, max) => value >= *min && value <= *max,
            StatConditionType::MaxValue(v) => value <= *v,
            StatConditionType::MinPercent(p) => {
                percent.expect("This stat doesn't have min/max values.") >= *p
            }
            StatConditionType::BetweenPercent(min, max) => {
                percent.expect("This stat doesn't have min/max values.") >= *min
                    && percent.expect("This stat doesn't have min/max values.") <= *max
            }
            StatConditionType::MaxPercent(p) => {
                percent.expect("This stat doesn't have min/max values.") <= *p
            }
            StatConditionType::DivisibleBy(p) => value as i32 % p == 0,
            StatConditionType::Custom(e) => e(value),
        }
    }
}


