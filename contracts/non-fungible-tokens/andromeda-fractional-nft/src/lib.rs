pub mod contract;
#[cfg(test)]
mod testing;

#[cfg(not(target_arch = "wasm32"))]
mod interface;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::interface::FractionalNftContract;
