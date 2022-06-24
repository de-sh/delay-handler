use std::collections::{hash_map::Entry, HashMap};
use std::fmt::Display;
use std::hash::Hash;
use std::time::Duration;

use tokio_stream::StreamExt;
use tokio_util::time::{delay_queue::Key, DelayQueue};

/// An abstration over [`DelayQueue`] that allows you to create a delay, with associated data.
///
/// Users can add data to the delay-map with [`insert()`](DelayMap::insert). The associated data
/// is removed and returned when delay is timedout by `.await`ing on [`next()`](DelayMap::next).
/// Users can also prematurely remove the delay from the delay-map with [`remove()`](DelayMap::remove).
///
/// ### Examples
/// 1. Insert 3 numbers into delay-map with 10s delays, print them as they timeout
/// ```rust
/// # use delay_map::DelayMap;
/// let mut delay_map = DelayMap::default();
/// // Adds 1, 2, 3 to the delay-map, each with 10s delay
/// delay_map.insert(1, Duration::from_secs(10));
/// delay_map.insert(2, Duration::from_secs(10));
/// delay_map.insert(3, Duration::from_secs(10));
///
/// // Expect a delay of ~10s, after which 1, 2, 3 should print to stdout, in quick succession.
/// While let Some(expired) in delay_map.next().await {
///     println!("{}", expired);
/// }
/// ```
/// 2. Insert 3 numbers into delay-map with different delays, print them as they timeout
/// ```rust
/// # use delay_map::DelayMap;
/// let mut delay_map = DelayMap::default();
/// // Adds 1, 2 to the delay-map, with different delays
/// delay_map.insert(1, Duration::from_secs(10));
/// delay_map.insert(2, Duration::from_secs(5));
///
/// // With a delay of ~5s between, the prints should come in the order of 2 and 1.
/// While let Some(expired) in delay_map.next().await {
///     println!("{}", expired);
/// }
/// ```
///
/// 3. Insert 3 numbers into delay-map with different delays, remove  print as delays are timedout
/// ```rust
/// # use delay_map::DelayMap;
/// let mut delay_map = DelayMap::default();
/// // Adds 1, 2, 3 to the delay-map, each with different delays
/// delay_map.insert(1, Duration::from_secs(15));
/// delay_map.insert(2, Duration::from_secs(5));
/// delay_map.insert(3, Duration::from_secs(10));
///
/// // Remove 3 from the delay-map
/// delay_map.remove(&3);
///
/// // Prints should be in the order of first 2 and ~10s later 1.
/// While let Some(expired) in delay_map.next().await {
///     println!("{}", expired);
/// }
/// ```
pub struct DelayMap<T> {
    queue: DelayQueue<T>,
    map: HashMap<T, Key>,
}

impl<T> DelayMap<T>
where
    T: Eq + Hash + Clone + Display,
{
    /// Insert new timeout into the map and queue if it doesn't already exist.
    /// If one already exists, don't .
    pub fn insert(&mut self, item: T, period: Duration) -> bool {
        match self.map.entry(item.clone()) {
            Entry::Vacant(v) => {
                let key = self.queue.insert(item, period);
                v.insert(key);

                true
            }
            _ => false,
        }
    }

    /// Prematurely removes timeout from delay-map, if it didn't already exist returns false.
    pub fn remove(&mut self, item: &T) -> bool {
        match self.map.remove(item) {
            Some(key) => {
                self.queue.remove(&key).into_inner();

                true
            }
            _ => false,
        }
    }

    /// Remove a key from map if it has timedout and return the name.
    pub async fn next(&mut self) -> Option<T> {
        let item = self.queue.next().await?.into_inner();
        self.map.remove(&item);

        Some(item)
    }

    /// Check if queue is empty. Could be used as precondition in an async select operation.
    /// NOTE: The following example assumes usage of `tokio::select`
    ///
    /// ```no_run
    /// select! {
    ///     ...
    ///     Some(expired) = delay_map.next(), if !delay_map.is_empty() => println!("{}", expired)
    ///     ...
    /// }
    /// ```
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

impl<T> Default for DelayMap<T>
where
    T: Eq + Hash + Clone + Display,
{
    fn default() -> Self {
        Self {
            queue: DelayQueue::new(),
            map: HashMap::new(),
        }
    }
}
