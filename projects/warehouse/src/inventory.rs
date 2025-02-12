pub const FLOOR_SPACE: i32 = 1000;
pub const MANAGER: &str = "Ivan Inventory";

pub fn talk_to_manager() {
    println!("hey, {MANAGER}, how's your coffee?");
}

pub mod products;
