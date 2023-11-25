pub mod decrease_maps {
    use crate::helpers::helpers;
    use crate::pb;
    use hex_literal::hex;
    use pb::gmx::{PositionDecrease, PositionDecreases};
    use substreams::Hex;
    use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

    const WBTC_MARKET: &str = "47c031236e19d024b42f8ae6780e44a573170703";
    const ARB_MARKET: &str = "c25cef6061cf5de5eb761b50e4743c1f5d7e5407";
    const WETH_MARKET: &str = "70d95587d40a2caf56bd97485ab3eec10bee6336";
    const WSOL: &str = "09400d9db990d5ed3f35d7be61dfaeb900af03c9";
    const LINK: &str = "7f1fa204bb700853d36994da19f830b6ad18455c";

    #[substreams::handlers::map]
    fn map_decreases(
        blk: eth::Block,
    ) -> Result<Option<PositionDecreases>, substreams::errors::Error> {
        let position_decreases: Vec<_> = blk
            .transaction_traces
            .iter()
            .map(|trx| {
                trx.calls.iter().map(|e| {
                    e.logs.iter().map(|g| {
                        g.topics.iter().filter_map(|d| {
                            if d.clone()
                                == hex!(
                                    "137a44067c8961cd7e1d876f4754a5a3a75989b4552f1843fc69c3b372def160"
                                )
                            {
                                let chunks: Vec<String> = helpers::get_chunks(g.data.clone());

                                if helpers::get_event_name(&chunks[4]) == "PositionDecrease".to_string() {
                                    //let market = helpers::get_address(&chunks[23]);
                                    let market = helpers::get_market(&chunks[23]);
                                    /*
                                    let execution_price = Hex::decode(&chunks[82]).unwrap();
                                    let execution_price =
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(
                                            &execution_price,
                                        );
                                    let execution_price =
                                        execution_price.to_string().parse::<f64>().unwrap();
                                    */
                                    let base_pnl = helpers::get_size_usd(&chunks[133]);

                                    Some(PositionDecrease {
                                        event_name: helpers::get_event_name(&chunks[4]),
                                        trx: Hex::encode(&trx.hash),
                                        account: helpers::get_address(&chunks[19]),
                                        market: market.market_name.clone(),
                                        market_address: market.market_address.clone(),
                                        execution_price: helpers::get_execution_price_old(&chunks[82]),
                                        size_usd: 0.01,
                                        size_tokens: helpers::get_size_in_tokens(
                                            &chunks[54],
                                        ),
                                        collateral_amount: 0.01,
                                        is_long: helpers::is_long(&chunks[146]),
                                        base_pnl,
                                        leverage: 10.0,
                                        order_type: helpers::get_order_type(&chunks[118]),
                                        order_key: chunks[156].clone(),
                                        position_key: chunks[160].clone(),
                                        timestamp: blk.timestamp().to_string(),
                                        block_number: blk.number,
                                    })
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                    })
                })
            })
            .flatten()
            .flatten()
            .flatten()
            .collect();

        Ok(Some(PositionDecreases { position_decreases }))
    }

    #[substreams::handlers::map]
    fn map_liquidations(
        decreases: PositionDecreases,
    ) -> Result<Option<PositionDecreases>, substreams::errors::Error> {
        let liquidations: Vec<_> = decreases
            .position_decreases
            .iter()
            .filter(|e| e.order_type == 7)
            .map(|e| e.clone())
            .collect();

        Ok(Some(PositionDecreases {
            position_decreases: liquidations,
        }))
    }

    #[substreams::handlers::map]
    fn map_wbtc_decreases(
        decreases: PositionDecreases,
    ) -> Result<Option<PositionDecreases>, substreams::errors::Error> {
        let wbtc: Vec<_> = decreases
            .position_decreases
            .iter()
            .filter(|e| e.market == WBTC_MARKET)
            .map(|e| e.clone())
            .collect();

        Ok(Some(PositionDecreases {
            position_decreases: wbtc,
        }))
    }

    #[substreams::handlers::map]
    fn map_arb_decreases(
        decreases: PositionDecreases,
    ) -> Result<Option<PositionDecreases>, substreams::errors::Error> {
        let arb: Vec<_> = decreases
            .position_decreases
            .iter()
            .filter(|e| e.market == ARB_MARKET)
            .map(|e| e.clone())
            .collect();

        Ok(Some(PositionDecreases {
            position_decreases: arb,
        }))
    }

    #[substreams::handlers::map]
    fn map_weth_decreases(
        decreases: PositionDecreases,
    ) -> Result<Option<PositionDecreases>, substreams::errors::Error> {
        let weth: Vec<_> = decreases
            .position_decreases
            .iter()
            .filter(|e| e.market == WETH_MARKET)
            .map(|e| e.clone())
            .collect();

        Ok(Some(PositionDecreases {
            position_decreases: weth,
        }))
    }

    #[substreams::handlers::map]
    fn map_wsol_decreases(
        decreases: PositionDecreases,
    ) -> Result<Option<PositionDecreases>, substreams::errors::Error> {
        let wsol: Vec<_> = decreases
            .position_decreases
            .iter()
            .filter(|e| e.market == WSOL)
            .map(|e| e.clone())
            .collect();

        Ok(Some(PositionDecreases {
            position_decreases: wsol,
        }))
    }

    #[substreams::handlers::map]
    fn map_link_decreases(
        decreases: PositionDecreases,
    ) -> Result<Option<PositionDecreases>, substreams::errors::Error> {
        let link: Vec<_> = decreases
            .position_decreases
            .iter()
            .filter(|e| e.market == LINK)
            .map(|e| e.clone())
            .collect();

        Ok(Some(PositionDecreases {
            position_decreases: link,
        }))
    }
}
