use std::collections::HashMap;
use crate::Node;

pub fn part_1(map: &HashMap<String,Node>, instructions: &Vec<char>)
{
    let mut working_node: &Node = map.get("AAA").unwrap();
    let mut instructions_index: usize = 0;
    let mut amount_of_steps: i32 = 0;
    while working_node.name != "ZZZ" {
        amount_of_steps += 1;
        if instructions_index == instructions.len(){
            instructions_index = 0;
        }
        match instructions.get(instructions_index).unwrap() {
            'L' => working_node = map.get(&working_node.left_node).unwrap(),
            'R' => working_node = map.get(&working_node.right_node).unwrap(),
            _ => println!("unexplained instruction")
        }
        instructions_index += 1;
    }
    println!("Part 1: Found ZZZ in {} steps", amount_of_steps);
}