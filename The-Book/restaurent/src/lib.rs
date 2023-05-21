mod front_of_the_house{
    pub mod hosting {
        pub fn add_to_waitlist()
        { 
            println!("Adding to waitlist");
        }
        fn seat_at_table()
        {
            println!("Seating at table");
        }
    }



    mod serving
    {
        fn take_order()
        {
            println!("Taking order");
        }
        fn server_order()
        {
            println!("Serving order");
        }
        fn take_paymnet()
        {
            println!("Taking payment");
        }
    }

}

use crate:: front_of_the_house::hosting;

pub fn eat_at_restaurent()
{   
    // Absolute path 
    hosting::add_to_waitlist();

    //relative path
    hosting::add_to_waitlist();

}