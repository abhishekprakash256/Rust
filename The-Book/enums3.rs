// make the enum for the optional 


fn main()
{
	let x: i8 = 5;
	let y: Option<i8> = None;

	let sum: i8 = x+ y.unwrap_or(0); 
}