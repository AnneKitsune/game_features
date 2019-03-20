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

#[derive(Debug, Clone, Serialize, Deserialize, new, Builder)]
pub struct StatInstance<K> {
    key: K,
    value: f64,
}

