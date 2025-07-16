pub fn get_multivector_norm(mv: &SolMultivector) -> f32 {
    mv.norm()
}

pub fn get_multivector_coefficients(mv: &SolMultivector) -> Vec<f32> {
    mv.coefficients()
}