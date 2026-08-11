// Usefull to get all the combinations of {0,1}^l according to the variables count
pub fn number_to_bits_vec(number: u64, n: usize) -> Vec<u64> {
    (0..n).rev().map(|i| (number >> i) & 1).collect()
}
