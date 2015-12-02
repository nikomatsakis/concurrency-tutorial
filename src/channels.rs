#![allow(dead_code)]

use std::collections::HashMap;
use std::f32::INFINITY;
use std::sync::mpsc::channel;
use std::thread;

struct Store {
    name: String,
    prices: HashMap<String, f32>,
}

impl Store {
    fn new(name: String) -> Store {
    Store {
        name: name,
        prices: HashMap::new(),
    }
    }

    fn add_item(&mut self, name: String, price: f32) {
        self.prices.insert(name, price);
    }

    fn price(&self, item_name: &str) -> f32 {
        self.prices[item_name]
    }
}

fn build_stores() -> Vec<Store> {
    let mut stores = vec![];

    let mut store = Store::new(format!("R-mart"));
    store.add_item(format!("chocolate"), 5.0);
    store.add_item(format!("doll"), 22.0);
    store.add_item(format!("bike"), 150.0);
    stores.push(store);

    let mut store = Store::new(format!("Bullseye"));
    store.add_item(format!("chocolate"), 2.0);
    store.add_item(format!("doll"), 23.0);
    store.add_item(format!("bike"), 145.0);
    stores.push(store);

    let mut store = Store::new(format!("Woolmart"));
    store.add_item(format!("chocolate"), 2.0);
    store.add_item(format!("doll"), 23.0);
    store.add_item(format!("bike"), 146.0);
    stores.push(store);

    stores
}

fn find_best_store(stores: Vec<Store>, shopping_list: &Vec<String>) -> String {
    assert!(stores.len() > 0);

    // TODO:
    // - create channel for receiving message
    // - start threads for each store, giving each a `tx` to send messages to
    // - have each thread send its name and sum
    // - find and return best name

    for store in stores {
        let shopping_list = shopping_list.clone();
        thread::spawn(move || {
            let sum = compute_sum(&store, &shopping_list);
            unimplemented!(); // send `sum` and `store.name` over channel
        });
    }

    let mut best = None;
    let mut best_price = INFINITY;
    loop { // <-- change to some suitable loop!
        let (sum, name) = unimplemented!(); // receive `sum` and `name` from channel
        if sum < best_price {
            best = Some(name);
            best_price = sum;
        }
    }

    best.unwrap() // there will always be at least one store
}

fn compute_sum(store: &Store, shopping_list: &Vec<String>) -> f32 {
    shopping_list.iter()
                 .map(|item_name| store.price(item_name))
                 .fold(0.0, |v, u| v + u)
}

pub fn main() {
    let shopping_list = vec![format!("chocolate"),
                             format!("doll"),
                             format!("bike")];
    let stores = build_stores();
    let best_store = find_best_store(stores, &shopping_list);
    println!("Best store: {}", best_store);
}

