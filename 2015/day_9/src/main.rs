mod file_processor_file;
mod city;
mod route;
mod country;
mod county_parser;
mod shortest_city_path_algoritm_mathieu;
mod shortest_city_path_algoritm_henri;
mod brute_force_all_paths;

use std::rc::Rc;

use file_processor_file::FileProcessor;
use county_parser::parse_country;

use crate::country::Country;


fn main() {
    const INPUT: &str = "input.txt";
    let country: Rc<Country>;

    if !FileProcessor::check_file_exists(INPUT)
    {
        panic!("Can't find input file!");
    }
    
    match FileProcessor::read_file(INPUT)
    {
        Ok(read_content) => country = Rc::new(parse_country(&read_content)),
        Err(_msg) => {
            println!("Could not read file!");
            panic!();
        }
    }
    
   
    match shortest_city_path_algoritm_mathieu::resolve_traveler_problem(country.clone()) {//Finding shortest path
        Some(shortest_route) =>  println!("Shortest route found by Mathieu's algoritm: {}", shortest_route),
        None => println!("Mathieu's algoritm could not find a shortest route that matched the amount of points with amount of cities."),
    }
        
    match shortest_city_path_algoritm_henri::resolve_traveler_problem(country.clone()) { //Finding shortest path
        Some(shortest_route) =>  println!("Shortest route found by Henri's algoritm: {}", shortest_route),
        None => println!("Henri's algoritm could not find a shortest route that matched the amount of points with amount of cities."),
    }

    match brute_force_all_paths::resolve_traveler_problem(country.clone()) {//Finding all paths
        Some(_) =>  (), //this algoritm prints the top of the stack to the terminal
        None => (),
    }
}
