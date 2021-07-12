mod front_of_house;

// tedious way; we're having to specify full path
// pub fn eat_at_restaurant() {
//     front_of_house::hosting::add_to_waitlist();
//     front_of_house::hosting::add_to_waitlist();
//     front_of_house::hosting::add_to_waitlist();
// }

// Better way, let's bring hosting path into scope
// relatively: use self::front_of_house::hosting;
pub use crate::front_of_house::hosting;

// Now we only need to specify function in relation to hosting
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
