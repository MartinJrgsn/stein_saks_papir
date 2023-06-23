mod atomic_buffer;
mod atomic_buffer_weak;
pub mod error;

pub use atomic_buffer::*;
pub use atomic_buffer_weak::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
