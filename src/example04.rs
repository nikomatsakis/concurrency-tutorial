pub fn main() {
    let mut name = format!("fellow ");
    adjust(&mut name);
    helper(&name);
    helper(&name);
}

fn adjust(name: &mut String) {
    name.push_str("Rustacean");
}

fn helper(name: &String) {
    println!("Hello, {}!", name);
}
