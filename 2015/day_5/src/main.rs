mod file_processor_file;

use crate::file_processor_file::file_processor::{*};

fn main() {
    println!("Hello, world!");
    const INPUT: &str = "input.txt";
    let file_content: String;
    

    if !check_file_exists(INPUT)
    {
        panic!("Can't find input file!");
    }
    
    match read_file(INPUT)
    {
        Ok(read_content) => file_content = read_content,
        Err(_msg) => {
            println!("Could not read file!");
            panic!();
        }
    }

    //println!("There are {:?} nice strings", part1(file_content));
    println!("There are {:?} nice strings", part2(file_content));
}

fn part1(file_content: String) -> u32
{
    let forbidden_strings: [&str; 4] = ["ab", "cd", "pq", "xy"];
    let vowels_list: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let mut nr_of_nice_str: u32 = 0;

    for line in file_content.lines()
    {
        let mut bad_str: bool = false;

        for forbidden_string in forbidden_strings
        {
            match line.find(forbidden_string)
            {
                Some(_) => { 
                    bad_str = true;
                    break;
                },
                None => (),
            }
        }
        if !bad_str
        {
            let mut nr_of_vowels: u32 = 0;
            let mut last_char: Option<char> = None;
            let mut double_char = false;
            for character in line.chars()
            {
                if last_char.is_some_and(|c| c == character)
                {
                    double_char = true;
                }
                if vowels_list.contains(&character)
                {
                    nr_of_vowels += 1;
                }

                last_char = Some(character.clone());
            }
            if double_char == false || nr_of_vowels < 3
            {
                bad_str = true;
            }
        }

        if !bad_str 
        {
            nr_of_nice_str += 1;
        }
    }

    return nr_of_nice_str;
}

fn part2(file_content: String) -> u32
{
    let mut nr_of_nice_str: u32 = 0;
    for line in file_content.lines()
    {
        if line.len() > 4 // at least 2 pair
        {
            let string_length: usize = line.len();
            let mut found_pair: bool = false;
            let mut match_letters: bool = false;
            
            //find pair of letters that reoccur
            let mut original_pair_index: usize = 0;
            let mut compare_pair_index: usize = original_pair_index + 2;

            while original_pair_index+3 < string_length
            {
                let original_pair: &str = &line[original_pair_index..(original_pair_index+2)];
                
                while compare_pair_index+1 < string_length
                {
                    let compare_pair: &str = &line[compare_pair_index..(compare_pair_index+2)];
                    if compare_pair == original_pair
                    {
                        found_pair = true;
                        break;
                    }
                    compare_pair_index += 1;
                }
                if found_pair
                {
                    break;
                }
                original_pair_index+=1;
                compare_pair_index = original_pair_index + 2;
            }
        
            //find pair of same letters with one letter in the middel
            let mut index_itterators: usize = 0;
            while index_itterators +2 < string_length
            {
                let first_pos_itterator: &str = &line[index_itterators..index_itterators+1];
                let third_pos_itterator: &str = &line[(index_itterators+2)..(index_itterators+3)];
                if !first_pos_itterator.trim().is_empty() && first_pos_itterator == third_pos_itterator
                {
                    match_letters = true;
                    break;
                }
                index_itterators += 1;
            }
                
            if match_letters && found_pair
            {
                nr_of_nice_str += 1;
            }
        }
    }
    return nr_of_nice_str;
}