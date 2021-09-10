use crate::util::get_pub;
mod ticket_buying;
mod util;
use std::{env};
fn main() {
    let arg: Vec<String> = env::args().collect();
    let pool_id = get_pub(&arg[1]);

    let tickets = ticket_buying::findtickets(&pool_id);

    for data in tickets {
        println!("{:?},{:?}", data.0, data.1);
    }
}
