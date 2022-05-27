use binance::api::*;
use binance::market::*;
use binance::model::KlineSummary;

fn main() {
    market_data();
}

fn market_data() {
    let market: Market = Binance::new(None, None);

    let coin_pair = "BNBBUSD";

    // Order book at default depth
    match market.get_depth(coin_pair) {
        Ok(answer) => println!("BNBBUSD Depth: {:?}\n\n", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Order book at depth 500
    match market.get_custom_depth(coin_pair, 500) {
        Ok(answer) => println!("BNBBUSD Top500 Depth: {:?}\n\n", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ALL symbols
    match market.get_all_prices() {
        Ok(answer) => println!("{:?}\n\n", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    // Latest price for ONE symbol
    match market.get_price(coin_pair) {
        Ok(answer) => println!("BNBBUSD Price: {:?}\n\n", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    // Current average price for ONE symbol
    match market.get_average_price(coin_pair) {
        Ok(answer) => println!("{:?}\n\n", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    // Best price/qty on the order book for ALL symbols
    match market.get_all_book_tickers() {
        Ok(answer) => println!("{:?}\n\n", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match market.get_book_ticker(coin_pair) {
        Ok(answer) => println!(
            "Bid Price: {}, Ask Price: {}\n\n",
            answer.bid_price, answer.ask_price
        ),
        Err(e) => println!("Error: {:?}", e),
    }

    // 24hr ticker price change statistics
    match market.get_24h_price_stats(coin_pair) {
        Ok(answer) => println!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}\n\n",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => println!("Error: {:?}", e),
    }

    // last 10 5min klines (candlesticks) for a symbol:
    match market.get_klines(coin_pair, "5m", 10, None, None) {
        Ok(klines) => {
            match klines {
                binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                    let kline: KlineSummary = klines[0].clone(); // You need to iterate over the klines
                    println!(
                        "Open: {}, High: {}, Low: {}\n\n",
                        kline.open, kline.high, kline.low
                    )
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
