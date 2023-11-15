pub mod liquidation_maps {
    use crate::helpers;
    use crate::pb;
    use helpers::helpers::{get_chunks, get_event_name};
    use hex_literal::hex;
    use pb::gmx::PositionIncreases;
    use substreams::Hex;
    use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

    const WBTC_MARKET: &str = "47c031236e19d024b42f8ae6780e44a573170703";
    const ARB_MARKET: &str = "c25cef6061cf5de5eb761b50e4743c1f5d7e5407";
    const WETH_MARKET: &str = "70d95587d40a2caf56bd97485ab3eec10bee6336";
    const WSOL: &str = "09400d9db990d5ed3f35d7be61dfaeb900af03c9";
    const LINK: &str = "7f1fa204bb700853d36994da19f830b6ad18455c";

    //    substreams_ethereum::init!();

    #[substreams::handlers::map]
    fn map_liquidations(
        blk: eth::Block,
    ) -> Result<Option<PositionIncreases>, substreams::errors::Error> {
        //let mut position_decreases: Vec<PositionIncrease> = vec![];
        for trx in &blk.transaction_traces {
            for e in &trx.calls {
                for g in &e.logs {
                    for d in &g.topics {
                        if d.clone()
                            == hex!(
                                "137a44067c8961cd7e1d876f4754a5a3a75989b4552f1843fc69c3b372def160"
                            )
                        {
                            let chunks: Vec<String> = get_chunks(g.data.clone());

                            if get_event_name(&chunks[4]) == "liquidation".to_string() {
                                substreams::log::info!("liquidation");
                                substreams::log::info!("trx: {}", Hex::encode(&trx.hash));
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }
}
