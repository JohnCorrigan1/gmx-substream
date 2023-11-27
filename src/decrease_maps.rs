use crate::helpers;
use crate::market::{get_market, Market};
use crate::pb;
use hex_literal::hex;
use pb::gmx::{PositionDecrease, PositionDecreases};
use substreams::Hex;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

#[substreams::handlers::map]
fn map_decreases(blk: eth::Block) -> Result<Option<PositionDecreases>, substreams::errors::Error> {
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

                            if helpers::get_event_name(&chunks[4]) == "PositionDecrease".to_string()
                            {
                                let market: Market = get_market(&chunks[23]);
                                let base_pnl = helpers::get_size_usd(&chunks[133]);

                                Some(PositionDecrease {
                                    event_name: helpers::get_event_name(&chunks[4]),
                                    trx: Hex::encode(&trx.hash),
                                    account: helpers::get_address(&chunks[19]),
                                    market: market.market_name.clone(),
                                    market_address: market.market_address.clone(),
                                    execution_price: helpers::get_execution_price_old(&chunks[82]),
                                    size_usd: 0.01,
                                    size_tokens: helpers::get_size_in_tokens(&chunks[54]),
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
        .filter(|e| e.market == "WBTC".to_string())
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
        .filter(|e| e.market == "ARB".to_string())
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
        .filter(|e| e.market == "WETH".to_string())
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
        .filter(|e| e.market == "WSOL".to_string())
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
        .filter(|e| e.market == "LINK".to_string())
        .map(|e| e.clone())
        .collect();

    Ok(Some(PositionDecreases {
        position_decreases: link,
    }))
}

#[substreams::handlers::map]
fn map_uni_decreases(
    decreases: PositionDecreases,
) -> Result<Option<PositionDecreases>, substreams::errors::Error> {
    let uni: Vec<_> = decreases
        .position_decreases
        .iter()
        .filter(|e| e.market == "UNI".to_string())
        .map(|e| e.clone())
        .collect();

    Ok(Some(PositionDecreases {
        position_decreases: uni,
    }))
}

#[substreams::handlers::map]
fn map_xrp_decreases(
    decreases: PositionDecreases,
) -> Result<Option<PositionDecreases>, substreams::errors::Error> {
    let xrp: Vec<_> = decreases
        .position_decreases
        .iter()
        .filter(|e| e.market == "XRP".to_string())
        .map(|e| e.clone())
        .collect();

    Ok(Some(PositionDecreases {
        position_decreases: xrp,
    }))
}

#[substreams::handlers::map]
fn map_doge_decreases(
    decreases: PositionDecreases,
) -> Result<Option<PositionDecreases>, substreams::errors::Error> {
    let doge: Vec<_> = decreases
        .position_decreases
        .iter()
        .filter(|e| e.market == "DOGE".to_string())
        .map(|e| e.clone())
        .collect();

    Ok(Some(PositionDecreases {
        position_decreases: doge,
    }))
}

#[substreams::handlers::map]
fn map_ltc_decreases(
    decreases: PositionDecreases,
) -> Result<Option<PositionDecreases>, substreams::errors::Error> {
    let ltc: Vec<_> = decreases
        .position_decreases
        .iter()
        .filter(|e| e.market == "LTC".to_string())
        .map(|e| e.clone())
        .collect();

    Ok(Some(PositionDecreases {
        position_decreases: ltc,
    }))
}
