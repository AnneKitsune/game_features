use std::collections::HashMap;

// TODO: extensibility. ie weight

pub struct ItemDefinition<K, D, T> {
    pub key: K,
    pub item_type: T,
    pub name: String,
    pub description: String,
    pub maximum_stack: u32,
    pub maximum_durability: Option<u32>,
    pub user_data: D,
}

pub struct ItemInstance<K> {
    pub item_key: K,
    pub count: u32,
    pub durability: Option<u32>,
}

pub type ItemDefinitionRepository<K> = HashMap<K, ItemDefinition<K>>;

pub trait SlotType<T> {
    fn can_insert_into(&self, item: &ItemDefinition) -> bool;
}

/// The way the inventory size is handled.
pub enum InventorySizingMode {
    /// The inventory uses a fixed size.
    Fixed(size: u32),
    /// The inventory grows and shrinks depending on the content.
    /// Slot restrictions are ignored in this mode.
    Dynamic(min_size: u32, max_size: u32),
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
pub struct Inventory<K, S: SlotType<T>, T, D> {
    pub content: Vec<Option<ItemInstance<K, D, T>>>, 
    /// Restricts what kind of item can go in different slots.
    /// This is not compatible with `InventorySizingMode::Dynamic`.
    pub slot_restriction: Vec<Option<S>>,
    /// Configures how item deletion is handled.
    pub move_to_front: MoveToFrontMode,
    /// Configures if the inventory resizes when item are inserted/removed or not.
    pub sizing_mode: InventorySizingMode,
}

impl<K> Inventory<K> {

    /// Will attempt to decrease the durability of the item at the specified index.
    /// If the item has no durability value (None) or a non zero durability, it will return this
    /// value.
    /// If the item has a durability of 0 when using it, it will break and
    /// `ItemError::ItemDestroyed` will be returned.
    pub fn use(&mut self, idx: u32) -> Result<Option<u32>, ItemError> {
        if let Some(ii) = self.content.get_mut(idx) {
            if ii.durability.is_some() {
                if ii.durability.unwrap() == 0 {
                    //rm item
                    Err(ItemError::ItemDestroyed(self.delete_stack(idx).unwrap())
                } else {
                    ii.durability -= 1;
                    Ok(Some(ii.durability))
                }
            } else {
                None
            }
        }
        Ok(())
    }

    /// Decreases the stack size by one and returns the current value.
    /// Once the stack size hits zero, it will return `ItemError::StackEmpty`.
    pub fn consume(&mut self, idx: u32) -> Result<u32, ItemError> {
        if let Some(ii) = self.content.get_mut(idx) {
            ii.count -= 1;
            if ii.count == 0 {
                Err(ItemError::StackEmpty(self.delete_stack(idx).unwrap))
            } else {
                ii.count
            }
        }
        Ok(())
    }

    /// Looks if there is enough space to add another item stack.
    pub fn has_space(&self) -> bool {
        match self.sizing_mode {
            SizingMode::Fixed(size) => {
                self.content.iter().contains(None)
            },
            SizingMode::Dynamic(_, max) => self.content.len() != max,
        }
    }

    pub fn transfer(&mut self, from_idx: u32, target: &mut Inventory, to_idx: u32, quantity: u32) -> Result<(), ItemError> {

    }

    pub fn transfer_stack(&mut self, from_idx: u32, target: &mut Inventory, to_idx: u32) -> Result<(), ItemError> {

    }

    pub fn move(&mut self, from_idx: u32, to_idx: u32, quantity: u32, with_overflow: bool) -> Result<(), ItemError> {

    }

    pub fn move_stack(&mut self, from_idx: u32, to_idx: u32, with_overflow: bool) -> Result<(), ItemError> {

    }

    pub fn delete(&mut self, idx: u32, quantity: u32) -> Result<ItemInstance<K>, ItemError> {

    }

    pub fn delete_stack(&mut self, idx: u32) -> Result<ItemInstance<K>, ItemError> {

    }

    pub fn delete_key(&mut self, key: K, quantity: u32) -> Result<ItemInstance<K>, ItemError> {

    }

    pub fn has_quantity(&self, key: K, quantity: u32) -> bool {

    }

    /// Checks if the inventory contains at least one `ItemInstance` of the specified key.
    pub fn has(&self, key: K) -> bool {
        self.content.iter().any(|ii| ii.key == key)
    }

    /// Gets an immutable reference to the `ItemInstance` at the specified index.
    pub fn get(&self, idx: u32) -> Option<&ItemInstance<K>> {
        self.content.get(idx)
    }

    /// Gets a mutable reference to the `ItemInstance` at the specified index.
    pub fn get_mut(&self, idx: u32) -> Option(&mut ItemInstance<K>) {
        self.content.get_mut(idx)
    }

    /// Finds the item instances using the specified key. Returns an iterator of immutable
    /// references.
    pub fn get_key(&self, key: K) -> Iterator<Item = <&mut ItemInstance<K>>> {
        self.content.iter().filter(|ii| ii.key == key)
    }

    /// Finds the item instances using the specified key. Returns an iterator of mutable
    /// references.
    pub fn get_key_mut(&mut self, key: K) -> Iterator<Item = <&mut ItemInstance<K>>> {
       self.content.iter_mut().filter(|ii| ii.key == key) 
    }

    pub fn insert_into(&mut self, item: ItemInstance<K>) -> Result<(), ItemError> {

    }

    pub fn insert(&mut self, item: ItemInstance<K>) -> Result<(), ItemError> {

    }

    pub fn first_empty_slot(&self) -> Option<u32> {
        match self.move_to_front_mode {
            MoveToFrontMode::None => 
            MoveToFrontMode::TakeLast | MoveToFrontMode::Offset => {
                
        }
    }

    pub fn first_empty_slot_filtered(&self, 
}

pub enum ItemError {
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

pub struct SingleEquippedItem {
    pub equipped_index: u32,
}

impl SingleEquippedItem {
    pub fn get_equipped(&self, &Inventory) -> Option<&ItemInstance<K>> {

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
