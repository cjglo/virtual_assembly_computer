use super::memory;

// computer memory set to 10 currently

pub struct Console
{
	mem: [memory::Register; 10]
}

impl Console 
{

	pub fn new(_file: &str) -> Console
	{
		Console
		{
			// potentially load here
			// file.open()

			mem: [memory::Register::new(); 10],
		}
	}

	pub fn handle_in(&mut self, raw_line:  &str) -> bool
	{
		let mut command = raw_line.split_whitespace();

		//// -- ASSEMBLER -- ////
		match command.next()
		{
			Some(".") => println!("YEP"),
			Some(y) => println!("didn't work"),
			None => println!("nothing put in"),
		}



		println!("should not reach here!");

		false // remove later once this function is complete -----------------------------------
	}


	pub fn save(&self)
	{

	}
	
}