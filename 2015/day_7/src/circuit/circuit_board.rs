use crate::component_traits::Component;
use crate::wire::Wire;

pub struct CircuitBoard
{
    connections : Vec<Wire>,
    components : Vec<Box<dyn Component>>
}

impl CircuitBoard
{
    pub fn new() -> Self
    {
        Self{connections: Vec::new(), components: Vec::new()}
    }

    pub fn get_mut_connections(&mut self) -> &Vec<Wire>
    {
        return &mut self.connections;
    }

    pub fn get_mut_components(&mut self) -> &Vec<Box<dyn Component>>
    {
        return &mut self.components;
    }

    pub fn connection_exist(&self, identifier: &str) -> bool
    {
        for connection in &self.connections
        {
            if connection.get_name() == identifier
            {
                return true
            }
        }
        false
    }
}