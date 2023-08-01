use crate::wire::Wire;
pub trait Component {
    fn add_input(&mut self, wire: &str);
    fn add_output(&mut self,  wire: &str);
    fn validate_component(&self) -> bool;
    fn compute_value(&mut self, wire_list: &mut Vec<Wire>);
}