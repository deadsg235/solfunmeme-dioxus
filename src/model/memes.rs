
// Cargo.toml
/*
[package]
name = "quine-meme-manager"
version = "0.1.0"
edition = "2021"

[dependencies]
dioxus = "0.4"
dioxus-web = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4"] }
nalgebra = "0.32"
console_error_panic_hook = "0.1"
wasm-bindgen = "0.2"
*/

use dioxus::prelude::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
//use std::fmt;
use uuid::Uuid;
use nalgebra::DVector;
use serde_json;
use crate::model::{binder::{BinderInfoData}, level::{level_to_string, Level}};


#[derive(Clone, Copy, PartialEq)]
pub struct Expression {
    //expressions: Vec<Expression>,
    // lam 
    // bind
    // app
    astring: Signal<String>
}   


#[derive(Clone, Copy, PartialEq)]
pub struct ExpressionListObj {
    expressions: Signal<Vec<Expression>>,
}   

#[derive(Clone, PartialEq)]
pub struct ExpressionList {
    expressions: Vec<Expression>,
}   

#[component]
fn ExpressionList2() -> Element {
    let state = use_signal(MemesAppState::default);

    let expression_ids = if state.read().search_query.is_empty() {
        state.read().expressions.keys().cloned().collect::<Vec<_>>()
    } else {
        state.read().filtered_expressions.clone()
    };

    rsx! {
        section {
            class: "expression-list",
            h2 { "Lifted Expressions" }
            div {
                class: "cards-container",
                for id in expression_ids {
                    if let Some(expr) = state.read().expressions.get(&id) {
                        ExpressionCard { expression: expr.clone() }
                    }
                }
            }
        }
    }
}

// Remove UseState2 and use Dioxus's built-in UseState<MemesAppState> instead.

// ============================================================================
// MONADS - Functional Programming Core
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct Maybe<T> {
    value: Option<T>,
}

impl<T> Maybe<T> {
    pub fn some(value: T) -> Self {
        Maybe { value: Some(value) }
    }
    
    pub fn none() -> Self {
        Maybe { value: None }
    }
    
    pub fn map<U, F>(self, f: F) -> Maybe<U>
    where
        F: FnOnce(T) -> U,
    {
        match self.value {
            Some(v) => Maybe::some(f(v)),
            None => Maybe::none(),
        }
    }
    
    pub fn flat_map<U, F>(self, f: F) -> Maybe<U>
    where
        F: FnOnce(T) -> Maybe<U>,
    {
        match self.value {
            Some(v) => f(v),
            None => Maybe::none(),
        }
    }
    
    pub fn unwrap_or(self, default: T) -> T {
        self.value.unwrap_or(default)
    }
    
    pub fn is_some(&self) -> bool {
        self.value.is_some()
    }
}

//#[derive(Debug)]
// pub struct IO<T> {
//     action: Box<dyn Fn() -> T + 'static>(),
// }

// impl<T> IO<T> {
//     pub fn pure(value: T) -> IO<T> 
//     where 
//         T: Clone + 'static,
//     {
//         IO {
//             action: Box::new(move || value.clone())
//         }
//     }
    
//     pub fn map<U, F>(self, f: F) -> IO<U>
//     where
//         F: Fn(T) -> U + 'static,
//         T: 'static,
//         U: 'static,
//     {
//         IO {
//             action: Box::new(move || f((self.action)())),
//         }
//     }
    
//     pub fn run(self) -> T {
//         (self.action)()
//     }
// }
// #[derive(Debug, Clone)]
// pub struct IO<T> {
//     action: fn() -> T,
// }

// impl<T> IO<T> {
//     pub fn pure(value: T) -> IO<T> 
//     where 
//         T: Clone + 'static,
//     {
//         IO {
//             action: move || value.clone(),
//         }
//     }
    
//     pub fn map<U, F>(self, f: F) -> IO<U>
//     where
//         F: Fn(T) -> U + 'static,
//         T: 'static,
//         U: 'static,
//     {
//         IO {
//             action: || f((self.action)()),
//         }
//     }
    
//     pub fn run(self) -> T {
//         (self.action)()
//     }
// }

// ============================================================================
// MODEL - Data Layer
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Quine {
    pub id: String,
    pub expression: String,
    pub self_reference: String,
    pub complexity_score: f64,
}

impl Quine {
    pub fn new(expression: String) -> Self {
        let id = Uuid::new_v4().to_string();
        let self_reference = format!("fn quine() -> String {{ \"{}\" }}", expression);
        let complexity_score = expression.len() as f64 * 0.1;
        
        Quine {
            id,
            expression,
            self_reference,
            complexity_score,
        }
    }
    
    pub fn vectorize(&self) -> DVector<f64> {
        let chars: Vec<f64> = self.expression
            .chars()
            .map(|c| c as u8 as f64)
            .collect();
        
        let mut vector = vec![0.0; 256]; // ASCII vector space
        for &char_val in &chars {
            if char_val < 256.0 {
                vector[char_val as usize] += 1.0;
            }
        }
        
        vector.push(self.complexity_score);
        vector.push(chars.len() as f64);
        
        DVector::from_vec(vector)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Meme {
    pub id: String,
    pub content: String,
    pub virality_score: f64,
    pub semantic_tags: Vec<String>,
    pub propagation_count: u32,
}

impl Meme {
    pub fn new(content: String, semantic_tags: Vec<String>) -> Self {
        let id = Uuid::new_v4().to_string();
        let virality_score = content.len() as f64 * semantic_tags.len() as f64 * 0.01;
        
        Meme {
            id,
            content,
            virality_score,
            semantic_tags,
            propagation_count: 0,
        }
    }
    
    pub fn propagate(&mut self) {
        self.propagation_count += 1;
        self.virality_score *= 1.1;
    }
    
    pub fn vectorize(&self) -> DVector<f64> {
        let content_chars: Vec<f64> = self.content
            .chars()
            .map(|c| c as u8 as f64)
            .collect();
        
        let mut vector = vec![0.0; 256]; // ASCII vector space
        for &char_val in &content_chars {
            if char_val < 256.0 {
                vector[char_val as usize] += 1.0;
            }
        }
        
        vector.push(self.virality_score);
        vector.push(self.propagation_count as f64);
        vector.push(self.semantic_tags.len() as f64);
        
        DVector::from_vec(vector)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LiftedExpression {
    pub id: String,
    pub quine: Option<Quine>,
    pub meme: Option<Meme>,
    pub lifted_at: String,
    pub vector_representation: Vec<f64>,    
    pub expr: SimpleExpr,
    pub name: String,
    pub description: String,    
    
    pub tags: Vec<String>,
}
impl LiftedExpression {
    // pub fn from_quine(quine: from_quine) -> Self {
    //     let vector_representation = quine.vectorize().data.as_vec().clone();
        
    //     LiftedExpression {
    //         id: Uuid::new_v4().to_string(),
    //         quine: Some(quine),
    //         meme: None,
    //         lifted_at: chrono::Utc::now().to_rfc3339(),
    //         vector_representation,
    //     }
    // }
    
    pub fn from_meme(meme: Meme) -> Self {
        let vector_representation = meme.vectorize().data.as_vec().clone();
        
        LiftedExpression {
            id: Uuid::new_v4().to_string(),
            quine: None,
            meme: Some(meme.clone()),
            lifted_at: chrono::Utc::now().to_rfc3339(),
            vector_representation,
            expr: SimpleExpr::const_expr(Name::new("meme".to_string()), vec![]),
            name: "Meme".to_string(),
            description: "".to_string(),
            tags: meme.semantic_tags.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemeExpressionType {
    Quine,
    Meme,
}
#[derive(Debug, Clone, Default)]
pub struct MemesAppState {
    pub expressions: HashMap<String, LiftedExpression>,
    pub current_input: String,
    pub current_tags: String,
    pub meme_expression_type : MemeExpressionType,
    pub expression_type: ExpressionType,
    pub search_query: String,
    pub filtered_expressions: Vec<String>,
}



#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionType {
    BVar,
    Sort,
    Const,
    App,
    Lambda,
    Forall,
    FromString,
}

impl Default for MemeExpressionType {
    fn default() -> Self {
        MemeExpressionType::Meme
    }
}

// ============================================================================
// CONTROLLER - Business Logic Layer
// ============================================================================



pub struct Controller;

impl Controller {
        pub fn add_expression() {}
        pub fn create_expression_from_type(state: &MemesAppState) -> Option<LiftedExpression> {
        // Example logic: create a LiftedExpression based on the current expression_type and inputs
        match state.meme_expression_type.clone() {
            MemeExpressionType::Quine | MemeExpressionType::Meme => {
                let content = state.current_input.trim();
                if content.is_empty() {
                    return None;
                }
                let meme = Meme::new(content.to_string(), vec![]);
                Some(LiftedExpression::from_meme(meme))
            }
            // Add logic for other ExpressionType variants as needed
            _ => None,
        }
    }
    // ... other methods ...

    pub fn add_quine_expression(
        state: &mut MemesAppState,
        expression: String,
    ) -> Maybe<LiftedExpression> {
        if expression.trim().is_empty() {
            return Maybe::none();
        }
        
        let meme = Meme::new(expression, Vec::new());
        let lifted = LiftedExpression::from_meme(meme);
        let id = lifted.id.clone();
        
        state.expressions.insert(id.clone(), lifted.clone());
        state.current_input.clear();
        
        Maybe::some(lifted)
    }
    
    pub fn add_meme_expression(
        state: &mut MemesAppState,
        content: String,
        tags: Vec<String>,
    ) -> Maybe<LiftedExpression> {
        if content.trim().is_empty() {
            return Maybe::none();
        }
        
        let meme = Meme::new(content, tags);
        let lifted = LiftedExpression::from_meme(meme);
        let id = lifted.id.clone();
        
        state.expressions.insert(id.clone(), lifted.clone());
        state.current_input.clear();
        state.current_tags.clear();
        
        Maybe::some(lifted)
    }
    
    pub fn search_expressions(
        state: &mut MemesAppState,
        query: String,
    ) -> Vec<String> {
        let query_lower = query.to_lowercase();
        
        let filtered: Vec<String> = state
            .expressions
            .iter()
            .filter(|(_, expr)| {
                if let Some(ref quine) = expr.quine {
                    quine.expression.to_lowercase().contains(&query_lower)
                } else if let Some(ref meme) = expr.meme {
                    meme.content.to_lowercase().contains(&query_lower) ||
                    meme.semantic_tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
                } else {
                    false
                }
            })
            .map(|(id, _)| id.clone())
            .collect();
        
        state.filtered_expressions = filtered.clone();
        filtered
    }
    
    pub fn propagate_meme(state: &mut MemesAppState, expression_id: String) -> Maybe<()> {
        if let Some(expr) = state.expressions.get_mut(&expression_id) {
            if let Some(ref mut meme) = expr.meme {
                meme.propagate();
                expr.vector_representation = meme.vectorize().data.as_vec().clone();
                return Maybe::some(());
            }
        }
        Maybe::none()
    }
    
    pub fn delete_expression(state: &mut MemesAppState, expression_id: String) -> Maybe<()> {
        match state.expressions.remove(&expression_id) {
            Some(_) => Maybe::some(()),
            None => Maybe::none(),
        }
    }
    
    pub fn get_vector_similarity(expr1: &LiftedExpression, expr2: &LiftedExpression) -> f64 {
        let v1 = DVector::from_vec(expr1.vector_representation.clone());
        let v2 = DVector::from_vec(expr2.vector_representation.clone());
        
        let dot_product = v1.dot(&v2);
        let norm1 = v1.norm();
        let norm2 = v2.norm();
        
        if norm1 == 0.0 || norm2 == 0.0 {
            0.0
        } else {
            dot_product / (norm1 * norm2)
        }
    }
}
            //ExpressionList()
// ============================================================================

// fn main() {
//     console_error_panic_hook::set_once();
//     dioxus_web::launch(App);
// }
// fn use_signal2<T: 'static + Default>() -> UseState<T> {
//         use_signal(T::default)
//     }    

#[component]
fn App() -> Element {
    //let state = use_signal(cx, MemesAppState::default);
    //let state = use_signal2(MemesAppState::default);
    // Custom use_signal2 hook for demonstration (returns the same as use_signal)
    
    rsx! {
        div {
            class: "app-container",
            Header {}
            InputSection {
               // state: state,
            }
            ExpressionList2 {
                //state: state,
            }
            VectorSpace {
                //state: state,
            }
            Footer {}
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        header {
            class: "app-header",
            h1 { 
                class: "title",
                "🧠 Lifted Language Expression Manager" 
            }
            p { 
                class: "subtitle",
                "Manage Quines & Memes in Vector Space" 
            }
        }
    }
}

//#[derive(Props)]
//struct InputSectionProps {
    //state: UseState<MemesAppState>,
//}
//cx: Scope<InputSectionProps>
#[component]
fn InputSection() -> Element {
    //let state = &cx.props.state;
    let mut state = use_signal(MemesAppState::default);

    rsx! {
        section {
            class: "input-section",
            div {
                class: "type-selector",
                label {
                    input {
                        r#type: "radio",
                        name: "expression_type",
                        checked: state.read().meme_expression_type == MemeExpressionType::Quine,
                        onchange: move |_| {
                            state.with_mut(|s| s.meme_expression_type = MemeExpressionType::Quine);
                        },
                    }
                    "Quine"
                }
                label {
                    input {
                        r#type: "radio",
                        name: "expression_type",
                        checked: state.read().meme_expression_type == MemeExpressionType::Meme,
                        onchange: move |_| {
                            state.with_mut(|s| s.meme_expression_type = MemeExpressionType::Meme);
                        },
                    }
                    "Meme"
                }
            }
            
            div {
                class: "input-controls",
                textarea {
                    class: "expression-input",
                    placeholder: match state.read().meme_expression_type {
                        MemeExpressionType::Quine => "Enter quine expression...",
                        MemeExpressionType::Meme => "Enter meme content...",
                    },
                    value: state.read().current_input.clone(),
                    oninput: move |evt| {
                        state.with_mut(|s| s.current_input = evt.value().clone());
                    },
                }
                
                { 
                    if state.read().meme_expression_type == MemeExpressionType::Meme {
                        
                    }
                }
            }
        }
    }
}

//pub struct Scope {}

fn InputSection2() -> Element {
    let mut  state = use_signal(MemesAppState::default);

    rsx! {
        section {
            class: "input-section",
            div {
                class: "type-selector",
                label {
                    input {
                        r#type: "radio",
                        name: "expression_type",
                        checked: state.read().meme_expression_type == MemeExpressionType::Quine,
                        onchange: move |_| {
                            state.write().meme_expression_type = MemeExpressionType::Quine;
                        },
                    }
                    "Quine"
                }
                label {
                    input {
                        r#type: "radio",
                        name: "expression_type",
                        checked: state.read().meme_expression_type == MemeExpressionType::Meme,
                        onchange: move |_| {
                            state.write().meme_expression_type = MemeExpressionType::Meme;
                        },
                    }
                    "Meme"
                }
            }

            div {
                class: "input-controls",
                textarea {
                    class: "expression-input",
                    placeholder: match state.read().meme_expression_type {
                        MemeExpressionType::Quine => "Enter quine expression...",
                        MemeExpressionType::Meme => "Enter meme content...",
                    },
                    value: state.read().current_input.clone(),
                    oninput: move |evt| {
                        state.write().current_input = evt.value().to_string();
                    },
                }

                {
                    if state.read().meme_expression_type == MemeExpressionType::Meme {
                        Some(rsx! {
                            input {
                                class: "tags-input",
                                placeholder: "Semantic tags (comma-separated)...",
                                value: state.read().current_tags.clone(),
                                oninput: move |evt| {
                                    state.write().current_tags = evt.value().to_string();
                                },
                            }
                        })
                    } else {
                        None
                    }
                }

                button {
                    class: "add-button",
                    onclick: move |_| {
                        let expression_type = state.read().meme_expression_type.clone();
                        let current_input = state.read().current_input.clone();
                        let current_tags = state.read().current_tags.clone();

                        let result = match expression_type {
                            MemeExpressionType::Quine => {
                                Controller::add_quine_expression(
                                    &mut state.write(),
                                    current_input
                                )
                            },
                            MemeExpressionType::Meme => {
                                let tags: Vec<String> = current_tags
                                    .split(',')
                                    .map(|s| s.trim())
                                    .filter(|s| !s.is_empty())
                                    .map(|s| s.to_string())
                                    .collect();

                                Controller::add_meme_expression(
                                    &mut state.write(),
                                    current_input,
                                    tags
                                )
                            }
                        };

                        if result.is_some() {
                            // Expression added successfully
                        }
                    },
                    "Lift Expression"
                }
            }

            div {
                class: "search-section",
                input {
                    class: "search-input",
                    placeholder: "Search expressions...",
                    value: state.read().search_query.clone(),
                    oninput: move |evt| {
                        let query = evt.value().to_string();
                        state.write().search_query = query.clone();
                        Controller::search_expressions(&mut state.write(), query);
                    },
                }
            }
        }
    }
}

// Add this component for rendering an expression card
#[derive(Props, PartialEq, Clone)]
struct ExpressionCardProps {
    expression: LiftedExpression,
}

#[component]
fn ExpressionCard(props: ExpressionCardProps) -> Element {
    let expr = props.expression.clone();
    let mut state = use_signal(MemesAppState::default);
    let expr_id = expr.id.clone();

    rsx! {
        div {
            class: "expression-card",

            div {
                class: "card-header",
                span {
                    class: "expression-type",
                    if expr.quine.is_some() { "🔄 QUINE" } else { "🎭 MEME" }
                }
                button {
                    class: "delete-button",
                    onclick: move |_| {
                        Controller::delete_expression(&mut state.write(), expr_id.clone());
                    },
                    "×"
                }
            }

            div {
                class: "card-content",
                {
                    if let Some(ref quine) = expr.quine {
                        Some(rsx! {
                            div {
                                p {
                                    class: "expression-text",
                                    strong { "Expression: " }
                                    "{quine.expression}"
                                }
                                p {
                                    class: "self-reference",
                                    strong { "Self-Reference: " }
                                    code { "{quine.self_reference}" }
                                }
                                p {
                                    class: "complexity",
                                    strong { "Complexity: " }
                                    "{quine.complexity_score:.2}"
                                }
                            }
                        })
                    } else if let Some(ref meme) = expr.meme {
                        Some(rsx! {
                            div {
                                p {
                                    class: "meme-content",
                                    strong { "Content: " }
                                    "{meme.content}"
                                }
                                p {
                                    class: "semantic-tags",
                                    strong { "Tags: " }
                                    "{meme.semantic_tags.join(\", \")}"
                                }
                                p {
                                    class: "virality",
                                    strong { "Virality: " }
                                    "{meme.virality_score:.2}"
                                }
                                p {
                                    class: "propagation",
                                    strong { "Propagations: " }
                                    // Move expr_id binding outside rsx! and use it here
                                    button {
                                        class: "propagate-button",
                                        onclick: {
                                            let expr_id = expr.id.clone();
                                            move |_| {
                                                Controller::propagate_meme(&mut state.write(), expr_id.clone());
                                            }
                                        },
                                        "🚀 Propagate"
                                    }
                                }
                            }
                        })
                    } else {
                        None
                    }
                }
            }

            div {
                class: "vector-info",
                p {
                    strong { "Vector Dimensions: " }
                    "{expr.vector_representation.len()}"
                }
                p {
                    strong { "Lifted: " }
                    "{expr.lifted_at}"
                }
            }
        }
    }
}

// #[derive(Props)]
// struct VectorSpaceProps<'a> {
//     state: &'a UseState<MemesAppState>,
// }

#[component]
fn VectorSpace() -> Element {
    let state = use_signal(MemesAppState::default);
    
    let avg_vector_dim = if state.read().expressions.is_empty() {
        "0".to_string()
    } else {
        let avg = state.read().expressions.values()
            .map(|e| e.vector_representation.len())
            .sum::<usize>() as f64 / state.read().expressions.len() as f64;
        format!("{:.0}", avg)
    };

    rsx! {
        section {
            class: "vector-space",
            h2 { "Vector Space Analysis" }
            
            div {
                class: "stats-grid",
                div {
                    class: "stat-card",
                    h3 { "Total Expressions" }
                    p { class: "stat-value", "{state.read().expressions.len()}" }
                }
                
                div {
                    class: "stat-card",
                    h3 { "Quines" }
                    p { 
                        class: "stat-value",
                        "{state.read().expressions.values().filter(|e| e.quine.is_some()).count()}"
                    }
                }
                
                div {
                    class: "stat-card",
                    h3 { "Memes" }
                    p { 
                        class: "stat-value",
                        "{state.read().expressions.values().filter(|e| e.meme.is_some()).count()}"
                    }
                }
                
                div {
                    class: "stat-card",
                    h3 { "Avg Vector Dim" }
                    p { 
                        class: "stat-value",
                        "{avg_vector_dim}"
                    }
                }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer {
            class: "app-footer",
            p { "Functional Reactive Architecture • Rust + Dioxus • MVC Pattern" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meme_new_initializes_fields() {
        let content = "Test meme".to_string();
        let tags = vec!["funny".to_string(), "rust".to_string()];
        let meme = Meme::new(content.clone(), tags.clone());

        assert_eq!(meme.content, content);
        assert_eq!(meme.semantic_tags, tags);
        assert_eq!(meme.propagation_count, 0);
        assert!(!meme.id.is_empty());
        assert!(meme.virality_score > 0.0);
    }

    #[test]
    fn test_meme_propagate_increases_count_and_virality() {
        let mut meme = Meme::new("viral".to_string(), vec!["tag".to_string()]);
        let initial_score = meme.virality_score;
        let initial_count = meme.propagation_count;

        meme.propagate();

        assert_eq!(meme.propagation_count, initial_count + 1);
        assert!(meme.virality_score > initial_score);
    }

    #[test]
    fn test_meme_vectorize_dimensions_and_values() {
        let meme = Meme::new("abc".to_string(), vec!["x".to_string(), "y".to_string()]);
        let vector = meme.vectorize();

        // 256 ASCII + virality + propagation + tags
        assert_eq!(vector.len(), 259);
        // Check that the vector contains the virality score, propagation count, and tag count at the end
        assert_eq!(vector[256], meme.virality_score);
        assert_eq!(vector[257], meme.propagation_count as f64);
        assert_eq!(vector[258], meme.semantic_tags.len() as f64);
    }

    #[test]
    fn test_meme_serialization_roundtrip() {
        let meme = Meme::new("serialize".to_string(), vec!["serde".to_string()]);
        let json = serde_json::to_string(&meme).unwrap();
        let deserialized: Meme = serde_json::from_str(&json).unwrap();
        assert_eq!(meme, deserialized);
    }
}




// from lean meme 

// ============================================================================
// CORE TYPES - Lambda Calculus Expression System
// ============================================================================

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct MemeLevel {
//     pub value: u64,
// }

// impl Level {
//     pub fn new(value: u64) -> Self {
//         Level { value }
//     }
    
//     pub fn zero() -> Self {
//         Level { value: 0 }
//     }
    
//     pub fn succ(&self) -> Self {
//         Level { value: self.value + 1 }
//     }
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Name {
    pub name: String,
}

impl Name {
    pub fn new(name: String) -> Self {
        Name { name }
    }
    
    pub fn anonymous() -> Self {
        Name { name: "_".to_string() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimpleExpr {
    BVar { index: u64 },
    Sort { level: Level },
    Const { name: Name, levels: Vec<Level> },
    App { func: Box<SimpleExpr>, arg: Box<SimpleExpr> },
    Lam {
        binder_name: Name,
        binder_type: Box<SimpleExpr>,
        body: Box<SimpleExpr>,
        binder_info: BinderInfoData,
    },
    ForallE {
        binder_name: Name,
        binder_type: Box<SimpleExpr>,
        body: Box<SimpleExpr>,
        binder_info: BinderInfoData,
    },
}

impl SimpleExpr {
    // Constructor methods
    pub fn bvar(index: u64) -> Self {
        SimpleExpr::BVar { index }
    }

    pub fn sort(level: Level) -> Self {
        SimpleExpr::Sort { level }
    }

    pub fn const_expr(name: Name, levels: Vec<Level>) -> Self {
        SimpleExpr::Const { name, levels }
    }

    pub fn app(func: SimpleExpr, arg: SimpleExpr) -> Self {
        SimpleExpr::App {
            func: Box::new(func),
            arg: Box::new(arg),
        }
    }

    pub fn lam(name: Name, binder_type: SimpleExpr, body: SimpleExpr, info: BinderInfoData) -> Self {
        SimpleExpr::Lam {
            binder_name: name,
            binder_type: Box::new(binder_type),
            body: Box::new(body),
            binder_info: info,
        }
    }

    pub fn forall_e(name: Name, binder_type: SimpleExpr, body: SimpleExpr, info: BinderInfoData) -> Self {
        SimpleExpr::ForallE {
            binder_name: name,
            binder_type: Box::new(binder_type),
            body: Box::new(body),
            binder_info: info,
        }
    }
    
    // Display methods
    pub fn to_string(&self) -> String {
        match self {
            SimpleExpr::BVar { index } => format!("#{}", index),
            SimpleExpr::Sort { level } => format!("Sort({})", level_to_string(level)),
            SimpleExpr::Const { name, levels } => {
                if levels.is_empty() {
                    name.name.clone()
                } else {
                    format!("{}.{{{}}}", name.name, 
                           levels.iter().map(|l| level_to_string(l)).collect::<Vec<_>>().join(", "))
                }
            },
            SimpleExpr::App { func, arg } => {
                format!("({} {})", func.to_string(), arg.to_string())
            },
            SimpleExpr::Lam { binder_name, binder_type, 
                body, 
                binder_info 
            } => {
                let implicit_marker = if binder_info.implicit { "{" } else { "(" };
                let implicit_end = if binder_info.implicit { "}" } else { ")" };
                format!("λ {}{} : {}{} → {}", 
                       implicit_marker, binder_name.name, binder_type.to_string(), implicit_end,
                       body.to_string())
            },
            SimpleExpr::ForallE { binder_name, binder_type, body, binder_info } => {
                let implicit_marker = if binder_info.implicit { "{" } else { "(" };
                let implicit_end = if binder_info.implicit { "}" } else { ")" };
                format!("∀ {}{} : {}{}, {}", 
                       implicit_marker, binder_name.name, binder_type.to_string(), implicit_end,
                       body.to_string())
            }
        }
    }
    
    // Calculate complexity based on AST depth and node count
    pub fn complexity(&self) -> f64 {
        match self {
            SimpleExpr::BVar { .. } | SimpleExpr::Sort { .. } | SimpleExpr::Const { .. } => 1.0,
            SimpleExpr::App { func, arg } => 1.0 + func.complexity() + arg.complexity(),
            SimpleExpr::Lam { binder_type, body, .. } => 2.0 + binder_type.complexity() + body.complexity(),
            SimpleExpr::ForallE { binder_type, body, .. } => 2.0 + binder_type.complexity() + body.complexity(),
        }
    }
    
    // Vectorize expression for similarity calculations
    pub fn vectorize(&self) -> DVector<f64> {
        let mut features = vec![0.0; 10]; // Feature vector
        
        // Node type features
        match self {
            SimpleExpr::BVar { index } => {
                features[0] = 1.0; // BVar indicator
                features[5] = *index as f64; // Index value
            },
            SimpleExpr::Sort { level } => {
                features[1] = 1.0; // Sort indicator
                features[6] = level_to_int(level) as f64; // Level value
            },
            SimpleExpr::Const { levels, .. } => {
                features[2] = 1.0; // Const indicator
                features[7] = levels.len() as f64; // Number of levels
            },
            SimpleExpr::App { func, arg } => {
                features[3] = 1.0; // App indicator
                let func_complexity = func.complexity();
                let arg_complexity = arg.complexity();
                features[8] = func_complexity + arg_complexity;
            },
            SimpleExpr::Lam { binder_info, .. } => {
                features[4] = 1.0; // Lam indicator
                features[9] = if binder_info.implicit { 1.0 } else { 0.0 };
            },
            SimpleExpr::ForallE { binder_info, .. } => {
                features[4] = 2.0; // ForallE indicator (distinct from Lam)
                features[9] = if binder_info.implicit { 1.0 } else { 0.0 };
            }
        }
        
        DVector::from_vec(features)
    }
}

fn level_to_int(level: &Level) -> f64 {
    todo!()
}

// ============================================================================
// LIFTED EXPRESSION - Wrapper for GUI Management
// ============================================================================



impl LiftedExpression {
    pub fn new(expr: SimpleExpr, name: String, description: String, tags: Vec<String>) -> Self {
        let vector_representation = expr.vectorize().data.as_vec().clone();
        
        LiftedExpression {
            id: Uuid::new_v4().to_string(),
            quine: None,
            meme: None,
            lifted_at: "2024-01-01T00:00:00Z".to_string(), // Would use chrono in real app
            vector_representation,
            expr,
            name,
            description,
            tags,
        }
    }
}

// ============================================================================
// APPLICATION STATE
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct MemeAppState {
    pub expressions: HashMap<String, LiftedExpression>,
    pub current_input: String,
    pub current_name: String,
    pub current_description: String,
    pub current_tags: String,
    pub expression_type: ExpressionType,
    pub search_query: String,
    pub filtered_expressions: Vec<String>,
    
    // For building complex expressions
    pub binder_name: String,
    pub index_input: String,
    pub level_input: String,
    pub const_name: String,
    pub implicit_binder: bool,
}


impl Default for ExpressionType {
    fn default() -> Self {
        ExpressionType::FromString
    }
}