mod register;

struct MemIOInterface
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

    pub fn execute(_instruction: &super::console::Console)
    {

    }

    pub fn shut_down()
    {

    }
}