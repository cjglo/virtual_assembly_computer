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
            console::AsbType::ADD => { let result = ArithmeticLogicUnit::add( mioi.operands(con) ); mioi.direct( con, result); }, 
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
        let mut num1 = a;
        let mut num2 = b;
        let mut carry: u8;

        while num2 != 0 {
            carry = num1 & num2;
            num1 = num1 ^ num2;

            let (result, _) = (carry as u32).overflowing_shl(1);

            num2 = result as u8;
        }

        num1
    }


    fn sub()
    {
        // BIG NOTE: NEED TO ALLOW OVERFLOW HERE! VARIABLES WILL NOT DO IT NATURALLY
    }



}


#[cfg(test)] 
mod tests {

    #[test]
    fn overflow_100_plus_200() { // testing ADD operator with overflow

        let result = super::ArithmeticLogicUnit::add( (100, 200) );

        println!("The result was: {}", result);

        assert_eq!(result, 44);
    }


}
