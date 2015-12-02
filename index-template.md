# Rust concurrency tutorial

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
    - Extra bonus: avoid cloning the store name
    - Time: 10 minutes
- [Channels](src/channels.rs):    
    - Goal: modify parallel <code>find_best_store</code> to use channels
    - Extra bonus: modify to make each store an actor and use RPC 
    - Time: 10 minutes
