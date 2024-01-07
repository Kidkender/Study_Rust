mod list;
mod hashs; 

use crate::list::store_list::store;
use crate::list::multi_type::examp;

use crate::hashs::store;

fn main() {
    store();
    examp(10, "Ten", 10.0);
    store::get_value();
    store::map();
    store::overwrite();
    store::update();
    store::update_old_value();
}
