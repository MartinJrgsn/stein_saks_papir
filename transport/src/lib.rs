#![feature(try_trait_v2)]

#![feature(is_some_and)]

moddef::moddef!(
    pub flat mod {
        error,
        event,
        transport,
        com
    },
    flat(pub) mod {
        para_listener,
        para_stream,
        receive_buffer
    }
);