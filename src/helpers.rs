pub mod helpers {

    use substreams::Hex;

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
        pub market_address: String,
    }

    impl Market {
        pub fn get_execution_price(&self, chunk: &str) -> f64 {
            let execution_price = Hex::decode(chunk).unwrap();
            let execution_price: substreams::scalar::BigInt =
                substreams::scalar::BigInt::from_unsigned_bytes_be(&execution_price);

            let execution_price: substreams::scalar::BigDecimal = match self.market {
                MarketToken::WBTC => execution_price / 1e22,
                MarketToken::ARB => execution_price / 1e22,
                MarketToken::WETH => execution_price / 1e1,
                MarketToken::WSOL => execution_price / 1e9,
                MarketToken::LINK => execution_price / 1e18,
                MarketToken::XRP => execution_price / 1e24,
                MarketToken::UNI => execution_price / 1e18,
                MarketToken::DOGE => execution_price / 1e22,
                MarketToken::LTC => execution_price / 1e22,
            };

            let execution_price = execution_price.to_string().parse::<f64>().unwrap();
            (execution_price * 100.0).round() / 100.0
        }

        pub fn get_tokens(&self, chunk: &str) -> f64 {
            let tokens = Hex::decode(chunk).unwrap();
            let tokens: substreams::scalar::BigInt =
                substreams::scalar::BigInt::from_unsigned_bytes_be(&tokens);

            let tokens: substreams::scalar::BigDecimal = match self.market {
                MarketToken::WBTC => tokens / 1e8,
                MarketToken::ARB => tokens / 1e8,
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

    pub fn get_market(chunk: &str) -> Market {
        let market = get_address(chunk);
        match market.as_str() {
            "47c031236e19d024b42f8ae6780e44a573170703" => Market {
                market: MarketToken::WBTC,
                market_address: "WBTC".to_string(),
            },
            "c25cef6061cf5de5eb761b50e4743c1f5d7e5407" => Market {
                market: MarketToken::ARB,
                market_address: "ARB".to_string(),
            },
            "70d95587d40a2caf56bd97485ab3eec10bee6336" => Market {
                market: MarketToken::WETH,
                market_address: "WETH".to_string(),
            },
            "09400d9db990d5ed3f35d7be61dfaeb900af03c9" => Market {
                market: MarketToken::WSOL,
                market_address: "WSOL".to_string(),
            },
            "7f1fa204bb700853d36994da19f830b6ad18455c" => Market {
                market: MarketToken::LINK,
                market_address: "LINK".to_string(),
            },
            "0ccb4faa6f1f1b30911619f1184082ab4e25813c" => Market {
                market: MarketToken::XRP,
                market_address: "XRP".to_string(),
            },
            "c7abb2c5f3bf3ceb389df0eecd6120d451170b50" => Market {
                market: MarketToken::UNI,
                market_address: "UNI".to_string(),
            },
            "6853ea96ff216fab11d2d930ce3c508556a4bdc4" => Market {
                market: MarketToken::DOGE,
                market_address: "DOGE".to_string(),
            },
            "d9535bb5f58a1a75032416f2dfe7880c30575a41" => Market {
                market: MarketToken::LTC,
                market_address: "LTC".to_string(),
            },
            /*            _ => Market {
                market: MarketToken::WBTC,
                market_address: "WBTC".to_string(),
            }, */
            _ => panic!("Unknown market: {}", market),
        }
    }

    pub fn get_chunks(data: Vec<u8>) -> Vec<String> {
        let mut chunks: Vec<String> = Vec::new();

        for i in (0..Hex::encode(&data).len()).step_by(64) {
            chunks.push(Hex::encode(&data).chars().skip(i).take(64).collect());
        }

        chunks
    }

    pub fn get_event_name(chunk: &str) -> String {
        let event_name = Hex::decode(chunk).unwrap();
        let event_name = String::from_utf8(event_name).unwrap();
        event_name.trim_matches(|c| c == '\0').to_string()
    }

    pub fn get_size_usd(chunk: &str) -> f64 {
        let usd = Hex::decode(chunk).unwrap();
        let usd: substreams::scalar::BigDecimal =
            substreams::scalar::BigInt::from_signed_bytes_be(&usd) / 1e30;
        let usd = usd.to_string().parse::<f64>().unwrap();
        (usd * 100.0).round() / 100.0
    }

    pub fn get_collat(chunk: &str) -> f64 {
        let collat = Hex::decode(chunk).unwrap();
        let collat: substreams::scalar::BigDecimal =
            substreams::scalar::BigInt::from_unsigned_bytes_be(&collat) / 1e6;
        let collat = collat.to_string().parse::<f64>().unwrap();
        collat
    }

    pub fn get_execution_price_old(chunk: &str) -> f64 {
        let execution_price = Hex::decode(chunk).unwrap();
        let execution_price: substreams::scalar::BigDecimal =
            substreams::scalar::BigInt::from_unsigned_bytes_be(&execution_price) / 1e22;
        let execution_price = execution_price.to_string().parse::<f64>().unwrap();
        (execution_price * 100.0).round() / 100.0
    }

    pub fn is_long(chunk: &str) -> bool {
        let is_long = Hex::decode(chunk).unwrap();
        let is_long_num = substreams::scalar::BigInt::from_unsigned_bytes_be(&is_long)
            .to_string()
            .parse::<i32>()
            .unwrap();
        let mut is_long: bool = false;
        if is_long_num == 1 {
            is_long = true;
        }
        is_long
    }

    pub fn get_size_in_tokens(chunk: &str) -> f64 {
        let size_in_tokens = Hex::decode(chunk).unwrap();
        let size_in_tokens: substreams::scalar::BigInt =
            substreams::scalar::BigInt::from_unsigned_bytes_be(&size_in_tokens);
        let size_in_tokens = size_in_tokens.to_string().parse::<f64>().unwrap();
        size_in_tokens
    }

    pub fn get_address(chunk: &str) -> String {
        let address = Hex::decode(chunk).unwrap();
        let address = Hex::encode(&address[12..].to_vec());
        address
    }

    pub fn get_order_type(chunk: &str) -> i32 {
        let order_type = Hex::decode(chunk).unwrap();
        let order_type: substreams::scalar::BigInt =
            substreams::scalar::BigInt::from_unsigned_bytes_be(&order_type);
        let order_type = order_type.to_string().parse::<i32>().unwrap();
        order_type
    }
}
