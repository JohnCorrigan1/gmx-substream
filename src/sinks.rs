use crate::pb;
use pb::gmx::PositionChanges;
// use substreams::Bigdecimal;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
// - map: map_increases

#[substreams::handlers::map]
pub fn graph_out(
    position_changes: PositionChanges,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    for position_change in position_changes.position_changes {
        if position_change.event_name == "PositionIncrease".to_string() {
            tables
                .create_row("PositionIncrease", position_change.order_key.clone())
                .set("event", position_change.event_name)
                .set("txHash", position_change.trx)
                .set("account", position_change.account)
                .set("market", position_change.market)
                .set("marketAddress", position_change.market_address)
                .set_bigdecimal(
                    "executionPrice",
                    &position_change.execution_price.to_string(),
                )
                .set_bigdecimal("sizeUsd", &position_change.size_usd.to_string())
                .set_bigdecimal("sizeTokens", &position_change.size_tokens.to_string())
                .set_bigdecimal(
                    "collateralAmount",
                    &position_change.collateral_amount.to_string(),
                )
                .set("isLong", position_change.is_long)
                .set_bigdecimal("leverage", &position_change.leverage.to_string())
                .set("orderType", position_change.order_type)
                .set("orderKey", position_change.order_key.clone())
                .set("positionKey", position_change.position_key)
                .set("timestamp", position_change.timestamp)
                .set("blockNumber", position_change.block_number);
        } else if position_change.event_name == "PositionDecrease".to_string() {
            tables
                .create_row("PositionDecrease", position_change.order_key.clone())
                .set("event", position_change.event_name)
                .set("txHash", position_change.trx)
                .set("account", position_change.account)
                .set("market", position_change.market)
                .set("marketAddress", position_change.market_address)
                .set_bigdecimal(
                    "executionPrice",
                    &position_change.execution_price.to_string(),
                )
                .set_bigdecimal("sizeUsd", &position_change.size_usd.to_string())
                .set_bigdecimal("sizeTokens", &position_change.size_tokens.to_string())
                .set_bigdecimal(
                    "collateralAmount",
                    &position_change.collateral_amount.to_string(),
                )
                .set("isLong", position_change.is_long)
                .set_bigdecimal("basePnl", &position_change.base_pnl.to_string())
                .set_bigdecimal("leverage", &position_change.leverage.to_string())
                .set("orderType", position_change.order_type)
                .set("orderKey", position_change.order_key)
                .set("positionKey", position_change.position_key)
                .set("timestamp", position_change.timestamp)
                .set("blockNumber", position_change.block_number);
        }
    }

    Ok(tables.to_entity_changes())
}
