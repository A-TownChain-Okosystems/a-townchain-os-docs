use crate::orderbook::{Order, OrderBook, Side};

#[derive(Debug, Default)]
pub struct OrderMatcher;

impl OrderMatcher {
    pub fn match_orders(&self, incoming: &Order, book: &OrderBook) -> Vec<Order> {
        match incoming.side {
            Side::Buy => book
                .asks()
                .iter()
                .filter(|o| o.price <= incoming.price)
                .cloned()
                .collect(),
            Side::Sell => book
                .bids()
                .iter()
                .filter(|o| o.price >= incoming.price)
                .cloned()
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_only_crossing_prices() {
        let mut b = OrderBook::default();
        b.add_order(Order {
            id: "s1".into(),
            price: 100,
            amount: 2,
            side: Side::Sell,
            timestamp: 1,
        });
        b.add_order(Order {
            id: "s2".into(),
            price: 101,
            amount: 2,
            side: Side::Sell,
            timestamp: 2,
        });
        let incoming = Order {
            id: "b".into(),
            price: 100,
            amount: 1,
            side: Side::Buy,
            timestamp: 3,
        };
        let matcher = OrderMatcher;
        assert_eq!(matcher.match_orders(&incoming, &b).len(), 1);
    }
}
