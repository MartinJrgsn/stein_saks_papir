mod atomic_buffer;
mod atomic_buffer_weak;
pub mod error;

pub use atomic_buffer::*;
pub use atomic_buffer_weak::*;

#[cfg(test)]
mod tests {
    use std::error::Error;

    use poison_error_obj::PoisonErrorUnguarded;

    use super::*;

    #[ignore]
    #[test]
    fn it_works() -> Result<(), Box<dyn Error>>
    {
        let buf = AtomicBuffer::new();
        let buf_weak = buf.downgrade();

        loop
        {
            let mut str = String::new();
            std::io::stdin().read_line(&mut str)?;

            buf.push_back(str).map_err(|error| PoisonErrorUnguarded::from(error))?;

            while let Some(str) = buf_weak.pop_front()?
            {
                print!("{}", str);
            }
        }
    }
}
