mod memory_io_interface;
mod console;
mod alu;
use std::io;

fn main() {

	// Computer Set Up of Drivers
	let mut con = boot_up();
	let mut mioi = memory_io_interface::MemIOInterface::new();
	// let alu = alu::ArithmeticLogicUnit::new();
	let _file: &str;
	// SET AMOUNT OF REGISTERS HERE
	let mem_amount = mioi.amount_of_memory();

	// Computer Run
	loop 
	{
		let mut asb_line = String::new();
		asb_line.clear();
		io::stdin().read_line(&mut asb_line).expect("Invalid Read of Standard In!");


		con.handle_in(&asb_line, mem_amount);
		if con.op == console::AsbType::BRK { break; }

		alu::ArithmeticLogicUnit::execute(&mut mioi, &con);

		// FOR DEBUG ONLY CURRENTLY:
		mioi.to_screen();
	}

	// mioi.shut_down();

}


fn boot_up() -> console::Console
{
	console::Console::new("no file")
}

