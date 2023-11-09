use crate::lightgrid::LightGrid;

pub fn process_next_frame(start_grid: &LightGrid)-> LightGrid
{
    let mut end_grid: LightGrid = start_grid.clone();
    let len_line: usize = end_grid.grid.len();
    let len_column: usize = end_grid.grid[0].len();
    for line_index in 0..len_line
    {
        for column_index in 0..len_column
        {
            let lights_surrounding: u32 = scan_neighbours_for_light(line_index, column_index, start_grid);
            if start_grid.grid[line_index][column_index].state
            {// check stay on conditions
                if lights_surrounding == 2 || lights_surrounding == 3
                {
                    end_grid.grid[line_index][column_index].state = true;
                }
                else 
                {
                    end_grid.grid[line_index][column_index].state = false;
                }
            }
            else
            {// check stay off position
                if lights_surrounding == 3
                {
                    end_grid.grid[line_index][column_index].state = true;
                }
                else {
                    end_grid.grid[line_index][column_index].state = false;
                }
            }
        }
    }

    end_grid
}

pub fn scan_neighbours_for_light (line_index: usize, column_index: usize, grid: &LightGrid) -> u32{
    let mut lights_on = 0;
    let max_column = grid.grid[0].len() -1;
    let max_line = grid.grid.len() -1;
    if line_index > 0
    {//scan light above
        if grid.grid[line_index -1][column_index].state
        {
            lights_on += 1;
        }
    }
    if line_index < max_line
    {//scan light below
        if grid.grid[line_index +1][column_index].state
        {
            lights_on += 1;
        }
    }
    if column_index > 0
    {//scan light left
        if grid.grid[line_index][column_index -1].state
        {
            lights_on += 1;
        }
    }
    if column_index < max_column
    {//scan light right
        if grid.grid[line_index][column_index +1].state
        {
            lights_on += 1;
        }
    }

    if line_index > 0 && column_index > 0
    {//scan light left above
        if grid.grid[line_index -1][column_index -1].state
        {
            lights_on += 1;
        }
    }
    if line_index > 0 && column_index < max_column
    { //scan light right above
        if grid.grid[line_index -1][column_index +1].state
        {
            lights_on += 1;
        }
    }
    if line_index < max_line && column_index > 0
    {//scan light left below
        if grid.grid[line_index +1][column_index -1].state
        {
            lights_on += 1;
        }
    }
    if line_index < max_line && column_index < max_column
    {//scan light right below
        if grid.grid[line_index +1][column_index +1].state
        {
            lights_on += 1;
        }
    }
    lights_on
}