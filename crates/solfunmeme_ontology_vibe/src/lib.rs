use anyhow::Result;
use sophia_inmem::graph::FastGraph;
use sophia_iri::Iri;

mod loader;
mod crate_data_processor;
mod emoji_data_processor;
mod serializer;

pub use loader::load_graph_internal as load_graph;
pub use crate_data_processor::add_crate_data_internal as add_crate_data;
pub use emoji_data_processor::add_emoji_data_internal as add_emoji_data;
pub use serializer::serialize_graph_internal as serialize_graph;
