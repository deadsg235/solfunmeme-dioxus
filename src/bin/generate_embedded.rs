use walkdir::WalkDir;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};

fn main() {
    let roots = vec!["src", "vendor", "founding_docs"];
    let generated_dir = Path::new("src/generated");
    let mut used_struct_names = HashSet::new(); // Track used struct names globally
    let mut module_name_counts = HashMap::new(); // Track module name occurrences
    let mut module_names = Vec::new(); // Track module names for mod.rs

    // Create the generated directory if it doesn't exist
    fs::create_dir_all(generated_dir).unwrap();

    // Process each root and its subdirectories
    for root in roots {
        for entry in WalkDir::new(root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_dir())
        {
            let path = entry.path();

            // Use / as separator in the folder attribute
            let folder_path = path.to_string_lossy().replace("\\", "/");

            // CamelCase struct name (removes - and _)
            let base_struct_name = format!(
                "Our{}Extractor",
                path_to_camel_case(path).to_string()
            );

            // Sanitize and resolve struct name conflicts with counter
            let struct_name = sanitize_struct_name(&base_struct_name, &mut used_struct_names);

            // Generate unique module name with counter for conflicts
            let module_name = path_to_module_name(path, &mut module_name_counts);

            // Store module name for mod.rs
            module_names.push(module_name.clone());

            // Generate module content
            let mut output = String::new();
            output.push_str("use rust_embed::Embed;\n\n");
            output.push_str(&format!(
                "#[derive(Embed)]\n#[folder = \"{folder}\"]\npub struct {struct_name};\n",
                folder = folder_path,
                struct_name = struct_name
            ));

            // Write to the module file
            let module_file = generated_dir.join(format!("{}.rs", module_name));
            let mut f = File::create(&module_file).unwrap();
            f.write_all(output.as_bytes()).unwrap();
        }
    }

    // Generate mod.rs to re-export all modules
    let mod_rs_path = generated_dir.join("mod.rs");
    let mut mod_rs = File::create(&mod_rs_path).unwrap();
    let mut mod_rs_content = String::new();
    for module_name in module_names {
        mod_rs_content.push_str(&format!("pub mod {};\n", module_name));
    }
    mod_rs.write_all(mod_rs_content.as_bytes()).unwrap();
}

fn path_to_camel_case(path: &std::path::Path) -> String {
    path.iter()
        .map(|os_str| {
            let s = os_str.to_string_lossy();
            s.split(|c: char| c == '-' || c == '_' || c == '.')
                .filter(|part| !part.is_empty())
                .map(|part| {
                    let mut chars = part.chars();
                    match chars.next() {
                        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                        None => String::new(),
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("")
}

// Sanitize struct name and append counter if name is already used
fn sanitize_struct_name(s: &str, used_names: &mut HashSet<String>) -> String {
    let base_name: String = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();

    let mut final_name = base_name.clone();
    let mut counter = 0;

    // Keep appending counter until the name is unique
    while !used_names.insert(final_name.clone()) {
        counter += 1;
        final_name = format!("{}{}", base_name, counter);
    }

    final_name
}

// Convert path to a valid module name, appending counter if necessary
fn path_to_module_name(path: &Path, module_name_counts: &mut HashMap<String, u32>) -> String {
    let base_name: String = path
        .iter()
        .map(|os_str| os_str.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("_")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();

    // Increment counter for this base_name
    let counter = module_name_counts.entry(base_name.clone()).or_insert(0);
    *counter += 1;

    // If counter is 1, use base_name; otherwise, append counter-1
    if *counter == 1 {
        base_name
    } else {
        format!("{}{}", base_name, *counter - 1)
    }
}
