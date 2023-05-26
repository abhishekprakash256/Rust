// A file to practice the loops in the rust language

/*
this is block comment 

*/

fn main()
{
	num_printer_while();

	num_printer_loop();

	num_printer_for();
}


fn num_printer_for()
{
	let arr: [i32;5] = [10,20,30,40,50];

	for element in arr.iter()
	{
		println!("{}",element +7);
	}

	for number in 1..10   //for the range printing 
	{
		println!("{}",number);
	}

}



fn num_printer_loop()
{
	let mut count:i32 = 0;

	loop
	{
		println!("{}",count);
		if count == 30 
		{
			break;
		}
		count +=1;
	}

}

fn num_printer_while()
{
	let mut counter : i32 = 0;

	while counter!=30
	{
		println!("{}",counter);
		counter +=1;
	}
}