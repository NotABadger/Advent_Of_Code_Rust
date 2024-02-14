mod file_processor_file;

use file_processor_file::FileProcessor;

use std::process::exit;

const FILE: &str = "input.txt";

fn main() {
    println!("Let's execute some instructions");
    let file_content: String;

    if !FileProcessor::check_file_exists(FILE)
    {
        println!("Sorry, but couldn't find the input file");
        exit(0);
    }
    match FileProcessor::read_file(FILE) {
        Ok(read_content) => file_content = read_content,
        Err(msg) => panic!("{}",msg),
    }

    let (a, b) = line_reader(&file_content);
    println!("value a: {}, value b: {}", a,b);

}

fn line_reader(file_content: &str) -> (u32, u32)
{
    let mut reg_a: u32 = 0;
    // let mut reg_a: u32 = 1; //uncomment this line for part 2
    let mut reg_b: u32 = 0;
    let lines: Vec<&str> = file_content.lines().collect();
    let mut current_line: usize = 0;
    println!("File has {} lines", lines.len());

    loop {
        println!("Executing line: {}", current_line);
        if current_line > lines.len() -1
        {
            println!("Jumped outside of the file");
            break;
        }
        match lines.get(current_line) {
            Some(line) => {
                    let instruction: Option<Instructions>;
                    
                    let mut word_itt = line.split_ascii_whitespace();
                    let mut word = word_itt.next();
                    match word {
                        Some("hlf") => { instruction=Some(Instructions::HLF) } ,
                        Some("tpl") => { instruction=Some(Instructions::TPL) } ,
                        Some("inc") => { instruction=Some(Instructions::INC) } ,
                        Some("jmp") => { instruction=Some(Instructions::JMP) } ,
                        Some("jie") => { instruction=Some(Instructions::JIE) } ,
                        Some("jio") => { instruction=Some(Instructions::JIO) } ,
                        _ => panic!("None of the instructions were called!"),
                    }
                    word = word_itt.next();
                    let mut reg: &str = word.unwrap();
                    reg = reg.trim_end_matches(',');
                    match instruction {
                        Some(Instructions::HLF) => { 
                            match reg {
                                "a" => hlf(&mut reg_a),
                                "b" => hlf(&mut reg_b),
                                _ => panic!("Expected register as second word."),
                            }
                            current_line += 1;
                        },
                        Some(Instructions::TPL) => { 
                            match reg {
                                "a" => tpl(&mut reg_a),
                                "b" => tpl(&mut reg_b),
                                _ => panic!("Expected register as second word."),
                            }
                            current_line += 1;
                        } ,
                        Some(Instructions::INC) => { 
                            match reg {
                                "a" => inc(&mut reg_a),
                                "b" => inc(&mut reg_b),
                                _ => panic!("Expected register as second word."),
                            }
                            current_line += 1;
                        } ,
                        Some(Instructions::JMP) => { 
                            let jump_dist : i32;
                            //expect + or - number
                            if reg.starts_with('+') {
                                reg = reg.trim_start_matches('+');
                                jump_dist = reg.parse::<i32>().expect("Expected a number for jump instruction!");
                            }
                            else {
                                jump_dist = reg.parse::<i32>().expect("Expected a negative number for jump instruction!");
                            }
                            current_line = jmp(jump_dist, current_line);
                        } ,
                        Some(Instructions::JIE) => { 
                            let reg_ref: &u32;
                            match reg {
                                "a" => reg_ref = &reg_a,
                                "b" => reg_ref = &reg_b,
                                _ => panic!("Expected register as second word."),
                            }
                            reg = word_itt.next().expect("expected possible jump distance here.");
                            let jump_dist : i32;
                            //expect + or - number
                            if reg.starts_with('+') {
                                reg = reg.trim_start_matches('+');
                                jump_dist = reg.parse::<i32>().expect("Expected a number for jump instruction!");
                            }
                            else {
                                jump_dist = reg.parse::<i32>().expect("Expected a negative number for jump instruction!");
                            }
                            current_line = jie(reg_ref, jump_dist, current_line);
                        } ,
                        Some(Instructions::JIO) => { 
                            let reg_ref: &u32;
                            match reg {
                                "a" => reg_ref = &reg_a,
                                "b" => reg_ref = &reg_b,
                                _ => panic!("Expected register as second word."),
                            }
                            reg = word_itt.next().expect("expected possible jump distance here.")    ;
                            let jump_dist : i32;
                            //expect + or - number
                            if reg.starts_with('+') {
                                reg = reg.trim_start_matches('+');
                                jump_dist = reg.parse::<i32>().expect("Expected a number for jump instruction!");
                            }
                            else {
                                jump_dist = reg.parse::<i32>().expect("Expected a negative number for jump instruction!");
                            }
                            current_line = jio(reg_ref, jump_dist, current_line);
                        } ,
                        _ => panic!("How the bleep did you get here?? #RustSafety"),
                    }
                },
            None => break,
        }
    }


    (reg_a, reg_b)
}

fn hlf(reg: &mut u32)
{
    *reg = *reg / 2;
}

fn tpl(reg: &mut u32)
{
    *reg = *reg * 3;
}

fn inc(reg: &mut u32)
{
    *reg += 1;
}

fn jmp(offset: i32, current_line: usize) -> usize
{
    if offset.is_negative() {
       let absolute: usize = offset.abs() as usize;
        if absolute > current_line {
            panic!("trying to jump back before the file lines started");
        }
        return current_line - absolute;
    }
    else {
        return offset as usize + current_line;
    }
}

fn jie(reg: &u32, offset: i32, current_line: usize) -> usize
{
    if *reg % 2 == 0 {    
        if offset.is_negative() {
            let absolute: usize = offset.abs() as usize;
            if absolute > current_line {
                panic!("trying to jump back before the file lines started");
            }
                return current_line - absolute;
            }
            else {
                return offset as usize + current_line;
            }
    }
    else {
        return current_line +1;
    }
}

fn jio(reg: &u32, offset: i32, current_line: usize) -> usize
{
    if *reg == 1 {    
        if offset.is_negative() {
            let absolute: usize = offset.abs() as usize;
            if absolute > current_line {
                panic!("trying to jump back before the file lines started");
            }
                return current_line - absolute;
            }
            else {
                return offset as usize + current_line;
            }
    }
    else {
        return current_line +1;
    }
}

enum Instructions {
    HLF,
    TPL,
    INC,
    JMP,
    JIE,
    JIO,
}