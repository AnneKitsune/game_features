use derivative::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
// Different properties of a player/item/entity

#[derive(Debug, Clone, Serialize, Deserialize, new, Builder)]
pub struct StatDefinition<K> {
    key: K,
    pub name: String,
    pub friendly_name: String,
    pub default_value: f64,
    #[new(default)]
    pub min_value: Option<f64>,
    #[new(default)]
    pub max_value: Option<f64>,
    #[new(default)]
    pub icon_path: Option<String>,
}

impl<K: Clone> StatDefinition<K> {
    /// Creates the default StatInstance for this StatDefinition.
    pub fn default_instance(&self) -> StatInstance<K> {
        StatInstance::new(self.key.clone(), self.default_value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, new, Builder)]
pub struct StatInstance<K> {
    pub key: K,
    pub value: f64,
    #[new(default)]
    pub value_with_effectors: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct StatDefinitions<K: Hash + Eq> {
    pub defs: HashMap<K, StatDefinition<K>>,
}

impl<K: Hash + Eq + Clone> StatDefinitions<K> {
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

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct EffectorDefinitions<K, E: Hash + Eq> {
    pub defs: HashMap<E, EffectorDefinition<K, E>>,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, new)]
pub struct StatSet<K: Hash + Eq> {
    pub stats: HashMap<K, StatInstance<K>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct EffectorSet<E> {
    pub effectors: Vec<EffectorInstance<E>>,
}

impl<E> Default for EffectorSet<E> {
    fn default() -> Self {
        Self {
            effectors: vec![],
        }
    }
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

#[derive(Clone, Debug, Serialize, Deserialize, new)]
pub struct StatCondition<K> {
    pub stat_key: K,
    pub condition: StatConditionType,
}

impl<K: Hash + Eq + Debug> StatCondition<K> {
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

#[derive(Clone, Serialize, Deserialize, new, Derivative)]
#[derivative(Debug)]
pub enum StatConditionType {
    MinValue(f64),
    BetweenValue(f64, f64),
    MaxValue(f64),
    MinPercent(f64),
    BetweenPercent(f64, f64),
    MaxPercent(f64),
    DivisibleBy(i32),
    #[serde(skip)]
    //Custom(#[derivative(Debug = "ignore")] std::sync::Arc<Box<dyn Fn(f64) -> bool>>),
    Custom(#[derivative(Debug = "ignore")] fn(f64) -> bool),
}

impl StatConditionType {
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
            StatConditionType::DivisibleBy(p) => {
                value as i32 % p == 0
            }
            StatConditionType::Custom(e) => {
                e(value)
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct EffectorDefinition<K, E> {
    pub key: E,
    // set to 0 for one shot
    pub duration: Option<f64>,
    pub effects: Vec<(K, EffectorType)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub enum EffectorType {
    Additive(f64),
    AdditiveMultiplier(f64),
    MultiplicativeMultiplier(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct EffectorInstance<E> {
    pub effector_key: E,
    pub disable_in: Option<f64>,
}
