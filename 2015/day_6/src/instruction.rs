pub mod instruction_mod
{
    pub struct Instruction
    {
        pub instruction_action : InstructionType,
        pub start_coordinate : (usize, usize),
        pub end_coordinate : (usize, usize),
    }

    pub enum InstructionType
    {
        TurnOn,
        TurnOff,
        Toggle,
    }
}