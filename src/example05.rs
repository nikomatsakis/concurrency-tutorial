pub fn main() {
    let mut name = Rc::new(format!("fellow "));
    helper(name.clone());
    helper(name.clone());
}

fn helper(name: Rc<String>) {
    println!("Hello, {}!", name);
}
