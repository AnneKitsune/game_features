use derivative::*;
use std::collections::HashMap;
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, new)]
pub struct EffectorSet<E> {
    pub effectors: Vec<EffectorInstance<E>>,
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

#[derive(Clone, Serialize, Deserialize, new, Derivative)]
#[derivative(Debug)]
//#[derivative(Debug)]
pub enum StatConditionType {
    MinValue(f64),
    BetweenValue(f64, f64),
    MaxValue(f64),
    MinPercent(f64),
    BetweenPercent(f64, f64),
    MaxPercent(f64),
    #[serde(skip)]
    Custom(#[derivative(Debug = "ignore")] std::sync::Arc<Box<dyn Fn(f64) -> bool>>),
}

impl StatConditionType {
    pub fn is_true(&self, value: f64, min_value: f64, max_value: f64) -> bool {
        let percent = (value - min_value) / (max_value - min_value);
        match &*self {
            StatConditionType::MinValue(v) => value >= *v,
            StatConditionType::BetweenValue(min, max) => value >= *min && value <= *max,
            StatConditionType::MaxValue(v) => value <= *v,
            StatConditionType::MinPercent(p) => percent >= *p,
            StatConditionType::BetweenPercent(min, max) => percent >= *min && percent <= *max,
            StatConditionType::MaxPercent(p) => percent <= *p,
            StatConditionType::Custom(e) => e(value),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct EffectorDefinition<K, E> {
    pub key: E,
    // set to 0 for one shot
    pub duration: Option<f64>,
    // TODO: modifier rules
    pub effects: Vec<K>,
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct EffectorInstance<E> {
    pub effector_key: E,
    pub active_since: f64,
    pub disable_in: Option<f64>,
}
