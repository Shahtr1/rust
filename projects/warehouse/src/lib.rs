/// This module represents tools for inventory management
pub mod inventory;

/// This module represents tools for orders management
pub mod orders;

// use inventory::products::{self, ProductCategory};
pub use inventory::{Item, ProductCategory, FLOOR_SPACE, MANAGER as INVENTORY_MANAGER};
pub use orders::MANAGER as ORDERS_MANAGER;
