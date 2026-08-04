pub fn number_to_bits_vec(number: u64, n: usize) -> Vec<u64> {
    (0..n).rev().map(|i| ((number >> i) & 1) as u64).collect()
}
