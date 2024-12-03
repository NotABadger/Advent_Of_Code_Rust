//find all galaxies, all routes, and print sum of all routes
use crate::universe::Universe;

pub fn part1(mut uni: Universe) {
    uni.expand_part1();
    let galaxies_coordinates: Vec<(usize, usize)> = find_all_galaxies(&uni);
    println!("found {} galaxies", galaxies_coordinates.len());
    let galaxy_routes: Vec<(usize, usize)> = find_all_routes(&galaxies_coordinates);
    println!("found {} pairs/routes", galaxy_routes.len());
    let total_distance : usize = calculate_distance_between_all_galaxies(&galaxies_coordinates, &galaxy_routes);
    println!("total part 1 distance is: {}", total_distance);
}

//vec from index galaxies, to index galaxies
fn find_all_routes(galaxies_lst: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut routes_between_galaxies: Vec<(usize, usize)> = Vec::new();
    for start_index in 0..(galaxies_lst.len() -1) { //last galaxy will not need to be routed 
        for end_index in (start_index + 1)..galaxies_lst.len() {
            routes_between_galaxies.push((start_index, end_index));
        }
    }
    routes_between_galaxies
}

fn find_all_galaxies(expanded_uni: &Universe) -> Vec<(usize,usize)> {
    let mut galaxies_coordinates: Vec<(usize, usize)> = Vec::new();
    for (coordinates, _) in expanded_uni.get_expaned_uni().iter()
                                                            .filter(|((_, _), char)| **char == Universe::GALAXY) {
        galaxies_coordinates.push(*coordinates);
    }
    galaxies_coordinates
}

fn calculate_distance_between_all_galaxies(galaxies_lst: &Vec<(usize, usize)>, routes_lst: &Vec<(usize, usize)>) -> usize {
    let mut total_distance: usize = 0;
    for (start_galax_index, end_galax_index) in routes_lst {
        let (start_gal_x, start_gal_y) = galaxies_lst.get(*start_galax_index).unwrap();
        let (end_gal_x, end_gal_y) = galaxies_lst.get(*end_galax_index).unwrap();
        total_distance += calculate_distance_galaxies((*start_gal_x, *start_gal_y),(*end_gal_x, *end_gal_y));
    }
    total_distance
}

fn calculate_distance_galaxies((galax_one_x, galax_one_y): (usize, usize), (galax_two_x, galax_two_y): (usize, usize)) -> usize {
    let x_diff = galax_one_x.abs_diff(galax_two_x);
    let y_diff = galax_one_y.abs_diff(galax_two_y);

    //println!("from {},{} to {},{} is {} distance", galax_one_x, galax_one_y, galax_two_x, galax_two_y, (x_diff + y_diff));
    x_diff + y_diff
}

