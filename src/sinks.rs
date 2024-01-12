use crate::pb;
use pb::gmx::{PositionDecreases, PositionIncreases};
// use substreams::Bigdecimal;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
// - map: map_increases

#[substreams::handlers::map]
pub fn graph_out(
    position_increases: PositionIncreases,
    position_decreases: PositionDecreases,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    for position_increase in position_increases.position_increases {
        let key = format!(
            "{}:{}:{}",
            position_increase.trx, position_increase.account, position_increase.market_address
        );

        tables
            .create_row("PositionIncrease", key)
            .set("event", position_increase.event_name)
            .set("txHash", position_increase.trx)
            .set("account", position_increase.account)
            .set("market", position_increase.market)
            .set("marketAddress", position_increase.market_address)
            .set_bigdecimal(
                "executionPrice",
                &position_increase.execution_price.to_string(),
            )
            .set_bigdecimal("sizeUsd", &position_increase.size_usd.to_string())
            .set_bigdecimal("sizeTokens", &position_increase.size_tokens.to_string())
            .set_bigdecimal(
                "collateralAmount",
                &position_increase.collateral_amount.to_string(),
            )
            .set("isLong", position_increase.is_long)
            .set_bigdecimal("leverage", &position_increase.leverage.to_string())
            .set("orderType", position_increase.order_type)
            .set("orderKey", position_increase.order_key)
            .set("positionKey", position_increase.position_key)
            .set("timestamp", position_increase.timestamp)
            .set("blockNumber", position_increase.block_number);
    }

    for position_decrease in position_decreases.position_decreases {
        let key = format!(
            "{}:{}:{}",
            position_decrease.trx, position_decrease.account, position_decrease.market_address
        );

        tables
            .create_row("PositionDecrease", key)
            .set("event", position_decrease.event_name)
            .set("txHash", position_decrease.trx)
            .set("account", position_decrease.account)
            .set("market", position_decrease.market)
            .set("marketAddress", position_decrease.market_address)
            .set_bigdecimal(
                "executionPrice",
                &position_decrease.execution_price.to_string(),
            )
            .set_bigdecimal("sizeUsd", &position_decrease.size_usd.to_string())
            .set_bigdecimal("sizeTokens", &position_decrease.size_tokens.to_string())
            .set_bigdecimal(
                "collateralAmount",
                &position_decrease.collateral_amount.to_string(),
            )
            .set("isLong", position_decrease.is_long)
            .set_bigdecimal("basePnl", &position_decrease.base_pnl.to_string())
            .set_bigdecimal("leverage", &position_decrease.leverage.to_string())
            .set("orderType", position_decrease.order_type)
            .set("orderKey", position_decrease.order_key)
            .set("positionKey", position_decrease.position_key)
            .set("timestamp", position_decrease.timestamp)
            .set("blockNumber", position_decrease.block_number);
    }

    Ok(tables.to_entity_changes())
}
