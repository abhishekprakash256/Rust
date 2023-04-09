// make the rust program for the variables

fn dummy3()
// function for making the dummy varibales
{
	 const SEC_IN_MIN : i32 = 60;
	 println!("{}",SEC_IN_MIN);

}



fn dummy2()
//shawoow varibale techniques
{
	let c = 5;

	println!("the value of the c {}",c);

	{
		let c = 7;
		println!("the value of c {}",c);

	}

	println!("the value of added c  :  {}",c+1);

}



fn dummy()
{
	let mut a = 5;
	println!("a is {}",a);
	a = 6;
	println!("changed value is {}",a);
}

fn main()
{
	let x = 4;
	println!("x is : {}",x);
	dummy();
	dummy2();
	dummy3();

} 