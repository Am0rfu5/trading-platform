use std::env;
use std::collections::{BTreeMap, VecDeque};
use std::error::Error;
use csv::{Reader, WriterBuilder};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
struct Order {
    id: String,
    #[serde(rename = "type")] // "type" is a reserved keyword in Rust
    order_type: String,
    price: u32,
    size: u32,
}

#[derive(Serialize)]
struct CompletedOrder {
    sid: String,
    bid: String,
    size: u32,
    price: u32,
}

struct OrderBook {
    buy_orders: BTreeMap::<u32, VecDeque<Order>>,
    sell_orders: BTreeMap::<u32, VecDeque<Order>>
}

impl OrderBook {
    fn new() -> OrderBook {
        OrderBook {
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        let orders = match order.order_type.as_str() {
            "buy" => self.buy_orders.entry(order.price).or_insert_with(VecDeque::new),
            "sell" => self.sell_orders.entry(order.price).or_insert_with(VecDeque::new),
            _ => return,
        };
        orders.push_back(order);
    } 
    
    fn match_orders(&mut self, matched_orders: &mut Vec<CompletedOrder>) {
        while let (Some((&buy_price, buy_orders)), Some((&sell_price, sell_orders))) =
            (self.buy_orders.range_mut(..).next_back(), self.sell_orders.range_mut(..).next()) 
        {
            if buy_price >= sell_price {
                let buy_order = buy_orders.front_mut().unwrap();
                let sell_order = sell_orders.front_mut().unwrap();
    
                let trade_size = std::cmp::min(buy_order.size, sell_order.size);
    
                matched_orders.push(CompletedOrder {
                    sid: sell_order.id.clone(),
                    bid: buy_order.id.clone(),
                    size: trade_size,
                    price: sell_price,
                });
    
                buy_order.size -= trade_size;
                sell_order.size -= trade_size;
    
                if buy_order.size == 0 {
                    buy_orders.pop_front();
                }
                if sell_order.size == 0 {
                    sell_orders.pop_front();
                }
    
                if buy_orders.is_empty() {
                    self.buy_orders.remove(&buy_price);
                }
                if sell_orders.is_empty() {
                    self.sell_orders.remove(&sell_price);
                }
            } else {
                break;
            }
        }
    }
    
    // This is a helper used for testing
    #[allow(dead_code)]
    fn get_front_order(&self, is_buy: bool, price: u32) -> Option<&Order> {
        let orders = if is_buy {
            self.buy_orders.get(&price)
        } else {
            self.sell_orders.get(&price)
        };
        orders?.front()
    }
    
}

fn export_matched_orders_to_csv(matched_orders: &[CompletedOrder], output_file: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_path(output_file)?;

    wtr.write_record(&["sid", "bid", "size", "price"])?;

    for order in matched_orders {
        wtr.serialize(order)?;
    }

    wtr.flush()?;
    Ok(())
}
    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_fill() {
        let mut order_book = OrderBook::new();
        let mut matched_orders: Vec<CompletedOrder> = Vec::new();

        // Add orders
        order_book.add_order(Order { id: "Bob".to_string(), order_type: "sell".to_string(), price: 20, size: 100 });
        order_book.add_order(Order { id: "Sue".to_string(), order_type: "buy".to_string(), price: 20, size: 50 });

        // Match orders
        order_book.match_orders(&mut matched_orders);

        // Check if the match is as expected
        assert_eq!(matched_orders.len(), 1);
        assert_eq!(matched_orders[0].sid, "Bob");
        assert_eq!(matched_orders[0].bid, "Sue");
        assert_eq!(matched_orders[0].size, 50);
        assert_eq!(matched_orders[0].price, 20);

        // Check remaining orders in the book
        assert_eq!(order_book.sell_orders.len(), 1);
        
        let remaining_order = order_book.get_front_order(false, 20).unwrap(); // false for sell order
        assert_eq!(remaining_order.id, "Bob");
        assert_eq!(remaining_order.size, 50);
    
    }
    
    #[test]
    fn test_multiple_partial_fills() {
        let mut order_book = OrderBook::new();
        let mut matched_orders: Vec<CompletedOrder> = Vec::new();

        // Add orders
        order_book.add_order(Order { id: "Bob".to_string(), order_type: "buy".to_string(), price: 20, size: 100 });
        order_book.add_order(Order { id: "Sue".to_string(), order_type: "sell".to_string(), price: 20, size: 50 });
        order_book.add_order(Order { id: "Tim".to_string(), order_type: "sell".to_string(), price: 20, size: 50 });

        // Match orders
        order_book.match_orders(&mut matched_orders);

        // Check matches
        assert_eq!(matched_orders.len(), 2);
        assert_eq!(matched_orders[0].bid, "Bob");
        assert_eq!(matched_orders[0].sid, "Sue");
        assert_eq!(matched_orders[0].size, 50);
        assert_eq!(matched_orders[1].bid, "Bob");
        assert_eq!(matched_orders[1].sid, "Tim");
        assert_eq!(matched_orders[1].size, 50);

        // Check remaining orders in the book
        assert!(order_book.buy_orders.is_empty());
        assert!(order_book.sell_orders.is_empty());
    }

    #[test]
    fn test_different_price_points_no_fill() {
        let mut order_book = OrderBook::new();
        let mut matched_orders: Vec<CompletedOrder> = Vec::new();
    
        // Add orders
        order_book.add_order(Order { id: "Bob".to_string(), order_type: "buy".to_string(), price: 20, size: 100 });
        order_book.add_order(Order { id: "Sue".to_string(), order_type: "sell".to_string(), price: 21, size: 100 });
    
        // Match orders
        order_book.match_orders(&mut matched_orders);
    
        // Check no match occurred
        assert!(matched_orders.is_empty());
    
        // Check remaining orders in the book
        let remaining_buy_order = order_book.get_front_order(true, 20).unwrap(); // true for buy order
        let remaining_sell_order = order_book.get_front_order(false, 21).unwrap(); // false for sell order
    
        assert_eq!(remaining_buy_order.id, "Bob");
        assert_eq!(remaining_buy_order.size, 100);
        assert_eq!(remaining_sell_order.id, "Sue");
        assert_eq!(remaining_sell_order.size, 100);
    }
    
    #[test]
    fn test_different_price_points_fill() {
        let mut order_book = OrderBook::new();
        let mut matched_orders: Vec<CompletedOrder> = Vec::new();

        // Add orders
        order_book.add_order(Order { id: "Bob".to_string(), order_type: "buy".to_string(), price: 20, size: 100 });
        order_book.add_order(Order { id: "Sue".to_string(), order_type: "sell".to_string(), price: 21, size: 100 });
        order_book.add_order(Order { id: "Tim".to_string(), order_type: "buy".to_string(), price: 21, size: 100 });

        // Match orders
        order_book.match_orders(&mut matched_orders);

        // Check if the match is as expected
        assert_eq!(matched_orders.len(), 1);
        assert_eq!(matched_orders[0].sid, "Sue");
        assert_eq!(matched_orders[0].bid, "Tim");
        assert_eq!(matched_orders[0].size, 100);
        assert_eq!(matched_orders[0].price, 21);

        // Check remaining orders in the book
        assert_eq!(order_book.buy_orders.len(), 1);
        let remaining_buy_order = order_book.get_front_order(true, 20).unwrap(); // true for buy order
        assert_eq!(remaining_buy_order.id, "Bob");
        assert_eq!(remaining_buy_order.size, 100);
        assert!(order_book.sell_orders.is_empty());
    }
    
    
    #[test]
    fn test_priority_queue_at_price_level() {
        let mut order_book = OrderBook::new();
        let mut matched_orders: Vec<CompletedOrder> = Vec::new();
    
        // Add orders
        order_book.add_order(Order { id: "Bob".to_string(), order_type: "buy".to_string(), price: 20, size: 100 });
        order_book.add_order(Order { id: "Tim".to_string(), order_type: "buy".to_string(), price: 20, size: 100 });
        order_book.add_order(Order { id: "Sue".to_string(), order_type: "sell".to_string(), price: 20, size: 100 });
    
        // Match orders
        order_book.match_orders(&mut matched_orders);
    
        // Check if the match is as expected
        assert_eq!(matched_orders.len(), 1);
        assert_eq!(matched_orders[0].sid, "Sue");
        assert_eq!(matched_orders[0].bid, "Bob");
        assert_eq!(matched_orders[0].size, 100);
        assert_eq!(matched_orders[0].price, 20);
    
        // Check remaining orders in the book using the helper function
        let remaining_buy_order = order_book.get_front_order(true, 20).unwrap(); // true for buy order
        assert_eq!(remaining_buy_order.id, "Tim");
        assert_eq!(remaining_buy_order.size, 100);
        assert!(order_book.sell_orders.is_empty());
    }
    

    #[test]
    fn test_priority_queue_with_large_order() {
        let mut order_book = OrderBook::new();
        let mut matched_orders: Vec<CompletedOrder> = Vec::new();
    
        // Add orders
        order_book.add_order(Order { id: "Bob".to_string(), order_type: "buy".to_string(), price: 20, size: 100 });
        order_book.add_order(Order { id: "Tim".to_string(), order_type: "buy".to_string(), price: 20, size: 100 });
        order_book.add_order(Order { id: "Sue".to_string(), order_type: "sell".to_string(), price: 20, size: 300 });
    
        // Match orders
        order_book.match_orders(&mut matched_orders);
    
        // Check if the matches are as expected
        assert_eq!(matched_orders.len(), 2);
        assert_eq!(matched_orders[0].sid, "Sue");
        assert_eq!(matched_orders[0].bid, "Bob");
        assert_eq!(matched_orders[0].size, 100);
        assert_eq!(matched_orders[1].sid, "Sue");
        assert_eq!(matched_orders[1].bid, "Tim");
        assert_eq!(matched_orders[1].size, 100);
    
        // Check remaining orders in the book
        assert!(order_book.buy_orders.is_empty());
        let remaining_sell_order = order_book.get_front_order(false, 20).unwrap(); // false for sell order
        assert_eq!(remaining_sell_order.id, "Sue");
        assert_eq!(remaining_sell_order.size, 100);
    }    
    
}
