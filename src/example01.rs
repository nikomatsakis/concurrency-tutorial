pub fn main() {
    let name = format!("fellow Rustacean");
    helper(name);
}

fn helper(name: String) {
    println!("Hello, {}!", name);
}

// Discuss:
// - &'static str vs String
// - lead to
