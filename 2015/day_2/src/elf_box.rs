pub struct ElfBox {
    length : u32,
    width : u32,
    height : u32,
    surface_area : u32,
    smallest_area : u32,
}

impl ElfBox {
    pub fn elf_box_from_string(sizes : &str) -> ElfBox
    {
        let dimentions_str : String= sizes.trim().to_string();
        let dimentions_list : Vec<&str> = dimentions_str.rsplit('x').collect();
        let my_box = ElfBox {
            height : dimentions_list[0].parse().expect("Length parse failed"),
            width : dimentions_list[1].parse().expect("Width parse failed"),
            length : dimentions_list[2].parse().expect("Height parse failed"),
            surface_area : 0,
            smallest_area : 0,
        };
        //split at the 'x's' 
        return my_box;
    }

    pub fn calculate_areas(&mut self)
    {
        let surface1 = self.length * self.width;
        let surface2 = self.width * self.height;
        let surface3 = self.length * self.height;

        if surface1 < surface2 && surface1 < surface3
        {
            self.smallest_area = surface1.clone();
        }
        else if surface2 < surface3
        {
            self.smallest_area = surface2.clone();
        }
        else
        {
            self.smallest_area = surface3.clone();
        }

        self.surface_area = 2 * surface1;
        self.surface_area += 2 * surface2;
        self.surface_area += 2* surface3;   
    }

    pub fn calculate_feet_of_paper_required(&self) -> u32
    {
        return self.surface_area + self.smallest_area;
    }

    pub fn calculate_feet_of_ribbon_required(&self) -> u32
    {
        let mut feet_of_ribbon: u32 = 0;
        println!("Length of package: {:?}", &self.length);
        println!("Width of package: {:?}", &self.width);
        println!("Height of package: {:?}", &self.height);
        if self.length > self.width && self.length > self.height
        {
            //use width & height
            println!("Length was bigest!");
            feet_of_ribbon = (2*self.width) + (2*self.height;)
        }
        else if self.width > self.length && self.width > self.height
        {
            //use height & length
            println!("Width was bigest!");
            feet_of_ribbon = (2*self.height) + (2*self.length);
        }
        else
        {
            //use width & length
            println!("Height was bigest!");
            feet_of_ribbon = (2*self.width) + (2*self.length);
        }
        let cubic_feet: u32= self.height * self.width * self.length;
        feet_of_ribbon += cubic_feet;

        println!("Total length in ribbon: {:?}", feet_of_ribbon);

        return feet_of_ribbon;
    }

}
