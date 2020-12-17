//! The data structure backing `Top100Ips`. For something more flexible, try this.
use std::collections::HashMap;

/// Keeps track of high frequency items.
///
/// `TopTracker` maintains a sorted list of all the most frequent items. It increments items in
/// constant time and uses memory proportional to the number of unique items.
pub struct TopTracker<A> {
    /// Number of top items to track.
    n: usize,

    /// The top n items in order, keyed by id.
    top_items: Vec<TopItem<A>>,

    /// All the items and their details
    item_summaries: HashMap<A, ItemSummary>,
}

struct ItemSummary {
    // Keeping track of the positions is not strictly necessary, but does reduce the bug surface
    // area. Adding an extra Option<usize> in a privial price to pay to avoid implementing a binary
    // search and dealing with the boundry conditions of searching.
    /// The current index of this event in `top_items`.
    position: Option<usize>,

    /// How many times increment(item) was called.
    count: u64,
}

struct TopItem<A> {
    /// The item `incremente`ed.
    item: A,

    /// The number of times `increment(item)` was called.
    count: u64,
}

impl<A> TopTracker<A>
where
    A: std::cmp::Eq + std::hash::Hash + Copy,
{
    /// Creates a `TopTracker` tracking the top `n` items.
    pub fn new(n: usize) -> TopTracker<A> {
        TopTracker {
            n,
            top_items: Vec::with_capacity(n),
            item_summaries: HashMap::with_capacity(n),
        }
    }

    /// Record an item's observation
    pub fn increment(&mut self, item: A) {
        let (position_in_top_items, item_occurence) = {
            let summary = self
                .item_summaries
                .entry(item)
                .and_modify(|summary| summary.count += 1)
                .or_insert(ItemSummary {
                    position: None,
                    count: 1,
                });

            (summary.position, summary.count)
        };

        // Add Count to existing top items
        if let Some(position) = position_in_top_items {
            self.top_items[position].count += 1;
        }

        // Items not in top_item have potential to be top_items...
        let item_index = position_in_top_items.or_else(|| {
            // if we don't have enough top_items,
            if self.top_items.len() != self.n {
                self.top_items.push(TopItem {
                    item,
                    count: item_occurence,
                });

                return Some(self.top_items.len() - 1);
            }

            // or it is bigger than the smallest top_item.
            if let Some(min_top_value) = self.top_items.last() {
                // Note: If the last() value was none, self.top_items.len() is 0.

                if min_top_value.count < item_occurence {
                    let new_position = self.top_items.len() - 1;
                    self.top_items[new_position] = TopItem {
                        item,
                        count: item_occurence,
                    };

                    return Some(new_position);
                }
            };

            None
        });

        self.item_summaries
            .get_mut(&item)
            .expect("item_summaries did not contain item")
            .position = item_index.map(|index| self.bubble(index))
    }

    /// The top items tracked.
    ///
    /// Note: In the case of a tie, the item order is undefined.
    pub fn top(&self) -> Vec<(A, u64)> {
        self.top_items
            .iter()
            .map(|TopItem { item, count }| (*item, *count))
            .collect()
    }

    fn bubble(&mut self, index: usize) -> usize {
        let mut index = index;

        while index != 0 && self.top_items[index - 1].count < self.top_items[index].count {
            self.top_items.swap(index - 1, index);
            index = index - 1;
        }

        return index;
    }
}
