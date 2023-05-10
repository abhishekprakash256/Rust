// The program for the ownsershipo of the data


fn main()
{
	let s1: String  = String::from("Hello");
	let len: usize = calculate_length(&s1);
	println!("The length of the '{}' is '{}'",s1,len);
}

fn calculate_length(s : &String) -> usize
{
	let length : usize = s.len();   // length of the string 
	length
}
