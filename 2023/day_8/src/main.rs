mod node;
mod part_1;
mod part_2;

use input_file_lib::read_file_content;

use std::collections::HashMap;

use node::Node;
use part_1::part_1;
use part_2::part_2;
fn main() {
    println!("Day 8 program");
    let file_cont: String = read_file_content().expect("Program expects an input file as parameter");
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut instructions: Vec<char> = Vec::new();
    for (index, line) in file_cont.lines().enumerate() {
        match index {
            0 => line.trim().chars().for_each(| f| instructions.push(f)),
            1 => (), // empty line
            _ => {
                let node = Node::from(line);
                nodes.insert(node.name.to_string(), node);
            },
        }
    }
    
    part_1(&nodes, &instructions);
    part_2(&nodes, &instructions);
}
