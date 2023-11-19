use simple_websockets::Responder;

#[inline]
pub fn assert_dim(dim: u8, values: &[Vec<f64>]) {
    assert!(values.iter().all(|s|s.len() == dim as usize))
}

pub fn nearest_neighbor(client: &Responder, dim: u8, values: &mut [Vec<f64>]) {

}

pub fn brute_force(client: &Responder, dim: u8, values: &mut [Vec<f64>]) {

}
