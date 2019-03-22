use crate::*;

pub struct ItemTransition<K, I, S, U: Default> {
    pub key: K,
    pub name: String,
    pub friendly_name: String,
    pub icon_path: Option<String>,
    pub input_items: Vec<(I, usize, UseMode)>,
    pub stat_conditions: Vec<StatCondition<S>>,
    pub stat_consume: Vec<Effector<S>>,
    pub output_items: Vec<ItemInstance<I, U>>,
    pub on_condition_lost: ConditionLostReaction,
    pub time_to_complete: f64,
    pub consume_input_immediate: bool,
}

pub enum UseMode {
    Consume,
    UseOnce{durability: f64},
    UsePerSecond{rate: f64},
}

pub enum ConditionLostReaction {
    None,
    Pause,
    Cancel,
}

pub struct ItemTransitionBatch<K> {
    transition: K,
    remaining: u32,
    next_completion_remaining: f64,
}

