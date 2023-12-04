mod matrix_parser;
mod matrix_number_scanner;
mod matrix_gear_scanner;

use input_file_lib::read_file_content;


fn main() {
    let input_file_content: String = read_file_content().expect("Reading file failed");
    let matrix_size: (usize, usize) = matrix_parser::check_matrix_sizes(&input_file_content).expect("We do expect a nice matrix");
    let matrix: Vec<Vec<char>> = matrix_parser::parse_matrix(&input_file_content, matrix_size);

    // part 1
    println!("The total value of the matrix is: {}", matrix_number_scanner::total_matrix_value(&matrix, matrix_size));

    //part two
    println!("The total value of the gear ratios is: {}", matrix_gear_scanner::total_matrix_value(&matrix, matrix_size));
}
