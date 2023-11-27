use crate::helpers;
use crate::market::{get_market, Market};
use crate::pb;
use hex_literal::hex;
use pb::gmx::{PositionIncrease, PositionIncreases};
use substreams::Hex;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_increases(blk: eth::Block) -> Result<Option<PositionIncreases>, substreams::errors::Error> {
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
                            )
                        {
                            let chunks: Vec<String> = helpers::get_chunks(g.data.clone());

                            if helpers::get_event_name(&chunks[4]) == "PositionIncrease".to_string()
                            {
                                let market: Market = get_market(&chunks[23]);
                                let size_usd = helpers::get_size_usd(&chunks[50]);
                                let collateral_amount = helpers::get_collat(&chunks[58]);
                                Some(PositionIncrease {
                                    event_name: helpers::get_event_name(&chunks[4]),
                                    trx: Hex::encode(&trx.hash),
                                    account: helpers::get_address(&chunks[19]),
                                    market: market.market_name.clone(),
                                    market_address: market.market_address.clone(),
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
            if p.market == "WBTC".to_string() {
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
            if p.market == "WETH".to_string() {
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
            if p.market == "ARB".to_string() {
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
            if p.market == "WSOL".to_string() {
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
            if p.market == "LINK".to_string() {
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

#[substreams::handlers::map]
fn map_uni_increases(position_increases: PositionIncreases) -> PositionIncreases {
    let uni_increases: Vec<_> = position_increases
        .position_increases
        .iter()
        .filter_map(|p| {
            if p.market == "UNI".to_string() {
                Some(p.clone())
            } else {
                None
            }
        })
        .collect();

    PositionIncreases {
        position_increases: uni_increases,
    }
}

#[substreams::handlers::map]
fn map_xrp_increases(position_increases: PositionIncreases) -> PositionIncreases {
    let xrp_increases: Vec<_> = position_increases
        .position_increases
        .iter()
        .filter_map(|p| {
            if p.market == "XRP".to_string() {
                Some(p.clone())
            } else {
                None
            }
        })
        .collect();

    PositionIncreases {
        position_increases: xrp_increases,
    }
}

#[substreams::handlers::map]
fn map_doge_increases(position_increases: PositionIncreases) -> PositionIncreases {
    let doge_increases: Vec<_> = position_increases
        .position_increases
        .iter()
        .filter_map(|p| {
            if p.market == "DOGE".to_string() {
                Some(p.clone())
            } else {
                None
            }
        })
        .collect();

    PositionIncreases {
        position_increases: doge_increases,
    }
}

#[substreams::handlers::map]
fn map_ltc_increases(position_increases: PositionIncreases) -> PositionIncreases {
    let ltc_increases: Vec<_> = position_increases
        .position_increases
        .iter()
        .filter_map(|p| {
            if p.market == "LTC".to_string() {
                Some(p.clone())
            } else {
                None
            }
        })
        .collect();

    PositionIncreases {
        position_increases: ltc_increases,
    }
}
