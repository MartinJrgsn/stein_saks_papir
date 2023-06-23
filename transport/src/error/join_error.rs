use std::any::Any;

pub struct JoinError(pub Box<dyn Any>);