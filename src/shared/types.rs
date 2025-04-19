#![allow(dead_code)]
use std::rc::Weak;

struct Tenant {
    warehouses: Vec<Warehouse>,
}

struct Warehouse {
    name: String,
    // NOTE: Möglicherweise auch aus den Slots berechnen
    capacity: i32,
    // used_capacity
    slots: Vec<Slot>,
}

struct Slot {
    warehouse: Weak<Warehouse>,
    label: String,
    capacity: i32,
    products: Vec<Product>,
}

struct Product {
    id: String,
    content: String,
    slot_size: i32,

    weight: i32,
    // extras...
}
