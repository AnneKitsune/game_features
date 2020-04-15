use std::collections::HashMap;
use std::hash::Hash;
// Different properties of a player/item/entity

#[derive(Debug, Clone, Serialize, Deserialize, new, Builder)]
pub struct StatDefinition<K> {
    key: K,
    name: String,
    friendly_name: String,
    default_value: f64,
    #[new(default)]
    min_value: Option<f64>,
    #[new(default)]
    max_value: Option<f64>,
    #[new(default)]
    icon_path: Option<String>,
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
pub struct StatSet<K: Hash+Eq> {
    pub stats: HashMap<K, StatInstance<K>>,
    pub active_stats_stats_effectors: Vec<Effector<K>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct StatCondition<K> {
    pub stat_key: K,
    pub condition: StatConditionType,
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub enum StatConditionType {
    MinValue(f64),
    BetweenValue(f64, f64),
    MaxValue(f64),
    MinPercent(f64),
    BetweenPercent(f64, f64),
    MaxPercent(f64),
}

impl StatConditionType {
    pub fn is_true(&self, value: f64, min_value: f64, max_value: f64) -> bool {
        let percent = (value - min_value) / (max_value - min_value);
        match *self {
            StatConditionType::MinValue(v) => value >= v,
            StatConditionType::BetweenValue(min, max) => value >= min && value <= max,
            StatConditionType::MaxValue(v) => value <= v,
            StatConditionType::MinPercent(p) => percent >= p,
            StatConditionType::BetweenPercent(min, max) => percent >= min && percent <= max,
            StatConditionType::MaxPercent(p) => percent <= p,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct Effector<K> {
    pub stat_key: K,
    pub active_since: f64,
    pub disable_in: Option<f64>,
    // TODO: modifier rules
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct ActiveEffectors<K> {
    pub effectors: Vec<Effector<K>>,
}

impl<K: Hash+Eq> ActiveEffectors<K> {
    pub fn update(&mut self, delta_time: f64, stat_set: &mut StatSet<K>) {
        let mut rm_idx = vec![];
        for (idx, stat) in self.effectors.iter_mut().enumerate() {
            // TODO: apply modifier rules and ordering.
            
            if let Some(left) = stat.disable_in.as_mut() {
                *left -= delta_time;
                if *left <= 0.0 {
                    rm_idx.push(idx);
                }
            }
        }
        
        rm_idx.reverse();
        for idx in rm_idx {
            self.effectors.swap_remove(idx);
        }
    }
}

