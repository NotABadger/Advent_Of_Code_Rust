use crate::universe::Universe;

pub fn part2(uni: Universe) {
    let galaxies_lst: Vec<(usize, usize)> = find_all_galaxies(&uni);
    let expanded_galaxies_lst: Vec<(usize,usize)> = expand_universe_part2(&galaxies_lst, uni.get_empty_rows(), uni.get_empty_columns());
    let routes: Vec<(usize,usize)> = find_all_routes(&expanded_galaxies_lst);
    let total_distance: usize = calculate_distance_between_all_galaxies(&expanded_galaxies_lst, &routes);
    println!("total part 2 distance is: {}", total_distance);
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

    x_diff + y_diff
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

fn expand_universe_part2(galaxies_lst: &Vec<(usize,usize)>, mut empty_rows: Vec<usize>, mut empty_columns: Vec<usize>) -> Vec<(usize,usize)> {
    let mut output = galaxies_lst.clone();
    empty_columns.sort_by(| a, b| a.cmp(b).reverse());
    empty_rows.sort_by(| a, b| a.cmp(b).reverse());
    for col in empty_columns.drain(..){
        for (x, _) in output.iter_mut().filter(|(x, _)|  *x > col) {
            *x += 999_999
        }
    }
    for row in empty_rows.drain(..){
        for (_, y) in output.iter_mut().filter(|(_, y)|  *y > row) {
            *y += 999_999
        }
    }
    output
}

fn find_all_galaxies(expanded_uni: &Universe) -> Vec<(usize,usize)> {
    let mut galaxies_coordinates: Vec<(usize, usize)> = Vec::new();
    for (coordinates, _) in expanded_uni.get_origi_uni().iter()
                                                            .filter(|((_, _), char)| **char == Universe::GALAXY) {
        galaxies_coordinates.push(*coordinates);
    }
    galaxies_coordinates
}