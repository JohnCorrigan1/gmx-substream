pub mod increase_maps {
    use crate::helpers::helpers;
    use crate::pb;
    use helpers::Market;
    use hex_literal::hex;
    use pb::gmx::{PositionIncrease, PositionIncreases};
    use substreams::Hex;
    use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

    const WBTC_MARKET: &str = "47c031236e19d024b42f8ae6780e44a573170703";
    const ARB_MARKET: &str = "c25cef6061cf5de5eb761b50e4743c1f5d7e5407";
    const WETH_MARKET: &str = "70d95587d40a2caf56bd97485ab3eec10bee6336";
    const WSOL: &str = "09400d9db990d5ed3f35d7be61dfaeb900af03c9";
    const LINK: &str = "7f1fa204bb700853d36994da19f830b6ad18455c";

    substreams_ethereum::init!();

    #[substreams::handlers::map]
    fn map_increases(
        blk: eth::Block,
    ) -> Result<Option<PositionIncreases>, substreams::errors::Error> {
        let position_increases: Vec<_> = blk
            .transaction_traces
            .iter()
            .map(|trx| {
                trx.calls.iter().map(|e| {
                    e.logs.iter().map(|g| {
                        g.topics.iter().filter_map(|d| {
                            if d.clone()
                                == hex!(
                                "137a44067c8961cd7e1d876f4754a5a3a75989b4552f1843fc69c3b372def160"
                            ) {
                                let chunks: Vec<String> = helpers::get_chunks(g.data.clone());

                                if helpers::get_event_name(&chunks[4])
                                    == "PositionIncrease".to_string()
                                {
                                    substreams::log::info!(
                                        "market: {}",
                                        helpers::get_address(&chunks[23])
                                    );
                                    let market: Market = helpers::get_market(&chunks[23]);
                                    let size_usd = helpers::get_size_usd(&chunks[50]);
                                    let collateral_amount = helpers::get_collat(&chunks[58]);

                                    Some(PositionIncrease {
                                        event_name: helpers::get_event_name(&chunks[4]),
                                        trx: Hex::encode(&trx.hash),
                                        account: helpers::get_address(&chunks[19]),
                                        market: market.market_address.clone(),
                                        execution_price: market.get_execution_price(&chunks[80]),
                                        size_usd,
                                        size_tokens: market.get_tokens(&chunks[54]),
                                        collateral_amount,
                                        is_long: helpers::is_long(&chunks[136]),
                                        leverage: ((size_usd / collateral_amount) * 10.0).round()
                                            / 10.0,
                                        order_type: helpers::get_order_type(&chunks[108]),
                                        order_key: chunks[146].clone(),
                                        position_key: chunks[150].clone(),
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

        Ok(Some(PositionIncreases { position_increases }))
    }

    #[substreams::handlers::map]
    fn map_wbtc_increases(position_increases: PositionIncreases) -> PositionIncreases {
        let wbtc_increases: Vec<_> = position_increases
            .position_increases
            .iter()
            .filter_map(|p| {
                if p.market == WBTC_MARKET {
                    Some(p.clone())
                } else {
                    None
                }
            })
            .collect();

        PositionIncreases {
            position_increases: wbtc_increases,
        }
    }

    #[substreams::handlers::map]
    fn map_weth_increases(position_increases: PositionIncreases) -> PositionIncreases {
        let weth_increases: Vec<_> = position_increases
            .position_increases
            .iter()
            .filter_map(|p| {
                if p.market == WETH_MARKET {
                    Some(p.clone())
                } else {
                    None
                }
            })
            .collect();

        PositionIncreases {
            position_increases: weth_increases,
        }
    }

    #[substreams::handlers::map]
    fn map_arb_increases(position_increases: PositionIncreases) -> PositionIncreases {
        let arb_increases: Vec<_> = position_increases
            .position_increases
            .iter()
            .filter_map(|p| {
                if p.market == ARB_MARKET {
                    Some(p.clone())
                } else {
                    None
                }
            })
            .collect();

        PositionIncreases {
            position_increases: arb_increases,
        }
    }

    #[substreams::handlers::map]
    fn map_wsol_increases(position_increases: PositionIncreases) -> PositionIncreases {
        let wsol_increases: Vec<_> = position_increases
            .position_increases
            .iter()
            .filter_map(|p| {
                if p.market == WSOL {
                    Some(p.clone())
                } else {
                    None
                }
            })
            .collect();

        PositionIncreases {
            position_increases: wsol_increases,
        }
    }

    #[substreams::handlers::map]
    fn map_link_increases(position_increases: PositionIncreases) -> PositionIncreases {
        let link_increases: Vec<_> = position_increases
            .position_increases
            .iter()
            .filter_map(|p| {
                if p.market == LINK {
                    Some(p.clone())
                } else {
                    None
                }
            })
            .collect();

        PositionIncreases {
            position_increases: link_increases,
        }
    }
}
