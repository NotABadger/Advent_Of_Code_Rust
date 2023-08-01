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
    pub fn new() -> Self
    {
        Self { in_construction: CircuitBoard::new() }
    }

    pub fn create_circuit_from_file(&mut self, file_content : &str)
    {
        self.in_construction = CircuitBoard::new();
        
        for line in file_content.lines()
        {
            //check for components
            if line.contains("AND")
            {
                self.parse_and_gate(line);
                continue;
            }
            if line.contains("OR")
            {
                self.parse_or_gate(line);
                continue;
            }
            if line.contains("NOT")
            {
                self.parse_not_gate(line);
                continue;
            }
            if line.contains("LSHIFT")
            {
                self.parse_left_shift(line);
                continue;
            }
            if line.contains("RSHIFT")
            {
                self.parse_right_shift(line);
                continue;
            }
            //No gate logic, value assigned to wire
            self.parse_wire_value(line);
        }

        for component in self.in_construction.get_mut_components()
        {
            if !component.validate_component()
            {
                println!("Not all components are used correctly");
            }
        }       
    }

    fn parse_and_gate(& mut self, line: &str)
    {
        let mut component : AndGate = AndGate::new();
        let mut wire : Wire;
        let mut itter = line.split_whitespace();
        let mut itter_string = itter.next().expect("Expected a wire string or value");
        match itter_string.parse::<u16>() //AND gates are also served static values. will handle them by making wire that has name & value of given value
        {
            Ok(value) => {
                if !self.in_construction.connection_exist(itter_string)
                {
                    wire = Wire::new(itter_string);
                    wire.set_value(value);
                    self.in_construction.add_wire(wire);
                }
            }
            _err => {
                if !self.in_construction.connection_exist(itter_string)
                {
                    wire = Wire::new(itter_string);
                    self.in_construction.add_wire(wire);
                }
            }
        }
        component.add_input(itter_string);

        itter.next().expect("This should have been the string \"AND\"");
        itter_string = itter.next().expect("This string should be the second input of the AND gate");

        match itter_string.parse::<u16>()
        {
            Ok(value) => {
                if !self.in_construction.connection_exist(itter_string)
                {
                    wire = Wire::new(itter_string);
                    wire.set_value(value);
                    self.in_construction.add_wire(wire);
                }
            }
            _err => {
                if !self.in_construction.connection_exist(itter_string)
                {
                    wire = Wire::new(itter_string);
                    self.in_construction.add_wire(wire);
                }
            }
        }
        component.add_input(itter_string);

        itter.next().expect("This should have been the arrow \"->\"");
        itter_string = itter.next().expect("This string should be the output wire of the AND gate");

        //output is NEVER a value
        if !self.in_construction.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            self.in_construction.add_wire(wire);
        }
        component.add_output(itter_string);

        self.in_construction.add_component(Box::new(component));
    }

    fn parse_or_gate(&mut self, line : &str)
    {
        let mut component : OrGate = OrGate::new();
        let mut wire : Wire;
        let mut itter = line.split_whitespace();
        let mut itter_string = itter.next().expect("Expected a wire string or value");
        if !self.in_construction.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            self.in_construction.add_wire(wire);
        }
        component.add_input(itter_string);

        itter.next().expect("This should have been the string \"OR\"");
        itter_string = itter.next().expect("This string should be the second input of the OR gate");

        if !self.in_construction.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            self.in_construction.add_wire(wire);
        }
        component.add_input(itter_string);

        itter.next().expect("This should have been the arrow \"->\"");
        itter_string = itter.next().expect("This string should be the output wire of the OR gate");

        if !self.in_construction.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            self.in_construction.add_wire(wire);
        }
        component.add_output(itter_string);
        self.in_construction.add_component(Box::new(component));
    }

    fn parse_not_gate(&mut self, line: &str)
    {
        let mut component : NGate = NGate::new();
        let mut wire : Wire;
        let mut itter = line.split_whitespace();
        itter.next().expect("This should have been the string \"NOT\"");
        let mut itter_string = itter.next().expect("This should have been the input of NOT gate");
        if !self.in_construction.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            self.in_construction.add_wire(wire);
        }
        component.add_input(itter_string);

        itter.next().expect("This should have been the arrow \"->\"");
        itter_string = itter.next().expect("This string should be the output wire of the NOT gate");

        if !self.in_construction.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            self.in_construction.add_wire(wire);
        }
        component.add_output(itter_string);
        self.in_construction.add_component(Box::new(component));

    }

    fn parse_left_shift(&mut self, line: &str)
    {
        let mut component : LShift = LShift::new();
        let mut wire : Wire;
        let mut itter = line.split_whitespace();
        let mut itter_string = itter.next().expect("This should have been the input wire of the LSHIFT");
        if !self.in_construction.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            self.in_construction.add_wire(wire);
        }
        component.add_input(itter_string);

        itter.next().expect("This should have been the string \"LSHIFT\"");
        itter_string = itter.next().expect("This should have been offset");
        match itter_string.parse::<u32>() //AND gates are also served static values. will handle them by making wire that has name & value of given value
        {
            Ok(value) => {
                component.set_offset(value);
            }
            _err => {
                panic!("Expected LSHIFT offset, but did not receive value but chars");
            }
        }

        itter.next().expect("This should have been the arrow \"->\"");
        itter_string = itter.next().expect("This string should be the output wire of the LSHIFT");
        
        if !self.in_construction.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            self.in_construction.add_wire(wire);
        }
        component.add_output(itter_string);
        self.in_construction.add_component(Box::new(component));
    }

    fn parse_right_shift(&mut self, line: &str)
    {
        let mut component : RShift = RShift::new();
        let mut wire : Wire;
        let mut itter = line.split_whitespace();
        let mut itter_string = itter.next().expect("This should have been the input wire of the RSHIFT");
        if !self.in_construction.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            self.in_construction.add_wire(wire);
        }
        component.add_input(itter_string);

        itter.next().expect("This should have been the string \"RSHIFT\"");
        itter_string = itter.next().expect("This should have been offset");
        match itter_string.parse::<u32>() //Offset or operation is a numeric value
        {
            Ok(value) => {
                component.set_offset(value);
            }
            _err => {
                panic!("Expected RSHIFT offset, but did not receive value but chars");
            }
        }

        itter.next().expect("This should have been the arrow \"->\"");
        itter_string = itter.next().expect("This string should be the output wire of the RSHIFT");
        
        if !self.in_construction.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            self.in_construction.add_wire(wire);
        }
        component.add_output(itter_string);
        self.in_construction.add_component(Box::new(component));
    }

    fn parse_wire_value(&mut self, line: &str)
    {
        let wire_value: u16;
        let wire_id: String;
        let mut itter = line.split_whitespace();
        let mut itter_string = itter.next().expect("This should be the value to assign to the wire");

        match itter_string.parse::<u16>() //Offset or operation is a numeric value
        {
            Ok(value) => {
                wire_value = value;
            }
            _err => {
                panic!("Expected numbers for wire value, but did not receive value but chars. \n received: {:?}", itter_string);
            }
        }

        itter.next().expect("This should have been the arrow \"->\"");
        itter_string = itter.next().expect("This should be the wire ID");
        if !self.in_construction.connection_exist(itter_string)
        {
            let mut wire = Wire::new(itter_string);
            wire.set_value(wire_value);
            self.in_construction.add_wire(wire);
        }
        else 
        {
            self.in_construction.find_wire(itter_string)
                .expect("already check existance of wire")
                .set_value(wire_value);
        }

    }


}