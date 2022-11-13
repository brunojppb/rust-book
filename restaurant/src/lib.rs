mod front_of_house;

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // full path
    hosting::add_to_waiting_list();
    // relative path
    hosting::add_to_waiting_list();
}
