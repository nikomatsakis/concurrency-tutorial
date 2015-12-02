mod example00;
mod example01;
mod example02;
mod example03;
mod example_sequential_search;
mod example_parallel_search;
mod example_parallel_search_sol;
mod example_port;
mod example_actor;
mod example_shared_memory;
mod example_mutex;

fn main() {
    println!("----------------------------------------------------------------------");
    example00::main();

    println!("----------------------------------------------------------------------");
    example01::main();

    println!("----------------------------------------------------------------------");
    example02::main();

    println!("----------------------------------------------------------------------");
    example03::main();

    println!("----------------------------------------------------------------------");
    example_sequential_search::main();

    println!("----------------------------------------------------------------------");
    example_parallel_search::main();
    example_parallel_search_sol::main();

    println!("----------------------------------------------------------------------");
    example_port::main();

    println!("----------------------------------------------------------------------");
    example_actor::main();

    println!("----------------------------------------------------------------------");
    example_shared_memory::main();

    println!("----------------------------------------------------------------------");
    example_mutex::main();
}
