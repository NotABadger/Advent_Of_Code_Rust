use crate::component_traits::Component;
use crate::wire::Wire;

use super::circuit_board::CircuitBoard;

pub struct CircuitResolver
{

}

impl CircuitResolver
{
    pub fn resolve_circuit(board: &mut CircuitBoard, exit_wire_id: &str) -> u16
    {
        if !board.connection_exist(exit_wire_id)
        {
            panic!("Wire that should have final value does not exist!");
        }

        let mut resolve_order: Vec<Vec<&Box<dyn Component>>> = vec![vec![]];
        let mut final_wire: &Wire = board.find_wire(exit_wire_id).expect("no need for message here, see check");
        let mut wires_to_check: Vec<String> = vec![final_wire.get_name()];
        let mut comps_to_check: Vec<&Box<dyn Component>> = CircuitResolver::find_related_components(&wires_to_check,board);
        //let mut input_wires = find_related_wires()
        while !comps_to_check.is_empty()
        {
            wires_to_check = CircuitResolver::find_related_wires(&comps_to_check);
            comps_to_check = CircuitResolver::find_related_components(&wires_to_check,board);
            resolve_order.push(comps_to_check);
        }
        

        0
    }
    fn find_related_wires(root_components:  &Vec<& Box<dyn Component>>) -> Vec<String>
    {
        let mut connecting_wires: Vec<String> = vec![];

        //Make inventory of all needed wires
        for component in root_components
        {
            for input_wire_id in component.get_input()
            {
                let mut wire_already_in_list: bool = false;
                for wire_id in connecting_wires
                {
                    if wire_id.eq(input_wire_id)
                    {
                        wire_already_in_list = true;
                        break; //wire found, no need to add
                    }
                }  
                if !wire_already_in_list
                {
                    connecting_wires.push(input_wire_id.to_string());
                } 
            }
        }
        connecting_wires
    }

    fn find_related_components(wires_ids:  &Vec<String>, circuit_board : &CircuitBoard ) -> Vec<Box<dyn Component>>
    {
        let mut component_return_list: Vec<&Box<dyn Component>> = vec![];
        for id in wires_ids //look up wires & check for value, if none, loop components with wire as output
        {
            match circuit_board.find_wire(id)
            {
                Some(wire) => {
                    if wire.get_value().is_none(){
                        let components: &mut Vec<Box<dyn Component>> = circuit_board.get_mut_components();
                        for component in components
                        {
                           if component.get_output() == id
                           {
                             component_return_list.push(component);
                           }
                        }
                    }
                },
                _ => println!("Wire was requested which was not in the circuit board!"),
            }
        }

        component_return_list
    }
}