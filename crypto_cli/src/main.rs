use exchanges::{Binance, Exchange};
mod exchanges;

fn main() {
    match Binance::get_instrument("BTCUSDT") {
        Ok(data) => {
            println!("{:?}", data);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
    match Binance::get_instruments(vec!["BTCUSDT", "ETHUSDT"]) {
        Ok(data) => {
            println!("{:?}", data);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
