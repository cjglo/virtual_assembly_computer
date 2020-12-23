use super::memory;
use crate::console::AsbType::INV;

// computer memory set to 10 currently

pub struct Console
{
	// mem: [memory::Register; 10],
	pub op: AsbType,
	pub dr: u8,
	pub s1: u8,
	pub s2: u8,
}

impl Console 
{

	pub fn new(_file: &str) -> Console
	{
		Console
		{
			// potentially load here
			// file.open()

			op: INV,
			dr: 0,
			s1: 0,
			s2: 0,
		}
	}

	pub fn handle_in(&mut self, raw_line:  &str)
	{
		let mut command = raw_line.split_whitespace();
		let mut skip = false;

		//// -- ASSEMBLER -- ////
		match command.next()
		{
			// exit command
			Some("BREAK") => { println!("Break command entered..."); self.op = AsbType::BRK; skip = true; }

			// Main Assembly Code
			Some("NOT") => { println!("Not command"); self.op = AsbType::NOT; }

			Some(y) => println!("not implemented..."),

			None => println!("nothing put in"),
		}

		if skip { return; }


	}




	pub fn save(&self)
	{

	}
	
}

/// ************** ASSEMBLY COMMAND HANDLING ************ ///

pub enum AsbType
{
	NOT,
	ADD,
	SUB,

	// System Commands
	INV,
	BRK,
}