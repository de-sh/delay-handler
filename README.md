# DelayHandler

[![crates.io page](https://img.shields.io/crates/v/delay-handler.svg)](https://crates.io/crates/delay-handler) [![docs.rs page](https://docs.rs/delay-handler/badge.svg)](https://docs.rs/delay-handler)

An abstration over `DelayQueue` that allows you to create a delay, with associated data.

Users can add data to the delay-map with `insert()`. The associated data is removed and returned when delay is timedout by `.await`ing on `next()`. Users can also prematurely remove the delay from the delay-map with `remove()`.

 ### Examples
 1. Insert 3 numbers into delay-map with 10s delays, print them as they timeout
 ```no_run
 let mut handler = DelayHandler::default();
 // Adds 1, 2, 3 to the delay-map, each with 10s delay
 handler.insert(1, Duration::from_secs(10));
 handler.insert(2, Duration::from_secs(10));
 handler.insert(3, Duration::from_secs(10));

 // Expect a delay of ~10s, after which 1, 2, 3 should print to stdout, in quick succession.
 while let Some(expired) = handler.next().await {
     println!("{}", expired);
 }
 ```
 2. Insert 3 numbers into delay-map with different delays, print them as they timeout
 ```no_run
 let mut handler = DelayHandler::default();
 // Adds 1, 2 to the delay-map, with different delays
 handler.insert(1, Duration::from_secs(10));
 handler.insert(2, Duration::from_secs(5));

 // With a delay of ~5s between, the prints should come in the order of 2 and 1.
 while let Some(expired) = handler.next().await {
     println!("{}", expired);
 }
 ```

 3. Insert 3 numbers into delay-map with different delays, remove  print as delays are timedout
 ```no_run
 let mut handler = DelayHandler::default();
 // Adds 1, 2, 3 to the delay-map, each with different delays
 handler.insert(1, Duration::from_secs(15));
 handler.insert(2, Duration::from_secs(5));
 handler.insert(3, Duration::from_secs(10));
 
 // Remove 3 from the delay-map
 handler.remove(&3);

 // Prints should be in the order of first 2 and ~10s later 1.
 while let Some(expired) = handler.next().await {
     println!("{}", expired);
 }
 ```