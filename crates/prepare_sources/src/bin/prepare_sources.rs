use walkdir::WalkDir;
use std::fs;
use std::collections::HashMap;
use solfunmeme_core_logic::code_analyzer::CodeAnalyzer;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::BTreeMap;
use regex::Regex;
use std::io::Write;

/**
idea : lets build a mini compiler right here
first filter out only rust files.
refactor this code into parts.
read the source
parse to ast
embed source into vector
report and identify duplicates

construction of functions to match struture of the code and 
reporting on the matches
analysis of the of the statistics of the matches
construction of new functions based on the statistics
construction of new functions based on the matches
construction of new functions based on the analysis
construction of new functions based on the reporting
construction of new functions based on the extraction
construction of new functions based on the mathematical model
construction of new functions based on the export
a recursive feedback loop using LLMS and statistics to dynamically create new functions and models in real time.


extraction of terms from identifiers
extraction of relcationships between terms
creation of mathematical mmodel

export data for gui (reduce the size)

 */

// --- AST Node Type Emoji Mapping (updated & deduplicated) ---
const EMOJI_TYPE_MAP: &[(&str, &str, &str)] = &[
    // Rust Core
    ("fn", "🦀⚙️", "Rust Core"),
    ("struct", "🏛️🧱", "Rust Core"),
    ("enum", "🎲", "Rust Core"),
    ("mod", "📦", "Rust Core"),
    ("use", "🔗", "Rust Core"),
    ("impl", "🔨", "Rust Core"),
    ("trait", "🧩", "Rust Core"),
    ("const", "🔒", "Rust Core"),
    ("static", "🪨", "Rust Core"),
    ("type", "🏷️", "Rust Core"),
    ("ident", "🆔", "Rust Core"),
    ("attrs", "🎨", "Rust Core"),
    ("fields", "🌱", "Rust Core"),
    ("meta", "🧠", "Rust Core"),
    ("path", "🛤️", "Rust Core"),
    ("lit", "💡", "Rust Core"),
    ("tokens", "🎟️", "Rust Core"),
    ("expr", "🧮", "Rust Core"),
    ("block", "🧱", "Rust Core"),
    ("call", "📞", "Rust Core"),
    ("method", "🔧", "Rust Core"),
    ("macro", "🪄", "Rust Core"),
    ("trait_object", "🦋", "Rust Core"),
    ("item", "📜", "Rust Core"),
    ("items", "📚", "Rust Core"),
    ("field", "🌿", "Rust Core"),
    ("inputs", "➡️", "Rust Core"),
    ("output", "⬅️", "Rust Core"),
    ("receiver", "📡", "Rust Core"),
    ("generics", "🔣", "Rust Core"),
    ("lifetime", "⏳", "Rust Core"),
    ("where_clause", "❓", "Rust Core"),
    ("tuple", "🤝", "Rust Core"),
    ("tuple_struct", "🏗️", "Rust Core"),
    ("array", "🔢", "Rust Core"),
    ("int", "#️⃣", "Rust Core"),
    ("float", "💧", "Rust Core"),
    ("bool", "✅", "Rust Core"),
    ("char", "🔤", "Rust Core"),
    ("str", "📝", "Rust Core"),
    ("closure", "🕸️", "Rust Core"),
    ("let", "📌", "Rust Core"),
    ("match", "🎯", "Rust Core"),
    ("if", "❓", "Rust Core"),
    ("else_branch", "🔄", "Rust Core"),
    ("then_branch", "➡️", "Rust Core"),
    ("for_loop", "🔁", "Rust Core"),
    ("while", "🔂", "Rust Core"),
    ("loop", "♾️", "Rust Core"),
    ("return", "↩️", "Rust Core"),
    ("break", "⛔", "Rust Core"),
    ("continue", "▶️", "Rust Core"),
    ("assign", "📝", "Rust Core"),
    ("op", "⚙️", "Rust Core"),
    ("unary", "➖", "Rust Core"),
    ("binary", "➗", "Rust Core"),
    ("cast", "🔀", "Rust Core"),
    ("index", "📍", "Rust Core"),
    ("range", "↔️", "Rust Core"),
    ("slice", "🍰", "Rust Core"),
    ("macro_rules", "📐", "Rust Core"),
    ("group", "👥", "Rust Core"),
    ("delim", "🚧", "Rust Core"),
    ("punct", "‼️", "Rust Core"),
    ("paren", "( )", "Rust Core"),
    ("bracket", "[ ]", "Rust Core"),
    ("brace", "{ }", "Rust Core"),
    ("attr", "🖼️", "Rust Core"),
    ("name_value", "🔑", "Rust Core"),
    ("value", "💎", "Rust Core"),
    ("style", "🎨", "Rust Core"),
    ("method_call", "📲", "Rust Core"),
    ("dyn", "🌀", "Rust Core"),
    ("mut", "🔄", "Rust Core"),
    ("ref", "🔗", "Rust Core"),
    ("self_ty", "🆔", "Rust Core"),
    ("super", "🌟", "Rust Core"),
    ("crate", "🚚", "Rust Core"),
    ("macro_input", "📥", "Rust Core"),
    ("macro_output", "📦", "Rust Core"),
    ("params", "⚙️", "Rust Core"),
    ("args", "📢", "Rust Core"),
    ("arguments", "🎙️", "Rust Core"),
    ("arm", "🛡️", "Rust Core"),
    ("arms", "🛠️", "Rust Core"),
    ("variant", "🎭", "Rust Core"),
    ("variants", "🔣", "Rust Core"),
    ("fields_named", "🏷️", "Rust Core"),
    ("fields_unnamed", "🌿", "Rust Core"),
    ("pat", "🖼️", "Rust Core"),
    ("stmt", "🖋️", "Rust Core"),
    ("stmts", "📜", "Rust Core"),
    ("ty", "🔖", "Rust Core"),
    ("bound", "⛓️", "Rust Core"),
    ("bounds", "🔗", "Rust Core"),
    ("vis", "👀", "Rust Core"),
    ("list", "✅", "Rust Core"),
    ("token", "🎟️", "Rust Core"),
    ("tree", "🌳", "Rust Core"),
    ("segment", "🧩", "Rust Core"),
    ("segments", "🧩", "Rust Core"),
    ("assoc_type", "🔗", "Rust Core"),
    ("async", "⏩", "Rust Core"),
    ("await", "⏳", "Rust Core"),
    ("base", "🏁", "Rust Core"),
    ("body", "🏃", "Rust Core"),
    ("colon_token", ":", "Rust Core"),
    ("delimiter", "🚧", "Rust Core"),
    ("angle_bracketed", "⟨⟩", "Rust Core"),
    ("cond", "❓", "Rust Core"),
    ("func", "🦀", "Rust Core"),
    ("init", "🚦", "Rust Core"),
    ("right", "👉", "Rust Core"),
    ("semi", ";", "Rust Core"),
    ("semi_token", ";", "Rust Core"),
    ("spacing", "↔️", "Rust Core"),
    ("start", "🔜", "Rust Core"),
    ("stream", "🌊", "Rust Core"),
    ("try", "🤞", "Rust Core"),
    ("bare_fn", "🦀", "Rust Core"),
    ("bounded_ty", "📏", "Rust Core"),
    ("byte_str", "💾", "Rust Core"),
    ("cases", "🎭", "Rust Core"),
    ("dot2_token", "•", "Rust Core"),
    ("elem", "📦", "Rust Core"),
    ("elems", "📦", "Rust Core"),
    ("end", "🔚", "Rust Core"),
    ("impl_trait", "🧩", "Rust Core"),
    ("left", "👈", "Rust Core"),
    ("len", "📏", "Rust Core"),
    ("limits", "📏", "Rust Core"),
    ("move", "🚚", "Rust Core"),
    ("named", "🏷️", "Rust Core"),
    ("or", "🔀", "Rust Core"),
    ("parenthesized", "( )", "Rust Core"),
    ("reference", "🔗", "Rust Core"),
    ("rename", "📝", "Rust Core"),
    ("repeat", "🔁", "Rust Core"),
    ("rest", "🔁", "Rust Core"),
    ("restricted", "🚫", "Rust Core"),
    ("turbofish", "🐟", "Rust Core"),
    ("typed", "🏷️", "Rust Core"),
    ("unnamed", "🏷️", "Rust Core"),
    ("unsafe", "☢️", "Rust Core"),
    // Web/CSS
    ("px", "📏", "Web/CSS"), ("deg", "🧭", "Web/CSS"), ("em", "🔠", "Web/CSS"), ("rem", "🔡", "Web/CSS"), ("vh", "📐", "Web/CSS"), ("vw", "📏", "Web/CSS"), ("s", "⏱️", "Web/CSS"), ("ms", "⏲️", "Web/CSS"),
    ("animation", "🎞️", "Web/CSS"), ("transition", "🔄", "Web/CSS"), ("absolute", "📐", "Web/CSS"), ("align", "📏", "Web/CSS"), ("app", "📱", "Web/CSS"), ("app_state", "🗄️", "Web/CSS"), ("accessibility", "♿", "Web/CSS"),
    ("adapter", "🔌", "Web/CSS"), ("actions", "🎬", "Web/CSS"), ("action", "🎬", "Web/CSS"), ("active", "🔥", "Web/CSS"),
    // Crypto/Security/Systems
    ("aead", "🔒", "Crypto"), ("aeads", "🔒", "Crypto"), ("aes", "🔑", "Crypto"), ("argon2", "🧂", "Crypto"), ("arc", "🧲", "Crypto"), ("addr2line", "📍", "Crypto"), ("aarch64", "📦", "Crypto"), ("amd64", "💻", "Crypto"), ("armv8", "💪", "Crypto"),
    ("crypto", "🔒", "Crypto"), ("curve25519", "➰", "Crypto"), ("ed25519", "📝", "Crypto"), ("elliptic", "➰", "Crypto"), ("fiat", "💵", "Crypto"), ("cbor", "📦", "Crypto"),
    // Project-specific
    ("agave", "🌵", "Project-Specific"), ("helius", "🌞", "Project-Specific"),
    // Internationalization
    ("icu4x", "🌐", "Internationalization"), ("cldr", "🌍", "Internationalization"), ("chinese", "🀄", "Internationalization"), ("hebrew", "✡️", "Internationalization"), ("coptic", "⛪", "Internationalization"), ("ethiopic", "🌄", "Internationalization"), ("calendar", "📅", "Internationalization"), ("datetime", "⏰", "Internationalization"),
    // Testing/Benchmarking
    ("criterion", "⏱️", "Testing"), ("benches", "🏋️", "Testing"), ("fuzz", "🧪", "Testing"), ("examples", "📚", "Testing"), ("docs", "📖", "Testing"),
    // Misc/General
    ("algebra", "➗", "General"), ("analysis", "🔍", "General"), ("analyze", "🔬", "General"), ("account", "👤", "General"), ("accounts", "👥", "General"),
    // Suffixes for versioning/hashes
    ("zm", "🧬", "Versioning"), ("h", "⏳", "Versioning"), ("v", "🔢", "Versioning"),
    // Color codes (hex)
    ("ff", "🎨", "Color"), ("00", "⚫", "Color"), ("ffffff", "⬜", "Color"), ("000000", "⬛", "Color"),
    // Numbers (for fun)
    ("0", "0️⃣", "Numbers"), ("1", "1️⃣", "Numbers"), ("2", "2️⃣", "Numbers"), ("3", "3️⃣", "Numbers"), ("4", "4️⃣", "Numbers"), ("5", "5️⃣", "Numbers"), ("6", "6️⃣", "Numbers"), ("7", "7️⃣", "Numbers"), ("8", "8️⃣", "Numbers"), ("9", "9️⃣", "Numbers"), ("10", "🔟", "Numbers"), ("100", "💯", "Numbers"), ("255", "🟧", "Numbers"),
    // Emoji codepoints
    ("1f3a8", "🎨", "Emoji"), ("1f4dd", "📝", "Emoji"), ("1f680", "🚀", "Emoji"), ("1f4a9", "💩", "Emoji"),
    // Heuristic/structural
    ("byte", "💾", "Numbers"), ("parenthes", "( )", "Rust Core"), ("case", "🎭", "Rust Core"), ("dot", "•", "General"), ("colon", ":", "General"), ("bounded", "📏", "General"),
    ("_", "⬜", "Rust Core"), ("colon2_token", ":", "Rust Core"), ("cond", "❓", "Rust Core"), ("content", "📦", "General"), ("if", "❓", "Rust Core"), ("where_clause", "📜", "Rust Core"),
    // Dependency Management
    ("cargo", "📦", "Dependencies"), ("toml", "📋", "Dependencies"), ("lock", "🔒", "Dependencies"), ("deps", "🔗", "Dependencies"), ("dependencies", "🔗", "Dependencies"), ("dev_deps", "🧪", "Dependencies"), ("features", "✨", "Dependencies"), ("workspace", "🏢", "Dependencies"), ("member", "👤", "Dependencies"), ("path", "🛤️", "Dependencies"), ("git", "🐙", "Dependencies"), ("branch", "🌿", "Dependencies"), ("tag", "🏷️", "Dependencies"), ("version", "🔢", "Dependencies"), ("semver", "📊", "Dependencies"), ("patch", "🩹", "Dependencies"), ("minor", "📈", "Dependencies"), ("major", "🚀", "Dependencies"), ("optional", "❓", "Dependencies"), ("default_features", "⚙️", "Dependencies"), ("no_default_features", "🚫", "Dependencies"),
    // Build System
    ("build", "🔨", "Build"), ("target", "🎯", "Build"), ("release", "🚀", "Build"), ("debug", "🐛", "Build"), ("profile", "📊", "Build"), ("optimization", "⚡", "Build"), ("compiler", "⚙️", "Build"), ("linker", "🔗", "Build"), ("artifact", "🎨", "Build"), ("binary", "💻", "Build"), ("library", "📚", "Build"), ("static", "🪨", "Build"), ("dynamic", "🌊", "Build"), ("wasm", "🌐", "Build"), ("native", "🏠", "Build"), ("cross_compile", "🔄", "Build"), ("platform", "🖥️", "Build"), ("architecture", "🏗️", "Build"), ("triple", "🎯", "Build"), ("toolchain", "🔧", "Build"),
    // Package Management
    ("crate", "📦", "Packages"), ("package", "📦", "Packages"), ("registry", "📚", "Packages"), ("publish", "📤", "Packages"), ("yank", "🗑️", "Packages"), ("download", "📥", "Packages"), ("install", "⚙️", "Packages"), ("uninstall", "🗑️", "Packages"), ("update", "🔄", "Packages"), ("upgrade", "⬆️", "Packages"), ("downgrade", "⬇️", "Packages"), ("pin", "📌", "Packages"), ("unpin", "📌", "Packages"), ("vendor", "🏪", "Packages"), ("offline", "📴", "Packages"), ("mirror", "🪞", "Packages"), ("source", "📄", "Packages"), ("repository", "📚", "Packages"), ("fork", "🍴", "Packages"), ("clone", "📋", "Packages"),
    // Module System
    ("mod", "📦", "Modules"), ("module", "📦", "Modules"), ("pub", "👀", "Modules"), ("private", "🔒", "Modules"), ("re_export", "📤", "Modules"), ("use", "🔗", "Modules"), ("extern", "🌍", "Modules"), ("crate", "📦", "Modules"), ("super", "⬆️", "Modules"), ("self", "🆔", "Modules"), ("as", "🏷️", "Modules"), ("glob", "🌟", "Modules"), ("prelude", "🎭", "Modules"), ("lib", "📚", "Modules"), ("main", "🎯", "Modules"), ("bin", "💻", "Modules"), ("example", "📝", "Modules"), ("test", "🧪", "Modules"), ("bench", "🏋️", "Modules"), ("doc", "📖", "Modules"),
    // Project Structure
    ("src", "📁", "Structure"), ("tests", "🧪", "Structure"), ("examples", "📝", "Structure"), ("benches", "🏋️", "Structure"), ("docs", "📖", "Structure"), ("assets", "🎨", "Structure"), ("resources", "📦", "Structure"), ("config", "⚙️", "Structure"), ("scripts", "📜", "Structure"), ("tools", "🔧", "Structure"), ("build_scripts", "🔨", "Structure"), ("proc_macro", "🪄", "Structure"), ("macro_rules", "📐", "Structure"), ("derive", "🧬", "Structure"), ("attribute", "🏷️", "Structure"), ("annotation", "📝", "Structure"), ("metadata", "📊", "Structure"), ("manifest", "📋", "Structure"), ("license", "📜", "Structure"), ("readme", "📖", "Structure"), ("changelog", "📝", "Structure"),
    ("binary", "⚫⚪", "Operators"),
    ("unary", "➖", "Operators"),
    ("embed", "📦✨", "Assets"),
    ("our", "👥💖", "Pronouns"),
    ("folder", "📁🌸", "Filesystem"),
    ("rust_embed", "🦀📦🌟", "Assets"),
    ("extractor", "🧲💎", "Processing"),
    ("borsh", "📦🧩", "Serialization"),
    ("windows", "🪟🌈", "Platform"),
    ("crates", "📦🧰", "Rust"),
    ("rs", "🦀🌟", "Rust"),
    ("data", "💾📊", "Data"),
    ("provider", "🤝🌟", "Service"),
    ("size", "📏📐", "Measurement"),
    ("idx", "#️⃣🔢", "Indexing"),
    ("libs", "📚✨", "Rust"),
    ("rust", "🦀💫", "Rust"),
    ("de", "⬇️", "Serialization"),
    ("ser", "⬆️", "Serialization"),
    ("github", "🐙", "Platform"),
    ("bincode", "🔢", "Serialization"),
    ("report", "📄", "Reporting"),
    ("speedy", "⚡", "Performance"),
    ("curves", "➰", "Math"),
    ("change", "🔄", "General"),
    ("string", "🔤", "Data"),
    ("leptos", "🌿", "Framework"),
    ("class", "🏷️", "OOP"),
    ("full", "🈵", "General"),
    ("objc2", "🍏", "Platform"),
    ("text", "📝", "Data"),
    ("dioxus", "🧬", "Framework"),
    ("emoji", "😀✨", "Meta"),
    ("vector", "➡️🟦", "Math"),
    ("mapping", "🗺️🔗", "Meta"),
    ("profile", "👤📋", "Meta"),
    ("pattern", "🔶🔍", "Meta"),
    ("scanner", "🔎📡", "Tool"),
    ("filter", "🚿🔬", "Tool"),
    ("sort", "🔢⬆️", "Tool"),
    ("frequency", "📊", "Stats"),
    ("coverage", "🗺️🌈", "Stats"),
    ("identifier", "🆔🔤", "Syntax"),
    ("literal", "🔤💎", "Syntax"),
    ("module", "📦🧩", "Code"),
    ("pipeline", "⛓️🚀", "System"),
    ("graph", "📈🔗", "System"),
    ("incremental", "➕⏳", "System"),
    ("compile", "⚙️🚦", "System"),
    ("analysis", "🔬🧠", "System"),
    ("GUI", "🖥️🎨", "UI"),
    ("MCP", "🧩🛠️", "System"),
    ("interactive", "🖱️💬", "UI"),
    ("review", "👀📝", "Process"),
    ("assign", "➡️🏷️", "Process"),
    ("vocabulary", "📚🗣️", "Meta"),
    ("automation", "🤖⚡", "System"),
    ("beauty", "🌸✨", "Meta"),
    ("layer", "🧅🎨", "Meta"),
    ("rich", "💎🌈", "Meta"),
    ("expressive", "🎭💬", "Meta"),
    ("system", "🖧⚙️", "System"),
    ("codebase", "💻🏗️", "Code"),
    ("component", "🧩🏗️", "Code"),
    ("crate", "📦🦀", "Rust"),
    ("signal", "📶🔔", "State"),
    ("state", "🔔🧠", "State"),
    ("dynamic", "🔄⚡", "Meta"),
    ("static", "🛑🏛️", "Meta"),
    ("modular", "🧩🔗", "Meta"),
    ("scan", "🔍📡", "Tool"),
    ("extract", "🧲📤", "Tool"),
    ("analyze", "🔬🧠", "Tool"),
    ("visual", "👁️🎨", "UI"),
    ("semantic", "🧠🔤", "Meta"),
    ("syntax", "🔤📐", "Meta"),
    ("ast", "🌳🧩", "Code"),
    ("node", "🔗🌳", "Code"),
    ("token", "🔖🔤", "Code"),
    ("snippet", "✂️📄", "Code"),
    ("example", "💡📄", "Meta"),
    ("template", "📄🧩", "Meta"),
    ("category", "🏷️📚", "Meta"),
    ("term", "🔤🏷️", "Meta"),
    ("unmapped", "❓🚫", "Stats"),
    ("mapped", "✅🔗", "Stats"),
    ("new", "✨", "Creation"),
    ("transaction", "🔗💰", "Blockchain"),
    ("solana", "⚡", "Blockchain"),
    ("workflows", "🗺️", "Process"),
    ("packages", "📦", "Packaging"),
    ("property", "🔑", "Attribute"),
    ("component", "🧩", "Architecture"),
    ("framework", "🏗️", "Architecture"),
    ("hashes", "#️⃣", "Crypto"),
    ("programs", "⚙️", "Execution"),
    ("11", "🔢🌟", "Math"),
    // --- Meta/Workflow Concepts ---
    ("refactor", "🛠️🔄", "Workflow"),
    ("modular", "🧩🔗", "Workflow"),
    ("vectorize", "➡️🟦", "Workflow"),
    ("visualize", "👁️🎨", "Workflow"),
    ("automate", "🤖⚡", "Workflow"),
    ("dynamic", "🔄⚡", "Workflow"),
    ("static", "🛑🏛️", "Workflow"),
    ("interactive", "🖱️💬", "Workflow"),
    ("pipeline", "⛓️🚀", "Workflow"),
    ("graph", "📈🔗", "Workflow"),
    ("scan", "🔍📡", "Workflow"),
    ("extract", "🧲📤", "Workflow"),
    ("analyze", "🔬🧠", "Workflow"),
    ("filter", "🚿🔬", "Workflow"),
    ("sort", "🔢⬆️", "Workflow"),
    ("review", "👀📝", "Workflow"),
    ("assign", "➡️🏷️", "Workflow"),
    ("report", "📄📊", "Workflow"),
    ("coverage", "🗺️🌈", "Workflow"),
    ("frequency", "📊🔁", "Workflow"),
    ("unmapped", "❓🚫", "Workflow"),
    ("mapped", "✅🔗", "Workflow"),
    // --- Rust/Codebase Concepts ---
    ("crate", "📦🦀", "Rust"),
    ("module", "📦🧩", "Rust"),
    ("component", "🧩🏗️", "Rust"),
    ("signal", "📶🔔", "Rust"),
    ("state", "🔔🧠", "Rust"),
    ("identifier", "🆔🔤", "Rust"),
    ("literal", "🔤💎", "Rust"),
    ("ast", "🌳🧩", "Rust"),
    ("node", "🔗🌳", "Rust"),
    ("token", "🔖🔤", "Rust"),
    ("snippet", "✂️📄", "Rust"),
    ("template", "📄🧩", "Rust"),
    ("pattern", "🔶🔍", "Rust"),
    ("function", "🦀⚙️", "Rust"),
    ("struct", "🏛️🧱", "Rust"),
    ("trait", "🏷️🧬", "Rust"),
    ("macro", "🪄🦀", "Rust"),
    ("prop", "🔑🏷️", "Rust"),
    ("emoji", "😀✨", "Rust"),
    ("vocabulary", "📚🗣️", "Rust"),
    // --- System/Architecture Concepts ---
    ("system", "🖧⚙️", "System"),
    ("framework", "🏗️🌿", "System"),
    ("GUI", "🖥️🎨", "System"),
    ("MCP", "🧩🛠️", "System"),
    ("beauty", "🌸✨", "System"),
    ("layer", "🧅🎨", "System"),
    ("expressive", "🎭💬", "System"),
    ("rich", "💎🌈", "System"),
    // --- Other/Playful Concepts ---
    ("brainrot", "🧠🌪️", "Playful"),
    ("meme", "😂📈", "Playful"),
    ("italian", "🇮🇹🍝", "Playful"),
    ("prime", "🌟🔢", "Playful"),
    ("idea", "💡✨", "Playful"),
    ("flow", "🌊➡️", "Playful"),
    ("hero", "🦸‍♂️🗺️", "Playful"),
    ("mythos", "📜🌌", "Playful"),
];

fn emoji_for_type(ty: &str) -> (&'static str, &'static str) {
    for &(name, emoji, category) in EMOJI_TYPE_MAP {
        if ty == name {
            return (emoji, category);
        }
    }
    ("❓🤷", "Uncategorized")
}

fn extract_string_literals(value: &serde_json::Value, out: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map.iter() {
                // Extract string literal values
                if (k == "lit" || k == "str") && v.is_string() {
                    if let Some(s) = v.as_str() {
                        out.push(s.to_string());
                    }
                }
                // Extract identifier-like fields
                if ["ident", "name", "label", "var", "field", "path", "ty", "type", "kind", "value"].contains(&k.as_str()) && v.is_string() {
                    if let Some(s) = v.as_str() {
                        out.push(s.to_string());
                    }
                }
                extract_string_literals(v, out);
            }
        },
        serde_json::Value::Array(arr) => {
            for v in arr {
                extract_string_literals(v, out);
            }
        },
        _ => {}
    }
}

fn split_words(s: &str) -> Vec<String> {
    // Split on whitespace, punctuation, underscores
    let mut words = Vec::new();
    let _re = Regex::new(r"[A-Za-z0-9]+_").unwrap(); // dummy, not used for splitting
    for part in s.split(|c: char| !c.is_alphanumeric() && c != '_') {
        if part.is_empty() { continue; }
        // Manually split CamelCase
        let mut last = 0;
        let chars: Vec<char> = part.chars().collect();
        for i in 1..chars.len() {
            if chars[i].is_uppercase() && chars[i - 1].is_lowercase() {
                words.push(chars[last..i].iter().collect::<String>().to_lowercase());
                last = i;
            }
        }
        if last < chars.len() {
            words.push(chars[last..].iter().collect::<String>().to_lowercase());
        }
    }
    words
}

fn main() {
    use std::env;
    let mut args = env::args().skip(1);
    let mut target_path: Option<String> = None;
    let mut limit: Option<usize> = None;
    while let Some(arg) = args.next() {
        if arg == "--limit" {
            if let Some(lim) = args.next() {
                limit = lim.parse().ok();
            }
        } else {
            target_path = Some(arg);
        }
    }

    let mut files = HashMap::new();
    let mut file_count = 0;
    let mut discovered_files = Vec::new();
    if let Some(ref path) = target_path {
        let path = Path::new(path);
        if path.is_file() {
            discovered_files.push(path.to_path_buf());
        } else if path.is_dir() {
            for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
                if entry.file_type().is_file() {
                    discovered_files.push(entry.path().to_path_buf());
                }
            }
        } else {
            println!("[ERROR] Path not found: {}", path.display());
            return;
        }
    } else {
        for entry in WalkDir::new(".").into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() {
                discovered_files.push(entry.path().to_path_buf());
            }
        }
    }
    if let Some(lim) = limit {
        discovered_files.truncate(lim);
    }
    println!("[INFO] Processing {} files:", discovered_files.len());
    for f in &discovered_files {
        println!("  - {}", f.display());
    }
    for path in discovered_files {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        match ext.as_str() {
            // Text/code formats
            "rs" | "md" | "json" | "ttl" => {
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        let tokens: Vec<&str> = content.split_whitespace().collect();
                        files.insert(path.to_string_lossy().to_string(), tokens.join(" "));
                        file_count += 1;
                    },
                    Err(e) => {
                        println!("[ERROR: could not read file: {}]", e);
                    }
                }
            },
            // Image formats
            "png" | "jpg" | "jpeg" | "gif" | "svg" => {
                println!("[TODO: Create AI description task for image file: {}]", path.display());
            },
            // Unknown/other formats
            _ => {
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        let tokens: Vec<&str> = content.split_whitespace().collect();
                        files.insert(path.to_string_lossy().to_string(), tokens.join(" "));
                        file_count += 1;
                    },
                    Err(_) => {}
                }
            }
        }
        if let Some(lim) = limit {
            if file_count >= lim {
                break;
            }
        }
    }

    // 2. Create HF dataset structure early
    println!("\n[INFO] Creating Hugging Face dataset structure...");
    let dataset_dir = "hf_dataset";
    if !Path::new(dataset_dir).exists() {
        match fs::create_dir_all(dataset_dir) {
            Ok(_) => println!("[INFO] Created dataset directory: {}", dataset_dir),
            Err(e) => {
                println!("[ERROR] Could not create dataset directory: {}", e);
                return;
            }
        }
    }
    
    // Create HF reports directory
    let hf_reports_dir = format!("{}/reports", dataset_dir);
    if !Path::new(&hf_reports_dir).exists() {
        match fs::create_dir_all(&hf_reports_dir) {
            Ok(_) => println!("[INFO] Created HF reports directory: {}", hf_reports_dir),
            Err(e) => {
                println!("[ERROR] Could not create HF reports directory: {}", e);
                return;
            }
        }
    }

    // 3. Analyze all files
    println!("[INFO] Initializing CodeAnalyzer ...");
    let mut analyzer = CodeAnalyzer::new(32, 0.8);
    println!("[INFO] Analyzing files ...");
    let analyses = match analyzer.analyze_multiple_files(files) {
        Ok(a) => a,
        Err(e) => {
            println!("[ERROR] Failed to analyze files: {}", e);
            return;
        }
    };
    println!("[INFO] Analysis complete. {} files analyzed.", analyses.len());

    // 3. Set up reports directory
    let reports_dir = "reports";
    if !Path::new(reports_dir).exists() {
        match fs::create_dir_all(reports_dir) {
            Ok(_) => println!("[INFO] Created reports directory: {}", reports_dir),
            Err(e) => {
                println!("[ERROR] Could not create reports directory: {}", e);
                return;
            }
        }
    }

    fn count_types_recursive(value: &serde_json::Value, type_counts: &mut BTreeMap<String, usize>, total_nodes: &mut usize) {
        match value {
            serde_json::Value::Object(map) => {
                *total_nodes += 1;
                for (k, v) in map.iter() {
                    *type_counts.entry(k.clone()).or_insert(0) += 1;
                    count_types_recursive(v, type_counts, total_nodes);
                }
            },
            serde_json::Value::Array(arr) => {
                for v in arr {
                    count_types_recursive(v, type_counts, total_nodes);
                }
            },
            _ => {}
        }
    }
    let mut dir_type_counts: HashMap<String, BTreeMap<String, usize>> = HashMap::new();
    let mut total_type_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut global_word_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut global_word_emoji_counts: BTreeMap<String, usize> = BTreeMap::new();
    for (i, analysis) in analyses.iter().enumerate() {
        match serde_json::from_str::<serde_json::Value>(&analysis.json_ast) {
            Ok(ast) => {
                let mut type_counts = BTreeMap::new();
                let mut total_nodes = 0;
                count_types_recursive(&ast, &mut type_counts, &mut total_nodes);
                // Extract string literals and process words
                let mut string_literals = Vec::new();
                extract_string_literals(&ast, &mut string_literals);
                let mut word_counts = BTreeMap::new();
                for s in &string_literals {
                    for word in split_words(s) {
                        *word_counts.entry(word).or_insert(0) += 1;
                    }
                }
                // Map words (from literals and identifiers) to emojis
                let mut word_emoji_counts = BTreeMap::new();
                for (word, count) in &word_counts {
                    let (emoji, category) = emoji_for_type(word);
                    // Always record the emoji mapping, even for identifiers and module names
                    if emoji != "❓" && emoji != "❓🤷" {
                        *word_emoji_counts.entry(emoji).or_insert(0usize) += *count;
                    }
                }
                // Count emojis in string literals
                let mut emoji_counts_in_strings = BTreeMap::new();
                for s in &string_literals {
                    for ch in s.chars() {
                        if ch.len_utf8() > 2 { // crude emoji filter
                            let e = ch.to_string();
                            *emoji_counts_in_strings.entry(e).or_insert(0) += 1;
                        }
                    }
                }
                // Write enriched report file directly to HF dataset
                let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                let node_count = ast.as_object().map(|o| o.len()).unwrap_or(0);
                let report = serde_json::json!({
                    "file_path": analysis.file_path,
                    "timestamp": timestamp,
                    "summary": {
                        "top_level_nodes": node_count,
                        "total_nodes": total_nodes,
                        "type_counts": type_counts,
                        "string_literals": string_literals,
                        "word_counts": word_counts,
                        "word_emoji_counts": word_emoji_counts,
                        "emoji_counts_in_strings": emoji_counts_in_strings
                    },
                    "ast": ast
                });
                
                // Directory aggregation
                let dir = analysis.file_path.rsplit_once('/').map(|(d, _)| d).unwrap_or("");
                let dir_entry = dir_type_counts.entry(dir.to_string()).or_default();
                for (ty, count) in &type_counts {
                    *dir_entry.entry(ty.clone()).or_insert(0) += *count;
                    *total_type_counts.entry(ty.clone()).or_insert(0) += *count;
                }
                
                // Create compact directory structure for HF dataset reports
                let path_parts: Vec<&str> = analysis.file_path.split(['/', '\\']).collect();
                let subdir_name = if path_parts.len() >= 3 {
                    let name = format!("{}_{}_{}", path_parts[0], path_parts[1], path_parts[2]);
                    if name.len() > 50 { name[..50].to_string() } else { name }
                } else if path_parts.len() == 2 {
                    let name = format!("{}_{}", path_parts[0], path_parts[1]);
                    if name.len() > 50 { name[..50].to_string() } else { name }
                } else if path_parts.len() == 1 {
                    let name = path_parts[0].to_string();
                    if name.len() > 50 { name[..50].to_string() } else { name }
                } else {
                    "root".to_string()
                };
                
                // Create a shorter filename to avoid Windows path length limits
                let original_filename = path_parts.last().unwrap_or(&"unknown");
                let short_filename = if original_filename.len() > 30 {
                    // Truncate long filenames to 30 chars
                    format!("{}.json", &original_filename[..30])
                } else {
                    format!("{}.json", original_filename)
                };
                let hf_report_path = format!("{}/reports/{}/{}", dataset_dir, subdir_name, short_filename);
                
                // Create the subdirectory if it doesn't exist
                let subdir_path = format!("{}/reports/{}", dataset_dir, subdir_name);
                if !Path::new(&subdir_path).exists() {
                    if let Err(e) = fs::create_dir_all(&subdir_path) {
                        println!("[ERROR] Failed to create directory {}: {}", subdir_path, e);
                        continue;
                    }
                }
                
                let report_json = serde_json::to_string_pretty(&report).unwrap();
                
                // Write to HF dataset reports directory
                match fs::write(&hf_report_path, &report_json) {
                    Ok(_) => {
                        // Structure summary
                        let mut emoji_counts = Vec::new();
                        let mut emoji_summary = String::new();
                        for (ty, count) in &type_counts {
                            let (emoji, category) = emoji_for_type(ty);
                            emoji_counts.push(format!("{}({})×{}", emoji, ty, count));
                            emoji_summary.push_str(&emoji.repeat(*count.min(&10)));
                        }
                        let emoji_counts_str = emoji_counts.join(" ");
                        let filename = format!("{}.json", path_parts.last().unwrap_or(&"unknown"));
                        if type_counts.is_empty() {
                            println!("{} | none |", filename);
                        } else {
                            println!("{} | {} | {}", filename, emoji_counts_str, emoji_summary);
                        }
                        // Emojis found in string literals
                        if !emoji_counts_in_strings.is_empty() {
                            let mut emoji_strs = Vec::new();
                            for (emoji, count) in &emoji_counts_in_strings {
                                emoji_strs.push(format!("{}×{}", emoji, count));
                            }
                            println!("[emojis in strings] {}", emoji_strs.join(" "));
                        }
                        // Words mapped to emojis (from literals and identifiers)
                        if !word_emoji_counts.is_empty() {
                            let mut word_emoji_strs = Vec::new();
                            for (emoji, count) in &word_emoji_counts {
                                word_emoji_strs.push(format!("{}×{}", emoji, count));
                            }
                            println!("[words mapped to emojis] {}", word_emoji_strs.join(" "));
                        }
                        // Aggregate global word counts
                        for (word, count) in &word_counts {
                            *global_word_counts.entry(word.clone()).or_insert(0) += *count;
                        }
                        for (emoji, count) in &word_emoji_counts {
                            *global_word_emoji_counts.entry(emoji.to_string()).or_insert(0) += *count;
                        }
                    },
                    Err(e) => println!("[ERROR] Failed to write report {}: {}", hf_report_path, e),
                }
            },
            Err(e) => {
                println!("[ERROR] Failed to parse AST: {}", e);
            }
        }
    }
    // Print per-directory summary table
    println!("\n=== Directory Emoji Summary Table ===");
    let mut dir_keys: Vec<_> = dir_type_counts.keys().collect();
    dir_keys.sort();
    let mut global_dir_reports = Vec::new();
    for dir in dir_keys {
        let type_counts = &dir_type_counts[dir];
        let mut emoji_counts = Vec::new();
        let mut emoji_summary = String::new();
        for (ty, count) in type_counts {
            let (emoji, category) = emoji_for_type(ty);
            emoji_counts.push(format!("{}({})×{}", emoji, ty, count));
            emoji_summary.push_str(&emoji.repeat((*count).min(10)));
        }
        let emoji_counts_str = emoji_counts.join(" ");
        let mut report = String::new();
        report.push_str(&format!("=== Directory Emoji Summary: {} ===\n", dir));
        if type_counts.is_empty() {
            report.push_str(&format!("none\n"));
        } else {
            report.push_str(&format!("{} | {}\n", emoji_counts_str, emoji_summary));
        }
        // Per-directory word/category/emoji breakdown
        let mut dir_word_counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut dir_word_emoji_counts: BTreeMap<String, usize> = BTreeMap::new();
        // Aggregate words for this directory
        for (i, analysis) in analyses.iter().enumerate() {
            if let Some(file_dir) = analysis.file_path.rsplit_once('/').map(|(d, _)| d) {
                if file_dir == dir {
                    if let Ok(ast) = serde_json::from_str::<serde_json::Value>(&analysis.json_ast) {
                        let mut string_literals = Vec::new();
                        extract_string_literals(&ast, &mut string_literals);
                        for s in &string_literals {
                            for word in split_words(s) {
                                *dir_word_counts.entry(word).or_insert(0) += 1;
                            }
                        }
                        for (word, count) in &dir_word_counts {
                            let (emoji, category) = emoji_for_type(word);
                            if emoji != "❓" && emoji != "❓🤷" {
                                dir_word_emoji_counts.entry(emoji.to_string()).or_insert(0usize).saturating_add(*count);
                            }
                        }
                    }
                }
            }
        }
        // Word report
        report.push_str("\n=== Directory Word Report ===\n");
        report.push_str(&format!("{:<20} | {:<8} | {:<18} | {}\n", "word", "count", "category", "emoji"));
        let mut word_keys: Vec<_> = dir_word_counts.keys().collect();
        word_keys.sort();
        let mut found_agave = false;
        let mut found_css = false;
        let mut found_crypto = false;
        let mut found_version = false;
        for word in word_keys.iter() {
            let count = dir_word_counts[*word];
            let (emoji, category) = emoji_for_type(word);
            if *word == "agave" { found_agave = true; }
            if ["px", "deg", "em", "rem", "vh", "vw", "animation", "transition", "absolute", "align", "app", "app_state", "accessibility"].contains(&word.as_str()) { found_css = true; }
            if ["aead", "aeads", "aes", "argon2", "arc", "addr2line", "aarch64", "amd64", "armv8", "crypto", "curve25519", "ed25519", "elliptic", "fiat", "cbor"].contains(&word.as_str()) { found_crypto = true; }
            if ["zm", "h", "v"].contains(&word.as_str()) { found_version = true; }
            if emoji != "❓" && emoji != "❓🤷" {
                report.push_str(&format!("{:<20} | {:<8} | {:<18} | {}\n", word, count, category, emoji));
            } else {
                report.push_str(&format!("{:<20} | {:<8} | {:<18} |\n", word, count, category));
            }
        }
        // Banners
        if found_agave {
            report.push_str("\n🌵🌵🌵 AGAVE detected! This project is spicy! 🌵🌵🌵\n");
        }
        if found_css {
            report.push_str("\n🎨 CSS/Frontend detected! Styling and animation everywhere!\n");
        }
        if found_crypto {
            report.push_str("\n🔒 Crypto detected! Security is strong in this codebase.\n");
        }
        if found_version {
            report.push_str("\n🔢 Versioning/Hash detected! Lots of unique IDs and versions.\n");
        }
        // Write to file
        let safe_dir = if dir.is_empty() { "root".to_string() } else { dir.replace('/', "_") };
        let report_path = format!("{}/summary_{}.txt", reports_dir, safe_dir);
        match fs::write(&report_path, &report) {
            Ok(_) => println!("[INFO] Wrote directory summary to {}", report_path),
            Err(e) => println!("[ERROR] Failed to write directory summary {}: {}", report_path, e),
        }
        global_dir_reports.push((dir.clone(), report_path));
    }
    // Print total summary (minimal)
    let mut total_report = String::new();
    total_report.push_str("=== Total Emoji Summary Table ===\n");
    total_report.push_str(&format!("{:<20} | {:<8} | {:<18} | {}\n", "Type", "Count", "Category", "Emoji"));
    let mut type_keys: Vec<_> = total_type_counts.keys().collect();
    type_keys.sort();
    for ty in type_keys {
        let count = total_type_counts[ty];
        let (emoji, category) = emoji_for_type(ty);
        total_report.push_str(&format!("{:<20} | {:<8} | {:<18} | {}\n", ty, count, category, emoji));
    }
    total_report.push_str(&format!("\n[INFO] Total files processed: {}\n", analyses.len()));
    // Write total summary
    let merged_path = format!("{}/summary_total.txt", reports_dir);
    match fs::write(&merged_path, &total_report) {
        Ok(_) => println!("[INFO] Wrote total summary to {}", merged_path),
        Err(e) => println!("[ERROR] Failed to write total summary: {}", e),
    }

    // 5. Create Hugging Face Dataset Structure
    println!("\n[INFO] Creating Hugging Face dataset structure...");
    
    // Create dataset metadata
    let dataset_info = serde_json::json!({
        "description": "Rust codebase AST analysis with emoji mapping",
        "license": "agpl-3.0",
        "features": {
            "file_path": {"dtype": "string"},
            "timestamp": {"dtype": "int64"},
            "ast": {"dtype": "string"},
            "summary": {
                "dtype": "map",
                "mapping": {
                    "top_level_nodes": {"dtype": "int64"},
                    "total_nodes": {"dtype": "int64"},
                    "type_counts": {"dtype": "map"},
                    "string_literals": {"dtype": "sequence", "feature": {"dtype": "string"}},
                    "word_counts": {"dtype": "map"},
                    "word_emoji_counts": {"dtype": "map"},
                    "emoji_counts_in_strings": {"dtype": "map"}
                }
            }
        },
        "builder_name": "rust_ast_emoji",
        "config_name": "default",
        "version": {"version_str": "0.1.0"},
        "splits": {
            "train": {
                "name": "train",
                "num_bytes": 0,
                "num_examples": 0,
                "shard_lengths": []
            }
        }
    });

    // Write dataset info
    let info_path = format!("{}/dataset_info.json", dataset_dir);
    match fs::write(&info_path, serde_json::to_string_pretty(&dataset_info).unwrap()) {
        Ok(_) => println!("[INFO] Wrote dataset info to {}", info_path),
        Err(e) => println!("[ERROR] Failed to write dataset info: {}", e),
    }

    // Split ASTs into chunks and organize into subdirectories
    let max_file_size = 1024 * 1024; // 1MB
    let max_files_per_dir = 10000;
    let mut current_chunk = Vec::new();
    let mut current_chunk_size = 0;
    let mut chunk_index = 0;
    let mut file_index = 0;
    let mut total_examples = 0;

    // Create data directory
    let data_dir = format!("{}/data", dataset_dir);
    if !Path::new(&data_dir).exists() {
        match fs::create_dir_all(&data_dir) {
            Ok(_) => println!("[INFO] Created data directory: {}", data_dir),
            Err(e) => {
                println!("[ERROR] Could not create data directory: {}", e);
                return;
            }
        }
    }

    // Copy reports to dataset
    let reports_data_dir = format!("{}/reports", dataset_dir);
    if !Path::new(&reports_data_dir).exists() {
        match fs::create_dir_all(&reports_data_dir) {
            Ok(_) => println!("[INFO] Created reports directory: {}", reports_data_dir),
            Err(e) => {
                println!("[ERROR] Could not create reports directory: {}", e);
                return;
            }
        }
    }


    
    // Write summary files directly to HF dataset
    let summary_files = [
        "summary_total.txt",
        "emoji_mapping.txt"
    ];
    
    for summary_file in &summary_files {
        let source_path = format!("{}/{}", reports_dir, summary_file);
        let target_path = format!("{}/reports/{}", dataset_dir, summary_file);
        
        if Path::new(&source_path).exists() {
            match fs::copy(&source_path, &target_path) {
                Ok(_) => println!("[INFO] Copied summary file: {}", summary_file),
                Err(e) => println!("[ERROR] Failed to copy {}: {}", summary_file, e),
            }
        }
    }

    // Process each analysis and create chunks
    for analysis in &analyses {
        if let Ok(ast) = serde_json::from_str::<serde_json::Value>(&analysis.json_ast) {
            let mut type_counts = BTreeMap::new();
            let mut total_nodes = 0;
            count_types_recursive(&ast, &mut type_counts, &mut total_nodes);
            
            let mut string_literals = Vec::new();
            extract_string_literals(&ast, &mut string_literals);
            let mut word_counts = BTreeMap::new();
            for s in &string_literals {
                for word in split_words(s) {
                    *word_counts.entry(word).or_insert(0) += 1;
                }
            }
            
            let mut word_emoji_counts = BTreeMap::new();
            for (word, count) in &word_counts {
                let (emoji, category) = emoji_for_type(word);
                if emoji != "❓" && emoji != "❓🤷" {
                    *word_emoji_counts.entry(emoji).or_insert(0usize) += *count;
                }
            }
            
            let mut emoji_counts_in_strings = BTreeMap::new();
            for s in &string_literals {
                for ch in s.chars() {
                    if ch.len_utf8() > 2 {
                        let e = ch.to_string();
                        *emoji_counts_in_strings.entry(e).or_insert(0) += 1;
                    }
                }
            }

            let example = serde_json::json!({
                "file_path": analysis.file_path,
                "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                "ast": ast,
                "summary": {
                    "top_level_nodes": ast.as_object().map(|o| o.len()).unwrap_or(0),
                    "total_nodes": total_nodes,
                    "type_counts": type_counts,
                    "string_literals": string_literals,
                    "word_counts": word_counts,
                    "word_emoji_counts": word_emoji_counts,
                    "emoji_counts_in_strings": emoji_counts_in_strings
                }
            });

            // Calculate the actual size this example will add to the chunk
            let example_json = serde_json::to_string(&example).unwrap();
            let example_size = example_json.len();
            
            // Debug: Print size information
            if example_size > 1024 * 1024 { // If any single example is > 1MB
                println!("[WARNING] Large example: {} bytes for {}", example_size, analysis.file_path);
            }

            // Check if adding this example would exceed the chunk size
            if current_chunk_size + example_size > max_file_size && !current_chunk.is_empty() {
                // Write current chunk
                let subdir = file_index / max_files_per_dir;
                let subdir_path = format!("{}/{:03}", data_dir, subdir);
                if !Path::new(&subdir_path).exists() {
                    match fs::create_dir_all(&subdir_path) {
                        Ok(_) => println!("[INFO] Created subdirectory: {}", subdir_path),
                        Err(e) => println!("[ERROR] Failed to create subdirectory: {}", e),
                    }
                }

                let chunk_path = format!("{}/chunk_{:05}.json", subdir_path, chunk_index);
                let chunk_data = serde_json::json!({
                    "examples": current_chunk,
                    "metadata": {
                        "chunk_index": chunk_index,
                        "num_examples": current_chunk.len(),
                        "total_size_bytes": current_chunk_size
                    }
                });

                match fs::write(&chunk_path, serde_json::to_string(&chunk_data).unwrap()) {
                    Ok(_) => println!("[INFO] Wrote chunk {} to {} ({} examples, {} bytes)", chunk_index, chunk_path, current_chunk.len(), current_chunk_size),
                    Err(e) => println!("[ERROR] Failed to write chunk {}: {}", chunk_index, e),
                }

                // Reset for next chunk
                current_chunk.clear();
                current_chunk_size = 0;
                chunk_index += 1;
            }

            // Add example to current chunk
            current_chunk.push(example);
            current_chunk_size += example_size;
            file_index += 1;
            total_examples += 1;
        }
    }

    // Write final chunk if not empty
    if !current_chunk.is_empty() {
        let subdir = file_index / max_files_per_dir;
        let subdir_path = format!("{}/{:03}", data_dir, subdir);
        if !Path::new(&subdir_path).exists() {
            match fs::create_dir_all(&subdir_path) {
                Ok(_) => println!("[INFO] Created subdirectory: {}", subdir_path),
                Err(e) => println!("[ERROR] Failed to create subdirectory: {}", e),
            }
        }

        let chunk_path = format!("{}/chunk_{:05}.json", subdir_path, chunk_index);
        let chunk_data = serde_json::json!({
            "examples": current_chunk,
            "metadata": {
                "chunk_index": chunk_index,
                "num_examples": current_chunk.len(),
                "total_size_bytes": current_chunk_size
            }
        });

        match fs::write(&chunk_path, serde_json::to_string(&chunk_data).unwrap()) {
            Ok(_) => println!("[INFO] Wrote final chunk {} to {}", chunk_index, chunk_path),
            Err(e) => println!("[ERROR] Failed to write final chunk {}: {}", chunk_index, e),
        }
    }

    // Create README for the dataset
    let readme_content = format!("# Rust AST Emoji Dataset

This dataset contains Rust codebase AST (Abstract Syntax Tree) analysis with emoji mapping for code understanding and visualization.

## Dataset Structure

- **Total Examples**: {}
- **Total Chunks**: {}
- **Max File Size**: 10KB per chunk
- **Max Files per Directory**: 10,000

## Features

- `file_path`: Path to the original Rust source file
- `timestamp`: Unix timestamp of analysis
- `ast`: Full AST representation in JSON
- `summary`: Analysis summary including:
  - `top_level_nodes`: Number of top-level AST nodes
  - `total_nodes`: Total number of AST nodes
  - `type_counts`: Count of each AST node type
  - `string_literals`: Extracted string literals
  - `word_counts`: Word frequency analysis
  - `word_emoji_counts`: Emoji mapping for words
  - `emoji_counts_in_strings`: Emojis found in string literals

## Usage

This dataset can be used for:
- Code understanding and visualization
- AST pattern analysis
- Emoji-based code summarization
- Codebase domain detection (Crypto, Web, i18n, etc.)

## License

AGPL-3.0 License
", total_examples, chunk_index + 1);

    let readme_path = format!("{}/README.md", dataset_dir);
    match fs::write(&readme_path, readme_content) {
        Ok(_) => println!("[INFO] Wrote README to {}", readme_path),
        Err(e) => println!("[ERROR] Failed to write README: {}", e),
    }

    println!("[INFO] Hugging Face dataset created successfully in '{}'", dataset_dir);
    println!("[INFO] Dataset contains {} examples across {} chunks", total_examples, chunk_index + 1);
}

// Function to write emoji ontology as Turtle (RDF)
fn write_emoji_ontology_turtle(path: &str) {
    use std::fs::File;
    use std::io::Write;
    let mut file = File::create(path).expect("Failed to create emoji ontology turtle file");
    writeln!(file, "@prefix em: <http://example.org/emoji#> .").ok();
    for (term, emoji, category) in EMOJI_TYPE_MAP.iter() {
        writeln!(file, "em:{} a em:Concept ; em:emoji \"{}\" ; em:category \"{}\" .", term.replace('-', "_"), emoji, category).ok();
    }
}

// Function to import emoji ontology from Turtle (RDF) file
fn import_emoji_ontology_turtle(path: &str) -> std::collections::HashMap<String, (String, String)> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::collections::HashMap;
    let file = File::open(path).expect("Failed to open emoji ontology turtle file");
    let reader = BufReader::new(file);
    let mut map = HashMap::new();
    let mut current_term = None;
    let mut emoji = None;
    let mut category = None;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("em:") {
            // Parse subject
            if let Some(idx) = line.find(' ') {
                let term = line[3..idx].replace('_', "-");
                current_term = Some(term);
            }
        }
        if let Some(start) = line.find("em:emoji \"") {
            let rest = &line[start + 10..];
            if let Some(end) = rest.find('"') {
                emoji = Some(rest[..end].to_string());
            }
        }
        if let Some(start) = line.find("em:category \"") {
            let rest = &line[start + 13..];
            if let Some(end) = rest.find('"') {
                category = Some(rest[..end].to_string());
            }
        }
        if line.trim_end().ends_with('.') {
            if let (Some(term), Some(e), Some(cat)) = (current_term.take(), emoji.take(), category.take()) {
                map.insert(term, (e, cat));
            }
        }
    }
    map
}

// Example: Using Sophia to read and write the emoji ontology Turtle file
#[allow(dead_code)]
fn sophia_read_write_emoji_ontology() -> Result<(), Box<dyn std::error::Error>> {
    // Use the vendored Sophia path
    use sophia_turtle::parser::turtle;
    use sophia_api::graph::Graph;
    use sophia_inmem::graph::FastGraph;
    use sophia_api::ns::Namespace;
    //use sophia_api::term::SimpleIri;
    
    use sophia_api::prelude::TripleSource;
    use sophia_api::prelude::TripleSerializer;
    use std::fs::File;
    use std::io::{BufReader, BufWriter};
    use sophia_api::prelude::Triple;
    use sophia_api::graph::MutableGraph;
    use sophia_api::ns::NsTerm;
    
    // Read the ontology
    let file = File::open("reports/emoji_ontology.ttl")?;
    let reader = BufReader::new(file);
    let mut graph: FastGraph = turtle::parse_bufread(reader).collect_triples()?;

    // Example: Iterate over all emoji concepts
    let em = Namespace::new("http://example.org/emoji#")?;
    let emoji_pred = em.get("emoji")?;
    for triple in graph.triples_matching(None::<&NsTerm>, Some(&emoji_pred), None::<&NsTerm>) {
        let t = triple?;
        println!("Term: {:?}, Emoji: {:?}", t.s(), t.o());
    }

    // Example: Add a new triple
    let new_term = em.get("example_term")?;
    let emoji_pred = em.get("emoji")?;
    let emoji_val = "🦀";
    graph.insert(&new_term, &emoji_pred, emoji_val)?;

    // Write the updated ontology
    let out_file = File::create("reports/emoji_ontology_out.ttl")?;
    let mut writer = BufWriter::new(out_file);
    //sophia_turtle::serializer::turtle::TurtleConfig::new()
    //.serialize_graph(&graph, &mut writer)?;
    let mut serializer = sophia_turtle::serializer::turtle::TurtleSerializer::new_with_config(&mut writer, sophia_turtle::serializer::turtle::TurtleConfig::new());  
    serializer.serialize_triples(graph.triples())?;

    
    Ok(())
}
