pub mod gate;
pub mod traits;
pub mod backends;

pub struct Simulator {
    backend: Box<traits::Backend>
}
