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

	pub fn handle_in(&mut self, raw_line:  &str) -> u8 // returns 0 for success and positives for error and type
	{   
		/*
			Error Codes:
			0 => Success
			1 => Not Implemented Operation
			2 => Null Command
			3 => dr, s1, or s2 not found
			4 => dr, s1, or s2 were nonnumeric address
		*/

		let mut command = raw_line.split_whitespace();
		let mut skip = false;

		//// -- ASSEMBLER -- ////
		match command.next()
		{
			// exit command
			Some("BREAK") => { self.op = AsbType::BRK; skip = true; }

			// Main Assembly Code
			Some("NOT") => { self.op = AsbType::NOT; }

			Some(_) => { self.op = AsbType::INV; skip = true; return 1; }

			None => { self.op = AsbType::INV; return 2; }
		}


		// Assigning registers

		// dr
		let first_arg = command.next(); // not sure if the multiple "let"s are good practice
		let first_arg = match first_arg 
		{
			Some(arg) => arg,
			None => { self.op = AsbType::INV; return 3; }
		};
		let first_arg = first_arg.parse::<i32>();
		let first_arg = match first_arg
		{
			Ok(arg) => arg,
			Err(_e) => { self.op = AsbType::INV; return 4; }
		};
		self.dr = first_arg as u8;

		// s1
		let second_arg = command.next();
		let second_arg = match second_arg
		{
			Some(arg) => arg,
			None => { self.op = AsbType::INV; return 3; }
		};
		let second_arg = second_arg.parse::<i32>();
		let second_arg = match second_arg
		{
			Ok(arg) => arg,
			Err(_e) => { self.op = AsbType::INV; return 4; }
		};
		self.s1 = second_arg as u8;

		// s2
		let third_arg = command.next();
		let third_arg = match third_arg
		{
			Some(arg) => arg,
			None => { self.op = AsbType::INV; return 3; }
		};
		let third_arg = third_arg.parse::<i32>();
		let third_arg = match third_arg
		{
			Ok(arg) => arg,
			Err(_e) => { self.op = AsbType::INV; return 4; }
		};
		self.s2 = third_arg as u8;

		0 // success returned
	}

	
}

/// ************** ASSEMBLY IMPL COMMANDS ************ ///
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