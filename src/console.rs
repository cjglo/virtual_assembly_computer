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

	pub fn handle_in(&mut self, raw_line:  &str, mem_space: i32)
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


		// Assigning registers

		// dr
		let first_arg = command.next().parse::<i32>().expect("DR not able to be read to integer");
		match first_arg
		{
			0..=mem_space => self.dr = first_arg,
			_ => panic!("Value invalid or out of range!"),
		}

		// s1
		let second_arg = command.next().parse::<i32>().expect("DR not able to be read to integer");
		match second_arg
		{
			0..=mem_space => self.dr = second_arg,
			_ => panic!("Value invalid or out of range!"),
		}

		// s2
		let third_arg = command.next().parse::<i32>().expect("DR not able to be read to integer");
		match third_arg
		{
			0..=mem_space => self.dr = third_arg,
			_ => panic!("Value invalid or out of range!"),
		}

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