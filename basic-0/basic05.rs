// the program to make the program for type casting
use std::io;

fn sum()
{
	let a = 5_u64;  //use the symbol
	let b = 7 as u8; //

	// type casting as the u64
	println!("{}",a+b as u64);


}


fn take_inp()
{
	let mut input = String::new();

	io::stdin().read_line(&mut input).expect("failed to read line");

	let int_input: i64 = input.trim().parse().unwrap();

	println!("{}",int_input + 2);

}

fn main()
{
	//let x:i64 = 5;
	//let y:i64 = 8;

	//println!("{}",x+y);

	//sum();
	println!("Give the input");
	take_inp();
}