use crate::container::ContainerType;


pub fn check_combos_with_largest_container(containers: &Vec<ContainerType>, volume_to_store: u32) -> u32
{
    //assumes containers are sorted largest to smallest
    //try all possible 
    let mut possible_combos : u32 = 0;
    let mut stacks_deep : u32 = 1;
    let mut least_amount_of_containers: u32 = 100;
    let mut possible_min_combos: u32 = 100;
    for index in 0..containers.len()    
    {
        recursive_check_adding(containers, volume_to_store, 0, index, &mut possible_combos, &mut stacks_deep, &mut least_amount_of_containers, &mut possible_min_combos );    
    }
    println!("possible_min_combos: {}", possible_min_combos);
    possible_combos
}

fn recursive_check_adding(containers: &Vec<ContainerType>, volume_to_store: u32, already_stored :u32, index_to_start_from: usize,  possible_combos: &mut u32, stacks_deep: &mut u32, least_amount_of_containers: &mut u32, possible_min_combos: &mut u32)
{
    let capacity_of_container: u32 = containers.get(index_to_start_from).expect("should be here").capacity;
 
    if (capacity_of_container + already_stored) > volume_to_store
    {
        return;
    }
    if (capacity_of_container + already_stored) == volume_to_store
    {
        *possible_combos += 1;
        if *stacks_deep == *least_amount_of_containers
        {
            *possible_min_combos += 1;
        }
        if *stacks_deep < *least_amount_of_containers
        {
            *least_amount_of_containers = *stacks_deep;
            *possible_min_combos = 1;
        }
        return;
    }
    
    *stacks_deep += 1;
    let mut itterations: usize = 0;
    //make sure we can check the remainder of the list
    for _container in (index_to_start_from+1)..containers.len()
    {
        itterations += 1;
        //recursive check
        recursive_check_adding(containers, volume_to_store, capacity_of_container + already_stored, index_to_start_from+ itterations, possible_combos, stacks_deep, least_amount_of_containers, possible_min_combos);
    }
    
    *stacks_deep -= 1;
    return;
}
