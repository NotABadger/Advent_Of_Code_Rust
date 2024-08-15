
//doesn't work fast enough, after 1 min still no solution.
//Works with small size in amount of correct steps
//TODO, make efficient
pub fn part_2(map: &HashMap<String,Node>, instructions: &Vec<char>)
{
    let mut working_nodes: Vec<&Node> = Vec::new();
    map.iter().filter(|(name, node)| name.ends_with('A'))
                .for_each(|(name, node)| working_nodes.push(node));

    println!("found {} starting points.", working_nodes.len()); // finds six starting points

    let mut instructions_index: usize = 0;
    let mut amount_of_steps: i32 = 0;
    let mut reached_last_nodes: bool = false;
    while !reached_last_nodes {
        
        if instructions_index == instructions.len(){
            instructions_index = 0;
        }
        for node in working_nodes.iter_mut()
        {
            match instructions.get(instructions_index).unwrap() {
                'L' => *node = &map.get(&node.left_node).unwrap(),
                'R' => *node = &map.get(&node.right_node).unwrap(),
                _ => println!("unexplained instruction")
            }
        }
        let mut all_nodes_end_z: bool = true;
        working_nodes.iter().for_each(| node_ref| if !node_ref.name.ends_with('Z') {all_nodes_end_z = false;});
        reached_last_nodes = all_nodes_end_z;
    
        amount_of_steps += 1;
        instructions_index += 1;
    }
    println!("Part 2: found everything ending in Z in {} steps", amount_of_steps);
}