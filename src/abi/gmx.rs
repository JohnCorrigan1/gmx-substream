    const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
    /// Contract's functions.
    #[allow(dead_code, unused_imports, unused_variables)]
    pub mod functions {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct EmitDataLog1 {
            pub topic1: [u8; 32usize],
            pub data: Vec<u8>,
        }
        impl EmitDataLog1 {
            const METHOD_ID: [u8; 4] = [249u8, 213u8, 192u8, 234u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    topic1: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::FixedBytes(self.topic1.as_ref().to_vec()),
                        ethabi::Token::Bytes(self.data.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for EmitDataLog1 {
            const NAME: &'static str = "emitDataLog1";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EmitDataLog2 {
            pub topic1: [u8; 32usize],
            pub topic2: [u8; 32usize],
            pub data: Vec<u8>,
        }
        impl EmitDataLog2 {
            const METHOD_ID: [u8; 4] = [221u8, 160u8, 219u8, 50u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    topic1: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    topic2: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::FixedBytes(self.topic1.as_ref().to_vec()),
                        ethabi::Token::FixedBytes(self.topic2.as_ref().to_vec()),
                        ethabi::Token::Bytes(self.data.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for EmitDataLog2 {
            const NAME: &'static str = "emitDataLog2";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EmitDataLog3 {
            pub topic1: [u8; 32usize],
            pub topic2: [u8; 32usize],
            pub topic3: [u8; 32usize],
            pub data: Vec<u8>,
        }
        impl EmitDataLog3 {
            const METHOD_ID: [u8; 4] = [179u8, 172u8, 28u8, 56u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    topic1: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    topic2: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    topic3: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::FixedBytes(self.topic1.as_ref().to_vec()),
                        ethabi::Token::FixedBytes(self.topic2.as_ref().to_vec()),
                        ethabi::Token::FixedBytes(self.topic3.as_ref().to_vec()),
                        ethabi::Token::Bytes(self.data.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for EmitDataLog3 {
            const NAME: &'static str = "emitDataLog3";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EmitDataLog4 {
            pub topic1: [u8; 32usize],
            pub topic2: [u8; 32usize],
            pub topic3: [u8; 32usize],
            pub topic4: [u8; 32usize],
            pub data: Vec<u8>,
        }
        impl EmitDataLog4 {
            const METHOD_ID: [u8; 4] = [238u8, 40u8, 140u8, 232u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    topic1: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    topic2: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    topic3: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    topic4: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::FixedBytes(self.topic1.as_ref().to_vec()),
                        ethabi::Token::FixedBytes(self.topic2.as_ref().to_vec()),
                        ethabi::Token::FixedBytes(self.topic3.as_ref().to_vec()),
                        ethabi::Token::FixedBytes(self.topic4.as_ref().to_vec()),
                        ethabi::Token::Bytes(self.data.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for EmitDataLog4 {
            const NAME: &'static str = "emitDataLog4";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EmitEventLog {
            pub event_name: String,
            pub event_data: (
                (Vec<(String, Vec<u8>)>, Vec<(String, Vec<Vec<u8>>)>),
                (
                    Vec<(String, substreams::scalar::BigInt)>,
                    Vec<(String, Vec<substreams::scalar::BigInt>)>,
                ),
                (
                    Vec<(String, substreams::scalar::BigInt)>,
                    Vec<(String, Vec<substreams::scalar::BigInt>)>,
                ),
                (Vec<(String, bool)>, Vec<(String, Vec<bool>)>),
                (Vec<(String, [u8; 32usize])>, Vec<(String, Vec<[u8; 32usize]>)>),
                (Vec<(String, Vec<u8>)>, Vec<(String, Vec<Vec<u8>>)>),
                (Vec<(String, String)>, Vec<(String, Vec<String>)>),
            ),
        }
        impl EmitEventLog {
            const METHOD_ID: [u8; 4] = [144u8, 108u8, 73u8, 246u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::String,
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Address]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Address))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Uint(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Int(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Int(256usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Bool]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bool))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::FixedBytes(32usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Bytes]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bytes))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::String]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::String))])))])
                                ],
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    event_name: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    event_data: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_address()
                                                    .expect(INTERNAL_ERR)
                                                    .as_bytes()
                                                    .to_vec(),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        inner
                                                            .into_address()
                                                            .expect(INTERNAL_ERR)
                                                            .as_bytes()
                                                            .to_vec()
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[1usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut v = [0 as u8; 32];
                                                        inner
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[2usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_int()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_signed_bytes_be(&v)
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut v = [0 as u8; 32];
                                                        inner
                                                            .into_int()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_signed_bytes_be(&v)
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[3usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_bool()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_bool().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[4usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut result = [0u8; 32];
                                                    let v = tuple_elements[1usize]
                                                        .clone()
                                                        .into_fixed_bytes()
                                                        .expect(INTERNAL_ERR);
                                                    result.copy_from_slice(&v);
                                                    result
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut result = [0u8; 32];
                                                        let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                                        result.copy_from_slice(&v);
                                                        result
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[5usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_bytes()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_bytes().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[6usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_string().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                        )
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::String(self.event_name.clone()),
                        ethabi::Token::Tuple(
                            vec![
                                ethabi::Token::Tuple(vec![{ let v = self.event_data.0.0
                                .iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()),
                                ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                .1))])).collect(); ethabi::Token::Array(v) }, { let v = self
                                .event_data.0.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::Address(ethabi::Address::from_slice(&
                                inner))).collect(); ethabi::Token::Array(v) }])).collect();
                                ethabi::Token::Array(v) }]), ethabi::Token::Tuple(vec![{ let
                                v = self.event_data.1.0.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                let v = self.event_data.1.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)).collect(); ethabi::Token::Array(v) }]))
                                .collect(); ethabi::Token::Array(v) }]),
                                ethabi::Token::Tuple(vec![{ let v = self.event_data.2.0
                                .iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let non_full_signed_bytes = inner.1
                                .to_signed_bytes_be(); let mut full_signed_bytes = [0xff as
                                u8; 32]; non_full_signed_bytes.into_iter().rev().enumerate()
                                .for_each(| (i, byte) | full_signed_bytes[31 - i] = byte);
                                ethabi::Token::Int(ethabi::Int::from_big_endian(full_signed_bytes
                                .as_ref())) }])).collect(); ethabi::Token::Array(v) }, { let
                                v = self.event_data.2.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner | { let
                                non_full_signed_bytes = inner.to_signed_bytes_be(); let mut
                                full_signed_bytes = [0xff as u8; 32]; non_full_signed_bytes
                                .into_iter().rev().enumerate().for_each(| (i, byte) |
                                full_signed_bytes[31 - i] = byte);
                                ethabi::Token::Int(ethabi::Int::from_big_endian(full_signed_bytes
                                .as_ref())) }).collect(); ethabi::Token::Array(v) }]))
                                .collect(); ethabi::Token::Array(v) }]),
                                ethabi::Token::Tuple(vec![{ let v = self.event_data.3.0
                                .iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), ethabi::Token::Bool(inner.1.clone())]))
                                .collect(); ethabi::Token::Array(v) }, { let v = self
                                .event_data.3.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::Bool(inner.clone())).collect();
                                ethabi::Token::Array(v) }])).collect();
                                ethabi::Token::Array(v) }]), ethabi::Token::Tuple(vec![{ let
                                v = self.event_data.4.0.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), ethabi::Token::FixedBytes(inner.1.as_ref()
                                .to_vec())])).collect(); ethabi::Token::Array(v) }, { let v
                                = self.event_data.4.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::FixedBytes(inner.as_ref().to_vec()))
                                .collect(); ethabi::Token::Array(v) }])).collect();
                                ethabi::Token::Array(v) }]), ethabi::Token::Tuple(vec![{ let
                                v = self.event_data.5.0.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), ethabi::Token::Bytes(inner.1.clone())]))
                                .collect(); ethabi::Token::Array(v) }, { let v = self
                                .event_data.5.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::Bytes(inner.clone())).collect();
                                ethabi::Token::Array(v) }])).collect();
                                ethabi::Token::Array(v) }]), ethabi::Token::Tuple(vec![{ let
                                v = self.event_data.6.0.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), ethabi::Token::String(inner.1.clone())]))
                                .collect(); ethabi::Token::Array(v) }, { let v = self
                                .event_data.6.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::String(inner.clone())).collect();
                                ethabi::Token::Array(v) }])).collect();
                                ethabi::Token::Array(v) }])
                            ],
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for EmitEventLog {
            const NAME: &'static str = "emitEventLog";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EmitEventLog1 {
            pub event_name: String,
            pub topic1: [u8; 32usize],
            pub event_data: (
                (Vec<(String, Vec<u8>)>, Vec<(String, Vec<Vec<u8>>)>),
                (
                    Vec<(String, substreams::scalar::BigInt)>,
                    Vec<(String, Vec<substreams::scalar::BigInt>)>,
                ),
                (
                    Vec<(String, substreams::scalar::BigInt)>,
                    Vec<(String, Vec<substreams::scalar::BigInt>)>,
                ),
                (Vec<(String, bool)>, Vec<(String, Vec<bool>)>),
                (Vec<(String, [u8; 32usize])>, Vec<(String, Vec<[u8; 32usize]>)>),
                (Vec<(String, Vec<u8>)>, Vec<(String, Vec<Vec<u8>>)>),
                (Vec<(String, String)>, Vec<(String, Vec<String>)>),
            ),
        }
        impl EmitEventLog1 {
            const METHOD_ID: [u8; 4] = [36u8, 222u8, 1u8, 228u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::String,
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Address]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Address))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Uint(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Int(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Int(256usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Bool]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bool))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::FixedBytes(32usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Bytes]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bytes))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::String]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::String))])))])
                                ],
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    event_name: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    topic1: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    event_data: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_address()
                                                    .expect(INTERNAL_ERR)
                                                    .as_bytes()
                                                    .to_vec(),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        inner
                                                            .into_address()
                                                            .expect(INTERNAL_ERR)
                                                            .as_bytes()
                                                            .to_vec()
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[1usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut v = [0 as u8; 32];
                                                        inner
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[2usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_int()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_signed_bytes_be(&v)
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut v = [0 as u8; 32];
                                                        inner
                                                            .into_int()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_signed_bytes_be(&v)
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[3usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_bool()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_bool().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[4usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut result = [0u8; 32];
                                                    let v = tuple_elements[1usize]
                                                        .clone()
                                                        .into_fixed_bytes()
                                                        .expect(INTERNAL_ERR);
                                                    result.copy_from_slice(&v);
                                                    result
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut result = [0u8; 32];
                                                        let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                                        result.copy_from_slice(&v);
                                                        result
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[5usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_bytes()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_bytes().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[6usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_string().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                        )
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::String(self.event_name.clone()),
                        ethabi::Token::FixedBytes(self.topic1.as_ref().to_vec()),
                        ethabi::Token::Tuple(
                            vec![
                                ethabi::Token::Tuple(vec![{ let v = self.event_data.0.0
                                .iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()),
                                ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                .1))])).collect(); ethabi::Token::Array(v) }, { let v = self
                                .event_data.0.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::Address(ethabi::Address::from_slice(&
                                inner))).collect(); ethabi::Token::Array(v) }])).collect();
                                ethabi::Token::Array(v) }]), ethabi::Token::Tuple(vec![{ let
                                v = self.event_data.1.0.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                let v = self.event_data.1.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)).collect(); ethabi::Token::Array(v) }]))
                                .collect(); ethabi::Token::Array(v) }]),
                                ethabi::Token::Tuple(vec![{ let v = self.event_data.2.0
                                .iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let non_full_signed_bytes = inner.1
                                .to_signed_bytes_be(); let mut full_signed_bytes = [0xff as
                                u8; 32]; non_full_signed_bytes.into_iter().rev().enumerate()
                                .for_each(| (i, byte) | full_signed_bytes[31 - i] = byte);
                                ethabi::Token::Int(ethabi::Int::from_big_endian(full_signed_bytes
                                .as_ref())) }])).collect(); ethabi::Token::Array(v) }, { let
                                v = self.event_data.2.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner | { let
                                non_full_signed_bytes = inner.to_signed_bytes_be(); let mut
                                full_signed_bytes = [0xff as u8; 32]; non_full_signed_bytes
                                .into_iter().rev().enumerate().for_each(| (i, byte) |
                                full_signed_bytes[31 - i] = byte);
                                ethabi::Token::Int(ethabi::Int::from_big_endian(full_signed_bytes
                                .as_ref())) }).collect(); ethabi::Token::Array(v) }]))
                                .collect(); ethabi::Token::Array(v) }]),
                                ethabi::Token::Tuple(vec![{ let v = self.event_data.3.0
                                .iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), ethabi::Token::Bool(inner.1.clone())]))
                                .collect(); ethabi::Token::Array(v) }, { let v = self
                                .event_data.3.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::Bool(inner.clone())).collect();
                                ethabi::Token::Array(v) }])).collect();
                                ethabi::Token::Array(v) }]), ethabi::Token::Tuple(vec![{ let
                                v = self.event_data.4.0.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), ethabi::Token::FixedBytes(inner.1.as_ref()
                                .to_vec())])).collect(); ethabi::Token::Array(v) }, { let v
                                = self.event_data.4.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::FixedBytes(inner.as_ref().to_vec()))
                                .collect(); ethabi::Token::Array(v) }])).collect();
                                ethabi::Token::Array(v) }]), ethabi::Token::Tuple(vec![{ let
                                v = self.event_data.5.0.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), ethabi::Token::Bytes(inner.1.clone())]))
                                .collect(); ethabi::Token::Array(v) }, { let v = self
                                .event_data.5.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::Bytes(inner.clone())).collect();
                                ethabi::Token::Array(v) }])).collect();
                                ethabi::Token::Array(v) }]), ethabi::Token::Tuple(vec![{ let
                                v = self.event_data.6.0.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), ethabi::Token::String(inner.1.clone())]))
                                .collect(); ethabi::Token::Array(v) }, { let v = self
                                .event_data.6.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::String(inner.clone())).collect();
                                ethabi::Token::Array(v) }])).collect();
                                ethabi::Token::Array(v) }])
                            ],
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for EmitEventLog1 {
            const NAME: &'static str = "emitEventLog1";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EmitEventLog2 {
            pub event_name: String,
            pub topic1: [u8; 32usize],
            pub topic2: [u8; 32usize],
            pub event_data: (
                (Vec<(String, Vec<u8>)>, Vec<(String, Vec<Vec<u8>>)>),
                (
                    Vec<(String, substreams::scalar::BigInt)>,
                    Vec<(String, Vec<substreams::scalar::BigInt>)>,
                ),
                (
                    Vec<(String, substreams::scalar::BigInt)>,
                    Vec<(String, Vec<substreams::scalar::BigInt>)>,
                ),
                (Vec<(String, bool)>, Vec<(String, Vec<bool>)>),
                (Vec<(String, [u8; 32usize])>, Vec<(String, Vec<[u8; 32usize]>)>),
                (Vec<(String, Vec<u8>)>, Vec<(String, Vec<Vec<u8>>)>),
                (Vec<(String, String)>, Vec<(String, Vec<String>)>),
            ),
        }
        impl EmitEventLog2 {
            const METHOD_ID: [u8; 4] = [99u8, 209u8, 99u8, 99u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::String,
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Address]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Address))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Uint(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Int(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Int(256usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Bool]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bool))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::FixedBytes(32usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Bytes]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bytes))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::String]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::String))])))])
                                ],
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    event_name: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    topic1: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    topic2: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    event_data: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_address()
                                                    .expect(INTERNAL_ERR)
                                                    .as_bytes()
                                                    .to_vec(),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        inner
                                                            .into_address()
                                                            .expect(INTERNAL_ERR)
                                                            .as_bytes()
                                                            .to_vec()
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[1usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut v = [0 as u8; 32];
                                                        inner
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[2usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_int()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_signed_bytes_be(&v)
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut v = [0 as u8; 32];
                                                        inner
                                                            .into_int()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_signed_bytes_be(&v)
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[3usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_bool()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_bool().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[4usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut result = [0u8; 32];
                                                    let v = tuple_elements[1usize]
                                                        .clone()
                                                        .into_fixed_bytes()
                                                        .expect(INTERNAL_ERR);
                                                    result.copy_from_slice(&v);
                                                    result
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut result = [0u8; 32];
                                                        let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                                        result.copy_from_slice(&v);
                                                        result
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[5usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_bytes()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_bytes().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[6usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_string().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                        )
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::String(self.event_name.clone()),
                        ethabi::Token::FixedBytes(self.topic1.as_ref().to_vec()),
                        ethabi::Token::FixedBytes(self.topic2.as_ref().to_vec()),
                        ethabi::Token::Tuple(
                            vec![
                                ethabi::Token::Tuple(vec![{ let v = self.event_data.0.0
                                .iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()),
                                ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                .1))])).collect(); ethabi::Token::Array(v) }, { let v = self
                                .event_data.0.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::Address(ethabi::Address::from_slice(&
                                inner))).collect(); ethabi::Token::Array(v) }])).collect();
                                ethabi::Token::Array(v) }]), ethabi::Token::Tuple(vec![{ let
                                v = self.event_data.1.0.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                let v = self.event_data.1.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)).collect(); ethabi::Token::Array(v) }]))
                                .collect(); ethabi::Token::Array(v) }]),
                                ethabi::Token::Tuple(vec![{ let v = self.event_data.2.0
                                .iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let non_full_signed_bytes = inner.1
                                .to_signed_bytes_be(); let mut full_signed_bytes = [0xff as
                                u8; 32]; non_full_signed_bytes.into_iter().rev().enumerate()
                                .for_each(| (i, byte) | full_signed_bytes[31 - i] = byte);
                                ethabi::Token::Int(ethabi::Int::from_big_endian(full_signed_bytes
                                .as_ref())) }])).collect(); ethabi::Token::Array(v) }, { let
                                v = self.event_data.2.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner | { let
                                non_full_signed_bytes = inner.to_signed_bytes_be(); let mut
                                full_signed_bytes = [0xff as u8; 32]; non_full_signed_bytes
                                .into_iter().rev().enumerate().for_each(| (i, byte) |
                                full_signed_bytes[31 - i] = byte);
                                ethabi::Token::Int(ethabi::Int::from_big_endian(full_signed_bytes
                                .as_ref())) }).collect(); ethabi::Token::Array(v) }]))
                                .collect(); ethabi::Token::Array(v) }]),
                                ethabi::Token::Tuple(vec![{ let v = self.event_data.3.0
                                .iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), ethabi::Token::Bool(inner.1.clone())]))
                                .collect(); ethabi::Token::Array(v) }, { let v = self
                                .event_data.3.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::Bool(inner.clone())).collect();
                                ethabi::Token::Array(v) }])).collect();
                                ethabi::Token::Array(v) }]), ethabi::Token::Tuple(vec![{ let
                                v = self.event_data.4.0.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), ethabi::Token::FixedBytes(inner.1.as_ref()
                                .to_vec())])).collect(); ethabi::Token::Array(v) }, { let v
                                = self.event_data.4.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::FixedBytes(inner.as_ref().to_vec()))
                                .collect(); ethabi::Token::Array(v) }])).collect();
                                ethabi::Token::Array(v) }]), ethabi::Token::Tuple(vec![{ let
                                v = self.event_data.5.0.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), ethabi::Token::Bytes(inner.1.clone())]))
                                .collect(); ethabi::Token::Array(v) }, { let v = self
                                .event_data.5.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::Bytes(inner.clone())).collect();
                                ethabi::Token::Array(v) }])).collect();
                                ethabi::Token::Array(v) }]), ethabi::Token::Tuple(vec![{ let
                                v = self.event_data.6.0.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), ethabi::Token::String(inner.1.clone())]))
                                .collect(); ethabi::Token::Array(v) }, { let v = self
                                .event_data.6.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::String(inner.0
                                .clone()), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::String(inner.clone())).collect();
                                ethabi::Token::Array(v) }])).collect();
                                ethabi::Token::Array(v) }])
                            ],
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for EmitEventLog2 {
            const NAME: &'static str = "emitEventLog2";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct RoleStore {}
        impl RoleStore {
            const METHOD_ID: [u8; 4] = [74u8, 74u8, 123u8, 4u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<u8>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for RoleStore {
            const NAME: &'static str = "roleStore";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for RoleStore {
            fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                Self::output(data)
            }
        }
    }
    /// Contract's events.
    #[allow(dead_code, unused_imports, unused_variables)]
    pub mod events {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct EventLog {
            pub msg_sender: Vec<u8>,
            pub event_name: String,
            pub event_name_hash: String,
            pub event_data: (
                (Vec<(String, Vec<u8>)>, Vec<(String, Vec<Vec<u8>>)>),
                (
                    Vec<(String, substreams::scalar::BigInt)>,
                    Vec<(String, Vec<substreams::scalar::BigInt>)>,
                ),
                (
                    Vec<(String, substreams::scalar::BigInt)>,
                    Vec<(String, Vec<substreams::scalar::BigInt>)>,
                ),
                (Vec<(String, bool)>, Vec<(String, Vec<bool>)>),
                (Vec<(String, [u8; 32usize])>, Vec<(String, Vec<[u8; 32usize]>)>),
                (Vec<(String, Vec<u8>)>, Vec<(String, Vec<Vec<u8>>)>),
                (Vec<(String, String)>, Vec<(String, Vec<String>)>),
            ),
        }
        impl EventLog {
            const TOPIC_ID: [u8; 32] = [
                126u8,
                59u8,
                222u8,
                43u8,
                167u8,
                172u8,
                164u8,
                168u8,
                73u8,
                150u8,
                8u8,
                202u8,
                87u8,
                243u8,
                176u8,
                193u8,
                193u8,
                201u8,
                58u8,
                206u8,
                99u8,
                255u8,
                211u8,
                116u8,
                26u8,
                159u8,
                171u8,
                32u8,
                65u8,
                70u8,
                252u8,
                154u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 992usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::String,
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Address]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Address))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Uint(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Int(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Int(256usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Bool]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bool))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::FixedBytes(32usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Bytes]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bytes))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::String]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::String))])))])
                                ],
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    event_name_hash: ethabi::decode(
                            &[ethabi::ParamType::String],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'event_name_hash' from topic of type 'string': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    msg_sender: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    event_name: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    event_data: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_address()
                                                    .expect(INTERNAL_ERR)
                                                    .as_bytes()
                                                    .to_vec(),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        inner
                                                            .into_address()
                                                            .expect(INTERNAL_ERR)
                                                            .as_bytes()
                                                            .to_vec()
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[1usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut v = [0 as u8; 32];
                                                        inner
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[2usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_int()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_signed_bytes_be(&v)
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut v = [0 as u8; 32];
                                                        inner
                                                            .into_int()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_signed_bytes_be(&v)
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[3usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_bool()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_bool().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[4usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut result = [0u8; 32];
                                                    let v = tuple_elements[1usize]
                                                        .clone()
                                                        .into_fixed_bytes()
                                                        .expect(INTERNAL_ERR);
                                                    result.copy_from_slice(&v);
                                                    result
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut result = [0u8; 32];
                                                        let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                                        result.copy_from_slice(&v);
                                                        result
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[5usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_bytes()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_bytes().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[6usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_string().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                        )
                    },
                })
            }
        }
        impl substreams_ethereum::Event for EventLog {
            const NAME: &'static str = "EventLog";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EventLog1 {
            pub msg_sender: Vec<u8>,
            pub event_name: String,
            pub event_name_hash: String,
            pub topic1: [u8; 32usize],
            pub event_data: (
                (Vec<(String, Vec<u8>)>, Vec<(String, Vec<Vec<u8>>)>),
                (
                    Vec<(String, substreams::scalar::BigInt)>,
                    Vec<(String, Vec<substreams::scalar::BigInt>)>,
                ),
                (
                    Vec<(String, substreams::scalar::BigInt)>,
                    Vec<(String, Vec<substreams::scalar::BigInt>)>,
                ),
                (Vec<(String, bool)>, Vec<(String, Vec<bool>)>),
                (Vec<(String, [u8; 32usize])>, Vec<(String, Vec<[u8; 32usize]>)>),
                (Vec<(String, Vec<u8>)>, Vec<(String, Vec<Vec<u8>>)>),
                (Vec<(String, String)>, Vec<(String, Vec<String>)>),
            ),
        }
        impl EventLog1 {
            const TOPIC_ID: [u8; 32] = [
                19u8,
                122u8,
                68u8,
                6u8,
                124u8,
                137u8,
                97u8,
                205u8,
                126u8,
                29u8,
                135u8,
                111u8,
                71u8,
                84u8,
                165u8,
                163u8,
                167u8,
                89u8,
                137u8,
                180u8,
                85u8,
                47u8,
                24u8,
                67u8,
                252u8,
                105u8,
                195u8,
                179u8,
                114u8,
                222u8,
                241u8,
                96u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() < 992usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::String,
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Address]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Address))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Uint(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Int(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Int(256usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Bool]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bool))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::FixedBytes(32usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Bytes]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bytes))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::String]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::String))])))])
                                ],
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    event_name_hash: ethabi::decode(
                            &[ethabi::ParamType::String],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'event_name_hash' from topic of type 'string': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    topic1: {
                        let mut result = [0u8; 32];
                        let v = ethabi::decode(
                                &[ethabi::ParamType::FixedBytes(32usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'topic1' from topic of type 'bytes32': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    msg_sender: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    event_name: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    event_data: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_address()
                                                    .expect(INTERNAL_ERR)
                                                    .as_bytes()
                                                    .to_vec(),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        inner
                                                            .into_address()
                                                            .expect(INTERNAL_ERR)
                                                            .as_bytes()
                                                            .to_vec()
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[1usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut v = [0 as u8; 32];
                                                        inner
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[2usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_int()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_signed_bytes_be(&v)
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut v = [0 as u8; 32];
                                                        inner
                                                            .into_int()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_signed_bytes_be(&v)
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[3usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_bool()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_bool().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[4usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut result = [0u8; 32];
                                                    let v = tuple_elements[1usize]
                                                        .clone()
                                                        .into_fixed_bytes()
                                                        .expect(INTERNAL_ERR);
                                                    result.copy_from_slice(&v);
                                                    result
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut result = [0u8; 32];
                                                        let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                                        result.copy_from_slice(&v);
                                                        result
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[5usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_bytes()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_bytes().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[6usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_string().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                        )
                    },
                })
            }
        }
        impl substreams_ethereum::Event for EventLog1 {
            const NAME: &'static str = "EventLog1";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EventLog2 {
            pub msg_sender: Vec<u8>,
            pub event_name: String,
            pub event_name_hash: String,
            pub topic1: [u8; 32usize],
            pub topic2: [u8; 32usize],
            pub event_data: (
                (Vec<(String, Vec<u8>)>, Vec<(String, Vec<Vec<u8>>)>),
                (
                    Vec<(String, substreams::scalar::BigInt)>,
                    Vec<(String, Vec<substreams::scalar::BigInt>)>,
                ),
                (
                    Vec<(String, substreams::scalar::BigInt)>,
                    Vec<(String, Vec<substreams::scalar::BigInt>)>,
                ),
                (Vec<(String, bool)>, Vec<(String, Vec<bool>)>),
                (Vec<(String, [u8; 32usize])>, Vec<(String, Vec<[u8; 32usize]>)>),
                (Vec<(String, Vec<u8>)>, Vec<(String, Vec<Vec<u8>>)>),
                (Vec<(String, String)>, Vec<(String, Vec<String>)>),
            ),
        }
        impl EventLog2 {
            const TOPIC_ID: [u8; 32] = [
                70u8,
                138u8,
                37u8,
                167u8,
                186u8,
                98u8,
                76u8,
                238u8,
                166u8,
                229u8,
                64u8,
                173u8,
                111u8,
                73u8,
                23u8,
                27u8,
                82u8,
                73u8,
                91u8,
                100u8,
                132u8,
                23u8,
                174u8,
                145u8,
                188u8,
                162u8,
                22u8,
                118u8,
                216u8,
                162u8,
                77u8,
                197u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() < 992usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::String,
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Address]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Address))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Uint(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Int(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Int(256usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Bool]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bool))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::FixedBytes(32usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize)))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Bytes]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bytes))])))]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::String]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::String))])))])
                                ],
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    event_name_hash: ethabi::decode(
                            &[ethabi::ParamType::String],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'event_name_hash' from topic of type 'string': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    topic1: {
                        let mut result = [0u8; 32];
                        let v = ethabi::decode(
                                &[ethabi::ParamType::FixedBytes(32usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'topic1' from topic of type 'bytes32': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    topic2: {
                        let mut result = [0u8; 32];
                        let v = ethabi::decode(
                                &[ethabi::ParamType::FixedBytes(32usize)],
                                log.topics[3usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'topic2' from topic of type 'bytes32': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    msg_sender: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    event_name: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    event_data: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_address()
                                                    .expect(INTERNAL_ERR)
                                                    .as_bytes()
                                                    .to_vec(),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        inner
                                                            .into_address()
                                                            .expect(INTERNAL_ERR)
                                                            .as_bytes()
                                                            .to_vec()
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[1usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut v = [0 as u8; 32];
                                                        inner
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[2usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_int()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_signed_bytes_be(&v)
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut v = [0 as u8; 32];
                                                        inner
                                                            .into_int()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_signed_bytes_be(&v)
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[3usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_bool()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_bool().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[4usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                {
                                                    let mut result = [0u8; 32];
                                                    let v = tuple_elements[1usize]
                                                        .clone()
                                                        .into_fixed_bytes()
                                                        .expect(INTERNAL_ERR);
                                                    result.copy_from_slice(&v);
                                                    result
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut result = [0u8; 32];
                                                        let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                                        result.copy_from_slice(&v);
                                                        result
                                                    })
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[5usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_bytes()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_bytes().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[6usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_string()
                                                    .expect(INTERNAL_ERR),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| inner.into_string().expect(INTERNAL_ERR))
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            },
                        )
                    },
                })
            }
        }
        impl substreams_ethereum::Event for EventLog2 {
            const NAME: &'static str = "EventLog2";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
    }