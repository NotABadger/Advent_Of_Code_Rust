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



        0
    }
    fn find_related_components<'a>(root_components: &'a Vec<&'a Box<dyn Component>>) -> Vec<&'a Box<dyn Component>>
    {
        let mut return_list: Vec<&'a Box<dyn Component>> = vec![];

        for component in root_components
        {

        }

        return return_list;
    }
}