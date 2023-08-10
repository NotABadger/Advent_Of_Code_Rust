use std::cell::RefCell;
use std::rc::Rc;

use crate::component_traits::Component;
use crate::components::wire::Wire;

pub struct CircuitBoard
{
    connections : Vec<Rc<RefCell<Wire>>>,
    components : Vec<Rc<RefCell<dyn Component>>>
}

impl CircuitBoard
{
    pub fn new() -> Self
    {
        Self{connections: Vec::new(), components: Vec::new()}
    }

    pub fn find_wire(&self, id: &str) -> Option<Rc<RefCell<Wire>>>
    {
        for connection in &self.connections
        {
            if connection.borrow().get_name() == id
            {
                return Some(connection.clone());
            }
        }
        None
    }

    pub fn add_wire(&mut self, wire: &Rc<RefCell<Wire>>)
    {
        self.connections.push(wire.clone());
    }

    pub fn add_component(&mut self, comp: &Rc<RefCell<dyn Component>>)
    {
        self.components.push(comp.clone());
    }

    pub fn resolve_board(&self, wire_id: &str) -> Option<u16>
    {
        for connection in &self.connections
        {
            if connection.borrow().get_name() == wire_id.to_string()
            {
                return Some(connection.borrow_mut().compute_value());
            }
        }
        println!("wire {:?} not found", wire_id);
        None
    }
    
    #[allow(unused)]
    pub fn print_all_wires(&self)
    {
        for wire in &self.connections
        {
            println!("Wire: {:?}", wire.borrow().get_name());
        }
    }

}