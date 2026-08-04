pub fn number_to_bits_vec(number: u64, n: usize) -> Vec<u8> {
    (0..n).rev().map(|i| ((number >> i) & 1) as u8).collect()
}
