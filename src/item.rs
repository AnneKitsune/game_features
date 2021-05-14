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

/// The way the inventory size is handled.
#[derive(new, Clone, Serialize, Deserialize, Debug)]
pub enum InventorySizingMode {
    /// The inventory uses a fixed size.
    Fixed {
        /// The size of the inventory.
        size: usize,
    },
    /// The inventory grows and shrinks depending on the content.
    /// Slot restrictions are ignored in this mode.
    Dynamic {
        /// The minimum size of the dynamic inventory.
        min_size: usize,
        /// The maximum size of the dynamic inventory.
        max_size: usize,
    },
}

/// The way items are removed from the inventory. Indicates if empty spots are left, and if not, how to fill them.
#[derive(new, Clone, Serialize, Deserialize, Debug)]
pub enum MoveToFrontMode {
    /// Don't move items to the front when there is available space.
    None,
    /// Takes the last element and puts it where the removed one was.
    TakeLast,
    /// Moves all elements after the removed one.
    Offset,
}

// for even more complex restrictions, like limit max weight -> wrap inventory in other struct and make
// the checks there.

// TODO Complete slot restriction integration
// TODO Respect maximum stack size

/// # Generics
/// - K: Item Type
/// - S: Type of inventory location
/// - U: Custom item data
#[derive(new, Clone, Serialize, Deserialize, Debug, Builder)]
pub struct Inventory<K, S: SlotType, U: Default> {
    /// The contents of the `Inventory`.
    /// None values indicate empty but existing inventory slots.
    pub content: Vec<Option<ItemInstance<K, U>>>,
    /// Restricts what kind of item can go in different slots.
    /// This is not compatible with `InventorySizingMode::Dynamic`.
    ///
    /// Maps to the inventory content using the index.
    /// None values indicate that there are no restrictions for that slot.
    #[builder(default)]
    pub slot_restriction: Vec<Option<S>>,
    /// Configures how item deletion is handled.
    pub move_to_front: MoveToFrontMode,
    /// Configures if the inventory resizes when item are inserted/removed or not.
    pub sizing_mode: InventorySizingMode,
}

impl<K: PartialEq + Clone + Debug, S: SlotType, U: Default + Clone + Debug> Inventory<K, S, U> {
    /// Creates a new `Inventory` with a fixed slot count.
    pub fn new_fixed(count: usize) -> Inventory<K, S, U> {
        let mut content = Vec::with_capacity(count);
        (0..count).for_each(|_| content.push(None));
        let mut slot_restriction = Vec::with_capacity(count);
        (0..count).for_each(|_| slot_restriction.push(None));
        Inventory {
            content,
            slot_restriction,
            move_to_front: MoveToFrontMode::None,
            sizing_mode: InventorySizingMode::new_fixed(count),
        }
    }

    /// Creates a new dynamically sized `Inventory`. A minimum of `minimum` slots are garanteed to
    /// be present at all time. The quantity of slots will not go over `maximum`.
    pub fn new_dynamic(minimum: usize, maximum: usize) -> Inventory<K, S, U> {
        let mut content = Vec::with_capacity(minimum);
        (0..minimum).for_each(|_| content.push(None));
        Inventory {
            content,
            slot_restriction: vec![],
            move_to_front: MoveToFrontMode::None,
            sizing_mode: InventorySizingMode::new_dynamic(minimum, maximum),
        }
    }

    /// Will attempt to decrease the durability of the item at the specified index.
    /// If the item has no durability value (None) or a non zero durability, it will return this
    /// value.
    /// If the item has a durability of 0 when using it, it will break and
    /// `ItemError::ItemDestroyed` will be returned.
    pub fn use_item(&mut self, idx: usize) -> Result<Option<usize>, ItemError<K, U>> {
        if let Some(Some(ii)) = self.content.get_mut(idx) {
            if ii.durability.is_some() {
                if ii.durability.unwrap() == 0 {
                    //rm item
                    Err(ItemError::ItemDestroyed(self.delete_stack(idx)?))
                } else {
                    *ii.durability.as_mut().unwrap() -= 1;
                    Ok(Some(ii.durability.unwrap()))
                }
            } else {
                Ok(None)
            }
        } else {
            Err(ItemError::SlotEmpty)
        }
    }

    /// Decreases the stack size by one and returns the current value.
    /// Once the stack size hits zero, it will return `ItemError::StackConsumed`.
    pub fn consume(&mut self, idx: usize) -> Result<usize, ItemError<K, U>> {
        if let Some(Some(ii)) = self.content.get_mut(idx) {
            ii.quantity -= 1;
            if ii.quantity == 0 {
                Err(ItemError::StackConsumed(self.delete_stack(idx)?))
            } else {
                Ok(ii.quantity)
            }
        } else {
            Err(ItemError::SlotEmpty)
        }
    }

    /// Looks if there is enough space to add another item stack.
    pub fn has_space(&self) -> bool {
        match self.sizing_mode {
            InventorySizingMode::Fixed { size: _ } => self.content.iter().any(|o| o.is_none()),
            InventorySizingMode::Dynamic {
                min_size: _,
                max_size,
            } => self.content.len() != max_size,
        }
    }

    // TODO transfer no target (ie transfer all)

    /// Transfers a specified quantity of item from one slot of this inventory to a specified slot
    /// of the provided target inventory.
    /// with_overflow indicates if the item can be spread out in free slots in case that the target
    /// slot does not have enough free space.
    ///
    /// Errors:
    /// See `Transform::delete` and `Transform::insert_into`.
    pub fn transfer(
        &mut self,
        from_idx: usize,
        target: &mut Inventory<K, S, U>,
        to_idx: usize,
        quantity: usize,
        _with_overflow: bool,
    ) -> Result<(), ItemError<K, U>> {
        let mv = self.delete(from_idx, quantity)?;
        target.insert_into(to_idx, mv)?;
        // TODO overflow control
        // TODO stack maximum size
        Ok(())
    }

    /// Transfers a whole stack from the specified slot into a specified slot of the provided
    /// target directory.
    /// with_overflow indicates if the item can be spread out in free slots in case that the target
    /// slot does not have enough free space.
    ///
    /// Errors:
    /// See `Transform::delete` and `Transform::insert_into`.
    pub fn transfer_stack(
        &mut self,
        from_idx: usize,
        target: &mut Inventory<K, S, U>,
        to_idx: usize,
        with_overflow: bool,
    ) -> Result<(), ItemError<K, U>> {
        if let Some(Some(qty)) = self
            .content
            .get(from_idx)
            .map(|i| i.as_ref().map(|i2| i2.quantity))
        {
            self.transfer(from_idx, target, to_idx, qty, with_overflow)
        } else {
            Err(ItemError::SlotEmpty)
        }
    }

    /// Moves a specified quantity of item from a slot to another.
    /// with_overflow indicates if the item can be spread out in free slots in case that the target
    /// slot does not have enough free space.
    ///
    /// Errors:
    /// See `Inventory::delete` and `Inventory::insert_into`.
    pub fn move_item(
        &mut self,
        from_idx: usize,
        to_idx: usize,
        quantity: usize,
        _with_overflow: bool,
    ) -> Result<(), ItemError<K, U>> {
        let mv = self.delete(from_idx, quantity)?;
        self.insert_into(to_idx, mv)?;
        Ok(())
    }

    // TODO: swap item stacks

    /// Moves a full stack of item from a slot to another.
    /// with_overflow indicates if the item can be spread out in free slots in case that the target
    /// slot does not have enough free space.
    ///
    /// Errors:
    /// * SlotEmpty: Nothing is present in the specified slot.
    pub fn move_stack(
        &mut self,
        from_idx: usize,
        to_idx: usize,
        with_overflow: bool,
    ) -> Result<(), ItemError<K, U>> {
        if let Some(Some(qty)) = self
            .content
            .get(from_idx)
            .map(|i| i.as_ref().map(|i2| i2.quantity))
        {
            self.move_item(from_idx, to_idx, qty, with_overflow)
        } else {
            Err(ItemError::SlotEmpty)
        }
    }

    /// Deletes a specified quantity of item from the specified slot.
    ///
    /// Errors:
    /// * NotEnoughQuantity: Not enough items are present in the item stack.
    /// * SlotEmpty: Nothing is present in the specified slot.
    pub fn delete(
        &mut self,
        idx: usize,
        quantity: usize,
    ) -> Result<ItemInstance<K, U>, ItemError<K, U>> {
        if let Some(Some(ii)) = self.content.get_mut(idx) {
            if ii.quantity >= quantity {
                ii.quantity -= quantity;
                let mut ret = ItemInstance::new(ii.key.clone(), quantity);
                ret.durability = ii.durability.clone();

                if ii.quantity == 0 {
                    self.remove_slot(idx);
                }

                Ok(ret)
            } else {
                Err(ItemError::NotEnoughQuantity)
            }
        } else {
            Err(ItemError::SlotEmpty)
        }
    }

    fn remove_slot(&mut self, idx: usize) -> Option<ItemInstance<K, U>> {
        match self.move_to_front {
            MoveToFrontMode::None => {
                if let Some(s) = self.content.get_mut(idx) {
                    let ret = s.clone();
                    *s = None;
                    ret
                } else {
                    None
                }
            }
            MoveToFrontMode::TakeLast => {
                let ret = self.content.swap_remove(idx);
                self.content.push(None);
                ret
            }
            MoveToFrontMode::Offset => {
                let ret = self.content.remove(idx);
                self.content.push(None);
                ret
            }
        }
    }

    /// Deletes a full stack of item at the provided index and returns it.
    ///
    /// Errors:
    /// See `Transform::delete`.
    pub fn delete_stack(&mut self, idx: usize) -> Result<ItemInstance<K, U>, ItemError<K, U>> {
        if let Some(Some(qty)) = self
            .content
            .get(idx)
            .map(|i| i.as_ref().map(|i2| i2.quantity))
        {
            self.delete(idx, qty)
        } else {
            Err(ItemError::SlotEmpty)
        }
    }

    /// Deletes items by matching the key until the deleted quantity reaches the specified
    /// quantity.
    ///
    /// Errors:
    /// * NotEnoughQuantity: Not enough items with the specified key are present in the inventory.
    pub fn delete_key(
        &mut self,
        key: &K,
        quantity: usize,
    ) -> Result<ItemInstance<K, U>, ItemError<K, U>> {
        if !self.has_quantity(key, quantity) {
            return Err(ItemError::NotEnoughQuantity);
        }
        let mut remaining = quantity;
        for idx in self
            .content
            .iter()
            .enumerate()
            .filter(|(_, ii)| ii.is_some() && ii.as_ref().unwrap().key == *key)
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>()
        {
            let avail = self
                .content
                .get(idx)
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap()
                .quantity;
            let rm = if avail >= remaining { remaining } else { avail };
            remaining -= rm;
            self.delete(idx, rm)
                .expect("Failed to delete from item stack during delete_key call. This is a bug.");
            if remaining == 0 {
                return Ok(ItemInstance::new(key.clone(), quantity));
            }
        }
        unreachable!();
    }

    /// Checks if the total quantity of items of the specified key are present in the inventory.
    pub fn has_quantity(&self, key: &K, quantity: usize) -> bool {
        let sum: usize = self
            .content
            .iter()
            .flatten()
            .filter(|ii| ii.key == *key)
            .map(|ii| ii.quantity)
            .sum();
        sum >= quantity
    }

    /// Checks if the inventory contains at least one `ItemInstance` of the specified key.
    pub fn has(&self, key: &K) -> bool {
        self.content
            .iter()
            .any(|ii| ii.is_some() && ii.as_ref().unwrap().key == *key)
    }

    /// Gets an immutable reference to the `ItemInstance` at the specified index.
    pub fn get(&self, idx: usize) -> &Option<ItemInstance<K, U>> {
        self.content.get(idx).unwrap_or(&None)
    }

    /// Gets a mutable reference to the `ItemInstance` at the specified index.
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut ItemInstance<K, U>> {
        self.content
            .get_mut(idx)
            .map(|opt| opt.as_mut())
            .unwrap_or(None)
    }

    /// Finds the item instances using the specified key. Returns an iterator of immutable
    /// references.
    pub fn get_key(&self, key: &K) -> impl Iterator<Item = &ItemInstance<K, U>> {
        let key = key.clone();
        self.content
            .iter()
            .flatten()
            .filter(move |ii| ii.key == key)
    }

    /// Finds the item instances using the specified key. Returns an iterator of mutable
    /// references.
    pub fn get_key_mut(&mut self, key: &K) -> impl Iterator<Item = &mut ItemInstance<K, U>> {
        let key = key.clone();
        self.content
            .iter_mut()
            .flatten()
            .filter(move |ii| ii.key == key)
    }

    /// Inserts the `ItemInstance` into the specified index.
    ///
    /// It will eventually attempt to merge stacks together, but this is not implemented yet.
    ///
    /// Errors:
    /// * SlotOccupied: The slot is currently occupied by another item type.
    pub fn insert_into(
        &mut self,
        idx: usize,
        item: ItemInstance<K, U>,
    ) -> Result<(), ItemError<K, U>> {
        // TODO match keys and see if stackable
        let opt = self.content.get_mut(idx);
        match opt {
            Some(Some(_)) => Err(ItemError::SlotOccupied),
            Some(None) => {
                *opt.unwrap() = Some(item);
                Ok(())
            }
            None => panic!("Out of bound inventory insertion at index {}", idx),
        }
    }

    /// Inserts the `ItemInstance` at the first available inventory space.
    /// If the inventory is dynamically size, it will attempt to create a slot and insert into it.
    ///
    /// It will eventually attempt to merge stacks together, but this is not implemented yet.
    ///
    /// Errors:
    /// * InventoryFull: The inventory is full and no more space can be created.
    pub fn insert(&mut self, item: ItemInstance<K, U>) -> Result<(), ItemError<K, U>> {
        if let Some(slot) = self.first_empty_slot() {
            self.insert_into(slot, item).unwrap();
            Ok(())
        } else {
            match self.sizing_mode {
                InventorySizingMode::Fixed { size: _ } => Err(ItemError::InventoryFull),
                InventorySizingMode::Dynamic {
                    min_size: _,
                    max_size: _,
                } => {
                    // Attempt to make room.
                    if self.has_space() {
                        self.content.push(None);
                        self.insert_into(self.content.len() - 1, item).unwrap();
                        Ok(())
                    } else {
                        Err(ItemError::InventoryFull)
                    }
                }
            }
        }
    }

    /// Returns the first empty slot if any is available.
    pub fn first_empty_slot(&self) -> Option<usize> {
        match self.move_to_front {
            MoveToFrontMode::None => {
                let ret = self
                    .content
                    .iter()
                    .enumerate()
                    .find(|t| t.1.is_none())
                    .map(|t| t.0);
                ret
            }
            MoveToFrontMode::TakeLast | MoveToFrontMode::Offset => {
                let max = match self.sizing_mode {
                    InventorySizingMode::Fixed { size } => size,
                    InventorySizingMode::Dynamic {
                        min_size: _,
                        max_size,
                    } => max_size,
                };
                if self.content.len() != max {
                    Some(self.content.len())
                } else {
                    None
                }
            }
        }
    }

    // TODO first insertable for key: &K

    //pub fn first_empty_slot_filtered(&self,
}

/// The different errors that can happen when interacting with the `Inventory`.
#[derive(Debug)]
pub enum ItemError<K: PartialEq + Debug, U: Default> {
    /// The stack doesn't fit completely inside of the slot.
    StackOverflow(ItemInstance<K, U>),
    /// The inventory is full and cannot be resized anymore.
    InventoryFull,
    /// The inventory cannot fit the specified items inside of itself.
    /// It has no more empty slots, cannot be resized and no item can be stacked with others.
    InventoryOverflow(Vec<ItemInstance<K, U>>),
    /// The item was used and the durability is now 0.
    ItemDestroyed(ItemInstance<K, U>),
    /// The stack size was decreased and is now 0.
    StackConsumed(ItemInstance<K, U>),
    /// The slot already has something inside of it.
    SlotOccupied,
    /// The specified item cannot be inserted into this type of slot.
    SlotRestricted,
    /// The origin slot is locked. The item cannot be moved or inserted.
    LockedOriginSlot,
    /// The remote slot is locked. The item cannot be moved or inserted.
    LockedRemoteSlot,
    /// The slot at the specified index is empty or non-existant.
    SlotEmpty,
    /// There is not enough of the specified item to satisfy the query.
    NotEnoughQuantity,
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
