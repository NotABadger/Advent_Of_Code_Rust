mod thread_data;
mod thread_communication;
mod worker_thread;
use std::{collections::HashMap, sync::mpsc::channel};
use crate::{part_2::thread_communication::{MainToWorker, WorkerToMain}, Node};

use thread_data::WorkerData;

/* Ideas: 
    1. Thread per found starting node, 2 Channels, 
        1st for thread -> main thread, sending amount of steps finding Z.
        2nd for main thread -> unblock or terminate thread
    node-threads send amount of steps until Z-ending node,
    the threads disagree and have lowest amount of steps  should continue.
    either find amount of steps on correct node, or not at all.        
*/
pub fn part_2(map: &HashMap<String,Node>, instructions: &Vec<char>)
{
    let mut worker_data: Vec<WorkerData> = Vec::new();
    //find how many start points there are
    let mut working_nodes: Vec<&Node> = Vec::new();
    map.iter().filter(|(name, _node)| name.ends_with('A'))
                .for_each(|(_name, node)| working_nodes.push(node));

    println!("found {} starting points.", working_nodes.len()); // finds six starting points

    //initializing worker threads
    let (to_main_sender, to_main_receiver) = channel::<WorkerToMain>(); //one receiver for all worker threads.
    for index in 0..working_nodes.len() {

        let (to_worker_sender, to_worker_receiver) = channel::<MainToWorker>();
        let worker_map = map.clone();
        let worker_instructions = instructions.clone();
        let worker_start_point: String = working_nodes.get(index).unwrap().name.clone();
        let worker_to_main_sender = to_main_sender.clone();
        let thread_hand: std::thread::JoinHandle<()> = std::thread::spawn(move || worker_thread::part_2_worker(worker_map, 
                                                                                                                worker_instructions, 
                                                                                                                worker_start_point,
                                                                                        worker_to_main_sender,
                                                                                        to_worker_receiver));
        
        let start_point: String = working_nodes.get(index).unwrap().name.clone();
        let last_steps_found_init: u32 = 0;
        worker_data.push(WorkerData { thread_handle: thread_hand, 
                                        start_point: start_point,
                                        communication_to_worker: to_worker_sender, 
                                        last_found_steps: last_steps_found_init,
                                        waiting_for_answer: false });
    }
    drop(working_nodes);

    
    let mut highest_steps: u32 = 0;
    while let Ok(message) = to_main_receiver.recv()
    {
        if message.steps < highest_steps
        {
            let sender_worker_data: &WorkerData = worker_data.iter().find(|worker| worker.thread_handle.thread().id() == message.thread_id).unwrap();
            _ = sender_worker_data.communication_to_worker.send(MainToWorker::KeepLooking);
            continue;
        }
        if message.steps == highest_steps
        {
            let sender_worker_data: &mut WorkerData = worker_data.iter_mut().find(|worker| worker.thread_handle.thread().id() == message.thread_id).unwrap();
            _ = sender_worker_data.waiting_for_answer = true;
            //check if all threads are waiting, AKA all at the same amount of steps -> Done!
            let mut all_done: bool = true;
            worker_data.iter().for_each(|worker| if worker.waiting_for_answer == false {all_done = false});
            if all_done {
                worker_data.iter().for_each(| worker| _ = worker.communication_to_worker.send(MainToWorker::Exit));
                println!("Exit after finding all steps!");
                break;
            }

            continue;
        }
        if message.steps > highest_steps
        {
            highest_steps = message.steps;
            println!("Highest steps is now: {}", highest_steps);
            let sender_worker_data: &mut WorkerData = worker_data.iter_mut().find(|worker| worker.thread_handle.thread().id() == message.thread_id).unwrap();
            sender_worker_data.waiting_for_answer = true;
            worker_data.iter_mut().for_each(|worker| {
                if worker.waiting_for_answer && worker.thread_handle.thread().id() != message.thread_id {
                    worker.waiting_for_answer = false;
                    _ = worker.communication_to_worker.send(MainToWorker::KeepLooking);
                }
            });
        }
    }

    for data in worker_data {
        _ = data.thread_handle.join();
    }

    //println!("Part 2: found everything ending in Z in {} steps", amount_of_steps);
}