use std::collections::HashMap;
use std::f32::INFINITY;
use std::sync::Arc;
use std::sync::Mutex;
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

fn find_best_store(stores: Vec<Store>, shopping_list: Vec<String>) -> String {
    struct Best {
        name: Option<String>,
        price: f32
    }

    let best = Arc::new(Mutex::new(Best {
        name: None,
        price: INFINITY
    }));
    let shopping_list = Arc::new(shopping_list);
    let threads: Vec<_> =
        stores.into_iter()
              .map(|store| {
                  let best = best.clone();
                  let shopping_list = shopping_list.clone();
                  thread::spawn(move || {
                      let sum = shopping_list.iter()
                                             .map(|item_name| store.price(item_name))
                                             .fold(0.0, |v, u| v + u);
                      let mut best = best.lock().unwrap(); // propagate panics
                      if sum < best.price {
                          best.name = Some(store.name);
                          best.price = sum;
                      }
                  })
              })
              .collect();

    for thread in threads {
        thread.join().unwrap(); // propoagate panics
    }

    let best = best.lock().unwrap();
    best.name.clone().unwrap()
}

pub fn main() {
    let shopping_list = vec![format!("chocolate"),
                             format!("doll"),
                             format!("bike")];

    let stores = build_stores();
    let best_store = find_best_store(stores, shopping_list);
    println!("Best store: {}", best_store);
}

