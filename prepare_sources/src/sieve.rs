use crate::clifford::SolMultivector;

pub fn get_sieve_address(multivector: &SolMultivector) -> String {
    let mut address = String::with_capacity(8);

    // The first 8 coefficients of the multivector correspond to the e1 through e8 basis vectors.
    // In tclifford, basis elements are indexed by their bitmask representation.
    for i in 0..8 {
        // The index for e(i+1) is 1 << i.
        let component = multivector.get_by_idx(1 << i);
        if component >= 0.0 {
            address.push('1');
        } else {
            address.push('0');
        }
    }
    address
}


