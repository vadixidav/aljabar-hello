use aljabar::{Vector, Zero};

struct BitArray<const B: usize>(Vector<u8, {(B + 7) / 8}>);

impl<const B: usize> BitArray<{B}> {
    fn zero() -> Self {
        Self(Vector::zero())
    }
}

#[cfg(test)]
#[test]
fn test() {
    let b: BitArray<1> = BitArray::zero();
    let b: [u8; 1] = b.0.into();
    assert_eq!(b, [0]);
}