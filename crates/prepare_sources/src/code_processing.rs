use anyhow::Result;
use std::io::Write;
use solfunmeme_input_fs::read_code_chunks;
use solfunmeme_function_analysis::CodeChunk;
use solfunmeme_clifford::SerializableMultivector;
use tclifford::Multivector;

pub fn process_code_chunks(path: Option<String>, limit: Option<usize>, output_writer: &mut Box<dyn Write>) -> Result<()> {
    let mut code_chunks = read_code_chunks(path, limit)?;

    eprintln!("[INFO] Processing {} chunks:", code_chunks.len());

    let mut processed_count = 0;
    for mut chunk in code_chunks.drain(..) {
        // Simulate embedding and Clifford vector generation
        let mut coeffs = [0.0; 8];
        // In a real scenario, you'd use a proper embedding model here.
        // For simulation, we'll just use some dummy values or a hash of the content.
        // For now, let's just use a simple incrementing value for demonstration.
        for i in 0..8 {
            coeffs[i] = (processed_count as f32 + i as f32) / 100.0;
        }
        let clifford_vector = SerializableMultivector(Multivector::from_vector(coeffs.to_vec()).unwrap());
        chunk.clifford_vector = Some(clifford_vector);
        chunk.embedding = coeffs.to_vec(); // Store the simulated embedding

        let json_chunk = serde_json::to_string(&chunk)?;
        writeln!(output_writer, "{}", json_chunk)?;

        processed_count += 1;
        if processed_count % 100 == 0 {
            eprintln!("[INFO] Processed {} chunks so far...", processed_count);
        }
    }
    eprintln!("[INFO] Finished processing all {} chunks.", processed_count);

    Ok(())
}
