// The program for the ownsershipo of the data


fn main()
{
	let s1: String  = String::from("Hello");
	let len: usize = calculate_length(&s1);
	println!("The length of the '{}' is '{}'",s1,len);

	let mut s2: String = String::from("This is text");
	change(&mut s2);

	println!("The new string is '{}'",s2);
}





fn calculate_length(s : &String) -> usize
{
	let length : usize = s.len();   // length of the string 
	length
}


fn change(some_str: &mut String)
{
	some_str.push_str(" and this is added text");
}
