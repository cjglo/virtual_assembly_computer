mod register;

pub struct MemIOInterface
{
    mem: [memory::Register; 10],
}

impl MemIOInterface
{
    pub fn new() -> MemIOInterface
    {
        MemIOInterface
        {
            mem: [memory::Register::new(); 10],
        }
    }

    // dr s1 and s2 returned
    pub fn operands(instruction: &super::console::Console) -> (u8, u8)
    {
        (mem[instruction.s1].val, mem[instruction.s2].val)
    }

    pub fn direct(instruction: &super::console::Console, result: u8)
    {
        mem[instruction.dr].val = result;
    }


    // FOR DEBUG ONLY
    pub fn to_screen(&self)
    {
        println!("Register 0 has: {} ", mem[0]);
        println!("Register 1 has: {} ", mem[1]);
        println!("Register 2 has: {} ", mem[2]);
        println!("Register 3 has: {} ", mem[3]);
        println!("Register 4 has: {} ", mem[4]);
        println!("Register 5 has: {} ", mem[5]);
        println!("Register 6 has: {} ", mem[6]);
        println!("Register 7 has: {} ", mem[7]);
        println!("Register 8 has: {} ", mem[8]);
        println!("Register 9 has: {} ", mem[9]);
    }


}