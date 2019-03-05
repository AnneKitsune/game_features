use std::collections::HashMap;

pub struct ItemDefinition<K> {
    pub key: K,
    pub name: String,
    pub description: String,
    pub maximum_stack: u32,
    pub maximum_durability: Option<u32>,
}

pub struct ItemInstance<K> {
    pub item_key: K,
    pub count: u32,
    pub durability: Option<u32>,
}

pub type ItemDefinitionRepository<K> = HashMap<K, ItemDefinition<K>>;

pub struct Inventory<K> {
    pub content: Vec<ItemInstance<K>>, // usually Stacked<T>
}
