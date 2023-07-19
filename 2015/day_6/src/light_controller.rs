pub mod light_controller
{
    use crate::light::Light;
    use crate::instruction::instruction_mod::{*};

    struct LightControllerData
    {
        lights : Vec<Vec<Light>>,
    }

    impl LightControllerData
    {
        fn new() -> Self
        {
            return Self{lights: vec![vec![Light::new(); 1000]; 1000]};
        }
    }

    pub fn execute_light_commands(commands: Box<Vec<Instruction>>) -> u32
    {
        println!("I reached the execute stage");
        let mut controller_data: LightControllerData = LightControllerData::new();
        for command in commands.iter()
        {
            match command.instruction_action
            {
                InstructionType::TurnOn => execute_turn_on(&mut controller_data, command),
                InstructionType::TurnOff => execute_turn_off(&mut controller_data, command),
                InstructionType::Toggle =>execute_toggle(&mut controller_data, command),
            }
        }
        
        return count_lights_turned_on(&controller_data);
    }

    fn execute_turn_on(controller_data: &mut LightControllerData, command: &Instruction)
    {
        for column_itt in &mut controller_data.lights[command.start_coordinate.0..command.end_coordinate.0+1]
        { 
            for row_itt in &mut column_itt[command.start_coordinate.1..command.end_coordinate.1+1]
            {
                row_itt.turn_on();
            }
        }
    }

    fn execute_turn_off(controller_data: &mut LightControllerData, command: &Instruction)
    {
        for column_itt in &mut controller_data.lights[command.start_coordinate.0..command.end_coordinate.0+1]
        {
            for row_itt in &mut column_itt[command.start_coordinate.1..command.end_coordinate.1+1]
            {
                row_itt.turn_off();
            }
        }
    }

    fn execute_toggle(controller_data: &mut LightControllerData, command: &Instruction)
    {
        for column_itt in &mut controller_data.lights[command.start_coordinate.0..command.end_coordinate.0+1]
        {
            for row_itt in &mut column_itt[command.start_coordinate.1..command.end_coordinate.1+1]
            {
                row_itt.toggle();
            }
        }
    }

    fn count_lights_turned_on(controller_data: &LightControllerData) ->u32
    {
        let mut amount_of_lights_on: u32 = 0;
        for column_itt in &controller_data.lights[0..]
        {
            for row_itt in column_itt
            {
                //if row_itt.current_state()
                //{
                    amount_of_lights_on += row_itt.current_state();
                    //amount_of_lights_on +=1;
                //}
            }
        }
        return amount_of_lights_on;
    }
}