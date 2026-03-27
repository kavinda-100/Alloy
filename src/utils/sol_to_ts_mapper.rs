/// Map Solidity types to TypeScript types for better type safety in generated code.
pub fn map_solidity_to_ts(sol_type: &str) -> String {
    if sol_type.contains("uint") || sol_type.contains("int") {
        "bigint".to_string() // Safer for blockchain numbers
    } else if sol_type == "bool" {
        "boolean".to_string()
    } else if sol_type == "string" || sol_type.starts_with("bytes") {
        "string".to_string()
    } else if sol_type == "address" {
        "`0x${string}`".to_string() // TypeScript template literal type for Ethereum addresses
    } else if sol_type.ends_with("[]") {
        let base_type = &sol_type[..sol_type.len() - 2];
        format!("{}[]", map_solidity_to_ts(base_type))
    } else {
        "any".to_string()
    }
}
