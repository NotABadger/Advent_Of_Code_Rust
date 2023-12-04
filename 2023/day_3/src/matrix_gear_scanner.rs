pub fn total_matrix_value(matrix: &Vec<Vec<char>>, size: (usize, usize)) -> u32
{
    let mut total_value: u32 = 0;

    for line in 0..size.0
    {
        for char in 0..size.1
        {
            //find gear
            if matrix[line][char] == '*'
            {
                //scan if there are 2 numbers
                match check_gear_for_ratios(matrix, size, (line,char)) {
                    None => continue,
                    Some((gear1, gear2)) => {
                        let gear_ratio: u32 = gear1.parse::<u32>().unwrap() * gear2.parse::<u32>().unwrap();
                        total_value += gear_ratio;
                    },
                }
            }

        }
    }
    total_value
}

fn check_gear_for_ratios(matrix: &Vec<Vec<char>>, size: (usize, usize), star_pos: (usize, usize)) -> Option<(String, String)>
{
    //check if it has 2 numbers, -> only 8 squares to check
    // bottom and top row -> check middle, if it's a digit. there is 1 number on the line. else check corners for 2 more numbers
    // left + right are always two numbers
    // expecting Exactly two numbers
    let mut number_str_list: Vec<String> = Vec::new();

    //left + right first
    if star_pos.1 > 0
    {
        if matrix[star_pos.0][star_pos.1 -1].is_digit(10)
        {
            //found number
            number_str_list.push(find_complete_number(matrix, size, (star_pos.0, star_pos.1 -1)));
        }
    }
    if star_pos.1 +1 < size.1
    {
        if matrix[star_pos.0][star_pos.1 +1].is_digit(10)
        {
            //found number
            number_str_list.push(find_complete_number(matrix, size, (star_pos.0, star_pos.1 +1)));
        }
    }

    //top line
    if star_pos.0 > 0
    {
        if matrix[star_pos.0 -1][star_pos.1].is_digit(10)
        {
            //found number
            number_str_list.push(find_complete_number(matrix, size, (star_pos.0 -1, star_pos.1 )));
        }
        else {
            if star_pos.1 > 0
            {
                if matrix[star_pos.0 -1][star_pos.1 -1].is_digit(10)
                {
                    //found number
                    number_str_list.push(find_complete_number(matrix, size, (star_pos.0 -1, star_pos.1 -1)));
                }
            }
            if star_pos.1 +1 < size.1
            {
                if matrix[star_pos.0 -1][star_pos.1 +1].is_digit(10)
                {
                    //found number
                    number_str_list.push(find_complete_number(matrix, size, (star_pos.0 -1, star_pos.1 +1)));
                }
            }
        }
    }

    //check bottom line
    if star_pos.0 +1 < size.0
    {
        if matrix[star_pos.0 +1][star_pos.1].is_digit(10)
        {
            //found number
            number_str_list.push(find_complete_number(matrix, size, (star_pos.0 +1, star_pos.1)));
        }
        else 
        {
            if star_pos.1 > 0
            {
                if matrix[star_pos.0 +1][star_pos.1 -1].is_digit(10)
                {
                    //found number
                    number_str_list.push(find_complete_number(matrix, size, (star_pos.0 +1, star_pos.1 -1)));
                }
            }
            if star_pos.1 +1 < size.1
            {
                if matrix[star_pos.0 +1][star_pos.1 +1].is_digit(10)
                {
                    //found number
                    number_str_list.push(find_complete_number(matrix, size, (star_pos.0 +1, star_pos.1 +1)));
                }
            }
        }
    }

    if number_str_list.len() == 2
    {
        println!("Found ratio's: {} * {}", number_str_list[0], number_str_list[1]);
        return Some((number_str_list.remove(0), number_str_list.remove(0)));
    }
    println!("NON-ratio's: {:?}", number_str_list);
    None
}

fn find_complete_number(matrix: &Vec<Vec<char>>, size: (usize, usize), first_num_found: (usize, usize)) -> String
{
    let mut found_number: bool = true;
    let mut reading_coordinate: (usize, usize) = first_num_found;
    let mut return_str: String = String::new();
    while found_number // check after 
    {
        found_number = false;
        if reading_coordinate.1 < size.1 && matrix[reading_coordinate.0][reading_coordinate.1].is_digit(10)
        {//check if we don't run outside of the matrix, and if digit
            return_str.push(matrix[reading_coordinate.0][reading_coordinate.1]);
            found_number = true;
        }
        reading_coordinate.1 += 1;
    }

    if first_num_found.1 > 1
    {
        found_number = true;
        reading_coordinate = first_num_found;

        while found_number // check before 
        {
            found_number = false;
            reading_coordinate.1 -= 1;
            if reading_coordinate.1 > 0 && matrix[reading_coordinate.0][reading_coordinate.1].is_digit(10)
            {//check if we don't run outside of the matrix, and if digit
                return_str.insert(0,matrix[reading_coordinate.0][reading_coordinate.1]);
                found_number = true;
            }
        }
    }

    return_str
}