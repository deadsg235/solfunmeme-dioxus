use tclifford::{declare_algebra, Multivector};

declare_algebra!(pub SolCliffordAlgebra, [+, +, +], []);

pub type SolMultivector = Multivector<f32, SolCliffordAlgebra>;
pub use SolCliffordAlgebra as SolCl;

pub fn get_multivector_norm(mv: &SolMultivector) -> f32 {
    mv.mag2().sqrt()
}

pub fn get_multivector_coefficients(mv: &SolMultivector) -> Vec<f32> {
    mv.coeff_array_view().to_vec()
}
