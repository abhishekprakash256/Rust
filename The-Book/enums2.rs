//make the address usingh the enum varibale

enum IpAddrKind
{
	V4(String),
	V6(String),
}


enum IpAddrKind2
{
	V4(u8,u8,u8,u8),
	V6(u8,u8,u8,u8),
}

struct IpAddr
{
	kind: IpAddrKind,
	address: String,
}

enum Message
{
	Quit,
	Move {x:i32,y:i32},
	Write(String),
	ChangeColor(i32,i32,i32),
}


fn main()
{
	let localhost = IpAddrKind::V4(String::from("127.0.0.1"));

	let localhost2 = IpAddrKind2::V4(127,0,0,1);

}