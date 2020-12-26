mod memory_io_interface;
mod console;
mod alu;
use std::io;

fn main() {

	let mut con = boot_up();
	let mioi = memory_io_interface::MemIOInterface::new();
	let alu = alu::ArithmeticLogicUnit;
	let _file: &str;
	// SET AMOUNT OF REGISTERS HERE
	let mem_amount = mioi.amount_of_memory();

	loop 
	{
		let mut asb_line = String::new();
		asb_line.clear();
		io::stdin().read_line(&mut asb_line).expect("Invalid Read of Standard In!");


		con.handle_in(&asb_line, mem_amount);
		if con.op == console::AsbType::BRK { break; }

		alu.execute(&mioi, &con);

		// FOR DEBUG ONLY CURRENTLY:
		mioi.to_screen();
	}

	// mioi.shut_down();

}


fn boot_up() -> console::Console
{
	console::Console::new("no file")
}

