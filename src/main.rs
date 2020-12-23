mod memory;
mod console;
use std::io;

fn main() {

	let mut con = boot_up();
	let _file: &str;

	loop 
	{
		let mut asb_line = String::new();
		asb_line.clear();
		io::stdin().read_line(&mut asb_line).expect("Invalid Read of Standard In!");
		// asb_line.pop();
		// println!("What is inside is lll{}lll", asb_line);

		// let mut asb_line: &str = &asb_line

		if con.handle_in(&asb_line )
		{
			break;
		}
	}

	shut_down(con);

}


fn boot_up() -> console::Console
{
	console::Console::new("no file")
}


fn shut_down(system: console::Console)
{
	system.save();

	println!("Shutting down system...");
}