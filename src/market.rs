use substreams::scalar::{BigDecimal, BigInt};

pub enum MarketToken {
    WBTC,
    ARB,
    WETH,
    WSOL,
    LINK,
    XRP,
    UNI,
    DOGE,
    LTC,
}

pub struct Market {
    pub market: MarketToken,
    pub market_name: String,
    pub market_address: String,
}

impl Market {
    pub fn get_execution_price(&self, execution_price: &BigInt) -> f64 {
        let execution_price = execution_price.clone();
        let execution_price: BigDecimal = match self.market {
            MarketToken::WBTC => execution_price / 1e22,
            MarketToken::ARB => execution_price / 1e12,
            MarketToken::WETH => execution_price / 1e12,
            MarketToken::WSOL => execution_price / 1e21,
            MarketToken::LINK => execution_price / 1e12,
            MarketToken::XRP => execution_price / 1e24,
            MarketToken::UNI => execution_price / 1e12,
            MarketToken::DOGE => execution_price / 1e22,
            MarketToken::LTC => execution_price / 1e22,
        };

        let execution_price = execution_price.to_string().parse::<f64>().unwrap();
        match self.market {
            MarketToken::DOGE => (execution_price * 10000.0).round() / 10000.0,
            MarketToken::XRP => (execution_price * 1000.0).round() / 1000.0,
            MarketToken::ARB => (execution_price * 1000.0).round() / 1000.0,
            _ => (execution_price * 100.0).round() / 100.0,
        }
    }

    pub fn get_tokens(&self, tokens: &BigInt) -> f64 {
        let tokens = tokens.clone();
        let tokens: substreams::scalar::BigDecimal = match self.market {
            MarketToken::WBTC => tokens / 1e8,
            MarketToken::ARB => tokens / 1e18,
            MarketToken::WETH => tokens / 1e18,
            MarketToken::WSOL => tokens / 1e9,
            MarketToken::LINK => tokens / 1e18,
            MarketToken::XRP => tokens / 1e6,
            MarketToken::UNI => tokens / 1e18,
            MarketToken::DOGE => tokens / 1e8,
            MarketToken::LTC => tokens / 1e8,
        };

        let tokens = tokens.to_string().parse::<f64>().unwrap();
        (tokens * 1000.0).round() / 1000.0
    }
}

pub fn get_market(market: String) -> Market {
    match market.as_str() {
        "47c031236e19d024b42f8ae6780e44a573170703" => Market {
            market: MarketToken::WBTC,
            market_address: market,
            market_name: "WBTC".to_string(),
        },
        "c25cef6061cf5de5eb761b50e4743c1f5d7e5407" => Market {
            market: MarketToken::ARB,
            market_address: market,
            market_name: "ARB".to_string(),
        },
        "70d95587d40a2caf56bd97485ab3eec10bee6336" => Market {
            market: MarketToken::WETH,
            market_address: market,
            market_name: "WETH".to_string(),
        },
        "09400d9db990d5ed3f35d7be61dfaeb900af03c9" => Market {
            market: MarketToken::WSOL,
            market_address: market,
            market_name: "WSOL".to_string(),
        },
        "7f1fa204bb700853d36994da19f830b6ad18455c" => Market {
            market: MarketToken::LINK,
            market_address: market,
            market_name: "LINK".to_string(),
        },
        "0ccb4faa6f1f1b30911619f1184082ab4e25813c" => Market {
            market: MarketToken::XRP,
            market_address: market,
            market_name: "XRP".to_string(),
        },
        "c7abb2c5f3bf3ceb389df0eecd6120d451170b50" => Market {
            market: MarketToken::UNI,
            market_address: market,
            market_name: "UNI".to_string(),
        },
        "6853ea96ff216fab11d2d930ce3c508556a4bdc4" => Market {
            market: MarketToken::DOGE,
            market_address: market,
            market_name: "DOGE".to_string(),
        },
        "d9535bb5f58a1a75032416f2dfe7880c30575a41" => Market {
            market: MarketToken::LTC,
            market_address: market,
            market_name: "LTC".to_string(),
        },
        _ => panic!("Unknown market: {}", market),
    }
}
