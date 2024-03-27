mod pool_initializer;
mod file_processor_file;
mod package_group;
mod first_group_finder;
mod other_groups_finder;

use file_processor_file::FileProcessor;
use first_group_finder::*;
use threadpool::ThreadPool;
use std::process::exit;
use std::sync::{Arc, Mutex};
use crate::other_groups_finder::find_other_groups;


const FILE: &str = "input.txt";

fn main() {
    println!("Day_24 challange");

    let file_content: String;
    let numbers: Vec<i32>;
    let third_of_tot_val: i32;

    if !FileProcessor::check_file_exists(FILE)
    {
        println!("Sorry, but couldn't find the input file");
        exit(0);
    }
    match FileProcessor::read_file(FILE) {
        Ok(read_content) => file_content = read_content,
        Err(msg) => panic!("{}",msg),
    }

    numbers = parse_file_content(&file_content);
    let mut total_value: i32 = 0;
    numbers.iter().for_each(| int| total_value += *int);
    third_of_tot_val = total_value / 3; //Change the '3' to a '4' for part two

    println!("The third of total val is: {}", third_of_tot_val);
    let mut initial_groups_list: Vec<PackageGroup> = find_first_combo(&numbers, third_of_tot_val);
    println!("Found {} intial package combo's", initial_groups_list.len());
    initial_groups_list.sort_by(| a, b| a.first_grp.len().cmp(&b.first_grp.len()));
    //filter on shortest list
    let shortest_list = initial_groups_list.first().unwrap().first_grp.len();
    let iter_short_lists = initial_groups_list.iter().filter(|group| group.first_grp.len() == shortest_list);
    let mut initial_short_list: Vec<PackageGroup> = Vec::new();
    iter_short_lists.for_each( | package_grp | initial_short_list.push(package_grp.clone()));
    drop(initial_groups_list);
    println!("Short list remains: {}", initial_short_list.len());
 
    let final_package_combos: Arc<Mutex<Vec<PackageGroup>>> = Arc::new(Mutex::new(Vec::new()));

    let thread_pool: ThreadPool = pool_initializer::initialize_threadpool();
    for item  in initial_short_list.drain(0..) {
        let local_final_package_list = final_package_combos.clone(); 
        thread_pool.execute(move || {
            let mut return_vec: Vec<PackageGroup> = find_other_groups(item, third_of_tot_val);
            local_final_package_list.lock().unwrap().append(&mut return_vec);
        });
    }
    thread_pool.join();
    {
        let mut final_list_no_mut = final_package_combos.lock().unwrap();
        println!("Found {} final package combo's", final_list_no_mut.len());
        final_list_no_mut.sort_by(|packgrp1, packgrp2| packgrp1.quantum_entanglement.cmp(&packgrp2.quantum_entanglement));
        println!("Lowest amount of quantum is: {}", final_list_no_mut.first().unwrap().quantum_entanglement);
    }
}

fn parse_file_content(file_content: &str) -> Vec<i32>
{
    let mut ret_val: Vec<i32> = Vec::new();
    let file_trimmed = file_content.trim();

    for line in file_trimmed.lines()
    {
        let line_trimmed: &str = line.trim();
        let number: i32 = line_trimmed.parse::<i32>().unwrap();
        ret_val.push(number);
    }

    ret_val
}