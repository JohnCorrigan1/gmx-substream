use substreams::Hex;

pub fn get_chunks(data: Vec<u8>) -> Vec<String> {
    let mut chunks: Vec<String> = Vec::new();

    for i in (0..Hex::encode(&data).len()).step_by(64) {
        chunks.push(Hex::encode(&data).chars().skip(i).take(64).collect());
    }
    chunks
}

pub fn get_event_name(chunk: &str) -> String {
    let event_name = Hex::decode(chunk).unwrap();
    let event_name = String::from_utf8(event_name).unwrap();
    event_name.trim_matches(|c| c == '\0').to_string()
}

pub fn get_size_usd(chunk: &str) -> f64 {
    let usd = Hex::decode(chunk).unwrap();
    let usd: substreams::scalar::BigDecimal =
        substreams::scalar::BigInt::from_signed_bytes_be(&usd) / 1e30;
    let usd = usd.to_string().parse::<f64>().unwrap();
    (usd * 100.0).round() / 100.0
}

pub fn get_collat(chunk: &str) -> f64 {
    let collat = Hex::decode(chunk).unwrap();
    let collat: substreams::scalar::BigDecimal =
        substreams::scalar::BigInt::from_unsigned_bytes_be(&collat) / 1e6;
    let collat = collat.to_string().parse::<f64>().unwrap();
    (collat * 100.0).round() / 100.0
}

pub fn get_execution_price_old(chunk: &str) -> f64 {
    let execution_price = Hex::decode(chunk).unwrap();
    let execution_price: substreams::scalar::BigDecimal =
        substreams::scalar::BigInt::from_unsigned_bytes_be(&execution_price) / 1e22;
    let execution_price = execution_price.to_string().parse::<f64>().unwrap();
    (execution_price * 100.0).round() / 100.0
}

pub fn is_long(chunk: &str) -> bool {
    let is_long = Hex::decode(chunk).unwrap();
    let is_long_num = substreams::scalar::BigInt::from_unsigned_bytes_be(&is_long)
        .to_string()
        .parse::<i32>()
        .unwrap();
    let mut is_long: bool = false;
    if is_long_num == 1 {
        is_long = true;
    }
    is_long
}

pub fn get_size_in_tokens(chunk: &str) -> f64 {
    let size_in_tokens = Hex::decode(chunk).unwrap();
    let size_in_tokens: substreams::scalar::BigInt =
        substreams::scalar::BigInt::from_unsigned_bytes_be(&size_in_tokens);
    let size_in_tokens = size_in_tokens.to_string().parse::<f64>().unwrap();
    size_in_tokens
}

pub fn get_address(chunk: &str) -> String {
    let address = Hex::decode(chunk).unwrap();
    let address = Hex::encode(&address[12..].to_vec());
    address
}

pub fn get_order_type(chunk: &str) -> i32 {
    let order_type = Hex::decode(chunk).unwrap();
    let order_type: substreams::scalar::BigInt =
        substreams::scalar::BigInt::from_unsigned_bytes_be(&order_type);
    let order_type = order_type.to_string().parse::<i32>().unwrap();
    order_type
}
//}
