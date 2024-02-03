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

    assert_eq!(x.bit_flags.len(), 0);
}

#[test]
fn correct_single_bit_is_set_in_large_bit_flag() {
    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(0);
    assert_eq!(x.bit_flags.len(), 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(1);
    assert_eq!(x.bit_flags.len(), 1);
    assert_eq!(x.bit_flags[0], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(30);
    assert_eq!(x.bit_flags.len(), 1);
    assert_eq!(x.bit_flags[0], 1 << 29);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(64);
    assert_eq!(x.bit_flags.len(), 1);
    assert_eq!(x.bit_flags[0], 1 << 63);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(65);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[1], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(95);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[1], 1 << 30);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(128);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[1], 1 << 63);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(12800);
    assert_eq!(x.bit_flags.len(), 200);
    assert_eq!(x.bit_flags[199], 1 << 63);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(12801);
    assert_eq!(x.bit_flags.len(), 201);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[200], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_single_bit(64000);
    assert_eq!(x.bit_flags.len(), 1000);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[999], 1 << 63);
}

#[test]
fn correct_array_and_single_bit_is_set_in_large_bit_flag() {
    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    assert_eq!(x.bit_flags.len(), 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 1);
    assert_eq!(x.bit_flags.len(), 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    assert_eq!(x.bit_flags.len(), 1);
    assert_eq!(x.bit_flags[0], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 30);
    assert_eq!(x.bit_flags.len(), 1);
    assert_eq!(x.bit_flags[0], 1 << 29);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 64);
    assert_eq!(x.bit_flags.len(), 1);
    assert_eq!(x.bit_flags[0], 1 << 63);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(2, 1);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[1], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(2, 31);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[1], 1 << 30);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(2, 64);
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[1], 1 << 63);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(200, 64);
    assert_eq!(x.bit_flags.len(), 200);
    assert_eq!(x.bit_flags[199], 1 << 63);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(201, 1);
    assert_eq!(x.bit_flags.len(), 201);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[200], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1000, 64);
    assert_eq!(x.bit_flags.len(), 1000);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[999], 1 << 63);
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

    assert_eq!(x.bit_flags.len(), 4);
    assert_eq!(x.bit_flags[0], 1 << 0);
    assert_eq!(x.bit_flags[1], 1 << 30);
    assert_eq!(x.bit_flags[2], 1 << 63);
    assert_eq!(x.bit_flags[3], 123456);
}

#[test]
fn verify_bit_and_in_large_bit_flag() {
    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let z: LargeBitFlag = &x & &y;
    assert_eq!(z.bit_flags.len(), 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let z: LargeBitFlag = &x & &y;
    assert_eq!(z.bit_flags.len(), 1);
    assert_eq!(z.bit_flags[0], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(2, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(2, 1);
    let z: LargeBitFlag = &x & &y;
    assert_eq!(z.bit_flags.len(), 2);
    assert_eq!(z.bit_flags[0], 0);
    assert_eq!(z.bit_flags[1], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(5, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(2, 1);
    let z: LargeBitFlag = &x & &y;
    assert_eq!(z.bit_flags.len(), 2);
    assert_eq!(z.bit_flags[0], 0);
    assert_eq!(z.bit_flags[1], 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 0 | 1 << 1]);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 0]);
    let z: LargeBitFlag = &x & &y;
    assert_eq!(z.bit_flags.len(), 1);
    assert_eq!(z.bit_flags[0], 1 << 0);
}

#[test]
fn verify_bit_and_assign_in_large_bit_flag() {
    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1 << 0);
    x &= &y;
    assert_eq!(x.bit_flags.len(), 0);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    x &= &y;
    assert_eq!(x.bit_flags.len(), 1);
    assert_eq!(x.bit_flags[0], 1 << 0);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(2, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(2, 1);
    x &= &y;
    assert_eq!(x.bit_flags.len(), 2);
    assert_eq!(x.bit_flags[0], 0);
    assert_eq!(x.bit_flags[1], 1 << 0);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(5, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    x &= &y;
    assert_eq!(x.bit_flags.len(), 1);
    assert_eq!(x.bit_flags[0], 0);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 0 | 1 << 1]);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 0]);
    x &= &y;
    assert_eq!(x.bit_flags.len(), 1);
    assert_eq!(x.bit_flags[0], 1 << 0);
}

#[test]
fn verify_bit_or_in_large_bit_flag() {
    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    let z: LargeBitFlag = &x | &y;
    assert_eq!(z.bit_flags.len(), 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 0);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let z: LargeBitFlag = &x | &y;
    assert_eq!(z.bit_flags.len(), 1);
    assert_eq!(z.bit_flags[0], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 5);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(10, 1);
    let z: LargeBitFlag = &x | &y;
    assert_eq!(z.bit_flags.len(), 10);
    assert_eq!(z.bit_flags[0], 1 << 4);
    assert_eq!(z.bit_flags[1], 0);
    assert_eq!(z.bit_flags[9], 1 << 0);

    let x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let z: LargeBitFlag = &x | &y;
    assert_eq!(z.bit_flags.len(), 1);
    assert_eq!(z.bit_flags[0], 1 << 0);
}

#[test]
fn verify_bit_or_assign_in_large_bit_flag() {
    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(0, 0);
    x |= &y;
    assert_eq!(x.bit_flags.len(), 0);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 0);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    x |= &y;
    assert_eq!(x.bit_flags.len(), 1);
    assert_eq!(x.bit_flags[0], 1 << 0);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 5);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(10, 1);
    x |= &y;
    assert_eq!(x.bit_flags.len(), 10);
    assert_eq!(x.bit_flags[0], 1 << 4);
    assert_eq!(x.bit_flags[1], 0);
    assert_eq!(x.bit_flags[9], 1 << 0);

    let mut x: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    let y: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(1, 1);
    x |= &y;
    assert_eq!(x.bit_flags.len(), 1);
    assert_eq!(x.bit_flags[0], 1 << 0);
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
