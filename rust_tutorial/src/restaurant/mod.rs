mod pizza_order{

    pub struct Pizza{
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }

    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: String::from("regular dough"),
                cheese: String::from("mozzarella"),
                topping: String::from(topping),
            }
        }
    }

    pub mod help_customer{
        fn seat_at_table(){
            println!("Customer seated at table");
        }
        // to access take_order both the parent "help_customer" and the child "take_order" need to be public
        pub fn take_order(){
            seat_at_table();
            let cust_pizza: super::Pizza =
                super::Pizza::lunch("veggies");
            serve_customer(cust_pizza);
        }
        fn serve_customer(cust_pizza: super::Pizza) {
            println!("The customer is served a regular pizza with {}", cust_pizza.topping);
        }
    }
}

// this is a public function that allows other files to access the stuff we want
// so in main.rs we just use order_food();
pub fn order_food(){
    // restaurant is the directory, pizza is the module, help customer is 
    // the module in pizza_order and take order is the function we want
    crate::restaurant::pizza_order::help_customer::take_order();
}