use crate::transport::Port;

use super::*;

pub struct Connection
{
    port: Port,
    name: String
}

impl Connection
{
    pub(super) fn new(port: Port, name: String) -> Self
    {
        Self
        {
            port,
            name
        }
    }

    pub fn get_port(&self) -> Port
    {
        self.port
    }

    pub fn get_name(&self) -> &str
    {
        &self.name
    }
}