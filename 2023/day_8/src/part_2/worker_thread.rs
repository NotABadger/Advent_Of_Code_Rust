use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use std::thread::{self};
use crate::part_2::thread_communication::{WorkerToMain, MainToWorker};
use crate::node::Node;

pub fn part_2_worker(map: HashMap<String,Node>, 
                     instructions: Vec<char>, 
                     start_point: String, 
                     communitaction_to_main: Sender<WorkerToMain>, 
                     communication_from_main: Receiver<MainToWorker>) {

    let mut working_node: &Node = map.get(&start_point).unwrap();
    let mut instructions_index: usize = 0;
    let mut amount_of_steps: u32 = 0;
    
    loop {
        let mut z_node_not_enough_steps: bool = true;
        while !working_node.name.ends_with('Z') || z_node_not_enough_steps  {
            z_node_not_enough_steps = false;
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
        } //found Z ending node

        let msg_to_main: WorkerToMain = WorkerToMain { steps: amount_of_steps, thread_id: thread::current().id() };
        _ = communitaction_to_main.send(msg_to_main);
        if let Ok(answer_from_main) = communication_from_main.recv()
        {
            if answer_from_main == MainToWorker::Exit {
                break; //breaks out of loop
            }
            //else, keep working
        }
        else {
            println!("Thread {:?} failed to receive a message.", thread::current().id());
            break;
        }
    }

    println!("Thread {:?} logging out after {} steps, with {} as end node", thread::current().id(), amount_of_steps, working_node.name);

}