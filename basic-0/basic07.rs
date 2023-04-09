//the implementation of the funcitom

fn add_num(num1: i64,num2:i64)->i64
{
	num1 + num2
}

fn add_num2(num1:i64 , num2:i64) ->i64
{
	return num1 + num2 ;
}



fn main()
{
	let x = 5;
	let y = 6;
	let z = add_num(x,y);
	println!("{}",z);
	let w = add_num2(7,8);
	println!("{}",w);
}


