use dioxus::prelude::*;
use crate::model::lean::style::Styles;
use crate::model::lean::level::{
    level_to_string, levels_8, levels_8f, LEVEL_U1F, LevelDescr,
};
use crate::model::lean::name::Name;
use crate::model::lean::binder::BinderInfoData;
use crate::model::simple_expr::{
    CnstInf, CnstInf2, CnstInfB, CnstInfB2, Foo, Foo2, Forbd, Forbd2, Name as SimpleName,
    Sig, Sig2, SimpleExpr, SimpleExprType,
};
use crate::model::lean::types::{
    cnst_inf::{CnstInf as LeanCnstInf, CnstInfB as LeanCnstInfB},
    forbd::Forbd as LeanForbd,
    sig::Sig as LeanSig,
};

#[component]
pub fn ExpressionParsing() -> Element {
    let mut state = use_signal(|| 0);
    let level = use_signal(|| LEVEL_U1F());
    let level_str = use_signal(|| level_to_string(&level.read()));
    let levels = use_signal(|| levels_8());
    let levels_f = use_signal(|| levels_8f());

    let simple_expr = use_signal(|| SimpleExpr::Const { 
        name: SimpleName::Anonymous, 
        levels: Vec::new() 
    });

    let app_expr = use_signal(|| SimpleExpr::app(
        simple_expr.read().clone(), 
        SimpleExpr::sort(level.read().clone())));
    let lam_expr = use_signal(|| SimpleExpr::lam(
        SimpleName::Anonymous,
        simple_expr.read().clone(),
        SimpleExpr::bvar(1),
        BinderInfoData::default(),
    ));

    let level_descr = use_signal(|| LevelDescr { level: level_str.read().clone(), kind: "example_kind".to_string() });

    rsx! {
        div {
            class: "{Styles::section()}",
            h2 { class: "{Styles::section_title()}", "Expression Parsing" }
            input {
                class: "{Styles::search_input()}",
                value: "{state.read()}",
                oninput: move |e| {
                    if let Ok(value) = e.value().parse() {
                        state.set(value);
                    }
                },
                placeholder: "Search expressions"
            }
            div {
                {format!("Expression Type: {:?}", state.read())}
                br {}
                {format!("Level: {}", level_str.read())}
                br {}
                {format!("Levels: {:?}", levels.read())}
            }
        }
    }
} 