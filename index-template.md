## Rust concurrency tutorial

- [Hello World](src/hello_world.rs):
    - Goal: make it greet you by name
    - Time: 3 minutes
- [Borrowing](src/borrowing.rs):
    - Goal: convert this code to use borrowing instead so that it compiles
    - Time: 3 minutes
- [Sequential search](src/sequential_search.rs):    
    - Goal: implement <code>find_best_store</code>
    - Time: 10 minutes
- [Parallel search](src/parallel_search.rs):
    - Goal: make this actually run in parallel
    - Hint: you will have to stop cloning the store name
    - Time: 10 minutes
- [Channels](src/channels.rs):    
    - Goal: modify parallel <code>find_best_store</code> to use channels
    - Extra bonus: modify to make each store an actor and use RPC 
    - Time: 10 minutes
- [Shared memory](src/shared_memory.rs):
    - Goal: Adapt your previous code to use `Arc`
    - Extra bonus: How can you minimize cloning the underlying shopping list?
    - Time: 10 minutes
- [Introduce mutex](src/mutex.rs):
    - Goal: Adapt your previous code to introduce a mutex and use it to find the best price
    - Time: 10 minutes

Thank you for coming to the tutorial! Before you go, please let me
know what you thought at
[this Google form](http://goo.gl/forms/CN4trE3rXe).
