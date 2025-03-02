use partial_function::LowerPartialFunction;
use rand::{rng, Rng};

/// A weighted node of a loot tree with the corresponding result.
#[derive(Deserialize)]
pub struct LootTreeNode<R> {
    /// The weight of this node.
    pub chances: i32,
    /// The result of this node.
    pub result: R,
}

/// A builder for the `LootTree`.
#[derive(Deserialize)]
pub struct LootTreeBuilder<R> {
    /// The nodes contained in this builder.
    pub nodes: Vec<LootTreeNode<R>>,
}

impl<R: Clone + 'static> LootTreeBuilder<R> {
    /// Creates a new builder.
    pub fn new() -> Self {
        LootTreeBuilder { nodes: vec![] }
    }

    /// Builds the loot tree.
    pub fn build(self) -> LootTree<R> {
        let mut f = LowerPartialFunction::new();
        let mut accum = 0;
        for n in self.nodes.into_iter() {
            let tmp = n.chances;
            f = f.with(accum, Box::new(move |_| n.result.clone()));
            accum = accum + tmp;
        }
        LootTree {
            partial_func: f.build(),
            max: accum,
        }
    }
}

/// A loot tree based on the lower partial function construct.
/// Each loot tree node has a chance associated with it.
///
/// Example:
/// { chance: 5, result: "item1" }
/// { chance: 2, result: "item2" }
///
/// Internally this becomes
/// [0,infinite[ -> item1
/// [5,infinite[ -> item2
/// maximum = 7 exclusive (that means 6)
///
/// Chances will effectively be:
/// [0,4] (5) -> item1
/// [5,6] (2) -> item2
pub struct LootTree<R> {
    partial_func: LowerPartialFunction<i32, R>,
    max: i32,
}

impl<R> LootTree<R> {
    /// Returns a random item from the loot tree.
    pub fn roll(&self) -> Option<R> {
        let rng = rng().random_range(0..self.max);
        self.partial_func.eval(rng)
    }
}
