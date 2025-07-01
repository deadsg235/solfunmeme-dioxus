use rust_embed::Embed;

#[derive(Embed)]
#[folder = "src"]
struct OurSource;

#[derive(Embed)]
#[folder = "src/bin"]
struct OurSourceBin;

#[derive(Embed)]
#[folder = "src/extractor"]
struct OurSourceExtractor;

#[derive(Embed)]
#[folder = "src/extractor/components"]
struct OurSourceExtractorComponents;

#[derive(Embed)]
#[folder = "src/extractor/model"]
struct OurSourceExtractorModel;

#[derive(Embed)]
#[folder = "src/extractor/system"]
struct OurSourceExtractorSystem;

#[derive(Embed)]
#[folder = "src/model"]
struct OurSourceModel;

#[derive(Embed)]
#[folder = "src/model/git"]
struct OurSourceModelGit;

#[derive(Embed)]
#[folder = "src/model/lean"]
struct OurSourceModelLean;

#[derive(Embed)]
#[folder = "src/model/lean/types"]
struct OurSourceModelLeanTypes;

#[derive(Embed)]
#[folder = "src/model/math"]
struct OurSourceModeMath;

#[derive(Embed)]
#[folder = "src/playground"]
struct OurSourcePlayground;

#[derive(Embed)]
#[folder = "src/state"]
struct OurSourceState;

#[derive(Embed)]
#[folder = "src/views"]
struct OurSourceView;

#[derive(Embed)]
#[folder = "src/views/component_memes"]
struct OurSourceViewComponent;

#[derive(Embed)]
#[folder = "src/views/crypto_frontend"]
struct OurSourceViewCrypto;

#[derive(Embed)]
#[folder = "src/views/extras_views"]
struct OurSourceViewextra;

#[derive(Embed)]
#[folder = "src/views/wikidata_memes"]
struct OurSourceViewWikwidata;

#[derive(Embed)]
#[folder = "src/views/workflow_memes"]
struct OurSourceViewWorkflow;

use dioxus_logger::tracing::info;

pub fn printall(){

    info!("PRINT ALL");
    for file in OurSource::iter() {
	println!("{}", file.as_ref());
	info!("print {}", file.as_ref());
    }
}
