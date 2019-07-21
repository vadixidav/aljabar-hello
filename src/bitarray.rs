struct BitArray<const B: usize>([u8; {(B + 7) / 8}]);

impl<const B: usize> BitArray<{B}> {
    fn zero() -> Self {
        Self([0; {(B + 7) / 8}])
    }
}

#[cfg(test)]
#[test]
fn test() {
    let b: BitArray<1> = BitArray::zero();
    assert_eq!(b.0[0], 0);
}