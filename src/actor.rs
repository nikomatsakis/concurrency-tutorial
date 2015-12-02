#![allow(dead_code)]

use std::collections::HashMap;
use std::f32::INFINITY;
use std::sync::mpsc::{channel, Sender, Receiver};
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

enum Message {
    PriceOf(String, Sender<f32>),
    StoreName(Sender<String>),
}

fn store_thread(store: Store, port: Receiver<Message>) {
    for msg in port {
        match msg {
            Message::PriceOf(item_name, respond_to) =>
                respond_to.send(store.price(&item_name)).unwrap(),
            Message::StoreName(respond_to) =>
                respond_to.send(store.name.clone()).unwrap(),
        }
    }
}

fn find_best_store(stores: Vec<Store>, shopping_list: &Vec<String>) -> String {
    assert!(stores.len() > 0);

    let store_txs: Vec<_> =
        stores.into_iter()
              .map(|store| {
                  let (tx, rx) = channel();
                  thread::spawn(move || store_thread(store, rx));
                  tx
              })
              .collect();

    let mut best = None;
    let mut best_price = INFINITY;
    for store_tx in store_txs {
        let mut sum = 0.0;
        for item in shopping_list {
            let (price_tx, price_rx) = channel();
            store_tx.send(Message::PriceOf(item.clone(), price_tx)).unwrap();
            let price = price_rx.recv().unwrap();
            sum += price;
        }
        if sum < best_price {
            let (name_tx, name_rx) = channel();
            store_tx.send(Message::StoreName(name_tx)).unwrap();
            let name = name_rx.recv().unwrap();
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

