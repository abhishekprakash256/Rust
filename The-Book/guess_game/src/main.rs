// The game to make the guess for the number 

// imports 
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main()
{
    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is : {}", secret_number);

    loop {

        println!("Enter the number to guess!  ");

        println!("Please input your guess ...  ");

        let mut guess: String = String:: new(); 

        io::stdin().read_line(&mut guess).expect("failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number");

        println!("You guess the number: {}", guess);

        match guess.cmp(&secret_number)
        {
            Ordering:: Less =>println!("To small!!"),
            Ordering:: Greater => println!("Too big!!"),
            Ordering:: Equal => println!("You win!!"),
        }

    }
}