// The game to make the guess for the number 

// imports 
use std::io;
use rand::Rng;

fn main()
{
	let secret_number = rand::thread_rng().gen_range(1,101);

	println!("The secret number is : {}", secret_number);

	println!("Enter the number to guess!  ");

	println!("Please input your guess ...  ");

	let mut guess: String = String:: new(); 

	io::stdin().read_line(&mut guess).expect("failed to read line");

	println!("You guess the number: {}", guess);

}