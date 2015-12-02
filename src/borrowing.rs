pub fn main() {
    let mut name = format!("fellow ");
    name.push_str("Rustacean");
    helper(name);
    helper(name);
}

fn helper(name: String) {
    println!("Hello, {}!", name);
}
