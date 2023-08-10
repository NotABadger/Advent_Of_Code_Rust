use std::cell::RefCell;
use std::rc::Rc;

use crate::component_traits::Component;
use crate::components::wire::Wire;
use super::circuit_board::CircuitBoard;
use crate::components::{and_gate::AndGate, or_gate::OrGate, l_shift::LShift, r_shift::RShift, n_gate::NGate};

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
            //No gate logic, value assigned to wire or connect wires
            CircuitFactory::parse_wire_logic(line, &mut in_construction);
        }

        in_construction    
    }

    fn parse_and_gate(line: &str, circuit_board: &mut CircuitBoard)
    {
        let and_gate : Rc<RefCell<AndGate>> = Rc::new(RefCell::new(AndGate::new()));
        let mut itter = line.split_whitespace();
        let mut itter_string = itter.next().expect("Expected a wire string or value");
        match itter_string.parse::<u16>() //AND gates are also served static values. will handle them by making wire that has name & value of given value
        {
            Ok(value) => {// numeric value
                match circuit_board.find_wire(itter_string)
                {
                    Some(wire_ref) => { // "fake" wire exists
                        let cast_to_dyn_component: Rc<RefCell<dyn Component>> = wire_ref.clone() as Rc<RefCell<dyn Component>>;
                        and_gate.borrow_mut().add_input(&cast_to_dyn_component);
                    },
                    None => { // "fake" wire does not exists. create one with name similar to value
                        let wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new(itter_string)));
                        wire.borrow_mut().set_value(value);
                        circuit_board.add_wire(&wire);
                        let cast_to_dyn_component: Rc<RefCell<dyn Component>> = wire.clone() as Rc<RefCell<dyn Component>>;
                        and_gate.borrow_mut().add_input(&cast_to_dyn_component);
                    },
                }
            }
            _err => { // not a numeric value
                match circuit_board.find_wire(itter_string)
                {
                    Some(wire_ref) => {//Looking for actual wire
                        let cast_to_dyn_component: Rc<RefCell<dyn Component>> = wire_ref.clone() as Rc<RefCell<dyn Component>>;
                        and_gate.borrow_mut().add_input(&cast_to_dyn_component);
                    },
                    None => {//wire does not exist, create one with name and link to and_gate + admin
                        let wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new(itter_string)));
                        circuit_board.add_wire(&wire);
                        let cast_to_dyn_component: Rc<RefCell<dyn Component>> = wire.clone() as Rc<RefCell<dyn Component>>;
                        and_gate.borrow_mut().add_input(&cast_to_dyn_component);
                    },
                }
            }
        }

        itter.next().expect("This should have been the string \"AND\"");
        itter_string = itter.next().expect("This string should be the second input of the AND gate");

        match itter_string.parse::<u16>() //AND gates are also served static values. will handle them by making wire that has name & value of given value
        {
            Ok(value) => {// numeric value
                match circuit_board.find_wire(itter_string)
                {
                    Some(wire_ref) => { // "fake" wire exists
                        let clone_to_dyn_component: Rc<RefCell<dyn Component>> = wire_ref.clone() as Rc<RefCell<dyn Component>>;
                        and_gate.borrow_mut().add_input(&clone_to_dyn_component);
                    },
                    None => { // "fake" wire does not exists. create one with name similar to value
                        let wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new(itter_string)));
                        wire.borrow_mut().set_value(value);
                        circuit_board.add_wire(&wire);
                        let clone_to_dyn_component: Rc<RefCell<dyn Component>> = wire.clone() as Rc<RefCell<dyn Component>>;
                        and_gate.borrow_mut().add_input(&clone_to_dyn_component);
                    },
                }
            }
            _err => { // not a numeric value
                match circuit_board.find_wire(itter_string)
                {
                    Some(wire_ref) => {//Looking for actual wire
                        let clone_to_dyn_component: Rc<RefCell<dyn Component>> = wire_ref.clone() as Rc<RefCell<dyn Component>>;
                        and_gate.borrow_mut().add_input(&clone_to_dyn_component);
                    },
                    None => {//wire does not exist, create one with name and link to and_gate + admin
                        let wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new(itter_string)));
                        circuit_board.add_wire(&wire);
                        let clone_to_dyn_component: Rc<RefCell<dyn Component>> = wire.clone() as Rc<RefCell<dyn Component>>;
                        and_gate.borrow_mut().add_input(&clone_to_dyn_component);
                    },
                }
            }
        }

        itter.next().expect("This should have been the arrow \"->\"");
        itter_string = itter.next().expect("This string should be the output wire of the AND gate");

        //Create output wire if needed and link this and_gate
        let and_gate_to_dyn_component: Rc<RefCell<dyn Component>> = and_gate.clone() as Rc<RefCell<dyn Component>>;
        match circuit_board.find_wire(itter_string)
        {
            Some(wire_ref) => {//wire found
                wire_ref.borrow_mut().add_input(&and_gate_to_dyn_component);
            },
            None => {//wire does not exist, create one with name and link to and_gate + admin
                let wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new(itter_string)));
                circuit_board.add_wire(&wire);
                wire.borrow_mut().add_input(&and_gate_to_dyn_component);
            },
        }
        circuit_board.add_component(&and_gate_to_dyn_component);
    }

    fn parse_or_gate(line : &str, circuit_board: &mut CircuitBoard)
    {
        let or_gate : Rc<RefCell<OrGate>> = Rc::new(RefCell::new(OrGate::new()));
        let mut itter = line.split_whitespace();
        let mut itter_string = itter.next().expect("Expected a wire string or value");
        match circuit_board.find_wire(itter_string)
        {
            Some(wire_ref) => {//wire found
                let clone_to_dyn_component: Rc<RefCell<dyn Component>> = wire_ref.clone() as Rc<RefCell<dyn Component>>;
                or_gate.borrow_mut().add_input(&clone_to_dyn_component);
            },
            None => {//wire does not exist, create one with name and link to and_gate + admin
                let wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new(itter_string)));
                let clone_to_dyn_component: Rc<RefCell<dyn Component>> = wire.clone() as Rc<RefCell<dyn Component>>;
                circuit_board.add_wire(&wire);
                or_gate.borrow_mut().add_input(&clone_to_dyn_component);
            },
        }

        itter.next().expect("This should have been the string \"OR\"");
        itter_string = itter.next().expect("This string should be the second input of the OR gate");

        match circuit_board.find_wire(itter_string)
        {
            Some(wire_ref) => {//wire found
                let clone_to_dyn_component: Rc<RefCell<dyn Component>> = wire_ref.clone() as Rc<RefCell<dyn Component>>;
                or_gate.borrow_mut().add_input(&clone_to_dyn_component);
            },
            None => {//wire does not exist, create one with name and link to and_gate + admin
                let wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new(itter_string)));
                circuit_board.add_wire(&wire);
                let clone_to_dyn_component: Rc<RefCell<dyn Component>> = wire.clone() as Rc<RefCell<dyn Component>>;
                or_gate.borrow_mut().add_input(&clone_to_dyn_component);
            },
        }

        itter.next().expect("This should have been the arrow \"->\"");
        itter_string = itter.next().expect("This string should be the output wire of the OR gate");

        //Create output wire if needed and link this and_gate
        match circuit_board.find_wire(itter_string)
        {
            Some(wire_ref) => {//wire found
                let clone_to_dyn_component: Rc<RefCell<dyn Component>> = or_gate.clone() as Rc<RefCell<dyn Component>>;
                wire_ref.borrow_mut().add_input(&clone_to_dyn_component);
            },
            None => {//wire does not exist, create one with name and link to and_gate + admin
                let wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new(itter_string)));
                circuit_board.add_wire(&wire);
                let clone_to_dyn_component: Rc<RefCell<dyn Component>> = or_gate.clone() as Rc<RefCell<dyn Component>>;
                wire.borrow_mut().add_input(&clone_to_dyn_component);
            },
        }

        let clone_to_dyn_component: Rc<RefCell<dyn Component>> = or_gate.clone() as Rc<RefCell<dyn Component>>;
        circuit_board.add_component(&clone_to_dyn_component);
    }

    fn parse_not_gate(line: &str, circuit_board: &mut CircuitBoard)
    {
        let n_gate : Rc<RefCell<NGate>> =  Rc::new(RefCell::new(NGate::new()));
        let mut itter = line.split_whitespace();
        itter.next().expect("This should have been the string \"NOT\"");
        let mut itter_string = itter.next().expect("This should have been the input of NOT gate");

       match circuit_board.find_wire(itter_string)
        {
            Some(wire_ref) => {//wire found
                let clone_to_dyn_component: Rc<RefCell<dyn Component>> = wire_ref.clone() as Rc<RefCell<dyn Component>>;
                n_gate.borrow_mut().add_input(&clone_to_dyn_component);
            },
            None => {//wire does not exist, create one with name and link to and_gate + admin
                let wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new(itter_string)));
                let clone_to_dyn_component: Rc<RefCell<dyn Component>> = wire.clone() as Rc<RefCell<dyn Component>>;
                circuit_board.add_wire(&wire);
                n_gate.borrow_mut().add_input(&clone_to_dyn_component);
            },
        }

        itter.next().expect("This should have been the arrow \"->\"");
        itter_string = itter.next().expect("This string should be the output wire of the NOT gate");

       //Create output wire if needed and link this and_gate
       let n_gate_to_dyn_component: Rc<RefCell<dyn Component>> = n_gate.clone() as Rc<RefCell<dyn Component>>;
       match circuit_board.find_wire(itter_string)
       {
           Some(wire_ref) => {//wire found
                wire_ref.borrow_mut().add_input(&n_gate_to_dyn_component);
           },
           None => {//wire does not exist, create one with name and link to and_gate + admin
               let wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new(itter_string)));
               circuit_board.add_wire(&wire);
               wire.borrow_mut().add_input(&n_gate_to_dyn_component);
           },
       }
       circuit_board.add_component(&n_gate_to_dyn_component);
    }

    fn parse_left_shift(line: &str, circuit_board: &mut CircuitBoard)
    {
        let l_shift : Rc<RefCell<LShift>> =  Rc::new(RefCell::new(LShift::new()));
        let mut itter = line.split_whitespace();
        let mut itter_string = itter.next().expect("This should have been the input wire of the LSHIFT");
        match circuit_board.find_wire(itter_string)
        {
            Some(wire_ref) => {//wire found
                let clone_to_dyn_component: Rc<RefCell<dyn Component>> = wire_ref.clone() as Rc<RefCell<dyn Component>>;
                l_shift.borrow_mut().add_input(&clone_to_dyn_component);
            },
            None => {//wire does not exist, create one with name and link to and_gate + admin
                let wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new(itter_string)));
                let clone_to_dyn_component: Rc<RefCell<dyn Component>> = wire.clone() as Rc<RefCell<dyn Component>>;
                circuit_board.add_wire(&wire);
                l_shift.borrow_mut().add_input(&clone_to_dyn_component);
            },
        }

        itter.next().expect("This should have been the string \"LSHIFT\"");
        itter_string = itter.next().expect("This should have been offset");
        match itter_string.parse::<u32>() //AND gates are also served static values. will handle them by making wire that has name & value of given value
        {
            Ok(value) => {
                l_shift.borrow_mut().set_offset(value);
            }
            _err => {
                panic!("Expected LSHIFT offset, but did not receive value but chars");
            }
        }

        itter.next().expect("This should have been the arrow \"->\"");
        itter_string = itter.next().expect("This string should be the output wire of the LSHIFT");
        
        //Create output wire if needed and link this and_gate
        let lshift_to_dyn_component: Rc<RefCell<dyn Component>> = l_shift.clone() as Rc<RefCell<dyn Component>>;
       match circuit_board.find_wire(itter_string)
       {
           Some(wire_ref) => {//wire found
                wire_ref.borrow_mut().add_input(&lshift_to_dyn_component);
           },
           None => {//wire does not exist, create one with name and link to and_gate + admin
               let wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new(itter_string)));
               circuit_board.add_wire(&wire);
               wire.borrow_mut().add_input(&lshift_to_dyn_component);
           },
       }
       
       circuit_board.add_component(&lshift_to_dyn_component);
    }

    fn parse_right_shift(line: &str, circuit_board: &mut CircuitBoard)
    {
        let r_shift : Rc<RefCell<RShift>> =  Rc::new(RefCell::new(RShift::new()));
        let mut itter = line.split_whitespace();
        let mut itter_string = itter.next().expect("This should have been the input wire of the RSHIFT");
        match circuit_board.find_wire(itter_string)
        {
            Some(wire_ref) => {//wire found
                let clone_to_dyn_component: Rc<RefCell<dyn Component>> = wire_ref.clone() as Rc<RefCell<dyn Component>>;
                r_shift.borrow_mut().add_input(&clone_to_dyn_component);
            },
            None => {//wire does not exist, create one with name and link to and_gate + admin
                let wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new(itter_string)));
                let clone_to_dyn_component: Rc<RefCell<dyn Component>> = wire.clone() as Rc<RefCell<dyn Component>>;
                circuit_board.add_wire(&wire);
                r_shift.borrow_mut().add_input(&clone_to_dyn_component);
            },
        }

        itter.next().expect("This should have been the string \"RSHIFT\"");
        itter_string = itter.next().expect("This should have been offset");
        match itter_string.parse::<u32>() //AND gates are also served static values. will handle them by making wire that has name & value of given value
        {
            Ok(value) => {
                r_shift.borrow_mut().set_offset(value);
            }
            _err => {
                panic!("Expected RSHIFT offset, but did not receive value but chars");
            }
        }

        itter.next().expect("This should have been the arrow \"->\"");
        itter_string = itter.next().expect("This string should be the output wire of the RSHIFT");
        
        //Create output wire if needed and link this and_gate
        let r_shift_to_dyn_component: Rc<RefCell<dyn Component>> = r_shift.clone() as Rc<RefCell<dyn Component>>;
        match circuit_board.find_wire(itter_string)
        {
            Some(wire_ref) => {//wire found
                    wire_ref.borrow_mut().add_input(&r_shift_to_dyn_component);
            },
            None => {//wire does not exist, create one with name and link to and_gate + admin
                let wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new(itter_string)));
                circuit_board.add_wire(&wire);
                wire.borrow_mut().add_input(&r_shift_to_dyn_component);
            },
        }
        circuit_board.add_component(&r_shift_to_dyn_component);
    }

    fn parse_wire_logic(line: &str, circuit_board: &mut CircuitBoard)
    {
        let mut itter = line.split_whitespace();
        let mut itter_string = itter.next().expect("This is either a value or string name to the wire");

        match itter_string.parse::<u16>()
        {
            Ok(value) => { // If first word is numeric value, put value in wire
                let wire_value : u16 = value;
                itter.next().expect("This should have been the arrow \"->\"");
                itter_string = itter.next().expect("This should be the wire ID");
                match circuit_board.find_wire(itter_string)
                {
                    Some(wire_ref) => {//wire found
                        wire_ref.borrow_mut().set_value(wire_value);
                    },
                    None => {//wire does not exist, create one with name and link to and_gate + admin
                        let wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new(itter_string)));
                        circuit_board.add_wire(&wire);
                        wire.borrow_mut().set_value(wire_value);
                    },
                }
            }
            _err => { // first word is wire ID connect 2 wires using connecting component
                let input_wire_str: &str = itter_string;
                let input_wire: Rc<RefCell<Wire>>;
                itter.next().expect("This should have been the arrow \"->\"");
                let output_wire_str: &str = itter.next().expect("This should be second wire Id");

                match circuit_board.find_wire(input_wire_str)
                {
                    Some(wire_ref) => {// input wire found
                        input_wire = wire_ref.clone();
                    },
                    None => {//input wire does not exist
                        let wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new(input_wire_str)));
                        circuit_board.add_wire(&wire);
                        input_wire = wire.clone();
                    },
                }

                let input_wire_to_dyn_component: Rc<RefCell<dyn Component>> = input_wire.clone() as Rc<RefCell<dyn Component>>;
                match circuit_board.find_wire(output_wire_str)
                {
                    Some(wire_ref) => {// output wire found
                        wire_ref.borrow_mut().add_input(&input_wire_to_dyn_component);
                    },
                    None => {//output wire does not exist
                        let wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new(output_wire_str)));
                        circuit_board.add_wire(&wire);
                        wire.borrow_mut().add_input(&input_wire_to_dyn_component);
                    },
                }
            }
        }
    }


}