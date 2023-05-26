// a rust example for the ownership of the system 

pub mod front_of_house{
	pub mod hosting {
		pub fn add_to_waitlist(){
			println!("The fn add to waitlist is called");
		}
	}
}




fn main()
{
	front_of_house::hosting::add_to_waitlist();
	
}

