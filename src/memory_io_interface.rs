mod register;

pub struct MemIOInterface
{
    mem: [register::Register; 10], // change memory means change this array as well as below size
    size: i32,
}

impl MemIOInterface
{
    pub fn new() -> MemIOInterface
    {
        const CHANGE_MEM_HERE: i32 = 10; // change memory size here!!!!
        MemIOInterface
        {
            mem: [register::Register::new(); CHANGE_MEM_HERE as usize],
            size: CHANGE_MEM_HERE,
        }
    }

    // dr s1 and s2 returned
    pub fn operands(&mut self, instruction: &super::console::Console) -> (u8, u8)
    {
        (self.mem[instruction.s1 as usize].val, self.mem[instruction.s2 as usize].val)
    }

    pub fn direct(&mut self, instruction: &super::console::Console, result: u8)
    {
        self.mem[instruction.dr as usize].val = result;
    }

    // Note: Below is no longer used, but kept in case of future use
    // pub fn amount_of_memory(&self) -> i32
    // {
    //    self._size
    // }


    pub fn give_registers(&self, i: u8) -> u8
    {
        self.mem[]
    }



}