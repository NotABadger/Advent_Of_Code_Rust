use crate::lightgrid::LightGrid; 

pub fn text_to_grid(text: &str) -> LightGrid
{
    let mut return_val: LightGrid = LightGrid::default();
    let text_trimmed: &str = text.trim();
    
    let mut line_index: usize = 0;
    for line in text_trimmed.lines()
    {
        let line_trimmed = line.trim();
        let mut char_index: usize = 0;
        for char in line_trimmed.chars()
        {
            if char == '#'
            {
                return_val.grid[line_index][char_index].state = true;
            }
            char_index += 1;
        }
        line_index += 1;
    }

    return_val
}