use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// An `ItemDefinition` stores the different properties of a type of item.
/// It is a schema that contains the data which isn't changing between different item instances.
///
/// Generic Types:
/// * K: Type of the key. Usually an enum or a number (ie u32).
/// * T: Type of the different item types. For example, an armor-type item can not be placed into a
/// weapon-type inventory slot. If you don't need to make the distinction between different item
/// types, use the `()` type.
/// * D: The type of the custom user data. If you don't have any, use the `()` type. It can (and
/// probably should) be different than the custom user data used on `ItemInstance`s
#[derive(new, Clone, Serialize, Deserialize, Debug, Builder)]
pub struct ItemDefinition<K, S, D: Default> {
    /// The key identifying this item definition.
    pub key: K,
    /// The type / item group that this item definition is part of.
    pub slot_type: S,
    /// The display name of this item definition.
    pub name: String,
    /// The friendly name of this item definition. Mostly used to find items by name instead of by
    /// key.
    pub friendly_name: String,
    /// The display description of this item definition.
    pub description: String,
    /// The maximum number of elements that can be in an item stack. Setting this value to 1
    /// disables the ability to stack this type of item.
    pub maximum_stack: Option<usize>,
    /// The default maximum durability of this item. Setting this to None means that this item type
    /// doesn't use the concept of durability and is unbreakable.
    pub maximum_durability: Option<usize>,
    /// Custom user data. For example: rarity, weight, list of allowed upgrades, etc...
    #[new(default)]
    #[builder(default)]
    pub user_data: D,
}

/// An `ItemInstance` is a stack of item.
/// It refers to the `ItemDefinition`'s key. The associated `ItemDefinition` contains the data
/// describing that object type.
/// An item stack can be composed of one or many of the same item.
/// It can also have a durability, which decreases when the item is used using
/// `Inventory::use_item`.
///
/// Custom data can be added into the user_data field. This data is specific to each item instance.
///
/// Generic Types:
/// * K: Type of the key. Usually an enum or a number (ie u32).
/// * U: The type of the custom user data. If you don't have any, use the `()` type.
/// It can (and probably should) be different than the custom user data used on `ItemInstance`s
#[derive(new, Clone, Serialize, Deserialize, Debug, Builder)]
pub struct ItemInstance<K, U: Default> {
    /// The key specifies which `ItemDefinition` defines the properties of this item stack.
    pub key: K,
    /// The number of items in the stack.
    /// Should not be over the associated `ItemDefinition::maximum_stack`.
    pub quantity: usize,
    /// The remaining durability of this item, if any. If set to None, it means that the item is
    /// unbreakable.
    #[new(default)]
    #[builder(default)]
    pub durability: Option<usize>,
    /// The custom user data.
    #[new(default)]
    #[builder(default)]
    pub user_data: U,
}

impl<K: Eq + Hash, U: Default + PartialEq> ItemInstance<K, U> {
    /// Attempts to move as much quantity from other to self as possible.
    pub fn merge<S, U2: Default>(
        &mut self,
        other: &mut Self,
        item_defs: &ItemDefinitions<K, S, U2>,
    ) {
        if self.key == other.key && self.user_data == other.user_data {
            if let Some(def) = item_defs.defs.get(&self.key) {
                let can_take = if def.maximum_stack.is_some() {
                    // can break if your stack is over the maximum amount allowed
                    std::cmp::min(def.maximum_stack.unwrap() - self.quantity, other.quantity)
                } else {
                    other.quantity
                };
                self.quantity += can_take;
                other.quantity -= can_take;
            }
        }
    }
}

/// A simple repository mapping the key K to the corresponding `ItemDefinition`.
#[derive(Serialize, Deserialize, Clone, new)]
pub struct ItemDefinitions<K: Hash + Eq, S, D: Default> {
    /// The definitions.
    pub defs: HashMap<K, ItemDefinition<K, S, D>>,
}

impl<K: Hash + Eq, S, D: Default> Default for ItemDefinitions<K, S, D> {
    fn default() -> Self {
        Self {
            defs: HashMap::default(),
        }
    }
}

impl<K: Hash + Eq + Clone, S, D: Default> From<Vec<ItemDefinition<K, S, D>>>
    for ItemDefinitions<K, S, D>
{
    fn from(t: Vec<ItemDefinition<K, S, D>>) -> Self {
        let defs = t
            .into_iter()
            .map(|s| (s.key.clone(), s))
            .collect::<HashMap<_, _>>();
        Self::new(defs)
    }
}

/// A trait defining which items can be inserted into each inventory slot type.
pub trait SlotType {
    /// Checks if the provided item type can be inserted in this slot type.
    fn can_insert_into(&self, item_type: &Self) -> bool;
}

impl SlotType for () {
    fn can_insert_into(&self, _: &Self) -> bool {
        true
    }
}

// TODO extra stuff
/*pub struct SingleEquippedItem<K> {
    pub equipped_index: usize,
    _phantom: PhantomData<K>,
}

impl<K: PartialEq> SingleEquippedItem<K> {
    pub fn get_equipped(&self, inventory: &Inventory<K, D, S>) -> Option<&ItemInstance<K, U>> {

    }
}

pub struct BaseRecipeDefinition<K: PartialEq> {
    pub inputs: Vec<ItemInstance<K, U>>,
    pub outputs: Vec<ItemInstance<K, U>>,
}

trait Recipe<K> {
    fn craft(&mut self, inputs: Vec<ItemInstance<K, U>>) -> Vec<ItemInstance<K, U>>;
}*/

/*#[cfg(test)]
mod test {
    use crate::*;

    #[derive(new, Debug, Clone, Serialize, Deserialize, Default)]
    struct CustomItemDefinitionData {
        pub weight: f32,
    }

    #[derive(new, Debug, Clone, Serialize, Deserialize, Default)]
    struct CustomItemInstanceData {
        pub xp: f32,
    }

    #[derive(new, Debug, Clone, Serialize, Deserialize, PartialEq)]
    enum ItemType {
    }

    #[derive(new, Debug, Clone, Serialize, Deserialize, PartialEq)]
    enum ItemType {
        Weapon,
        Armor,
        Other,
        Consumable,
        // slot types
        Regular,
        Equipment,
    }

    // Work around for broken derive_builder.
    impl Default for ItemType {
        fn default() -> Self {
            ItemType::Other
        }
    }

    impl SlotType for ItemType {
        // slot type -> item type
        fn can_insert_into(&self, other: &ItemType) -> bool {
            match *self {
                ItemType::Regular => true,
                ItemType::Equipment => {
                    *other == ItemType::Weapon || *other == ItemType::Armor
                }
            }
        }
    }

    #[test]
    fn inventory_insert_empty_fixed() {
        let ii = ItemInstance::<u32, ()>::new(1u32, 1);
        let mut inv = Inventory::<u32, (), (), ()>::new_fixed(8);
        inv.insert(ii).expect("");
    }

    #[test]
    fn complex_items() {
        // Weight, enchants and Effects
        // TODO: Effectors

        let item_def = ItemDefinitionBuilder::default()
            .key(1u32)
            .item_type(ItemType::Consumable)
            .name("Apple".to_string())
            .friendly_name("apple".to_string())
            .description("Food is nice".to_string())
            .maximum_stack(16)
            .maximum_durability(None)
            .user_data(CustomItemDefinitionData::new(1.0))
            .build()
            .unwrap();
        let ii = ItemInstanceBuilder::default()
            .key(1u32)
            .quantity(4)
            .durability(Some(64))
            .user_data(CustomItemInstanceData::new(0.0))
            .build()
            .unwrap();
        let mut inv =
            Inventory::<u32, ItemType, CustomItemInstanceData>::new_fixed(8);
        let mut inv2 =
            Inventory::<u32, ItemType, CustomItemInstanceData>::new_fixed(8);
        inv.insert(ii.clone()).expect("");
        inv2.insert(ii).expect("");
        inv.transfer(0, &mut inv2, 1, 2, false).expect("");
    }
}*/

