# Sophia RDF Library Migration Policy (gemini.md)

## Overview
This document provides guidance for handling user queries related to Sophia RDF library migration issues, particularly the breaking changes between v0.7 and v0.8.

## Key Migration Issues to Recognize

### 1. Import Path Changes
**Problem**: Users importing `sophia_api::term::literal::Literal` or `sophia_api::term::bnode::BNode`
**Solution**: These modules no longer exist. Direct users to use `SimpleTerm` variants instead.

### 2. `SimpleTerm` enum structure changed
**Problem**: Code using `SimpleTerm::Literal` or `SimpleTerm::BNode` variants
**Solution**: 

Replace with:
- `SimpleTerm::LiteralDatatype(MownStr<'a>, IriRef<MownStr<'a>>)` for typed literals
- `SimpleTerm::LiteralLanguage(MownStr<'a>, LanguageTag<MownStr<'a>>)` for language-tagged literals  
- `SimpleTerm::BlankNode(BnodeId<MownStr<'a>>)` for blank nodes

### 3. IRI type conversion issues
**Problem**: `sophia_iri::Iri<String>` cannot be directly converted to `IriRef<MownStr<'_>>` . You need to use the proper conversion methods.

### 4. Parser API changes
**Problem**: `TurtleParser::new()` doesn't exist in the current API .

## Migration Solutions

**For literals**, replace:
```rust
SimpleTerm::Literal(Literal::new_untyped(value))
```
with:
```rust
SimpleTerm::LiteralDatatype(value.into(), xsd::string.iri().unwrap())
```

**For blank nodes**, replace:
```rust
SimpleTerm::BNode(BNode::new(id)?)
```
with:
```rust
SimpleTerm::BlankNode(BnodeId::new(id)?)
```

**For IRIs**, use proper conversion:
```rust
let iri_ref = IriRef::new_unchecked(MownStr::from(iri_string));
SimpleTerm::Iri(iri_ref)
```

The v0.8 migration introduced GATs (Generic Associated Types) and significantly restructured the Term trait system. You'll need to update your code to use the new `SimpleTerm` variants and construction patterns.

## Notes

Consider using the `FromTerm` and `TryFromTerm` traits for easier term construction. The migration guide in the Sophia book provides comprehensive examples for updating v0.7 code.

Wiki pages you might want to explore:
- [Overview (pchampin/sophia_rs)](/wiki/pchampin/sophia_rs#1)
- [API Foundation (pchampin/sophia_rs)](/wiki/pchampin/sophia_rs#2.1)