pub fn total_matrix_value(matrix: &Vec<Vec<char>>, size: (usize, usize)) -> u32
{
    let mut total_value: u32 = 0;

    for line in 0..size.0
    {
        let mut first_digit_found: bool = true;
        for char in 0..size.1
        {
            //1. Find char is digit
            if matrix[line][char].is_digit(10)
            {
                //2. Check if it is the first digit on the line
                if first_digit_found
                {
                    first_digit_found = false;
                    //3. Get rest of number
                    let number_str: String = find_complete_number(matrix, size, (line, char));
                    
                    //4. Check if there are symbols surrounding
                    if is_valid_number(matrix, size, (line, char), &number_str)
                    {
                        //5. Add to total value
                        total_value += number_str.parse::<u32>().unwrap();
                    }
                }
                
            }
            else { 
                //reset algorithm
                first_digit_found = true;
            }
        }
    }
    total_value
}

fn find_complete_number(matrix: &Vec<Vec<char>>, size: (usize, usize), first_num_found: (usize, usize)) -> String
{
    let mut found_number: bool = true;
    let mut reading_coordinate: (usize, usize) = first_num_found;
    let mut return_str: String = String::new();
    while found_number
    {
        found_number = false;
        if reading_coordinate.1 < size.1 && matrix[reading_coordinate.0][reading_coordinate.1].is_digit(10)
        {//check if we don't run outside of the matrix, and if digit
            return_str.push(matrix[reading_coordinate.0][reading_coordinate.1]);
            found_number = true;
        }
        reading_coordinate.1 += 1;
    }

    return_str
}

fn is_valid_number(matrix: &Vec<Vec<char>>, size: (usize, usize), first_num_found: (usize, usize), number_str: &str) -> bool
{
    //Has to check line above (include diagonal)
    if first_num_found.0 > 0 
    {//make sure we don't check outside of matrix
        //check from horizontal position up to horizontal position + str_len
        for index in (first_num_found.1)..(first_num_found.1 + number_str.len())
        {
            let found_char: char = matrix[first_num_found.0 - 1][index];
            if is_symbol(&found_char)
            {
                return true;
            }
        }
        //check 2 diagonals, but check out of bounds
    
         // left top
        if first_num_found.1 > 1
        {//make sure our number didn't start at the beginning of the matrix line, or can 
            let found_char: char = matrix[first_num_found.0 - 1][first_num_found.1 - 1];
            if is_symbol(&found_char)
            {
                return true;
            }
        }
        //right top
        if first_num_found.1 + number_str.len()< size.1 - 1
        {
            let found_char: char = matrix[first_num_found.0 - 1][first_num_found.1 + number_str.len()];
            if is_symbol(&found_char)
            {
                return true;
            }
        }
    }
    
    //Has to check left neighbour
    if first_num_found.1 > 0
    {
        let found_char: char = matrix[first_num_found.0][first_num_found.1 - 1];
        if is_symbol(&found_char)
        {
            return true;
        }
    }
    //Has to check right neighbour
    if first_num_found.1 + number_str.len() < size.1 - 1
    {
        let found_char: char = matrix[first_num_found.0][first_num_found.1 + number_str.len()];
        if is_symbol(&found_char)
        {
            return true;
        }
    }

    //Has to check line below (include diagonal)
    if first_num_found.0 < size.0 - 1 
    {//make sure we don't check outside of matrix
        //check from horizontal position up to horizontal position + str_len
        for index in (first_num_found.1)..(first_num_found.1 + number_str.len())
        {
            let found_char: char = matrix[first_num_found.0 + 1][index];
            if is_symbol(&found_char)
            {
                return true;
            }
        }
        //check 2 diagonals, but check out of bounds
    
         // left bottom
        if first_num_found.1 > 1
        {//make sure our number didn't start at the beginning of the matrix line, or can 
            let found_char: char = matrix[first_num_found.0 + 1][first_num_found.1 - 1];
            if is_symbol(&found_char)
            {
                return true;
            }
        }
        //right bottom
        if first_num_found.1 + number_str.len() < size.1
        {
            let found_char: char = matrix[first_num_found.0 + 1][first_num_found.1 + number_str.len()];
            if is_symbol(&found_char)
            {
                return true;
            }
        }
    }
   
    false
}

fn is_symbol(character_ref: &char) ->bool
{
    if *character_ref != '.' && !character_ref.is_digit(10)
    {
        return true;
    }
    false
}