mod hello_world;
// mod borrowing; not expected to compile
mod sequential_search;
mod parallel_search;
mod parallel_search_sol;
mod channels;
mod channels_sol;
mod actor;
mod shared_memory;
mod mutex;

fn main() {
    println!("----------------------------------------------------------------------");
    hello_world::main();

    println!("----------------------------------------------------------------------");
    sequential_search::main();

    println!("----------------------------------------------------------------------");
    parallel_search::main();
    parallel_search_sol::main();

    println!("----------------------------------------------------------------------");
    // channels::main(); // not expected to execute
    channels_sol::main();

    println!("----------------------------------------------------------------------");
    actor::main();

    println!("----------------------------------------------------------------------");
    shared_memory::main();

    println!("----------------------------------------------------------------------");
    mutex::main();
}
