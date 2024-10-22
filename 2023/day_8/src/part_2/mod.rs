mod least_common_multiple;
use crate::node::Node;

use::threadpool_lib::initialize_threadpool;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
/* Ideas: 
    // Check amount of steps until ALL 6 are at a node ending in Z
    // This is a Least-Common-Multiple problem, 
    // https://en.wikipedia.org/wiki/Least_common_multiple
    // we will need to see which amount of steps every node needs until it "restarts"
    // E.G. start node number 1 will need 10 steps to get to Z, then 12, then 24, then 10, and 12, and so on. 
    // the values will be 12, 24, 10. since 12 will repeat the patern
*/
pub fn part_2(map: &HashMap<String,Node>, instructions: &Vec<char>)
{
    //find how many start points there are
    let mut starting_nodes: Vec<&Node> = Vec::new();
    map.iter().filter(|(name, _node)| name.ends_with('A'))
                .for_each(|(_name, node)| starting_nodes.push(node));


    println!("found {} starting points.", starting_nodes.len()); // finds six starting points

    // execute logic with threadpool
    let pool = initialize_threadpool();
    let answers: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::new()));
    for startnode in starting_nodes {
        let thread_answer = answers.clone();
        let thread_map = map.clone();
        let thread_instructions = instructions.clone();
        let start_node_str = startnode.name.clone();
        pool.execute(move || { thread_answer.lock().unwrap().append(
            &mut thread_task(thread_map, thread_instructions, start_node_str)
        );});
    }
    
    pool.join();
    println!("Executed all tasks.");
    
    let multiplication = least_common_multiple::find_least_common_multiplicator(answers.lock().unwrap().clone());
    println!("Least amount of steps from all {}", multiplication);
   
}

pub fn thread_task(map: HashMap<String,Node>, instructions: Vec<char>, start_node: String) -> Vec<u32> {
    let mut list_of_relative_intervals :Vec<u32> = Vec::new();
    let mut list_of_absolute_intervals: Vec<u32> = Vec::new();

    let mut working_node: &Node = map.get(&start_node).unwrap();
    let mut instructions_index: usize = 0;
    let mut relative_amount_of_steps: u32 = 0;
    let mut abosulte_amount_of_steps: u32 = 0;

    while !working_node.name.ends_with('Z') {
        relative_amount_of_steps += 1;
        abosulte_amount_of_steps += 1;
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
    list_of_absolute_intervals.push(abosulte_amount_of_steps);
    list_of_relative_intervals.push(relative_amount_of_steps);
    relative_amount_of_steps = 0;

    'found_all_intervals: loop { 
        //go to next note after ending with "Z"
        //Go to next node:
        relative_amount_of_steps += 1;
        abosulte_amount_of_steps += 1;
        if instructions_index == instructions.len(){
            instructions_index = 0;
        }
        match instructions.get(instructions_index).unwrap() {
            'L' => working_node = map.get(&working_node.left_node).unwrap(),
            'R' => working_node = map.get(&working_node.right_node).unwrap(),
            _ => println!("unexplained instruction")
        }
        instructions_index += 1;
        //Continue doing this while not ending in Z
        while !working_node.name.ends_with('Z') {
            relative_amount_of_steps += 1;
            abosulte_amount_of_steps += 1;
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
        //found node ending in Z
        if relative_amount_of_steps == *list_of_relative_intervals.first().unwrap() {
            break 'found_all_intervals;
        }
        else {
            relative_amount_of_steps = 0;
        }
        //Only add to list if new
        list_of_absolute_intervals.push(abosulte_amount_of_steps);
        list_of_relative_intervals.push(relative_amount_of_steps);
    } 

    return list_of_relative_intervals;
}