// use std::env;
// use std::error::Error;
// use csv::Reader;
// use matching_engine::order_book::OrderBook;

// fn main() -> Result<(), Box<dyn Error>> {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 3 {
//         eprintln!("Usage: matching_engine <input.csv> <output.csv>");
//         return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid number of arguments")));
//     }

//     let input_file = &args[1];
//     let output_file = &args[2];

//     let mut order_book = OrderBook::new();
//     let mut matched_orders: Vec<CompletedOrder> = Vec::new();

//     // Read orders from CSV
//     let mut rdr = Reader::from_path(input_file)?;
//     for result in rdr.deserialize() {
//         let order: Order = result?;
//         order_book.add_order(order);
//     }

//     order_book.match_orders(&mut matched_orders);

//     export_matched_orders_to_csv(&matched_orders, output_file)?;

//     Ok(())
// }

fn main() {
    
}
