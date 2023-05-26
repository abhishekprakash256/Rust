// to make the use of the crate 


pub mod front_of_house{
	pub mod hosting {
		pub fn add_to_waitlist(){
			println!("The fn add to waitlist is called");
		}
	}
}


use crate:: front_of_house::hosting;

fn main()
{
	hosting::add_to_waitlist();
	
}

