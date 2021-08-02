mod front_of_house; //look for the module in the file with the same name

mod back_of_house {
    pub struct Breakfast { // the pub word makes the struct accessible
        pub toast: String, // only pub fields are accessible to parent modules
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    } 
}

pub use crate::front_of_house::hosting;
pub use self::back_of_house::Breakfast as Bkfst;    // links Bkfst with self::back_of_house::Breakfast 
                                                    // and makes the link available to parent modules

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    let mut meal = Bkfst::summer("Rye");

    meal.toast = String::from("wheat"); //we can access toast but not seasonal_fruit
    println!("toast: {}",meal.toast);
}
