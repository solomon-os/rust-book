mod front_of_house;

use crate::front_of_house::hosting;

fn main() {
    eat_at_restaurant();
}

fn eat_at_restaurant() {
    hosting::add_to_waiting_list();
}
