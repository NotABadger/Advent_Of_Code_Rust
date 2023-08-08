use crate::wire::Wire;
pub trait Component {
    fn add_input(&mut self, wire: &str);
    fn get_input(&self) -> Vec<String>;
    fn add_output(&mut self,  wire: &str);
    fn get_output(&self) -> String;
    fn validate_component(&self) -> bool;
    fn compute_value(&mut self, wire_list: &mut Vec<Wire>);
}