#![feature(is_some_and)]

moddef::moddef!(
    pub flat mod {
        error,
        transport,
        com
    },
    flat(pub) mod {
        para_listener,
        para_stream
    }
);