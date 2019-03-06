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


// even more complex restrictions, like limit max weight -> wrap inventory in other struct and make
// the checks there.
pub struct Inventory<K, S: SlotType<T>, T, D> {
    pub content: Vec<Option<ItemInstance<K, D, T>>>, 
    pub slot_restriction: Vec<Option<S>>,
    /// Forces items to go as far as possible to the front of the list, filing any empty space.
    pub move_to_front: bool,
    pub max_size: Option<u32>,
}

impl<K> Inventory<K> {
    pub fn use(&mut self, idx: u32) -> Result<(), ItemError> {
        if let Some(ii) = self.content.get_mut(idx) {
            if ii.durability.is_some() {
                ii.durability -= 1;
                if ii.durability.unwrap() < 0 {
                    //rm item
                    self.content.
                }
            }
        }
    }

    pub fn consume(&mut self, idx: u32) -> Result<(), ItemError> {
        // decrease stack size
    }

    pub fn has_space(&self) -> bool {
        if let Some(s) = self.max_size {
            if 
        } else {
            true
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

    pub fn has(&self, key: K) -> bool {

    }

    pub fn get(&self, idx: u32) -> Option<&ItemInstance<K>> {
        
    }

    pub fn get_mut(&self, idx: u32) -> Option(&mut ItemInstance<K>) {

    }

    pub fn get_key(&self, key: K) -> Vec<&ItemInstance<K>> {

    }

    pub fn get_key_mut(&mut self, key: K) -> Vec<&mut ItemInstance<K>> {
        
    }

    pub fn insert_into(&mut self, item: ItemInstance<K>) -> Result<(), ItemError> {

    }

    pub fn insert(&mut self, item: ItemInstance<K>) -> Result<(), ItemError> {

    }

    pub fn first_empty_slot(&self) -> Option<u32> {

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
