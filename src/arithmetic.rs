/// Given two bitstrings of length 2^i representing elements of the field of this size, return
/// the bitstring representing their multiplication.
/// (Efficiency could be improved using Karatsuba)
pub fn mul(left: &[bool], right: &[bool]) -> Vec<bool> {
    let n = left.len();
    assert_eq!(n, right.len());
    if n == 1 {
        return vec![left[0] & right[0]];
    }
    assert_eq!(n % 2, 0);
    let half_n = n / 2;
    let a = &left[0..half_n];
    let b = &left[half_n..n];
    let c = &right[0..half_n];
    let d = &right[half_n..n];
    let ac = mul(a, c);
    let ad = mul(a, d);
    let bc = mul(b, c);
    let bd = mul(b, d);
    let result_low_bits = add(&ac, &bd);
    let result_high_bits = add(&ad, &add(&bc, &rot(&bd)));
    let mut result = vec![false; n];
    result[..half_n].copy_from_slice(&result_low_bits[..half_n]);
    result[half_n..(half_n + half_n)].copy_from_slice(&result_high_bits[..half_n]);
    result
}

/// Return the inverse of a non-zero field element.
/// Algorithm from Fan & Paar: On Efficient Inversion in Tower Fields of Characteristic Two (1997)
pub fn inv(operand: &[bool]) -> Vec<bool> {
    let n = operand.len();
    assert!(operand.iter().any(|&x| x)); // zero is not invertible
    if n == 1 {
        return operand.into();
    }
    assert_eq!(n % 2, 0);
    let half_n = n / 2;
    let low_bits = &operand[0..half_n];
    let high_bits = &operand[half_n..n];
    let rot_high_bits = rot(high_bits);
    let delta = &add(
        &mul(low_bits, &add(low_bits, &rot_high_bits)),
        &mul(high_bits, high_bits),
    );
    let delta_inv = &inv(delta);
    let result_high_bits = mul(delta_inv, high_bits);
    let result_low_bits = mul(delta_inv, &add(low_bits, &rot_high_bits));
    let mut result = vec![false; n];
    result[..half_n].copy_from_slice(&result_low_bits[..half_n]);
    result[half_n..(half_n + half_n)].copy_from_slice(&result_high_bits[..half_n]);
    result
}

/// Given two bitstrings of length 2^i representing elements of the field of this size, return
/// the bitstring representing their addition.
/// (Addition is just bitwise XOR).
pub fn add(left: &[bool], right: &[bool]) -> Vec<bool> {
    assert_eq!(left.len(), right.len());
    let n = left.len();
    let mut result = vec![false; n];
    for i in 0..n {
        result[i] = left[i] ^ right[i];
    }
    result
}

/// Given a bitstrings of length 2^i representing an element of the binary field of this size,
/// conceived of as the top of a Wiedemann tower of fields, return the bitstring that represents the
/// multiplication of this field element by the image of the indeterminate in the top-most quadratic
/// extension.
pub fn rot(operand: &[bool]) -> Vec<bool> {
    let n = operand.len();
    if n == 1 {
        return operand.into();
    }
    assert_eq!(n % 2, 0);
    let half_n = n / 2;
    let low_bits = &operand[0..half_n];
    let high_bits = &operand[half_n..n];
    let result_low_bits = high_bits;
    let result_high_bits = add(low_bits, &rot(high_bits));
    let mut result = vec![false; n];
    result[..half_n].copy_from_slice(&result_low_bits[..half_n]);
    result[half_n..(half_n + half_n)].copy_from_slice(&result_high_bits[..half_n]);
    result
}

#[test]
fn test_mul() {
    // test some base cases
    assert_eq!(mul(&vec![false], &vec![true]), vec![false]);
    assert_eq!(mul(&vec![true], &vec![true]), vec![true]);
    // test with longer bitstrings
    // X_0 . (1 + X_0) = 1
    assert_eq!(
        mul(&vec![false, true], &vec![true, true]),
        vec![true, false]
    );

    // X2 ^2 = X1X2 + 1
    let x2 = vec![false, false, false, false, true, false, false, false];
    assert_eq!(
        mul(&x2, &x2),
        vec![true, false, false, false, false, false, true, false]
    );
}

#[test]
fn test_inv() {
    // test the unique base case
    assert_eq!(inv(&vec![true]), vec![true]);
    // some easy to derive inversions
    assert_eq!(
        inv(&vec![false, false, true, false]),
        vec![false, true, true, false]
    );
    assert_eq!(
        inv(&vec![false, false, false, false, true, false, false, false]),
        vec![false, false, true, false, true, false, false, false]
    );
    // ensure consistent with multiplication
    // find all (element, inverse) pairs (for nonzero element) over F_16 by brute force
    let mut f16: Vec<Vec<bool>> = vec![];
    for i in 0..(1 << 4) {
        f16.push(vec![
            i & 0b1000 != 0,
            i & 0b0100 != 0,
            i & 0b0010 != 0,
            i & 0b0001 != 0,
        ]);
    }
    let mut elem_and_inv = vec![];
    let one = vec![true, false, false, false];
    for elem in &f16 {
        if !elem.iter().any(|&x| x) {
            continue;
        }
        for other_elem in &f16 {
            if mul(&elem, &other_elem) == one {
                elem_and_inv.push((elem.clone(), other_elem.clone()));
            }
        }
    }
    assert_eq!(elem_and_inv.len(), 15);
    elem_and_inv.iter().for_each(|(elem, inverse)| {
        assert_eq!(inv(&elem), *inverse);
    });
}

#[test]
fn test_add() {
    // test some base cases (F_2)
    assert_eq!(add(&vec![false], &vec![true]), vec![true]);
    assert_eq!(add(&vec![true], &vec![true]), vec![false]);
    // test with longer bitstrings
    assert_eq!(
        add(&vec![false, false], &vec![true, false]),
        vec![true, false]
    );
}

#[test]
fn test_rot() {
    // trivial cases (thinking of X_{-1} as 1):
    assert_eq!(rot(&vec![false]), vec![false]);
    assert_eq!(rot(&vec![true]), vec![true]);

    // smallest non-trivial cases:
    // X_0 . (1 + X_0) = 1, so rot(true, true) = (true, false)
    assert_eq!(rot(&vec![true, true]), vec![true, false]);
    // X_0 . 0 = 0
    assert_eq!(rot(&vec![false, false]), vec![false, false]);
    // X_0 . 1 = X_0
    assert_eq!(rot(&vec![true, false]), vec![false, true]);

    // tests on longer bit strings

    // X_1 . 0 = 0
    assert_eq!(rot(&vec![false; 4]), vec![false; 4]);
    // X_1 . 1 = X_1
    assert_eq!(
        rot(&vec![true, false, false, false]),
        vec![false, false, true, false]
    );

    // if the high limb is all false, should just interchange high and low limbs
    assert_eq!(
        rot(&vec![true, false, false, true, false, false, false, false]),
        vec![false, false, false, false, true, false, false, true]
    );

    // X2 (X1.X2 + 1) = X1 + X0.X1.X2
    assert_eq!(
        rot(&vec![true, false, false, false, false, false, true, false]),
        vec![false, false, true, false, false, false, false, true]
    );
}
