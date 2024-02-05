#![feature(unsize)]
#![feature(let_chains)]

moddef::moddef!(
    pub mod {
        game,
        actor,
        message,
        ui,
        player,
        error,
        event,
        session
    },
    mod {
        tests for cfg(test)
    }
);