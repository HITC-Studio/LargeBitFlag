use large_bit_flag::LargeBitFlag;

#[test]
fn test_eq_large_bit_flag() {
    let x: LargeBitFlag = LargeBitFlag::new();
    let y: LargeBitFlag = LargeBitFlag::new();
    assert_eq!(&x == &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    assert_eq!(&x == &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    assert_eq!(&x == &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 5);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 6);
    assert_eq!(&x == &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 5);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 5);
    assert_eq!(&x == &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[]);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[]);
    assert_eq!(&x == &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 0]);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 0]);
    assert_eq!(&x == &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 1]);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 1]);
    assert_eq!(&x == &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 0]);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 1]);
    assert_eq!(&x == &y, false);
}

#[test]
fn test_ne_large_bit_flag() {
    let x: LargeBitFlag = LargeBitFlag::new();
    let y: LargeBitFlag = LargeBitFlag::new();
    assert_eq!(&x != &y, false);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(2);
    assert_eq!(&x != &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    assert_eq!(&x != &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 5);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 6);
    assert_eq!(&x != &y, false);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 5);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(2, 5);
    assert_eq!(&x != &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[]);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[]);
    assert_eq!(&x != &y, false);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 0]);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 4]);
    assert_eq!(&x != &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 1, 1 << 1]);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 1]);
    assert_eq!(&x != &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 1]);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 1]);
    assert_eq!(&x != &y, false);
}

#[test]
fn test_bit_and_large_bit_flag() {
    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let z: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    assert_eq!(&(&x & &y) == &z, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(2);
    let z: LargeBitFlag = LargeBitFlag::new_set_single_bit(0);
    assert_eq!(&(&x & &y) == &z, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    let z: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    assert_eq!(&(&x & &y) == &z, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(0);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let z: LargeBitFlag = LargeBitFlag::new_set_single_bit(0);
    assert_eq!(&(&x & &y) == &z, true);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let z: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    x &= &y;
    assert_eq!(&x == &z, true);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(2);
    let z: LargeBitFlag = LargeBitFlag::new_set_single_bit(0);
    x &= &y;
    assert_eq!(&x == &z, true);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    let z: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    x &= &y;
    assert_eq!(&x == &z, true);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_single_bit(0);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let z: LargeBitFlag = LargeBitFlag::new_set_single_bit(0);
    x &= &y;
    assert_eq!(&x == &z, true);
}

#[test]
fn test_bit_or_large_bit_flag() {
    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let z: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    assert_eq!(&(&x | &y) == &z, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(2);
    let z: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 0 | 1 << 1]);
    assert_eq!(&(&x | &y) == &z, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    let z: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    assert_eq!(&(&x | &y) == &z, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(0);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let z: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    assert_eq!(&(&x | &y) == &z, true);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let z: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    x |= &y;
    assert_eq!(&x == &z, true);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(2);
    let z: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 0 | 1 << 1]);
    x |= &y;
    assert_eq!(&x == &z, true);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    let z: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    x |= &y;
    assert_eq!(&x == &z, true);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_single_bit(0);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let z: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    x |= &y;
    assert_eq!(&x == &z, true);
}
