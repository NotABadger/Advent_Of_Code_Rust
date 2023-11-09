use crate::light::Light;

#[derive(Clone)]
pub struct LightGrid {
    pub grid: Vec<Vec<Light>>
}

impl Default for LightGrid{
    fn default() -> Self {
        Self { grid: vec![vec![Light::default(); 100]; 100]}
    }
}

impl LightGrid {
    pub fn lights_currently_on(&self) -> u32 {
        let mut amount_turned_on: u32 = 0;
        self.grid.iter().for_each(|list| list.iter().for_each(|light | if light.state {amount_turned_on += 1;}));
        amount_turned_on
    }


    pub fn print_to_string(&self) -> String {
        let mut return_string: String = String::new();
        
        for line_index in 0..self.grid.len()
        {
            for column_index in 0..self.grid[0].len()
            {
                if self.grid[line_index][column_index].state
                {// check stay on conditions
                    return_string.push('#');
                }
                else
                {// check stay off position
                    return_string.push('.');
                }
            }
            return_string.push('\n');
        }
        return_string
    }
}