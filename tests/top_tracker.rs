#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use top_100_ips::top_tracker::TopTracker;

#[test]
fn empty_trackers_have_no_top_items() {
    let tracker: TopTracker<u8> = TopTracker::new(10);

    assert_eq!(tracker.top().len(), 0);
}

#[test]
fn tracking_0_items_has_no_top_items() {
    let mut tracker = TopTracker::new(0);
    tracker.increment(0);

    assert_eq!(tracker.top().len(), 0);
}

#[test]
fn more_top_items_than_unique_events() {
    let mut tracker = TopTracker::new(10);
    tracker.increment('A');
    tracker.increment('B');
    tracker.increment('C');

    assert_eq!(tracker.top(), vec![('A', 1), ('B', 1), ('C', 1)]);
}

#[test]
fn keeps_track_of_frequency() {
    let mut tracker = TopTracker::new(3);
    for i in 1..=5 {
        for _ in 0..i {
            // Add n, n times.
            tracker.increment(i);
            println!("{:?}", tracker.top());
        }
    }

    assert_eq!(tracker.top(), vec![(5, 5), (4, 4), (3, 3)]);
}

#[quickcheck]
fn never_keeps_too_many(items: Vec<u8>, size: u8) -> bool {
    let top = top_n(items, size);

    top.len() <= size.into()
}

#[quickcheck]
fn test_sorted(items: Vec<u8>) -> bool {
    let mut top_sorted = top_n(items, 100);
    let top = top_sorted.clone();

    top_sorted.sort_by_key(|(_, count)| -1 * (*count as i128));

    top == top_sorted
}

fn top_n<A>(items: Vec<A>, size: u8) -> Vec<(A, u64)>
where
    A: std::cmp::Eq + std::hash::Hash + Copy,
{
    let mut tracker = TopTracker::new(size.into());

    for item in items {
        tracker.increment(item);
    }

    tracker.top()
}
