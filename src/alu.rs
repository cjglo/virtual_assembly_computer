use super::memory_io_interface;
use super::console;

pub struct ArithmeticLogicUnit
{

}

impl ArithmeticLogicUnit
{

    pub fn new() -> ArithmeticLogicUnit
    {
        ArithmeticLogicUnit
        {

        }
    }

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

    fn add()
    {

    }

    fn sub()
    {

    }



}