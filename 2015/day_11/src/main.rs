fn main() {
    const INPUT: &str = "vzbxkghb";
    let mut input_vec: Vec<u8> = INPUT.to_ascii_lowercase().into_bytes();
    //increment_string(&mut input_vec); //only for part two
    
    while   !check_three_letter_increment(&input_vec) ||
            !check_no_illegal_char(&input_vec) ||
            !check_2_pairs(&input_vec)
    {
        increment_string(&mut input_vec);
    }

    let result_string = String::from_utf8(input_vec).unwrap();
    println!("New password: {:?}", result_string);
    
}

fn check_three_letter_increment(char_vec: &Vec<u8>) -> bool
{ // contain one three letter range like abc, bcd, or cde    
    for win in char_vec.windows(3)
    {
        let pos0 = win.get(0).unwrap();
        let pos1 = win.get(1).unwrap();
        let pos2 = win.get(2).unwrap();

        if *pos2 == *pos1 + 1 &&
            *pos1 == *pos0 + 1
        {
            return true;
        }
    }
    return false;
}

fn check_no_illegal_char(char_vec: &Vec<u8>) -> bool
{//Check for i, o, l
    const LETTER_I: u8 = 'i' as u8;
    const LETTER_O: u8 = 'o' as u8;
    const LETTER_L: u8 = 'l' as u8;
    
    for i in char_vec
    {
        if  *i == LETTER_I ||
            *i == LETTER_O ||
            *i == LETTER_L
        {
            return false;
        }
    }
    true
}

fn check_2_pairs(char_vec: &Vec<u8>) -> bool
{//two different one letter pair aabb , ccdd, etc
    let mut found_first: bool = false;
    let mut first_char: u8 = 'z' as u8;
    for win in char_vec.windows(2)
    {
        let pos0 = win.get(0).unwrap();
        let pos1 = win.get(1).unwrap();

        if *pos0 == *pos1
        {
            if found_first && *pos0 != first_char
            {
                return true;
            }
            found_first = true;
            first_char = *pos0;
        }
    }
    return false;
}

fn increment_string(char_vec: &mut Vec<u8>)
{
    //increment last letter a -> b, b -> c etc.
    //when letter increments above z, it should flip to a, and next letter should be incremented.
    const LETTER_A: u8 = 'a' as u8;
    const LETTER_Z: u8 = 'z' as u8;
    let mut index: usize = char_vec.len();
    let mut done: bool = false;
    // on char

    while !done
    {
        index = index -1;
        if index > (char_vec.len() -1)
        {
            break;
        }
        let var = char_vec.get_mut(index).unwrap();
        *var += 1;
        if *var > LETTER_Z
        {
            *var = LETTER_A;
            continue;
        }
        done = true;
    }

}
