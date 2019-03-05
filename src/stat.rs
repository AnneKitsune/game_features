
// Different properties of a player/item/entity

pub trait Stat {}

// StatEffector = Effect

pub struct EffectDefinition<K> {
    pub key: K,
    pub name: String,
    pub description: String,
}

pub struct EffectInstance<K> {
    pub effector: K,
}

// Stat of T driving a transition of T to T'
pub trait StatTransition {
    // stat transition can fail (ie missing mana)

    // add key
}