
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Universe {
    original_width: usize,
    original_height: usize,
    universe_original: HashMap<(usize, usize), char>,
    universe_original_vec: Vec<Vec<char>>,
    expanded_width: usize,
    expanded_height: usize,
    universe_expanded: HashMap<(usize, usize), char>,
}

impl Universe {
    pub const EMPTY : char = '.';
    pub const GALAXY : char = '#';

    pub fn get_orig_height(&self) -> usize {
        self.original_height
    }
    pub fn get_orig_width(&self) -> usize {
        self.original_width
    }
    pub fn get_expand_height(&self) -> usize {
        self.expanded_height
    }
    pub fn get_expand_width(&self) -> usize {
        self.expanded_width
    }
    pub fn get_origi_uni(&self) -> &HashMap<(usize, usize), char> {
        &self.universe_original
    }
    pub fn get_expaned_uni(&self) -> &HashMap<(usize, usize), char> {
        &self.universe_expanded
    }
    pub fn get_empty_rows(&self) -> Vec<usize>{
    //Filter per row/column on matching row/column, and check if there are any 'Galaxies' in the row/column
    let mut empty_rows: Vec<usize> = Vec::new();
        for row in 0..self.original_height {
            if !self.universe_original.iter().filter(| ((_, y), _) | *y == row)
            .any(| ((_, _), char)| *char == Self::GALAXY ) {
                empty_rows.push(row);    
            }
        }
        empty_rows
    }
    pub fn get_empty_columns(&self) -> Vec<usize>{
        //Filter per row/column on matching row/column, and check if there are any 'Galaxies' in the row/column
        let mut empty_columns: Vec<usize> = Vec::new();
            for column in 0..self.original_height {
                if !self.universe_original.iter().filter(| ((x, _), _) | *x == column)
                .any(| ((_, _), char)| *char == Self::GALAXY ) {
                    empty_columns.push(column);    
                }
            }
            empty_columns
        }

    pub fn from_file_content(file_content: &str) -> Self {
        let mut universe: HashMap<(usize,usize), char> = HashMap::new();
        let mut universe_vec: Vec<Vec<char>> = Vec::new();
        let mut width: usize = 0;
        for (y_index, line) in file_content.lines().enumerate() {
            let mut line_vec: Vec<char> = Vec::new();
            for (x_index, character) in line.chars().enumerate()
            {
                if !Self::check_legit_character(&character) {
                    panic!("found illegal character when reading file");
                }
                if x_index > width { width = x_index}
                universe.insert((x_index, y_index), character);
                line_vec.push(character);
            }
            universe_vec.push(line_vec);
        }
        width += 1;
        Self { original_width: width, original_height: universe_vec.len(), 
                universe_original: universe, universe_original_vec: universe_vec ,
                expanded_width: 0, expanded_height: 0, universe_expanded: HashMap::new() }
    }

    pub fn expand_part1(&mut self) {
        let mut columns_to_add: Vec<usize> = self.get_empty_columns();
        let mut rows_to_add: Vec<usize> = self.get_empty_rows();

        self.inject_columns_and_rows(&mut columns_to_add, &mut rows_to_add)
    }

    fn inject_columns_and_rows(&mut self, empty_columns: &mut Vec<usize>, empty_rows: &mut Vec<usize>)
    {
        //Rev sort so we don't ruin order
        empty_columns.sort_by(|a , b|  a.cmp(b).reverse());
        empty_rows.sort_by(|a , b|  a.cmp(b).reverse());
        //be aware, Vec is y,x system, while Hashmap is x,y system
        let mut expanded_uni_vec: Vec<Vec<char>> = self.universe_original_vec.clone();
        let mut expanded_uni: HashMap<(usize,usize), char> = HashMap::new();
        //y-axis, insert rows
        for number in empty_rows.drain(..) {
            let insert_column: Vec<char> = vec![Self::EMPTY; self.original_width];
            expanded_uni_vec.insert(number+1, insert_column);
        }
        //x-axis, insert columns
        for number in empty_columns.drain(..){
            for column in expanded_uni_vec.iter_mut() {
                column.insert(number+1, Self::EMPTY);
            }
        }

        //translate to Hashmap for performance in searching
        self.expanded_height = expanded_uni_vec.len();
        let mut width = 0;
        for (y_index, row) in expanded_uni_vec.iter().enumerate() {
            for (x_index, character) in row.iter().enumerate()
            {
                if !Self::check_legit_character(&character) {
                    panic!("found illegal character when reading file");
                }
                if x_index > width { width = x_index}
                expanded_uni.insert((x_index, y_index), *character);
            }
        }
        width += 1;
        self.expanded_width = width;
        self.universe_expanded = expanded_uni;
    }

    fn check_legit_character(character: &char) -> bool {
        match *character {
            Self::EMPTY => true,
            Self::GALAXY => true,
            _ => false
        }
    }
}

impl ToString for Universe {
    fn to_string(&self) -> String {
        let mut ret_val = String::new();
        let universe_to_print: &HashMap<(usize,usize), char>;
        let height: usize;
        if self.universe_expanded.len() > 0 {
            universe_to_print = &self.universe_expanded;
            height = self.expanded_height;
        }
        else {
            if self.universe_original.len() == 0 {
                return ret_val
            }
            universe_to_print = &self.universe_original;
            height = self.original_height;
        }

        for index in 0..height {
            let mut line_vec: Vec<(&(usize, usize), &char)> = universe_to_print.iter()
                                                                .filter(|((_, y_cor), _)| *y_cor == index)
                                                                .collect();
            line_vec.sort_by(|((x, _), _), ((x_two, _), _)| x.cmp(x_two));
            line_vec.iter().for_each(|((_,_),space)| ret_val.push(**space));
            ret_val.push('\n');
        }
        ret_val
    }
}