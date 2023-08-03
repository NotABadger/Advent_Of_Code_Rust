use crate::component_traits::Component;
use crate::wire::Wire;
use super::circuit_board::CircuitBoard;
use crate::components::{and_gate::AndGate, or_gate::OrGate, l_shift::LShift, r_shift::RShift, n_gate::NGate, wire_connect::WireConnect};

pub struct CircuitFactory
{
}

impl CircuitFactory
{
    pub fn create_circuit_from_file(file_content : &str) -> CircuitBoard
    {
        let mut in_construction = CircuitBoard::new();
        
        for line in file_content.lines()
        {
            //check for components
            if line.contains("AND")
            {
                CircuitFactory::parse_and_gate(line, &mut in_construction);
                continue;
            }
            if line.contains("OR")
            {
                CircuitFactory::parse_or_gate(line, &mut in_construction);
                continue;
            }
            if line.contains("NOT")
            {
                CircuitFactory::parse_not_gate(line, &mut in_construction);
                continue;
            }
            if line.contains("LSHIFT")
            {
                CircuitFactory::parse_left_shift(line, &mut in_construction);
                continue;
            }
            if line.contains("RSHIFT")
            {
                CircuitFactory::parse_right_shift(line, &mut in_construction);
                continue;
            }
            //No gate logic, value assigned to wire
            CircuitFactory::parse_wire_logic(line, &mut in_construction);
        }

        for component in in_construction.get_mut_components()
        {
            if !component.validate_component()
            {
                println!("Not all components are used correctly");
            }
        }   
        
        in_construction    
    }

    fn parse_and_gate(line: &str, circuit_board: &mut CircuitBoard)
    {
        let mut component : AndGate = AndGate::new();
        let mut wire : Wire;
        let mut itter = line.split_whitespace();
        let mut itter_string = itter.next().expect("Expected a wire string or value");
        match itter_string.parse::<u16>() //AND gates are also served static values. will handle them by making wire that has name & value of given value
        {
            Ok(value) => {
                if !circuit_board.connection_exist(itter_string)
                {
                    wire = Wire::new(itter_string);
                    wire.set_value(value);
                    circuit_board.add_wire(wire);
                }
            }
            _err => {
                if !circuit_board.connection_exist(itter_string)
                {
                    wire = Wire::new(itter_string);
                    circuit_board.add_wire(wire);
                }
            }
        }
        component.add_input(itter_string);

        itter.next().expect("This should have been the string \"AND\"");
        itter_string = itter.next().expect("This string should be the second input of the AND gate");

        match itter_string.parse::<u16>()
        {
            Ok(value) => {
                if !circuit_board.connection_exist(itter_string)
                {
                    wire = Wire::new(itter_string);
                    wire.set_value(value);
                    circuit_board.add_wire(wire);
                }
            }
            _err => {
                if !circuit_board.connection_exist(itter_string)
                {
                    wire = Wire::new(itter_string);
                    circuit_board.add_wire(wire);
                }
            }
        }
        component.add_input(itter_string);

        itter.next().expect("This should have been the arrow \"->\"");
        itter_string = itter.next().expect("This string should be the output wire of the AND gate");

        //output is NEVER a value
        if !circuit_board.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            circuit_board.add_wire(wire);
        }
        component.add_output(itter_string);

        circuit_board.add_component(Box::new(component));
    }

    fn parse_or_gate(line : &str, circuit_board: &mut CircuitBoard)
    {
        let mut component : OrGate = OrGate::new();
        let mut wire : Wire;
        let mut itter = line.split_whitespace();
        let mut itter_string = itter.next().expect("Expected a wire string or value");
        if !circuit_board.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            circuit_board.add_wire(wire);
        }
        component.add_input(itter_string);

        itter.next().expect("This should have been the string \"OR\"");
        itter_string = itter.next().expect("This string should be the second input of the OR gate");

        if !circuit_board.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            circuit_board.add_wire(wire);
        }
        component.add_input(itter_string);

        itter.next().expect("This should have been the arrow \"->\"");
        itter_string = itter.next().expect("This string should be the output wire of the OR gate");

        if !circuit_board.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            circuit_board.add_wire(wire);
        }
        component.add_output(itter_string);
        circuit_board.add_component(Box::new(component));
    }

    fn parse_not_gate(line: &str, circuit_board: &mut CircuitBoard)
    {
        let mut component : NGate = NGate::new();
        let mut wire : Wire;
        let mut itter = line.split_whitespace();
        itter.next().expect("This should have been the string \"NOT\"");
        let mut itter_string = itter.next().expect("This should have been the input of NOT gate");
        if !circuit_board.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            circuit_board.add_wire(wire);
        }
        component.add_input(itter_string);

        itter.next().expect("This should have been the arrow \"->\"");
        itter_string = itter.next().expect("This string should be the output wire of the NOT gate");

        if !circuit_board.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            circuit_board.add_wire(wire);
        }
        component.add_output(itter_string);
        circuit_board.add_component(Box::new(component));

    }

    fn parse_left_shift(line: &str, circuit_board: &mut CircuitBoard)
    {
        let mut component : LShift = LShift::new();
        let mut wire : Wire;
        let mut itter = line.split_whitespace();
        let mut itter_string = itter.next().expect("This should have been the input wire of the LSHIFT");
        if !circuit_board.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            circuit_board.add_wire(wire);
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
        
        if !circuit_board.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            circuit_board.add_wire(wire);
        }
        component.add_output(itter_string);
        circuit_board.add_component(Box::new(component));
    }

    fn parse_right_shift(line: &str, circuit_board: &mut CircuitBoard)
    {
        let mut component : RShift = RShift::new();
        let mut wire : Wire;
        let mut itter = line.split_whitespace();
        let mut itter_string = itter.next().expect("This should have been the input wire of the RSHIFT");
        if !circuit_board.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            circuit_board.add_wire(wire);
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
        
        if !circuit_board.connection_exist(itter_string)
        {
            wire = Wire::new(itter_string);
            circuit_board.add_wire(wire);
        }
        component.add_output(itter_string);
        circuit_board.add_component(Box::new(component));
    }

    fn parse_wire_logic(line: &str, circuit_board: &mut CircuitBoard)
    {
        let mut itter = line.split_whitespace();
        let mut itter_string = itter.next().expect("This is either a value or string name to the wire");

        match itter_string.parse::<u16>() // If first word is numeric value, put value in wire
        {
            Ok(value) => {
                let wire_value : u16 = value;
                itter.next().expect("This should have been the arrow \"->\"");
                itter_string = itter.next().expect("This should be the wire ID");
                if !circuit_board.connection_exist(itter_string)
                {
                    let mut wire = Wire::new(itter_string);
                    wire.set_value(wire_value);
                    circuit_board.add_wire(wire);
                }
                else 
                {
                    circuit_board.find_wire(itter_string)
                        .expect("already check existance of wire")
                        .set_value(wire_value);
                }
            }
            _err => { // f first word is wire ID connect 2 wires using connecting component
                let output_wire: &str = itter_string;
                if !circuit_board.connection_exist(output_wire)
                {
                    let wire: Wire = Wire::new(itter_string);
                    circuit_board.add_wire(wire);
                }

                itter.next().expect("This should have been the arrow \"->\"");
                let input_wire: &str = itter.next().expect("This should be second wire Id");
                if !circuit_board.connection_exist(input_wire)
                {
                    let wire: Wire = Wire::new(input_wire);
                    circuit_board.add_wire(wire);
                }

                let mut wire_connect: WireConnect = WireConnect::new();
                wire_connect.add_input(output_wire); // seems confusing, but the output of one wire needs to be connected to input of second wire.
                wire_connect.add_output(input_wire);
            }
        }
    }


}