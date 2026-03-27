pub mod dtos;
pub mod sol_to_ts_mapper;

use crate::utils::dtos::AbiRoot;
use crate::utils::sol_to_ts_mapper::map_solidity_to_ts;

pub fn generate_typescript_content(contract_name: &str, abi_root: AbiRoot) -> String {
    let mut ts_content = format!("export type {} = {{\n", contract_name);

    for item in abi_root.abi {
        if item.item_type == "function" {
            let name = item.name.unwrap_or_else(|| "unknown".to_string());

            // Map inputs: (arg1: type, arg2: type)
            let inputs = item
                .inputs
                .as_ref()
                .map(|params| {
                    params
                        .iter()
                        .enumerate()
                        .map(|(i, input)| {
                            let param_name = if input.name.is_empty() {
                                format!("arg{}", i)
                            } else {
                                input.name.clone()
                            };
                            format!("{}: {}", param_name, map_solidity_to_ts(&input.param_type))
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_default();

            // Map outputs: Promise<type> or Promise<[type1, type2]>
            let outputs = match item.outputs {
                Some(ref out) if out.is_empty() => "void".to_string(),
                Some(ref out) if out.len() == 1 => map_solidity_to_ts(&out[0].param_type),
                Some(ref out) => {
                    let tuple = out
                        .iter()
                        .map(|o| map_solidity_to_ts(&o.param_type))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("[{}]", tuple)
                }
                None => "void".to_string(),
            };

            ts_content.push_str(&format!("  {}({}): Promise<{}>;\n", name, inputs, outputs));
        }
    }

    ts_content.push_str("}\n");
    ts_content
}

// Utility function to write the generated TypeScript content to a file
pub fn write_typescript_file(output_name: &str, content: String) -> std::io::Result<()> {
    // Directory name for output.
    let output_dir = "types";

    // Create the output directory if it doesn't exist
    std::fs::create_dir_all(output_dir)?;

    // check if output_name ends with .ts, if not append it
    let output_name = if output_name.ends_with(".ts") {
        output_name.to_string()
    } else {
        format!("{}.ts", output_name)
    };

    // Construct the full output path
    let output_path = std::path::Path::new(output_dir).join(&output_name);

    // check if file already exists inside the output directory, if so, remove it
    if output_path.exists() {
        std::fs::remove_file(&output_path)?;
    }

    // write the content to the file
    std::fs::write(output_path, content)
}
