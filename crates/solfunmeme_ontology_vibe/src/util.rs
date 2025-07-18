use sophia_api::term::{SimpleTerm, Term};
use sophia_api::MownStr;

pub fn create_literal_simple_term(s: &str) -> SimpleTerm<'static> {
    SimpleTerm::LiteralDatatype(
        s.to_string().into(),
        sophia_api::ns::xsd::string.iriref().to_owned(),
    )
}
