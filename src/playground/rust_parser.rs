use dioxus::prelude::*;
use syn_serde::json;
use syn;

#[derive(PartialEq, Clone)]
pub struct RustParserState {
    pub input_code: String,
    pub parsed_ast: Option<String>,
    pub error_message: Option<String>,
    pub is_pretty: bool,
}

impl Default for RustParserState {
    fn default() -> Self {
        Self {
            input_code: r#"fn main() {
    println!("Hello, world!");
}"#.to_string(),
            parsed_ast: None,
            error_message: None,
            is_pretty: true,
        }
    }
}

#[component]
pub fn RustParserApp() -> Element {
    let mut state = use_signal(|| RustParserState::default());

    let handle_parse = move |_| {
        let code = state.read().input_code.clone();
        let pretty = state.read().is_pretty;
        
        match syn::parse_str::<syn::File>(&code) {
            Ok(ast) => {
                let json_str = if pretty {
                    json::to_string_pretty(&ast)
                } else {
                    json::to_string(&ast)
                };
                state.write().parsed_ast = Some(json_str);
                state.write().error_message = None;
            }
            Err(e) => {
                state.write().error_message = Some(format!("Parse error: {}", e));
                state.write().parsed_ast = None;
            }
        }
    };

    let handle_input_change = move |evt: Event<FormData>| {
        let value = evt.value();
        state.write().input_code = value;
    };

    let toggle_pretty = move |_| {
        let current = state.read().is_pretty;
        state.write().is_pretty = !current;
    };

    let clear_output = move |_| {
        state.write().parsed_ast = None;
        state.write().error_message = None;
    };

    let load_example = move |example: &str| {
        // First, update the input code
        state.write().input_code = example.to_string();
        
        // Then parse the code in a separate operation
        let pretty = state.read().is_pretty;
        match syn::parse_str::<syn::File>(example) {
            Ok(ast) => {
                let json_str = if pretty {
                    json::to_string_pretty(&ast)
                } else {
                    json::to_string(&ast)
                };
                state.write().parsed_ast = Some(json_str);
                state.write().error_message = None;
            }
            Err(e) => {
                state.write().error_message = Some(format!("Parse error: {}", e));
                state.write().parsed_ast = None;
            }
        }
    };

    rsx! {
        div {
            class: "max-w-7xl mx-auto p-6 space-y-6",
            
            RustParserHeader {}
            RustParserControls {
                on_parse: handle_parse,
                on_clear: clear_output,
                on_load_example: load_example,
                is_pretty: state.read().is_pretty,
                on_toggle_pretty: toggle_pretty,
            }
            RustParserMainArea {
                state: state.read().clone(),
                on_input_change: handle_input_change,
            }
            RustParserExamples {
                on_load_example: load_example,
            }
        }
    }
}

#[component]
fn RustParserHeader() -> Element {
    rsx! {
        div {
            class: "text-center mb-8",
            h1 {
                class: "text-3xl font-bold text-gray-900 mb-2",
                "Rust AST Parser"
            }
            p {
                class: "text-gray-600",
                "Parse Rust code and view the Abstract Syntax Tree (AST) in JSON format"
            }
        }
    }
}

#[component]
fn RustParserControls(
    on_parse: EventHandler<()>,
    on_clear: EventHandler<()>,
    on_load_example: EventHandler<&'static str>,
    is_pretty: bool,
    on_toggle_pretty: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "flex flex-wrap gap-4 mb-6",
            
            button {
                class: "px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors",
                onclick: move |_| on_parse.call(()),
                "Parse Code"
            }
            
            button {
                class: "px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors",
                onclick: move |_| on_clear.call(()),
                "Clear Output"
            }
            
            button {
                class: "px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors",
                onclick: move |_| on_load_example.call(EXAMPLE_FUNCTION_CODE),
                "Load Function Example"
            }
            
            button {
                class: "px-4 py-2 bg-purple-600 text-white rounded hover:bg-purple-700 transition-colors",
                onclick: move |_| on_load_example.call(EXAMPLE_STRUCT_CODE),
                "Load Struct Example"
            }
            
            button {
                class: "px-4 py-2 bg-orange-600 text-white rounded hover:bg-orange-700 transition-colors",
                onclick: move |_| on_load_example.call(EXAMPLE_TRAIT_CODE),
                "Load Trait Example"
            }
            
            label {
                class: "flex items-center gap-2 cursor-pointer",
                input {
                    r#type: "checkbox",
                    checked: is_pretty,
                    onchange: move |_| on_toggle_pretty.call(()),
                }
                span { "Pretty Print" }
            }
        }
    }
}

#[component]
fn RustParserMainArea(
    state: RustParserState,
    on_input_change: EventHandler<Event<FormData>>,
) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 lg:grid-cols-2 gap-6",
            
            RustCodeInput {
                input_code: state.input_code,
                on_input_change: on_input_change,
            }
            
            RustAstOutput {
                parsed_ast: state.parsed_ast,
                error_message: state.error_message,
            }
        }
    }
}

#[component]
fn RustCodeInput(
    input_code: String,
    on_input_change: EventHandler<Event<FormData>>,
) -> Element {
    rsx! {
        div {
            class: "space-y-4",
            h2 {
                class: "text-xl font-semibold text-gray-800",
                "Rust Code Input"
            }
            textarea {
                class: "w-full h-96 p-4 border border-gray-300 rounded-lg font-mono text-sm bg-gray-50 focus:ring-2 focus:ring-blue-500 focus:border-transparent",
                placeholder: "Enter Rust code here...",
                value: "{input_code}",
                oninput: on_input_change,
            }
        }
    }
}

#[component]
fn RustAstOutput(
    parsed_ast: Option<String>,
    error_message: Option<String>,
) -> Element {
    rsx! {
        div {
            class: "space-y-4",
            h2 {
                class: "text-xl font-semibold text-gray-800",
                "AST Output"
            }
            
            if let Some(ast) = parsed_ast {
                div {
                    class: "w-full h-96 p-4 border border-gray-300 rounded-lg font-mono text-sm bg-gray-50 overflow-auto",
                    pre {
                        class: "whitespace-pre-wrap",
                        "{ast}"
                    }
                }
            } else if let Some(error) = error_message {
                div {
                    class: "w-full h-96 p-4 border border-red-300 rounded-lg font-mono text-sm bg-red-50 overflow-auto",
                    pre {
                        class: "whitespace-pre-wrap text-red-700",
                        "{error}"
                    }
                }
            } else {
                div {
                    class: "w-full h-96 p-4 border border-gray-300 rounded-lg font-mono text-sm bg-gray-50 flex items-center justify-center text-gray-500",
                    "Click 'Parse Code' to see the AST output"
                }
            }
        }
    }
}

// Example code strings for the RustParserExamples cards
const EXAMPLE_SIMPLE_FUNCTION_CODE: &str = r#"fn greet(name: &str) { println!("Hello, {}!", name); }"#;
const EXAMPLE_ENUM_CODE: &str = r#"enum Color { Red, Green, Blue }"#;
const EXAMPLE_MODULE_CODE: &str = r#"use std::collections::HashMap; mod utils {{ pub fn helper() -> i32 {{ 42 }}}}"#;
const EXAMPLE_MAIN_RS_LABEL: &str = "Click to load main.rs";
const EXAMPLE_APP_RS_LABEL: &str = "Click to load app.rs";
const EXAMPLE_HEADER_RS_LABEL: &str = "Click to load header.rs";
const EXAMPLE_UTILS_RS_LABEL: &str = "Click to load utils.rs";
const EXAMPLE_FETCH_PARSER_RS_LABEL: &str = "Click to load fetch_parser.rs";

#[component]
fn RustParserExamples(on_load_example: EventHandler<&'static str>) -> Element {
    rsx! {
        div {
            class: "mt-8 p-6 bg-gray-50 rounded-lg",
            h3 {
                class: "text-lg font-semibold text-gray-800 mb-4",
                "Quick Examples"
            }
            div {
                class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                ExampleCard {
                    title: "Simple Function",
                    code: EXAMPLE_SIMPLE_FUNCTION_CODE,
                    button_class: "bg-blue-100 text-blue-700 hover:bg-blue-200",
                    on_load: move |_| on_load_example.call(EXAMPLE_SIMPLE_FUNCTION_CODE),
                },
                ExampleCard {
                    title: "Enum Definition",
                    code: EXAMPLE_ENUM_CODE,
                    button_class: "bg-green-100 text-green-700 hover:bg-green-200",
                    on_load: move |_| on_load_example.call(EXAMPLE_ENUM_CODE),
                },
                ExampleCard {
                    title: "Module with Imports",
                    code: EXAMPLE_MODULE_CODE,
                    button_class: "bg-purple-100 text-purple-700 hover:bg-purple-200",
                    on_load: move |_| on_load_example.call(EXAMPLE_MODULE_CODE),
                },
                ExampleCard {
                    title: "Main.rs (Self-Parse)",
                    code: EXAMPLE_MAIN_RS_LABEL,
                    button_class: "bg-red-100 text-red-700 hover:bg-red-200",
                    on_load: move |_| on_load_example.call(EMBEDDED_MAIN_RS),
                },
                ExampleCard {
                    title: "App.rs (Self-Parse)",
                    code: EXAMPLE_APP_RS_LABEL,
                    button_class: "bg-orange-100 text-orange-700 hover:bg-orange-200",
                    on_load: move |_| on_load_example.call(EMBEDDED_APP_RS),
                },
                ExampleCard {
                    title: "Header.rs (Self-Parse)",
                    code: EXAMPLE_HEADER_RS_LABEL,
                    button_class: "bg-indigo-100 text-indigo-700 hover:bg-indigo-200",
                    on_load: move |_| on_load_example.call(EMBEDDED_HEADER_RS),
                },
                ExampleCard {
                    title: "Utils.rs (Self-Parse)",
                    code: EXAMPLE_UTILS_RS_LABEL,
                    button_class: "bg-teal-100 text-teal-700 hover:bg-teal-200",
                    on_load: move |_| on_load_example.call(EMBEDDED_UTILS_RS),
                },
                ExampleCard {
                    title: "Fetch Parser (Self-Parse)",
                    code: EXAMPLE_FETCH_PARSER_RS_LABEL,
                    button_class: "bg-pink-100 text-pink-700 hover:bg-pink-200",
                    on_load: move |_| on_load_example.call(EMBEDDED_FETCH_PARSER_RS),
                },
            }
        }
    }
}

#[component]
fn ExampleCard(
    title: &'static str,
    code: &'static str,
    button_class: &'static str,
    on_load: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "p-4 bg-white rounded border",
            h4 {
                class: "font-medium text-gray-800 mb-2",
                "{title}"
            }
            pre {
                class: "text-xs text-gray-600 mb-3",
                "{code}"
            }
            button {
                class: "w-full px-3 py-1 {button_class} rounded text-sm",
                onclick: move |_| on_load.call(()),
                "Load"
            }
        }
    }
}

const EXAMPLE_FUNCTION_CODE: &str = r#"fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}"#;

const EXAMPLE_STRUCT_CODE: &str = r#"#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: Option<String>,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Self {
            name,
            age,
            email: None,
        }
    }
    
    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
}"#;

const EXAMPLE_TRAIT_CODE: &str = r#"trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
    
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}"#;

// Embedded source code of the project for self-parsing demonstration
const EMBEDDED_MAIN_RS: &str = include_str!("../main.rs");

const EMBEDDED_APP_RS: &str = include_str!("../app.rs");

// Note: We can't include rust_parser.rs itself due to circular dependency
// Instead, we'll include a different source file
const EMBEDDED_HEADER_RS: &str = include_str!("../header.rs");

// Add some other interesting source files
const EMBEDDED_UTILS_RS: &str = include_str!("../utils.rs");

const EMBEDDED_FETCH_PARSER_RS: &str = include_str!("../fetch_parser.rs"); 
