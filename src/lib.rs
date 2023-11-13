mod abi;
mod pb;
use ethereum_types::{H160, H256};
use hex_literal::hex;
use pb::gmx::{PositionIncrease, PositionIncreases, Thing, Things};
use substreams::Hex;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;
//use substreams_ethereum::Event;

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

                            let execution_price = chunks[80].clone();
                            let execution_price = Hex::decode(&execution_price).unwrap();
                            let execution_price: substreams::scalar::BigDecimal =
                                substreams::scalar::BigInt::from_unsigned_bytes_be(
                                    &execution_price,
                                ) / 1e22;
                            let execution_price =
                                execution_price.to_string().parse::<f64>().unwrap();
                            substreams::log::info!("execution_price: {}", execution_price);
                            let is_long = chunks[136].clone();
                            let is_long = Hex::decode(&is_long).unwrap();
                            let is_long_num =
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&is_long)
                                    .to_string()
                                    .parse::<i32>()
                                    .unwrap();
                            let mut is_long: bool = false;
                            if is_long_num == 1 {
                                is_long = true;
                            }

                            let size_in_tokens = chunks[54].clone();
                            let size_in_tokens = Hex::decode(&size_in_tokens).unwrap();
                            let size_in_tokens: substreams::scalar::BigInt =
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&size_in_tokens);
                            let size_in_tokens = size_in_tokens.to_string().parse::<f64>().unwrap();

                            let account =
                                Hex::encode(&Hex::decode(&chunks[19]).unwrap()[12..].to_vec());

                            substreams::log::info!("account: {}", account);

                            substreams::log::info!("size_in_tokens: {}", size_in_tokens);

                            substreams::log::info!("is_long: {:?}", is_long);

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
