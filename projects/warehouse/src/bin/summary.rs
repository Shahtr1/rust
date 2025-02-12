use warehouse::{FLOOR_SPACE, INVENTORY_MANAGER, ORDERS_MANAGER};

/// Create a summary of Managers
fn main() {
    println!(
        "Our managers are {} and {} and we have space {}",
        FLOOR_SPACE, INVENTORY_MANAGER, ORDERS_MANAGER
    );
}
