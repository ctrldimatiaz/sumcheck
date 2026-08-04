use crate::protocol::sumcheck::SumCheck;

pub struct ToyExample<const P: u64> {
    sumcheck: SumCheck<P>,
}
