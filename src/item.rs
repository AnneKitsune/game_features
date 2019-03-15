
use std::fmt::Debug;
use std::marker::PhantomData;
use std::collections::HashMap;

// TODO: extensibility. ie weight

#[derive(new, Clone, Serialize, Deserialize, Debug, Builder)]
pub struct ItemDefinition<K, T, D: Default> {
    pub key: K,
    pub item_type: T,
    pub name: String,
    pub friendly_name: String,
    pub description: String,
    pub maximum_stack: usize,
    pub maximum_durability: Option<usize>,
    #[builder(default)]
    pub user_data: D,
}

#[derive(new, Clone, Serialize, Deserialize, Debug, Builder)]
pub struct ItemInstance<K, U: Default> {
    pub item_key: K,
    pub count: usize,
    #[new(default)]
    #[builder(default)]
    pub durability: Option<usize>,
    #[new(default)]
    #[builder(default)]
    pub user_data: U,
}


pub type ItemDefinitionRepository<K, T, D> = HashMap<K, ItemDefinition<K, T, D>>;

pub trait SlotType<T> {
    fn can_insert_into(&self, item_type: &T) -> bool;
}

impl SlotType<()> for () {
    fn can_insert_into(&self, item_type: &()) -> bool {
        true
    }
}

impl SlotType<usize> for usize {
    fn can_insert_into(&self, item_type: &usize) -> bool {
        *self == *item_type
    }
}

/// The way the inventory size is handled.
#[derive(new, Clone, Serialize, Deserialize, Debug)]
pub enum InventorySizingMode {
    /// The inventory uses a fixed size.
    Fixed{size: usize},
    /// The inventory grows and shrinks depending on the content.
    /// Slot restrictions are ignored in this mode.
    Dynamic{min_size: usize, max_size: usize},
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

// TODO Inventory definition separated from inventory instance?
// TODO Complete slot restriction integration

#[derive(new, Clone, Serialize, Deserialize, Debug, Builder)]
pub struct Inventory<K, T, S: SlotType<T>, U: Default> {
    pub content: Vec<Option<ItemInstance<K, U>>>, 
    /// Restricts what kind of item can go in different slots.
    /// This is not compatible with `InventorySizingMode::Dynamic`.
    #[builder(default)]
    pub slot_restriction: Vec<Option<S>>,
    /// Configures how item deletion is handled.
    pub move_to_front: MoveToFrontMode,
    /// Configures if the inventory resizes when item are inserted/removed or not.
    pub sizing_mode: InventorySizingMode,
    #[builder(default)]
    #[new(default)]
    _phantom: PhantomData<T>,
}

impl<K: PartialEq + Clone + Debug, T, S: SlotType<T>, U: Default + Clone + Debug> Inventory<K, T, S, U> {

    pub fn new_fixed(count: usize) -> Inventory<K, T, S, U> {
        let mut content = Vec::with_capacity(count);
        (0..count).for_each(|_| content.push(None));
        let mut slot_restriction = Vec::with_capacity(count);
        (0..count).for_each(|_| content.push(None));
        Inventory {
            content,
            slot_restriction,
            move_to_front: MoveToFrontMode::None,
            sizing_mode: InventorySizingMode::new_fixed(count),
            _phantom: PhantomData,
        }
    }

    pub fn new_dynamic(minimum: usize, maximum: usize) -> Inventory<K, T, S, U> {
        let mut content = Vec::with_capacity(minimum);
        (0..minimum).for_each(|_| content.push(None));
        Inventory {
            content,
            slot_restriction: vec![],
            move_to_front: MoveToFrontMode::None,
            sizing_mode: InventorySizingMode::new_dynamic(minimum, maximum),
            _phantom: PhantomData,
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

    // TODO transfer no target (ie transfer all)

    pub fn transfer(&mut self, from_idx: usize, target: &mut Inventory<K, T, S, U>, to_idx: usize, quantity: usize, with_overflow: bool) -> Result<(), ItemError<K, U>> {
        let mv = self.delete(from_idx, quantity)?;
        target.insert_into(to_idx, mv)?;
        // TODO overflow control
        Ok(())
    }

    pub fn transfer_stack(&mut self, from_idx: usize, target: &mut Inventory<K, T, S, U>, to_idx: usize, with_overflow: bool) -> Result<(), ItemError<K, U>> {
        if let Some(Some(ii)) = self.content.get(from_idx) {
            self.transfer(from_idx, target, to_idx, ii.count, with_overflow)
        } else {
            Err(ItemError::SlotEmpty)
        }
    }

    pub fn move_item(&mut self, from_idx: usize, to_idx: usize, quantity: usize, with_overflow: bool) -> Result<(), ItemError<K, U>> {
        let mv = self.delete(from_idx, quantity)?;
        self.insert_into(to_idx, mv)?;
        Ok(())
    }

    // TODO: swap item stacks

    pub fn move_stack(&mut self, from_idx: usize, to_idx: usize, with_overflow: bool) -> Result<(), ItemError<K, U>> {
        if let Some(Some(ii)) = self.content.get(from_idx) {
            self.move_item(from_idx, to_idx, ii.count, with_overflow)
        } else {
            Err(ItemError::SlotEmpty)
        }
    }

    pub fn delete(&mut self, idx: usize, quantity: usize) -> Result<ItemInstance<K, U>, ItemError<K, U>> {
        if let Some(Some(ii)) = self.content.get_mut(idx) {
            if ii.count >= quantity {
                ii.count -= quantity;
                let mut ret = ItemInstance::new(ii.item_key.clone(), quantity);
                ret.durability = ii.durability.clone();

                if ii.count == 0 {
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

    pub fn delete_stack(&mut self, idx: usize) -> Result<ItemInstance<K, U>, ItemError<K, U>> {
        if let Some(Some(ii)) = self.content.get(idx) {
            self.delete(idx, ii.count)
        } else {
            Err(ItemError::SlotEmpty)
        }
    }

    
    pub fn delete_key(&mut self, key: &K, quantity: usize) -> Result<ItemInstance<K, U>, ItemError<K, U>> {
        if !self.has_quantity(key, quantity) {
            return Err(ItemError::NotEnoughQuantity);
        }
        let mut remaining = quantity;
        for idx in self.content.iter().enumerate().filter(|(_, ii)| ii.is_some() && ii.as_ref().unwrap().item_key == *key).map(|(idx, _)| idx).collect::<Vec<_>>() {
            let avail = self.content.get(idx).as_ref().unwrap().as_ref().unwrap().count;
            let rm = if avail >= remaining {
                remaining
            } else {
                avail
            };
            remaining -= rm;
            self.delete(idx, rm).expect("Failed to delete from item stack during delete_key call. This is a bug.");
            if remaining == 0 {
                return Ok(ItemInstance::new(key.clone(), quantity));
            }
        }
        unreachable!();
    }

    pub fn has_quantity(&self, key: &K, quantity: usize) -> bool {
        let sum: usize = self.content.iter().flatten().filter(|ii| ii.item_key == *key).map(|ii| ii.count).sum();
        sum >= quantity
    }

    /// Checks if the inventory contains at least one `ItemInstance` of the specified key.
    pub fn has(&self, key: &K) -> bool {
        self.content.iter().any(|ii| ii.is_some() && ii.as_ref().unwrap().item_key == *key)
    }

    /// Gets an immutable reference to the `ItemInstance` at the specified index.
    pub fn get(&self, idx: usize) -> &Option<ItemInstance<K, U>> {
        self.content.get(idx).unwrap_or(&None)
    }

    /// Gets a mutable reference to the `ItemInstance` at the specified index.
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut ItemInstance<K, U>> {
        self.content.get_mut(idx).map(|opt| opt.as_mut()).unwrap_or(None)
    }

    /// Finds the item instances using the specified key. Returns an iterator of immutable
    /// references.
    pub fn get_key(&self, key: &K) -> impl Iterator<Item = &ItemInstance<K, U>> {
        let key = key.clone();
        self.content.iter().flatten().filter(move |ii| ii.item_key == key)
    }

    /// Finds the item instances using the specified key. Returns an iterator of mutable
    /// references.
    pub fn get_key_mut(&mut self, key: &K) -> impl Iterator<Item = &mut ItemInstance<K, U>> {
        let key = key.clone();
        self.content.iter_mut().flatten().filter(move |ii| ii.item_key == key)
    }

    pub fn insert_into(&mut self, idx: usize, item: ItemInstance<K, U>) -> Result<(), ItemError<K, U>> {
        // TODO match keys and see if stackable
        if let Some(opt) = self.content.get_mut(idx) {
            if opt.is_some() {
                return Err(ItemError::SlotOccupied);
            }
            *opt = Some(item);
            Ok(())
        } else {
            panic!("Out of bound inventory insertion at index {}", idx);
        }
    }

    pub fn insert(&mut self, item: ItemInstance<K, U>) -> Result<(), ItemError<K, U>> {
        if let Some(slot) = self.first_empty_slot() {
            self.insert_into(slot, item)
        } else {
            match self.sizing_mode {
                InventorySizingMode::Fixed{size} => Err(ItemError::InventoryFull),
                InventorySizingMode::Dynamic{min_size, max_size} => {
                    // Attempt to make room.
                    if self.has_space() {
                        self.content.push(None);
                        self.insert_into(self.content.len() - 1, item)
                    } else {
                        Err(ItemError::InventoryFull)
                    }
                }
            }
        }
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

    // TODO first insertable for key: &K

    //pub fn first_empty_slot_filtered(&self, 
}

#[derive(Debug)]
pub enum ItemError<K: PartialEq + Debug, U: Default> {
    StackOverflow(ItemInstance<K, U>),
    InventoryFull,
    InventoryOverflow(Vec<ItemInstance<K, U>>),
    ItemDestroyed(ItemInstance<K, U>),
    StackConsumed(ItemInstance<K, U>),
    SlotOccupied,
    SlotRestricted,
    LockedOriginSlot,
    LocketRemoteSlot,
    SlotEmpty,
    NotEnoughQuantity,
}

// TODO extra stuff
/*pub struct SingleEquippedItem<K> {
    pub equipped_index: usize,
    _phantom: PhantomData<K>,
}

impl<K: PartialEq> SingleEquippedItem<K> {
    pub fn get_equipped(&self, inventory: &Inventory<K, T, D, S>) -> Option<&ItemInstance<K, U>> {

    }
}

pub struct BaseRecipeDefinition<K: PartialEq> {
    pub inputs: Vec<ItemInstance<K, U>>,
    pub outputs: Vec<ItemInstance<K, U>>,
}

trait Recipe<K> {
    fn craft(&mut self, inputs: Vec<ItemInstance<K, U>>) -> Vec<ItemInstance<K, U>>;
}*/

#[cfg(test)]
mod test {
    use crate::*;

    #[derive(new, Debug, Clone, Serialize, Deserialize)]
    struct CustomItemDefinitionData {
        pub weight: f32,
    }
    
    #[derive(new, Debug, Clone, Serialize, Deserialize)]
    struct CustomItemInstanceData {
        pub xp: f32,
    }

    #[derive(new, Debug, Clone, Serialize, Deserialize, PartialEq)]
    enum ItemType {
        Regular,
        Armor,
        Heavy,
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
        
        let item_def = ItemDefinitionBuilder::new().item_key(1u32).item_type(ItemType::Regular).name(String::from("Apple")).friendly_name("apple").description("Food is nice").maximum_stack(16).maximum_durability(None).user_data(CustomItemDefinitionData::new(1.0)).build();
        let ii = ItemInstanceBuilder::new().item_key(1u32).quantity(1).durability(Some(64)).user_data(CustomItemInstanceData::new(0.0)).build();
        let mut inv = Inventory::<u32, ItemType, ItemType, CustomItemInstanceData>::new_fixed(8);
        inv.insert(ii).expect("");
    }
}
