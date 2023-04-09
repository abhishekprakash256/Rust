// the data types in rust 

fn dummy()
{
	
	let tup:(i32,char,char) = (6,'C','Z');

	println!("{}",tup.0);
}


fn dummy2()
{
	let arr = [1,2,5,4,5,6];
	let arr2: [char;5] = ['a','b','c','d','e'];
	println!("{}",arr[0]);
	println!("{}",arr2[3])

}




fn main()
{
	dummy();
	dummy2();
}