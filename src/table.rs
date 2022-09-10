pub struct RoutingTable {
    pub id: [u8; 32],
    pub k: i32,
}

impl RoutingTable {
    pub fn new(id: [u8; 32]) -> Self {
        Self::with_k(id, 20)
    }

    pub fn with_k(id: [u8; 32], k: i32) -> Self {
        Self { id, k }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::helpers;

    #[test]
    fn new() {
        let table: RoutingTable = RoutingTable::new(helpers::random_id());

        assert_eq!(table.k, 20, "Defaults k to 20")
    }
}
