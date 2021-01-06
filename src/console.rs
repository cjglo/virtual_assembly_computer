// use super::memory_io_interface;
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

			Some(_) => { println!("not implemented..."); self.op = AsbType::INV; skip = true; }

			None => { println!("Command Not Entered..."); self.op = AsbType::INV; skip = true; }
		}

		if skip { return; }


		// Assigning registers

		// dr
		let first_arg = command.next().unwrap().parse::<i32>().expect("DR not able to be read to integer");
		self.dr = first_arg as u8;

		// s1
		let second_arg = command.next().unwrap().parse::<i32>().expect("DR not able to be read to integer");
		self.s1 = second_arg as u8;

		// s2
		let third_arg = command.next().unwrap().parse::<i32>().expect("DR not able to be read to integer");
		self.s2 = third_arg as u8;

	}




	// pub fn save(&self)
	// {

	// }
	
}

/// ************** ASSEMBLY COMMAND HANDLING ************ ///
#[derive(PartialEq)] // Can implement trait for practice later
pub enum AsbType
{
	NOT,
	ADD,
	SUB,

	// System Commands
	INV,
	BRK,
}