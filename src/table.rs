use rand::RngCore;

pub fn id() -> [u8; 32] {
    let mut buf = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut buf);
    buf
}

// k-bucket size
pub const K_VALUE: usize = 20;

// Maximum number of k-buckets.
pub const ROWS_COUNT: usize = 256;

pub type ID = [u8; 32];

#[derive(Debug)]
pub struct Node {
    id: ID,
}

#[derive(Debug, Copy, Clone)]
pub enum MaybeNode {
    Node,
    None,
}

#[derive(Debug, Copy, Clone)]
pub struct Row {
    bucket: [MaybeNode; K_VALUE],
}

#[derive(Debug)]
pub struct RoutingTable {
    pub id: ID,
    pub rows: [Row; ROWS_COUNT],
}

impl Node {
    pub fn new(id: ID) -> Self {
        Self { id }
    }
}

impl Row {
    pub fn new() -> Self {
        Self {
            bucket: [0; K_VALUE].map(|_| MaybeNode::None),
        }
    }
}

impl RoutingTable {
    pub fn new() -> Self {
        Self {
            id: id(),
            rows: [0; ROWS_COUNT].map(|_| Row::new()),
        }
    }

    pub fn row(self, index: usize) -> Row {
        self.rows[index]
    }
}
