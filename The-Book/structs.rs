// the structs in rust


struct User 
{
	username: String,
	email: String,
	id : u32,
	active: bool,
}


fn main()
{ 

	let User_1 = User
	{
		username: String::from("Abhi"),
		email: String::from("abhi@gmail.com"),
		id: 5,
		active: true
	};

	let mut User_2 = User
	{
		username: String::from("Abhi"),
		email: String::from("anny@gmail.com"),
		id:2,
		active: true
	};

	let username_2 = String::from("Anny");
	User_2.username = username_2;

	let User_3 = User
	{
		username: String::from("AbhiAnny"),
		email: String::from("abhianny25@gmail.com"),
		..User_2

	};


	
	println!("the username is {}",User_1.username);

	println!("the username is {}", User_2.username);

	println!("the username is {}", User_3.username);
}