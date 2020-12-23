mod memory_io_interface;
mod console;
use std::io;

fn main() {

	let mut con = boot_up();
	let mioi = memory_io_interface::new();
	let _file: &str;

	loop 
	{
		let mut asb_line = String::new();
		asb_line.clear();
		io::stdin().read_line(&mut asb_line).expect("Invalid Read of Standard In!");


		con.handle_in(&asb_line );
		if con.op == console::AsbType::BRK { break; }

		mioi.execute(&con);

	}

	mioi.shut_down();

}


fn boot_up() -> console::Console
{
	console::Console::new("no file")
}
