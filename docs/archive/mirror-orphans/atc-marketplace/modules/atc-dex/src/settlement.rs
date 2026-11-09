#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trade {
    pub id: u64,
    pub buyer: String,
    pub seller: String,
    pub price: u128,
    pub amount: u128,
    pub timestamp: u64,
}

#[derive(Debug, Default)]
pub struct Settlement {
    trades: Vec<Trade>,
    next_id: u64,
}

impl Settlement {
    pub fn settle(
        &mut self,
        buyer: String,
        seller: String,
        price: u128,
        amount: u128,
        timestamp: u64,
    ) -> Trade {
        let trade = Trade {
            id: self.next_id,
            buyer,
            seller,
            price,
            amount,
            timestamp,
        };
        self.next_id = self.next_id.checked_add(1).expect("trade id exhausted");
        self.trades.push(trade.clone());
        trade
    }
    pub fn trades(&self) -> &[Trade] {
        &self.trades
    }
}
