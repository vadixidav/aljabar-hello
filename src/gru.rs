//! Lets test if we can make a basic GRU neural network forward prop without infinitely expanding matrix bounds.
//! 
//! Because aljabar uses column vectors, we will use them.

use aljabar::{Matrix, Vector};

#[derive(Clone, Debug)]
struct NNet<F, const I: usize, const O: usize> {
    hidden_matrix: Matrix<f32, {O}, {O}>,
    input_matrix: Matrix<f32, {O}, {I}>,
    biases: Vector<f32, {O}>,
    activation: F,
}

impl<F, const I: usize, const O: usize> NNet<F, {I}, {O}>
    where F: Fn(f32) -> f32
{
    pub fn forward(&self, hidden: &Vector<f32, {O}>, input: &Vector<f32, {I}>) -> Vector<f32, {O}> {
        let mut out: [f32; {O}] = (self.hidden_matrix * hidden.clone() + self.input_matrix * input.clone() + self.biases).into();
        for n in out.iter_mut() {
            *n = (self.activation)(*n);
        }
        out.into()
    }
}

#[cfg(test)]
#[test]
fn test() {
    use aljabar::{mat2x2, vec2};
    let nnet: NNet<_, 2, 2> = NNet {
        hidden_matrix: mat2x2(0.5, 0.5, 0.5, 0.5),
        input_matrix: mat2x2(0.5, 0.5, 0.5, 0.5),
        biases: vec2(-0.5, 0.5),
        activation: |n: f32| (1.0 + (-n).exp()).recip(),
    };

    dbg!(nnet.forward(vec2(0.2, 0.1), vec2(1.0, 0.0)));
}
