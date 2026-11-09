#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub id: String,
    pub price: u128,
    pub amount: u128,
    pub side: Side,
    pub timestamp: u64,
}

#[derive(Debug, Default, Clone)]
pub struct OrderBook {
    bids: Vec<Order>,
    asks: Vec<Order>,
}

impl OrderBook {
    pub fn add_order(&mut self, order: Order) {
        match order.side {
            Side::Buy => self.bids.push(order),
            Side::Sell => self.asks.push(order),
        }
        self.bids.sort_by(|a, b| {
            b.price
                .cmp(&a.price)
                .then(a.timestamp.cmp(&b.timestamp))
                .then(a.id.cmp(&b.id))
        });
        self.asks.sort_by(|a, b| {
            a.price
                .cmp(&b.price)
                .then(a.timestamp.cmp(&b.timestamp))
                .then(a.id.cmp(&b.id))
        });
    }
    pub fn remove_order(&mut self, id: &str) {
        self.bids.retain(|o| o.id != id);
        self.asks.retain(|o| o.id != id);
    }
    pub fn best_bid(&self) -> Option<&Order> {
        self.bids.first()
    }
    pub fn best_ask(&self) -> Option<&Order> {
        self.asks.first()
    }
    pub fn bids(&self) -> &[Order] {
        &self.bids
    }
    pub fn asks(&self) -> &[Order] {
        &self.asks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordering_is_price_then_time_then_id() {
        let mut b = OrderBook::default();
        b.add_order(Order {
            id: "b2".into(),
            price: 10,
            amount: 1,
            side: Side::Buy,
            timestamp: 2,
        });
        b.add_order(Order {
            id: "b1".into(),
            price: 10,
            amount: 1,
            side: Side::Buy,
            timestamp: 1,
        });
        assert_eq!(b.best_bid().unwrap().id, "b1");
    }
}
