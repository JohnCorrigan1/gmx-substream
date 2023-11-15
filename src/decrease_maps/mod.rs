pub mod decrease_maps {
    use crate::helpers;
    use crate::pb;
    use helpers::helpers::{get_chunks, get_event_name};
    use hex_literal::hex;
    use num_bigint::BigInt;
    use pb::gmx::PositionIncreases;
    use substreams::Hex;
    use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

    const WBTC_MARKET: &str = "47c031236e19d024b42f8ae6780e44a573170703";
    const ARB_MARKET: &str = "c25cef6061cf5de5eb761b50e4743c1f5d7e5407";
    const WETH_MARKET: &str = "70d95587d40a2caf56bd97485ab3eec10bee6336";
    const WSOL: &str = "09400d9db990d5ed3f35d7be61dfaeb900af03c9";
    const LINK: &str = "7f1fa204bb700853d36994da19f830b6ad18455c";

    //    substreams_ethereum::init!();

    #[substreams::handlers::map]
    fn map_decreases(
        blk: eth::Block,
    ) -> Result<Option<PositionIncreases>, substreams::errors::Error> {
        //let mut position_decreases: Vec<PositionIncrease> = vec![];
        for trx in &blk.transaction_traces {
            for e in &trx.calls {
                for g in &e.logs {
                    for d in &g.topics {
                        if d.clone()
                            == hex!(
                                "137a44067c8961cd7e1d876f4754a5a3a75989b4552f1843fc69c3b372def160"
                            )
                        {
                            let chunks: Vec<String> = get_chunks(g.data.clone());

                            if get_event_name(&chunks[4]) == "PositionDecrease".to_string() {
                                substreams::log::info!("PositionDecrease");
                                substreams::log::info!("trx: {}", Hex::encode(&trx.hash));

                                let size_usd = helpers::helpers::get_size_usd(&chunks[52]);
                                let collateral_amount = helpers::helpers::get_collat(&chunks[60]);
                                let account = helpers::helpers::get_address(&chunks[19]);
                                let market = helpers::helpers::get_address(&chunks[23]);
                                let size_tokens = helpers::helpers::get_size_in_tokens(&chunks[56]);
                                let execution_price = Hex::decode(&chunks[82]).unwrap();
                                let execution_price =
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(
                                        &execution_price,
                                    );

                                let order_type = helpers::helpers::get_order_type(&chunks[118]);
                                let base_pnl = helpers::helpers::get_size_usd(&chunks[133]);
                                //let base_pnl = &chunks[133].clone();
                                //substreams::log::info!("len: {}", base_pnl.len());
                                /*
                                                                let base_pnl = Hex::decode(&base_pnl).unwrap();
                                                                let base_pnl =
                                                                    substreams::scalar::BigInt::from_signed_bytes_be(&base_pnl);
                                                                let base_pnl: substreams::scalar::BigDecimal = base_pnl / 1e30;
                                                                let base_pnl: f64 = base_pnl.to_string().parse::<f64>().unwrap();
                                                                let base_pnl = (base_pnl * 100.0).round() / 100.0;
                                                                substreams::log::info!("base_pnl: {}", base_pnl);
                                */
                                //let base_pnl =
                                //   substreams::scalar::BigDecimal::from_str_radix(&base_pnl, 32);
                                //substreams::log::info!("real: {:?}", base_pnl);
                                //let base_pnl = base_pnl.trim_matches(|c| c == 'f').to_string();
                                //let base_pnl = helpers::helpers::get_size_usd(&chunks[133]);
                                let is_long = helpers::helpers::is_long(&chunks[146]);

                                //helpers::helpers::get_execution_price(&chunks[82]);
                                let position_key = Hex::decode(&chunks[160].clone());
                                substreams::log::info!("size_usd: {}", size_usd);
                                substreams::log::info!("collat: {}", collateral_amount);
                                substreams::log::info!("account: {}", account);
                                substreams::log::info!("size_tokens: {}", size_tokens);
                                substreams::log::info!("market: {}", market);
                                substreams::log::info!("execution_price: {}", execution_price);
                                substreams::log::info!("order_type: {}", order_type);
                                substreams::log::info!("base_pnl: {}", base_pnl);
                                substreams::log::info!("is_long: {}", is_long);
                                substreams::log::info!("position_key: {:?}", position_key);
                            }
                        }
                    }
                }
            }
        }

        Ok(None)
    }
}
