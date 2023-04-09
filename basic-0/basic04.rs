// the io in the rust

//import for the standard io
use std::io;


fn main()
{
	println!("the main function");
	let mut input = String::new();

	io::stdin().read_line(&mut input).expect("failed to read line");
	println!("{}",input);

}
