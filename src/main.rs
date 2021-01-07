mod memory_io_interface;
mod console;
mod alu;
mod display;
use std::io;

fn main() {

	// Computer Set Up of Drivers
	let mut con = boot_up();
	let mut mioi = memory_io_interface::MemIOInterface::new();
	// let alu = alu::ArithmeticLogicUnit::new();
	let _file: &str;
	// Note: Set amount of registers inside mem_io

	// Computer Run
	display::clear_screen();
	display::window("NONE", &mioi, 5); // default values for display (specifically, 5 displays no result of last command)
	loop 
	{
		let mut asb_line = String::new();
		asb_line.clear();
		io::stdin().read_line(&mut asb_line).expect("Invalid Read of Standard In!"); // Reads command

		// con: Console handles the command and seperates it into dr s1 and s2 (as its own fields)
		let e_result = con.handle_in(&asb_line);
		if con.op == console::AsbType::BRK { break; } 
		if con.op == console::AsbType::INV { asb_line.pop(); display::window(&asb_line, &mioi, e_result); continue;  }
		
		alu::ArithmeticLogicUnit::execute(&mut mioi, &con); // handles register bitwise operations

		// display
		asb_line.pop();
		display::window(&asb_line, &mioi, e_result); // displays registers and command
	}


}


fn boot_up() -> console::Console
{
	console::Console::new("no file")
}