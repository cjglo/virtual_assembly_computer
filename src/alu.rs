use super::memory_io_interface;
use super::console;

pub struct ArithmeticLogicUnit
{

}

impl ArithmeticLogicUnit
{
    // MAY INCLUDE THIS LATER, DO WE WANT TO BUILD AN ALU ITSELF?
    // pub fn new() -> ArithmeticLogicUnit
    // {
    //     ArithmeticLogicUnit
    //     {

    //     }
    // }

    pub fn execute(mioi: &mut memory_io_interface::MemIOInterface, con: &console::Console)
    {
        match con.op
        {
            console::AsbType::NOT => { let result = ArithmeticLogicUnit::not( mioi.operands(con) ); mioi.direct( con, result); },
            console::AsbType::ADD => ArithmeticLogicUnit::add(), // not implemented
            console::AsbType::SUB => ArithmeticLogicUnit::sub(), // not implemented
            // ADD HERE!!
            // ----------

            // not valid
            console::AsbType::INV => panic!("Literal Invalid (INV) Read into ALU!"),
            console::AsbType::BRK => panic!("Break (BRK) Read into ALU!"),
        }
    }


    ///// Operation Logic /////

    fn not( (a, _b): (u8, u8) ) -> u8
    {
        !a
    }

    fn add( (a, b): (u8, u8) ) -> u8
    {
        // BIG NOTE: NEED TO ALLOW OVERFLOW HERE! VARIABLES WILL NOT DO IT NATURALLY

    }

    fn sub()
    {
        // BIG NOTE: NEED TO ALLOW OVERFLOW HERE! VARIABLES WILL NOT DO IT NATURALLY
    }



}


struct BitForm
{
    curr_num: u8,
    empty: bool,  // !!! Don't want this, gotta change it to counter so outputs 0s until 8 bits are filled
}

impl Iterator for BitForm  // !!! LOGIC IS STILL OFF!!!
{
    type Item = bool;

    fn next(&mut self) -> Option<bool> // note: NO WAY TO RESET (as currently is)!
    {
        if self.curr_num % 2 != 0 
        {
            self.curr_num -= 1;
            return Option<true>;
        }
        else if !empty
        {
            if curr_num == 0 { empty = true; }
            self.curr_num /= 2;
            return Option<false>;
        }
        else
        {
            None
        }

    }
}