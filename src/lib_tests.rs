use super::*;

#[test]
fn usize_bits() {
    assert_eq!(get_arch_bits(), usize::try_from(usize::BITS).unwrap());
}

/*
https://doc.rust-lang.org/src/core/num/mod.rs.html
#[cfg(target_pointer_width = "##")]
impl usize {
    ...
    BITS = ##,
    ...
}
*/
#[test]
fn usize_bits_target_pointer_width() {
    #[cfg(target_pointer_width = "8")]
    assert_eq!(get_arch_bits(), 8);
    #[cfg(target_pointer_width = "16")]
    assert_eq!(get_arch_bits(), 16);
    #[cfg(target_pointer_width = "32")]
    assert_eq!(get_arch_bits(), 32);
    #[cfg(target_pointer_width = "64")]
    assert_eq!(get_arch_bits(), 64);
}

#[test]
fn empty_large_bit_flag() {
    let x: LargeBitFlag = LargeBitFlag::new();

    assert_eq!(x.bit_flags.len(), 2);
}

#[test]
fn correct_single_bit_is_set_in_large_bit_flag() {
    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(0);
    assert_eq!(x.bit_flags.len(), 2);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[1], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(30);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[1], 1 << 29);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(64);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[1], 1 << 63);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(65);
    assert_eq!(x.bit_flags.len(), 3);
    assert_eq!(x.bit_flags[1], 0);
    assert_eq!(x.bit_flags[2], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(95);
    assert_eq!(x.bit_flags.len(), 3);
    assert_eq!(x.bit_flags[1], 0);
    assert_eq!(x.bit_flags[2], 1 << 30);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(128);
    assert_eq!(x.bit_flags.len(), 3);
    assert_eq!(x.bit_flags[1], 0);
    assert_eq!(x.bit_flags[2], 1 << 63);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(12800);
    assert_eq!(x.bit_flags.len(), 201);
    assert_eq!(x.bit_flags[1], 0);
    assert_eq!(x.bit_flags[200], 1 << 63);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(12801);
    assert_eq!(x.bit_flags.len(), 202);
    assert_eq!(x.bit_flags[1], 0);
    assert_eq!(x.bit_flags[201], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(64000);
    assert_eq!(x.bit_flags.len(), 1001);
    assert_eq!(x.bit_flags[1], 0);
    assert_eq!(x.bit_flags[1000], 1 << 63);
}

#[test]
fn correct_array_and_single_bit_is_set_in_large_bit_flag() {
    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    assert_eq!(x.bit_flags.len(), 2);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 1);
    assert_eq!(x.bit_flags.len(), 2);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[1], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 30);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[1], 1 << 29);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 64);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[1], 1 << 63);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(2, 1);
    assert_eq!(x.bit_flags.len(), 3);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[2], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(2, 31);
    assert_eq!(x.bit_flags.len(), 3);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[2], 1 << 30);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(2, 64);
    assert_eq!(x.bit_flags.len(), 3);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[2], 1 << 63);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(200, 64);
    assert_eq!(x.bit_flags.len(), 201);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[200], 1 << 63);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(201, 1);
    assert_eq!(x.bit_flags.len(), 202);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[201], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1000, 64);
    assert_eq!(x.bit_flags.len(), 1001);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[1000], 1 << 63);
}

#[test]
#[should_panic(expected = "Passed in Bit:")]
fn wrong_bit_for_array_and_single_bit_is_set_in_large_bit_flag() {
    LargeBitFlag::new_set_array_and_single_bit(1, usize::BITS as usize + 1);
}

#[test]
#[should_panic(expected = "Passed in Array:")]
fn wrong_array_for_array_and_single_bit_is_set_in_large_bit_flag() {
    LargeBitFlag::new_set_array_and_single_bit(-1, 1);
}

#[test]
fn correct_array_is_set_in_large_bit_flag() {
    let bits: Vec<usize> = vec![1 << 0, 1 << 30, 1 << 63, 123456];

    let x: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&bits[..]);

    assert_eq!(x.bit_flags.len(), 5);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[1], 1 << 0);
    assert_eq!(x.bit_flags[2], 1 << 30);
    assert_eq!(x.bit_flags[3], 1 << 63);
    assert_eq!(x.bit_flags[4], 123456);
}

#[test]
fn verify_and_bits() {
    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let z: LargeBitFlag = and_bits(&x, &y);
    assert_eq!(z.bit_flags.len(), 2);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let z: LargeBitFlag = and_bits(&x, &y);
    assert_eq!(z.bit_flags.len(), 2);
    assert_eq!(z.bit_flags[0], 0);
    assert_eq!(z.bit_flags[1], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(2, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(2, 1);
    let z: LargeBitFlag = and_bits(&x, &y);
    assert_eq!(z.bit_flags.len(), 3);
    assert_eq!(z.bit_flags[0], 0);
    assert_eq!(z.bit_flags[2], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(5, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(2, 1);
    let z: LargeBitFlag = and_bits(&x, &y);
    assert_eq!(z.bit_flags.len(), 3);
    assert_eq!(z.bit_flags[0], 0);
    assert_eq!(z.bit_flags[2], 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 0 | 1 << 1]);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 0]);
    let z: LargeBitFlag = and_bits(&x, &y);
    assert_eq!(z.bit_flags.len(), 2);
    assert_eq!(z.bit_flags[0], 0);
    assert_eq!(z.bit_flags[1], 1 << 0);
}

#[test]
fn verify_and_bit_in_large_bit_flag() {
    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1 << 0);
    x.and_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    x.and_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[1], 1 << 0);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(2, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(2, 1);
    x.and_bit(&y);
    assert_eq!(x.bit_flags.len(), 3);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[2], 1 << 0);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(5, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    x.and_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[1], 0);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 0 | 1 << 1]);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 0]);
    x.and_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[1], 1 << 0);
}

#[test]
fn verify_or_bits() {
    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    let z: LargeBitFlag = or_bits(&x, &y);
    assert_eq!(z.bit_flags.len(), 2);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 0);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let z: LargeBitFlag = or_bits(&x, &y);
    assert_eq!(z.bit_flags.len(), 2);
    assert_eq!(z.bit_flags[0], 0);
    assert_eq!(z.bit_flags[1], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 5);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(10, 1);
    let z: LargeBitFlag = or_bits(&x, &y);
    assert_eq!(z.bit_flags.len(), 11);
    assert_eq!(z.bit_flags[0], 0);
    assert_eq!(z.bit_flags[1], 1 << 4);
    assert_eq!(z.bit_flags[2], 0);
    assert_eq!(z.bit_flags[10], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let z: LargeBitFlag = or_bits(&x, &y);
    assert_eq!(z.bit_flags.len(), 2);
    assert_eq!(z.bit_flags[0], 0);
    assert_eq!(z.bit_flags[1], 1 << 0);
}

#[test]
fn verify_or_bit_in_large_bit_flag() {
    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    x.or_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 0);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    x.or_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[1], 1 << 0);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 5);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(10, 1);
    x.or_bit(&y);
    assert_eq!(x.bit_flags.len(), 11);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[1], 1 << 4);
    assert_eq!(x.bit_flags[2], 0);
    assert_eq!(x.bit_flags[10], 1 << 0);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    x.or_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[1], 1 << 0);
}

#[test]
fn verify_set_bit_in_large_bit_flag() {
    let mut x: LargeBitFlag = LargeBitFlag::new();
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(10);
    x.set_bit(&y);
    assert_eq!(&x == &y, true);

    let mut x: LargeBitFlag = LargeBitFlag::new();
    let y: LargeBitFlag = LargeBitFlag::new();
    x.set_bit(&y);
    assert_eq!(&x == &y, true);

    let mut x: LargeBitFlag = LargeBitFlag::new();
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    x.set_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[1], 1 << 0);
    assert_eq!(&x == &y, true);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(2);
    x.set_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[1], 1 << 0 | 1 << 1);
    assert_eq!(&x == &y, false);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(10);
    x.set_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[1], 1 << 4 | 1 << 9);
    assert_eq!(&x == &y, false);
}

#[test]
fn verify_unset_bit_in_large_bit_flag() {
    let mut x: LargeBitFlag = LargeBitFlag::new();
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(10);
    x.set_bit(&y);
    assert_eq!(&x == &y, true);
    x.unset_bit(&y);
    assert_eq!(&x == &y, false);
    assert_eq!(&x == &LargeBitFlag::new(), true);

    let mut x: LargeBitFlag = LargeBitFlag::new();
    let y: LargeBitFlag = LargeBitFlag::new();
    x.set_bit(&y);
    assert_eq!(&x == &y, true);
    x.unset_bit(&y);
    assert_eq!(&x == &y, true);

    let mut x: LargeBitFlag = LargeBitFlag::new();
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    x.set_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[1], 1 << 0);
    assert_eq!(&x == &y, true);
    x.unset_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[1], 0);
    assert_eq!(&x == &y, false);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(2);
    x.set_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[1], 1 << 0 | 1 << 1);
    assert_eq!(&x == &y, false);
    x.unset_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[1], 1 << 0);
    assert_eq!(&x == &y, false);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(10);
    x.set_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[1], 1 << 4 | 1 << 9);
    assert_eq!(&x == &y, false);
    x.unset_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[1], 1 << 4);
    assert_eq!(&x == &y, false);
}

#[test]
fn verify_is_bit_set_in_large_bit_flag() {
    let mut x: LargeBitFlag = LargeBitFlag::new();
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(10);
    x.set_bit(&y);
    assert_eq!(&x == &y, true);
    assert_eq!(x.is_bit_set(&y), true);

    let mut x: LargeBitFlag = LargeBitFlag::new();
    let y: LargeBitFlag = LargeBitFlag::new();
    x.set_bit(&y);
    assert_eq!(&x == &y, true);
    assert_eq!(x.is_bit_set(&y), false);

    let mut x: LargeBitFlag = LargeBitFlag::new();
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    x.set_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[1], 1 << 0);
    assert_eq!(&x == &y, true);
    assert_eq!(x.is_bit_set(&y), true);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(2);
    x.set_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[1], 1 << 0 | 1 << 1);
    assert_eq!(&x == &y, false);
    assert_eq!(x.is_bit_set(&y), true);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
    let y: LargeBitFlag = LargeBitFlag::new_set_single_bit(10);
    x.set_bit(&y);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[1], 1 << 4 | 1 << 9);
    assert_eq!(&x == &y, false);
    assert_eq!(x.is_bit_set(&y), true);
}

#[test]
fn verify_invert_in_large_bit_flag() {
    let mut x: LargeBitFlag = LargeBitFlag::new();
    let y: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[!(0)]);
    x.invert();
    assert_eq!(&x == &y, true);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 1 | 1 << 3 | 1 << 5]);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[!(1 << 1 | 1 << 3 | 1 << 5)]);
    x.invert();
    assert_eq!(&x == &y, true);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 1 | 1 << 3 | 1 << 5]);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 1 | 1 << 3 | 1 << 5]);
    x.invert();
    assert_eq!(&x == &y, false);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 3]);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 5]);
    x.invert();
    assert_eq!(&x == &y, false);
}

#[test]
fn verify_cmp_eq_in_large_bit_flag() {
    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    assert_eq!(&x == &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 0);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 0);
    assert_eq!(&x == &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    assert_eq!(&x == &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(5, 5);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(5, 5);
    assert_eq!(&x == &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    assert_eq!(&x == &y, false);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(10, 5);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(5, 5);
    assert_eq!(&x == &y, false);
}

#[test]
fn verify_cmp_ne_in_large_bit_flag() {
    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    assert_eq!(&x != &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 5);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    assert_eq!(&x != &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(10, 5);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(3, 2);
    assert_eq!(&x != &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 2);
    assert_eq!(&x != &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 2);
    assert_eq!(&x != &y, true);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    assert_eq!(&x != &y, false);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 0);
    assert_eq!(&x != &y, false);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 5);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 5);
    assert_eq!(&x != &y, false);
}

#[test]
fn verify_move_of_large_bit_flag() {
    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 5);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 5);
    let z: LargeBitFlag = y;
    assert_eq!(&x == &z, true);
}

#[test]
fn verify_ref_of_large_bit_flag() {
    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 5);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 5);
    let ref z: LargeBitFlag = y;
    assert_eq!(&x == z, true);
}

#[test]
fn verify_clone_of_large_bit_flag() {
    let v: Vec<LargeBitFlag> = vec![LargeBitFlag::new_set_array_and_single_bit(1, 5)];
    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 5);
    let y: LargeBitFlag = v[0].clone();
    assert_eq!(&x == &y, true);
}
