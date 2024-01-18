use crate::abi;
use crate::helpers;
use crate::market::{get_market, Market};
use crate::pb;
use hex_literal::hex;
use pb::gmx::{PositionChange, PositionChanges};
use substreams::Hex;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

const EVENT_EMITTER: [u8; 20] = hex!("C8ee91A54287DB53897056e12D9819156D3822Fb");

#[substreams::handlers::map]
fn map_positions(blk: eth::Block) -> Result<Option<PositionChanges>, substreams::errors::Error> {
    let position_changes = blk
        .events::<abi::gmx::events::EventLog1>(&[&EVENT_EMITTER])
        .map(|(event, log)| {
            if event.event_name == "PositionDecrease" {
                let market: Market = get_market(Hex::encode(&event.event_data.0 .0[1].1));
                let size_usd = helpers::get_usd(&event.event_data.1 .0[0].1);
                let collateral_amount = helpers::get_collateral(&event.event_data.1 .0[2].1);
                Some(PositionChange {
                    event_name: event.event_name.clone(),
                    trx: "0x".to_string() + &Hex::encode(&log.receipt.transaction.hash),
                    account: "0x".to_string() + &Hex::encode(&event.event_data.0 .0[0].1),
                    market: market.market_name.clone(),
                    market_address: "0x".to_string() + market.market_address.as_str(),
                    execution_price: market.get_execution_price(&event.event_data.1 .0[7].1),
                    size_usd,
                    size_tokens: market.get_tokens(&event.event_data.1 .0[1].1),
                    collateral_amount,
                    is_long: event.event_data.3 .0[0].1,
                    base_pnl: helpers::get_usd(&event.event_data.2 .0[1].1),
                    leverage: helpers::get_leverage(size_usd, collateral_amount),
                    order_type: event.event_data.1 .0[16].1.to_i32(),
                    order_key: Hex::encode(&event.event_data.4 .0[0].1),
                    position_key: Hex::encode(&event.event_data.4 .0[1].1),
                    timestamp: blk.timestamp().to_string(),
                    block_number: blk.number,
                })
            } else if event.event_name == "PositionIncrease" {
                let market: Market = get_market(Hex::encode(&event.event_data.0 .0[1].1));
                let size_usd = helpers::get_usd(&event.event_data.1 .0[0].1);
                let collateral_amount = helpers::get_collateral(&event.event_data.1 .0[2].1);
                Some(PositionChange {
                    event_name: event.event_name.clone(),
                    trx: "0x".to_string() + &Hex::encode(&log.receipt.transaction.hash),
                    account: "0x".to_string() + &Hex::encode(&event.event_data.0 .0[0].1),
                    market: market.market_name.clone(),
                    market_address: "0x".to_string() + market.market_address.as_str(),
                    execution_price: market.get_execution_price(&event.event_data.1 .0[7].1),
                    size_usd,
                    size_tokens: market.get_tokens(&event.event_data.1 .0[1].1),
                    collateral_amount,
                    is_long: event.event_data.3 .0[0].1,
                    base_pnl: 0.0,
                    leverage: helpers::get_leverage(size_usd, collateral_amount),
                    order_type: event.event_data.1 .0[14].1.to_i32(),
                    order_key: Hex::encode(&event.event_data.4 .0[0].1),
                    position_key: Hex::encode(&event.event_data.4 .0[1].1),
                    timestamp: blk.timestamp().to_string(),
                    block_number: blk.number,
                })
            } else {
                None
            }
        })
        .flatten()
        .collect();

    Ok(Some(PositionChanges { position_changes }))
}
