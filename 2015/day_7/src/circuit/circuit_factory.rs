use crate::component_traits::Component;
use crate::wire::Wire;
use super::circuit_board::CircuitBoard;
use crate::components::{and_gate::AndGate, or_gate::OrGate, l_shift::LShift, r_shift::RShift, n_gate::NGate};

pub struct CircuitFactory
{
    in_construction: CircuitBoard,
}

impl CircuitFactory
{
    pub fn create_circuit_from_file(&mut self, file_content : &str)
    {
        self.in_construction = CircuitBoard::new();
        
        for line in file_content.lines()
        {
            //check for components
            if line.contains("AND")
            {
                self.parse_and_gate(line);
            }
        }       
    }

    fn parse_and_gate(&self, line: &str)
    {
        let mut component : AndGate = AndGate::new();
        for word in line.split_whitespace()
        {
            match word.parse::<u16>()
            {
                Ok(value) => {
                    
                }
                _err => (),
            }
        }

    }
}