#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeeCalculator {
    rate_bps: u64,
}

impl FeeCalculator {
    pub const DEFAULT_RATE_BPS: u64 = 30;

    pub fn new(rate_bps: u64) -> Self {
        Self { rate_bps }
    }
    pub fn calculate_fee(&self, amount: u128) -> u128 {
        amount.saturating_mul(self.rate_bps as u128) / 10_000
    }
    pub fn calculate_net(&self, amount: u128) -> u128 {
        amount.saturating_sub(self.calculate_fee(amount))
    }
    pub fn rate_bps(&self) -> u64 {
        self.rate_bps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fee_is_integer_and_deterministic() {
        let f = FeeCalculator::new(FeeCalculator::DEFAULT_RATE_BPS);
        assert_eq!(f.calculate_fee(100_000), 300);
        assert_eq!(f.calculate_net(100_000), 99_700);
    }
}
