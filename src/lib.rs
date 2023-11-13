mod abi;
mod pb;
use ethereum_types::{H160, H256};
use hex_literal::hex;
use pb::gmx::{PositionIncrease, PositionIncreases, Thing, Things};
use substreams::Hex;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;
//use substreams_ethereum::Event;

// Bored Ape Club Contract
//const TRACKED_CONTRACT: [u8; 20] = hex!("C8ee91A54287DB53897056e12D9819156D3822Fb");
//const WBTC_MARKET: &str = "0x47c031236e19d024b42f8ae6780e44a573170703"; 00000000000000000000000047c031236e19d024b42f8ae6780e44a573170703
substreams_ethereum::init!();

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<Option<Things>, substreams::errors::Error> {
    /*
        let things: Vec<_> = blk
            .events::<abi::gmx::events::EventLog>(&[&TRACKED_CONTRACT])
            .collect();
    */

    for trx in &blk.transaction_traces {
        for e in &trx.calls {
            for g in &e.logs {
                for d in &g.topics {
                    if d.clone()
                        == hex!("137a44067c8961cd7e1d876f4754a5a3a75989b4552f1843fc69c3b372def160")
                    {
                        let mut chunks: Vec<String> = Vec::new();

                        for i in (0..Hex::encode(&g.data).len()).step_by(64) {
                            chunks.push(Hex::encode(&g.data).chars().skip(i).take(64).collect());
                        }

                        if chunks[4]
                            == "506f736974696f6e496e63726561736500000000000000000000000000000000"
                                .to_string()
                        {
                            let event_name = Hex::decode(&chunks[4]).unwrap();
                            let event_name = String::from_utf8(event_name).unwrap();
                            substreams::log::info!("eventName: {}", event_name);

                            let usd = chunks[50].clone();
                            let usd = Hex::decode(&usd).unwrap();
                            let usd: substreams::scalar::BigDecimal =
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&usd) / 1e30;
                            let usd = usd.to_string().parse::<f64>().unwrap();
                            substreams::log::info!("usd: {}", usd);

                            let collat = chunks[58].clone();
                            let collat = Hex::decode(&collat).unwrap();
                            let collat: substreams::scalar::BigDecimal =
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&collat) / 1e6;
                            let collat = collat.to_string().parse::<f64>().unwrap();

                            substreams::log::info!("collat: {}", collat);

                            substreams::log::info!("leverage: {}", usd / collat);

                            let increase = PositionIncrease {
                                trx: Hex::encode(&trx.hash),
                                market: Hex::encode(
                                    &Hex::decode(&chunks[23]).unwrap()[12..].to_vec(),
                                ),
                                size_usd: usd.to_string(),
                                size_tokens: chunks[54].clone(),
                                collateral_amount: chunks[58].clone(),
                                collateral_usd: collat.to_string(),
                                is_long: true,
                                leverage: (usd / collat).to_string(),
                            };
                            substreams::log::info!("increase: {:?}", increase);
                        }
                    }
                }
            }
        }
    }

    let things: Vec<Thing> = vec![];

    Ok(Some(Things { things }))
}
