mod register;

pub struct MemIOInterface
{
    mem: [register::Register; 10], // change memory means change this array as well as below size
    _size: i32,
}

impl MemIOInterface
{
    pub fn new() -> MemIOInterface
    {
        const CHANGE_MEM_HERE: i32 = 10; // change memory size here!!!!
        MemIOInterface
        {
            mem: [register::Register::new(); CHANGE_MEM_HERE as usize],
            _size: CHANGE_MEM_HERE,
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
    //     self._size
    // }


    // FOR DEBUG ONLY
    pub fn to_screen(&self)
    {
        println!("Register 0 has: {} ", self.mem[0].val);
        println!("Register 1 has: {} ", self.mem[1].val);
        println!("Register 2 has: {} ", self.mem[2].val);
        println!("Register 3 has: {} ", self.mem[3].val);
        println!("Register 4 has: {} ", self.mem[4].val);
        println!("Register 5 has: {} ", self.mem[5].val);
        println!("Register 6 has: {} ", self.mem[6].val);
        println!("Register 7 has: {} ", self.mem[7].val);
        println!("Register 8 has: {} ", self.mem[8].val);
        println!("Register 9 has: {} ", self.mem[9].val);
    }

}