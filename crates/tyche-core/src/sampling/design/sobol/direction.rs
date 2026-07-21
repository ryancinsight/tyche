//! Canonical direction-number table for the supported dimensions.

pub(super) const MAX_DIMENSIONS: usize = 3;
pub(super) const BITS: usize = 32;

pub(super) const DIRECTIONS: [[u32; BITS]; MAX_DIMENSIONS] = [
    first_dimension(),
    from_parameters::<1>(0, [1]),
    from_parameters::<2>(1, [1, 3]),
];

const fn first_dimension() -> [u32; BITS] {
    let mut directions = [0; BITS];
    let mut bit = 0;
    while bit < BITS {
        directions[bit] = 1_u32 << (31 - bit);
        bit += 1;
    }
    directions
}

const fn from_parameters<const DEGREE: usize>(
    coefficient_bits: u32,
    initial: [u32; DEGREE],
) -> [u32; BITS] {
    let mut directions = [0; BITS];
    let mut bit = 0;
    while bit < DEGREE {
        directions[bit] = initial[bit] << (31 - bit);
        bit += 1;
    }

    while bit < BITS {
        let mut direction = directions[bit - DEGREE] ^ (directions[bit - DEGREE] >> DEGREE);
        let mut term = 1;
        while term < DEGREE {
            let coefficient = (coefficient_bits >> (DEGREE - 1 - term)) & 1;
            if coefficient == 1 {
                direction ^= directions[bit - term];
            }
            term += 1;
        }
        directions[bit] = direction;
        bit += 1;
    }
    directions
}

const _: () = {
    assert!(DIRECTIONS[0][0] == 0x8000_0000);
    assert!(DIRECTIONS[1][1] == 0xC000_0000);
    assert!(DIRECTIONS[2][2] == 0x6000_0000);

    let mut dimension = 0;
    while dimension < MAX_DIMENSIONS {
        let mut bit = 0;
        while bit < BITS {
            let diagonal = 1_u32 << (31 - bit);
            let less_significant = diagonal - 1;
            assert!(DIRECTIONS[dimension][bit] & diagonal != 0);
            assert!(DIRECTIONS[dimension][bit] & less_significant == 0);
            bit += 1;
        }
        dimension += 1;
    }
};
