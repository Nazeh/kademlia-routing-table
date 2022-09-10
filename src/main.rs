mod helpers;
mod table;

use crate::table::RoutingTable;
use crate::helpers::random_id;

fn main() {
    let table = RoutingTable::new(random_id());
    println!("{:?}", table.id)
}
