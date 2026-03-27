use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct AbiRoot {
    abi: Vec<AbiItem>,
}

#[derive(Deserialize, Debug)]
struct AbiItem {
    #[serde(rename = "type")]
    item_type: String,
    name: Option<String>,
    inputs: Option<Vec<AbiParameter>>,
    outputs: Option<Vec<AbiParameter>>,
    #[serde(rename = "stateMutability")]
    state_mutability: Option<String>,
}

#[derive(Deserialize, Debug)]
struct AbiParameter {
    name: String,
    #[serde(rename = "type")]
    param_type: String,
    #[serde(rename = "internalType")]
    internal_type: Option<String>,
}
