#![feature(unsize)]

moddef::moddef!(
    pub mod {
        game,
        session,
        message,
        ui,
        error,
        event
    },
    mod {
        tests for cfg(test)
    }
);