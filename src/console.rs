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

	pub fn handle_in(&mut self, raw_line: String) -> bool
	{
		let mut asb_line = raw_line.split(" ").peekable();

		let command = asb_line.next().expect("Invalid Command Read!");



		//match command
		//{
		//	Some(x) =>
		//			{
		//				println!("THe line was {}", x);
		//				// let () = x;
		//				if x == str::from(".")
		//				{
		//					println!("made it inside");
		//					return true;
		//				}
		//				else
		//				{
		//					return false;
		//				}
		//			},
		//	None =>
		//				{    println!(
		//						"No Command Read! \n
		//						Command must follow 16 bit \n
		//						Assembly provided in manual.txt"
		//					);
		//					return false;
		//				},
		//}


		println!("should not reach here!");
	}


	pub fn save(&self)
	{

	}
	
}