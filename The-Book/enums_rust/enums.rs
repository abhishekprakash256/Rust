// to make the enums 

enum IpAddrKind
//to make the emun to define value
{
	V4,
	V6,

}

struct IpAddr
//make a struct for the ipaddr
{
	kind: IpAddrKind,
	address: String,
}


fn main()
{
	let four = IpAddrKind::V4;
	let six = IpAddrKind::V6;


	let localhost = IpAddr
	// the localhost struct takes the value of the ip address
	{
		kind:IpAddrKind::V4,
		address: from::String("192.0.8.127"),
	}
}

