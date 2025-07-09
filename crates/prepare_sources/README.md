## Usage

To run the tool, use the following command:

```
cargo run -- <file_path>
```

This will process the specified file and:

1.  Generate a BERT embedding for its content.
2.  Reduce the BERT embedding to an 8-dimensional Clifford multivector.
3.  Calculate a sieve address based on the multivector's components.
4.  Find the closest emoji representation based on a semantic ontology.

Example Output:

```
Processing file: src/lib.rs
Successfully generated multivector for file: MultivectorBase { ... }
Sieve Address: 00111010
Closest emoji: ➡️🟦 (Math) with distance: 0.04577425
```