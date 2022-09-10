use rand::RngCore;

/**
 * Returns a 32 bytes u8 array
 */
pub fn random_id () -> [u8; 32]{
    let mut buf = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut buf);
    buf
}

