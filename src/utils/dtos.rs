use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AbiRoot {
    pub abi: Vec<AbiItem>,
}

#[derive(Deserialize, Debug)]
pub struct AbiItem {
    #[serde(rename = "type")]
    pub item_type: String,
    pub name: Option<String>,
    pub inputs: Option<Vec<AbiParameter>>,
    pub outputs: Option<Vec<AbiParameter>>,
    #[serde(rename = "stateMutability")]
    pub state_mutability: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AbiParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    #[serde(rename = "internalType")]
    pub internal_type: Option<String>,
}
