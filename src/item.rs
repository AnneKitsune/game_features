use std::marker::PhantomData;
use std::collections::HashMap;

// TODO: extensibility. ie weight

pub struct ItemDefinition<K, T, D> {
    pub key: K,
    pub item_type: T,
    pub name: String,
    pub description: String,
    pub maximum_stack: usize,
    pub maximum_durability: Option<usize>,
    pub user_data: D,
}

pub struct ItemInstance<K: PartialEq> {
    pub item_key: K,
    pub count: usize,
    pub durability: Option<usize>,
}

pub type ItemDefinitionRepository<K, T, D> = HashMap<K, ItemDefinition<K, T, D>>;

pub trait SlotType<K, T, D> {
    fn can_insert_into(&self, item: &ItemDefinition<K, T, D>) -> bool;
}

/// The way the inventory size is handled.
pub enum InventorySizingMode {
    /// The inventory uses a fixed size.
    Fixed{size: usize},
    /// The inventory grows and shrinks depending on the content.
    /// Slot restrictions are ignored in this mode.
    Dynamic{min_size: usize, max_size: usize},
}

pub enum MoveToFrontMode {
    /// Don't move items to the front when there is available space.
    None,
    /// Takes the last element and puts it where the removed one was.
    TakeLast,
    /// Moves all elements after the removed one.
    Offset,
}

// even more complex restrictions, like limit max weight -> wrap inventory in other struct and make
// the checks there.
pub struct Inventory<K, T, D, S: SlotType<K, T, D>> {
    pub content: Vec<Option<ItemInstance<K>>>, 
    /// Restricts what kind of item can go in different slots.
    /// This is not compatible with `InventorySizingMode::Dynamic`.
    pub slot_restriction: Vec<Option<S>>,
    /// Configures how item deletion is handled.
    pub move_to_front: MoveToFrontMode,
    /// Configures if the inventory resizes when item are inserted/removed or not.
    pub sizing_mode: InventorySizingMode,
}

impl<K: PartialEq, T, D, S: SlotType<K, T, D>> Inventory<K, T, D, S> {

    /// Will attempt to decrease the durability of the item at the specified index.
    /// If the item has no durability value (None) or a non zero durability, it will return this
    /// value.
    /// If the item has a durability of 0 when using it, it will break and
    /// `ItemError::ItemDestroyed` will be returned.
    pub fn use_item(&mut self, idx: usize) -> Result<Option<usize>, ItemError<K>> {
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
    pub fn consume(&mut self, idx: usize) -> Result<usize, ItemError<K>> {
        if let Some(Some(ii)) = self.content.get_mut(idx) {
            ii.count -= 1;
            if ii.count == 0 {
                Err(ItemError::StackConsumed(self.delete_stack(idx)?))
            } else {
                Ok(ii.count)
            }
        } else {
            Err(ItemError::SlotEmpty)
        }
    }

    /// Looks if there is enough space to add another item stack.
    pub fn has_space(&self) -> bool {
        match self.sizing_mode {
            InventorySizingMode::Fixed{size} => {
                self.content.iter().any(|o| o.is_none())
            },
            InventorySizingMode::Dynamic{min_size, max_size} => self.content.len() != max_size,
        }
    }

    pub fn transfer(&mut self, from_idx: usize, target: &mut Inventory<K, T, D, S>, to_idx: usize, quantity: usize) -> Result<(), ItemError<K>> {

    }

    pub fn transfer_stack(&mut self, from_idx: usize, target: &mut Inventory<K, T, D, S>, to_idx: usize) -> Result<(), ItemError<K>> {

    }

    pub fn move_item(&mut self, from_idx: usize, to_idx: usize, quantity: usize, with_overflow: bool) -> Result<(), ItemError<K>> {

    }

    pub fn move_stack(&mut self, from_idx: usize, to_idx: usize, with_overflow: bool) -> Result<(), ItemError<K>> {

    }

    pub fn delete(&mut self, idx: usize, quantity: usize) -> Result<ItemInstance<K>, ItemError<K>> {

    }

    pub fn delete_stack(&mut self, idx: usize) -> Result<ItemInstance<K>, ItemError<K>> {

    }

    pub fn delete_key(&mut self, key: K, quantity: usize) -> Result<ItemInstance<K>, ItemError<K>> {

    }

    pub fn has_quantity(&self, key: K, quantity: usize) -> bool {

    }

    /// Checks if the inventory contains at least one `ItemInstance` of the specified key.
    pub fn has(&self, key: K) -> bool {
        self.content.iter().any(|ii| ii.is_some() && ii.unwrap().item_key == key)
    }

    /// Gets an immutable reference to the `ItemInstance` at the specified index.
    pub fn get(&self, idx: usize) -> &Option<ItemInstance<K>> {
        self.content.get(idx).unwrap_or(&None)
    }

    /// Gets a mutable reference to the `ItemInstance` at the specified index.
    pub fn get_mut(&self, idx: usize) -> &mut Option<ItemInstance<K>> {
        self.content.get_mut(idx).unwrap_or(&mut None)
    }

    /// Finds the item instances using the specified key. Returns an iterator of immutable
    /// references.
    pub fn get_key(&self, key: K) -> impl Iterator<Item = &ItemInstance<K>> {
        self.content.iter().flatten().filter(|ii| ii.item_key == key)
    }

    /// Finds the item instances using the specified key. Returns an iterator of mutable
    /// references.
    pub fn get_key_mut(&mut self, key: K) -> impl Iterator<Item = &mut ItemInstance<K>> {
       self.content.iter_mut().flatten().filter(|ii| ii.item_key == key)
    }

    pub fn insert_into(&mut self, item: ItemInstance<K>) -> Result<(), ItemError<K>> {

    }

    pub fn insert(&mut self, item: ItemInstance<K>) -> Result<(), ItemError<K>> {

    }

    pub fn first_empty_slot(&self) -> Option<usize> {
        match self.move_to_front {
            MoveToFrontMode::None => {
                let mut ret = self.content.iter().enumerate().find(|t| t.1.is_none()).map(|t| t.0);
                if let InventorySizingMode::Dynamic{min_size, max_size} = self.sizing_mode {
                    if ret.is_none() && self.content.len() < max_size {
                        ret = Some(self.content.len());
                    }
                }
                ret
            },
            MoveToFrontMode::TakeLast | MoveToFrontMode::Offset => {
                let max = match self.sizing_mode {
                    InventorySizingMode::Fixed{size} => size,
                    InventorySizingMode::Dynamic{min_size, max_size} => max_size,
                };
                if self.content.len() != max {
                    Some(self.content.len())
                } else {
                    None
                }
            }
        }
    }

    //pub fn first_empty_slot_filtered(&self, 
}

pub enum ItemError<K: PartialEq> {
    StackOverflow(ItemInstance<K>),
    InventoryFull(Vec<ItemInstance<K>>),
    InventoryOverflow(Vec<ItemInstance<K>>),
    ItemDestroyed(ItemInstance<K>),
    StackConsumed(ItemInstance<K>),
    SlotOccupied,
    SlotRestricted,
    LockedOriginSlot,
    LocketRemoteSlot,
    SlotEmpty,
    NotEnoughQuantity,
}

pub struct SingleEquippedItem<K> {
    pub equipped_index: usize,
    _phantom: PhantomData<K>,
}

impl<K> SingleEquippedItem<K> {
    pub fn get_equipped(&self, inventory: &Inventory<K, T, D, S>) -> Option<&ItemInstance<K>> {

    }
}

pub struct BaseRecipeDefinition<K> {
    pub inputs: Vec<ItemInstance<K>>,
    pub outputs: Vec<ItemInstance<K>>,
}

trait Recipe<K> {
    fn craft(&mut self, inputs: Vec<ItemInstance<K>>) -> Vec<ItemInstance<K>>;
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn inventory() {
        
    }
    
    #[test]
    fn complex_items() {
       // Weight, enchants and Effects 
    }
}
