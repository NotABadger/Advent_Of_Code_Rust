use crate::replacement::Replacement;

pub fn parse_replacements(text :&str) -> Vec<Replacement>
{
    let mut replacements_list: Vec<Replacement> = Vec::new();
    let trimmed_text = text.trim();

    for line in trimmed_text.lines()
    {
        let trimmed_line = line.trim();
        if trimmed_line.contains("=>")
        {
            let replace: Replacement = Replacement::new(
                &parse_match_str(trimmed_line), 
                &parse_replace_str(trimmed_line));
            replacements_list.push(replace)
        }
    }

    replacements_list
}

pub fn parse_molecule(text :&str) -> String
{
    let trimmed_text = text.trim();
    let last_line = trimmed_text.lines().last().expect("expecting molecule line on the end of the file").trim();
    last_line.to_string()    
}

fn parse_match_str(line: &str) -> String
{
    let mut itt = line.split_whitespace();
    itt.next().expect("Expected match value").to_string()
}

fn parse_replace_str(line: &str) -> String
{
    let mut itt = line.split_whitespace();
    itt.next();
    itt.next();
    itt.next().expect("expected replace value").to_string()
}