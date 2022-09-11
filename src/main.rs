mod table;

use table::*;

fn main () {
    let table = RoutingTable::new();
    println!("Row: {:?}", table.rows[0])
}
