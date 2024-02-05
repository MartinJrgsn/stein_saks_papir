#![feature(try_trait_v2)]
#![feature(let_chains)]

#![feature(is_some_and)]

moddef::moddef!(
    pub mod {
        error,
        event,
        transport
    },
    flat(pub) mod {
        para_listener,
        para_stream,
        receive_buffer
    }
);

pub use atomic_buffer;