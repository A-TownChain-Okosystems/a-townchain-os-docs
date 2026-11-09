#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiquidityPool {
    reserve_a: u128,
    reserve_b: u128,
}

impl LiquidityPool {
    pub fn new(reserve_a: u128, reserve_b: u128) -> Self {
        Self {
            reserve_a,
            reserve_b,
        }
    }
    pub fn reserves(&self) -> (u128, u128) {
        (self.reserve_a, self.reserve_b)
    }
    pub fn swap_a_for_b(&mut self, amount_a: u128) -> Option<u128> {
        if amount_a == 0 || self.reserve_a == 0 || self.reserve_b == 0 {
            return None;
        }
        let k = self.reserve_a.checked_mul(self.reserve_b)?;
        let new_a = self.reserve_a.checked_add(amount_a)?;
        let new_b = k / new_a;
        let out = self.reserve_b.checked_sub(new_b)?;
        if out == 0 {
            return None;
        }
        self.reserve_a = new_a;
        self.reserve_b = new_b;
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn constant_product_swap_is_integer_only() {
        let mut p = LiquidityPool::new(1_000_000, 1_000_000);
        let out = p.swap_a_for_b(100_000).unwrap();
        assert_eq!(out, 90_910);
    }
}
