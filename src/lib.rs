mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
    }

    fn cook_order() {}
}
