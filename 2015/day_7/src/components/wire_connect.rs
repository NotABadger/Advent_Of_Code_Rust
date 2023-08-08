use crate::component_traits::Component;
use crate::wire::Wire;

/* Not a component in assignmen, but had to add because rust doesn't support polymorphic linked list/tree */
pub struct WireConnect
{
    input : String,
    output : String,
}

impl WireConnect
{
    pub fn new() -> Self
    {
        Self{input : String::new(), output : String::new()}
    }
}

impl Component for WireConnect
{
    fn add_input(&mut self, wire: &str)
    {
        self.input = wire.to_string();
    }

    fn get_input(&self) -> Vec<String>
    {
        vec![&self.input.to_string()]
    }

    fn add_output(&mut self, wire: &str)
    {
        self.output = wire.to_string();
    }

    fn get_output(&self) -> String
    {
        &self.output.to_string()
    }

    fn validate_component(&self) -> bool
    {
        if self.input.len() > 1 && !self.output.is_empty()
        {
            return true;
        }
        false
    }

    fn compute_value(&mut self, wire_list: &mut Vec<Wire>)
    {
        let mut value : u16 = 0;
        if self.input.is_empty() || self.output.is_empty()
        {
            println!("Input: {:?}, output: {:?}", self.input, self.output);
            panic!("Wire connect where input or output is empty");
        }
        
        for wire in &mut *wire_list
        {
            if self.input == wire.get_name()
            {
                value |= wire.get_value().unwrap();
                break;
            }
        }
        
        for wire in wire_list
        {
            if wire.get_name() == self.output
            {
                wire.set_value(value);
                break;
            }
        }
    }
}