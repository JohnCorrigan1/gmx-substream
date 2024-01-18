use substreams::scalar::{BigDecimal, BigInt};

pub fn get_usd(usd: &BigInt) -> f64 {
    let usd: BigDecimal = usd.clone() / 1e30;
    let usd = usd.to_string().parse::<f64>().unwrap();
    (usd * 100.0).round() / 100.0
}

pub fn get_collateral(collateral: &BigInt) -> f64 {
    if collateral.is_zero() {
        return 0.0;
    }
    let collat: BigDecimal = collateral.clone() / 1e6;
    let collat = collat.to_string().parse::<f64>().unwrap();
    (collat * 100.0).round() / 100.0
}

pub fn get_leverage(size_usd: f64, collateral_amount: f64) -> f64 {
    if collateral_amount == 0.0 || size_usd == 0.0 {
        return 0.0;
    }
    let leverage = size_usd / collateral_amount;
    (leverage * 100.0).round() / 100.0
}
