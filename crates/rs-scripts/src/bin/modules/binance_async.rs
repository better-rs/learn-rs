use binance_async::{
    account::*,
    api::*,
    config::Config,
    errors::Error as BinanceLibError,
    general::*,
    market::*,
    rest_model::{OrderSide, OrderType, SymbolPrice, TimeInForce},
};
use log::{debug, error, info, warn};
#[allow(unused_imports)]
use pretty_env_logger;

pub async fn general() {
    let general: General = Binance::new(None, None);

    let ping = general.ping().await;
    match ping {
        Ok(answer) => info!("{:?}", answer),
        Err(err) => {
            match err {
                BinanceLibError::BinanceError { response } => match response.code {
                    -1000_i16 => error!("An unknown error occured while processing the request"),
                    _ => error!("Non-catched code {}: {}", response.code, response.msg),
                },
                _ => error!("Other errors: {:?}.", err),
            };
        },
    }

    let result = general.get_server_time().await;
    match result {
        Ok(answer) => info!("Server Time: {}", answer.server_time),
        Err(e) => error!("Error: {:?}", e),
    }

    let result = general.exchange_info().await;
    match result {
        Ok(answer) => {
            // info!("Exchange information: {:?}", answer);
            info!(
                "Exchange info: server_time={:?}, timezone={:?},",
                answer.server_time, answer.timezone,
            );
        },
        Err(e) => error!("Error: {:?}", e),
    }
}
