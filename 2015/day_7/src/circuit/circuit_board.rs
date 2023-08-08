use crate::component_traits::Component;
use crate::wire::Wire;

pub struct CircuitBoard<'a>
{
    connections : Vec<Wire>,
    components : Vec<Box<dyn Component>>
}

impl CircuitBoard<'_>
{
    pub fn new() -> Self
    {
        Self{connections: Vec::new(), components: Vec::new()}
    }

    pub fn get_mut_connections(&mut self) -> &mut Vec<Wire>
    {
        return &mut self.connections;
    }

    pub fn get_mut_components(&mut self) -> &mut Vec<Box<dyn Component>>
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

    pub fn add_wire(&mut self, wire: Wire)
    {
        self.connections.push(wire);
    }

    pub fn find_wire(&mut self, wire_name: &str) -> Option<&mut Wire>
    {
        for wire in &mut self.connections
        {
            if wire.get_name() == wire_name
            {
                return Some(wire);
            }
        }
        None
    }

    pub fn add_component(&mut self, component: Box<dyn Component>)
    {
        self.components.push(component);
    }

    pub fn find_component_on_output(&mut self, comp_output_name: &str) -> Option<&Box<dyn Component>>
    {
        for component in &mut self.components
        {
            if component.get_output() == comp_output_name
            {
                return Some(component);
            }
        }
        None
    }

}