use crate::component_traits::Component;
use crate::wire::Wire;

pub struct OrGate
{
    input : Vec<String>,
    output : String,
}

impl OrGate
{
    pub fn new() -> Self
    {
        Self{input : vec![], output : String::new()}
    }
}

impl Component for OrGate
{
    fn add_input(&mut self, wire: &str)
    {
        self.input.push(wire.to_string());
    }

    fn add_output(&mut self, wire: &str)
    {
        self.output = wire.to_string();
    }

    fn compute_value(&mut self, wire_list: &mut Vec<Wire>)
    {
        let mut value : u16 = 0;
        if self.input.len() == 0
        {
            panic!("Or_gate with no inputs");
        }
        for input_name in &self.input
        {
            for wire in &mut *wire_list
            {
                if *input_name == wire.get_name()
                {
                    value |= wire.get_value().unwrap();
                    break;
                }
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