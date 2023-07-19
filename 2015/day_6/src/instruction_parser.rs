pub mod instruction_parser_mod{
    use crate::instruction::instruction_mod::{*};

    pub fn parse_file_to_list_instructions(file_content: &str) -> Box<Vec<Instruction>>
    {
        let mut instructions_list: Box<Vec<Instruction>> = Box::new(Vec::new());
        for line in file_content.lines()
        { 
            let mut parsing_instruction: Instruction = Instruction{ instruction_action: InstructionType::TurnOff, start_coordinate: (0,0), end_coordinate: (0,0)};
            match instruction_type_pars(line)
            {
                Some(result) => parsing_instruction.instruction_action = result,
                None => {
                    println!("Had a line I couldn't parse: {:?}", line);
                    continue; //continue on next line
                }
            }
            match parse_start_coordinate(line)
            {
                Some(result) => parsing_instruction.start_coordinate = result,
                None => {
                    println!("Couldn't parse start coördinates: {:?}", line);
                    continue; //continue on next line
                }
            }
            match parse_end_coordinate(line)
            {
                Some(result) => parsing_instruction.end_coordinate = result,
                None => {
                    println!("Couldn't parse end coördinates: {:?}", line);
                    continue; //continue on next line
                }
            }
            instructions_list.push(parsing_instruction);
        }
        return instructions_list;
    }

    fn instruction_type_pars(line: &str) -> Option<InstructionType>
    {
        if line.starts_with("turn on")
        {
            return Some(InstructionType::TurnOn);
        }
        if line.starts_with("turn off")
        {
            return Some(InstructionType::TurnOff);
        }
        if line.starts_with("toggle")
        {
            return Some(InstructionType::Toggle);
        }
        return None;
    }

    fn parse_start_coordinate(line: &str) -> Option<(usize,usize)>
    {
        let string_refs: Vec<&str> = line.split("through").collect();
        if string_refs.len() > 1
        {     
            let words: Vec<&str> = string_refs[0].split(" ").collect();   
            for piece in words
            {
                if piece.contains(',')
                {
                    let numbers: Vec<&str> = piece.split(',').collect();
                    if numbers.len() > 1
                    {
                        let coordinate1 = numbers[0].trim().parse::<usize>().expect("was expecting first coördinate.");
                        let coordinate2 = numbers[1].trim().parse::<usize>().expect("was expecting second coördinate.");
                        return Some((coordinate1,coordinate2));
                    }
                }
            }
        }
        return None;
    }

    fn parse_end_coordinate(line: &str) -> Option<(usize,usize)>
    {
        let string_refs: Vec<&str> = line.split("through").collect();
        if string_refs.len() > 1
        {     
            let words: Vec<&str> = string_refs[1].split(" ").collect();   
            for piece in words
            {
                if piece.contains(',')
                {
                    let numbers: Vec<&str> = piece.split(',').collect();
                    if numbers.len() > 1
                    {
                        let coordinate1 = numbers[0].trim().parse::<usize>().expect("was expecting first coördinate.");
                        let coordinate2 = numbers[1].trim().parse::<usize>().expect("was expecting second coördinate.");
                        return Some((coordinate1,coordinate2));
                    }
                }
            }
        }
        return None;
    }

}