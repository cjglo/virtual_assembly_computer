#[derive(Debug, Copy, Clone)]
pub struct Register
{
	pub val: u8,
}


impl Register
{	

	pub fn new() -> Register
	{
		Register
		{
			val: 0,
		}
	}

	
}	