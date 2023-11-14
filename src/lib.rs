mod abi;
mod helpers;
mod pb;
use helpers::helpers::{get_chunks, get_event_name};
use hex_literal::hex;
use pb::gmx::{PositionIncrease, PositionIncreases};
use substreams::Hex;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

//const WBTC_MARKET: &str = "0x47c031236e19d024b42f8ae6780e44a573170703"; 00000000000000000000000047c031236e19d024b42f8ae6780e44a573170703
substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_increases(blk: eth::Block) -> Result<Option<PositionIncreases>, substreams::errors::Error> {
    for trx in &blk.transaction_traces {
        for e in &trx.calls {
            for g in &e.logs {
                for d in &g.topics {
                    if d.clone()
                        == hex!("137a44067c8961cd7e1d876f4754a5a3a75989b4552f1843fc69c3b372def160")
                    {
                        let chunks: Vec<String> = get_chunks(g.data.clone());

                        if get_event_name(&chunks[4]) == "PositionIncrease".to_string() {
                            let size_usd = helpers::helpers::get_size_usd(&chunks[50]);
                            let collateral_amount = helpers::helpers::get_collat(&chunks[58]);

                            let increase = PositionIncrease {
                                event_name: get_event_name(&chunks[4]),
                                trx: Hex::encode(&trx.hash),
                                account: helpers::helpers::get_address(&chunks[19]),
                                market: helpers::helpers::get_address(&chunks[23]),
                                execution_price: helpers::helpers::get_execution_price(&chunks[80]),
                                size_usd,
                                size_tokens: helpers::helpers::get_size_in_tokens(&chunks[54]),
                                collateral_amount,
                                is_long: helpers::helpers::is_long(&chunks[136]),
                                leverage: ((size_usd / collateral_amount) * 10.0).round() / 10.0,
                                order_type: helpers::helpers::get_order_type(&chunks[108]),
                                order_key: chunks[146].clone(),
                                position_key: chunks[150].clone(),
                                timestamp: blk.timestamp().to_string(),
                                block_number: blk.number,
                            };
                            substreams::log::info!("increase: {:?}", increase);
                        }
                    }
                }
            }
        }
    }

    let position_increases: Vec<PositionIncrease> = vec![];

    Ok(Some(PositionIncreases { position_increases }))
}
