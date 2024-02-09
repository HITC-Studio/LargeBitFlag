use std::cmp;

#[cfg(test)]
mod lib_tests;

fn get_arch_bits() -> usize {
    return match usize::try_from(usize::BITS) {
        Ok(value) => value,
        Err(error) => panic!(
            "Unable to convert usize::BITS: {} to usize. Error: {}",
            usize::BITS,
            error
        ),
    };
}

#[derive(Clone, Debug)]
pub struct LargeBitFlag {
    bit_flags: Vec<usize>,
}

// impl fmt::Debug for LargeBitFlag {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for index in (self.bit_flags.len()-1)..=0 {
//             let _ = write!(f, "{}", self.bit_flags[index]);
//         }
//         return write!(f, "\n");
//     }
// }

impl LargeBitFlag {
    pub fn new() -> LargeBitFlag {
        return LargeBitFlag {
            bit_flags: vec![0; 2],
        };
    }

    // which_bit: 0-usize_max
    // But a 0 is no bit set
    // Converts which_bit to the correct
    // array and bit
    pub fn new_set_single_bit(which_bit: usize) -> LargeBitFlag {
        if which_bit == 0 {
            return LargeBitFlag::new();
        }

        let arch_bits: usize = get_arch_bits();

        let modified_bit: usize = match which_bit.checked_sub(1) {
            Some(value) => value,
            None => panic!("Unable to subtract 1 from {}", which_bit),
        };

        let which_array: usize = match modified_bit.checked_div(arch_bits) {
            Some(value) => value,
            None => panic!("Unable to divide {} by {}", modified_bit, arch_bits),
        };

        if which_array == 0 {
            let modified_which_array: usize = match which_array.checked_add(1) {
                Some(value) => value,
                None => panic!("Unable to add 1 to which_array: {}.", which_array),
            };

            let converted_modified_which_array: isize = match isize::try_from(modified_which_array)
            {
                Ok(value) => value,
                Err(e) => panic!(
                    "Unable to convert {} from usize to isize. Error: {}",
                    modified_which_array, e
                ),
            };
            return Self::new_set_array_and_single_bit(converted_modified_which_array, which_bit);
        }

        let number_of_bits_used: usize = match arch_bits.checked_mul(which_array) {
            Some(value) => value,
            None => panic!("Unable to multiply {} by {}", arch_bits, which_array),
        };

        let which_real_bit: usize = match modified_bit.checked_sub(number_of_bits_used) {
            Some(value) => value,
            None => panic!(
                "Unable to subtract {} by {}",
                modified_bit, number_of_bits_used
            ),
        };

        let modified_which_real_bit: usize = match which_real_bit.checked_add(1) {
            Some(value) => value,
            None => panic!("Unable to add 1 to {}", which_real_bit),
        };

        let modified_which_array: usize = match which_array.checked_add(1) {
            Some(value) => value,
            None => panic!("Unable to add 1 to which_array: {}.", which_array),
        };

        let converted_modified_which_array: isize = match isize::try_from(modified_which_array) {
            Ok(value) => value,
            Err(e) => panic!(
                "Unable to convert {} from usize to isize. Error: {}",
                which_array, e
            ),
        };
        return Self::new_set_array_and_single_bit(
            converted_modified_which_array,
            modified_which_real_bit,
        );
    }

    // which_array: 0-isize_max
    // But a 0 is no array (or bit) set
    // which_bit: 0-arch_size (8, 16, 32, 64, 128, etc...)
    // But a 0 is no bit set
    // Set a single bit for a single array
    pub fn new_set_array_and_single_bit(which_array: isize, which_bit: usize) -> LargeBitFlag {
        if which_bit == 0 || which_array == 0 {
            return LargeBitFlag::new();
        }

        let arch_bits: usize = get_arch_bits();
        assert!(
            which_bit <= arch_bits,
            "Passed in Bit: {} was not <= {} arch bits",
            which_bit,
            arch_bits
        );

        assert!(
            which_array >= 0,
            "Passed in Array: {} was not >= 0",
            which_array
        );

        // Rant: A vector can only be isize::MAX in size,
        // but requires a usize when setting the size or indexing the vector
        let which_array_as_usize: usize = match usize::try_from(which_array) {
            Ok(value) => value,
            Err(error) => panic!(
                "Unable to convert the isize: {} to a usize. Error: {}",
                which_array, error
            ),
        };

        let modified_which_array_as_usize: usize = match which_array_as_usize.checked_add(1) {
            Some(value) => value,
            None => panic!("Unable to add 1 to {}", which_array_as_usize),
        };

        let mut result: LargeBitFlag = LargeBitFlag {
            // Have to reserve, in order to index the correct location
            // to directly set the bit
            bit_flags: vec![0; modified_which_array_as_usize],
        };

        let modified_bit: usize = match which_bit.checked_sub(1) {
            Some(value) => value,
            None => panic!("Unable to subtract 1 from {}", which_bit),
        };

        result.bit_flags[which_array_as_usize] = 1 << modified_bit;

        return result;
    }

    // which_bits: Array of bits set
    // Copy over all array of bits
    pub fn new_set_array_of_bits(which_bits: &[usize]) -> LargeBitFlag {
        assert!(
            which_bits.len() <= usize::try_from(isize::MAX).unwrap(),
            "Passed in Bit slice lenth: {}, exceeds max allowed length: {}",
            which_bits.len(),
            isize::MAX
        );

        let mut result: LargeBitFlag = LargeBitFlag {
            bit_flags: Vec::<usize>::with_capacity(which_bits.len() + 1),
        };

        result.bit_flags.push(0);
        result.bit_flags.extend_from_slice(which_bits);

        return result;
    }

    pub fn and_bit(&mut self, rhs: &LargeBitFlag) {
        let self_size: usize = self.bit_flags.len();
        let rhs_size: usize = rhs.bit_flags.len();

        let truncate_size: usize = if self_size == rhs_size {
            self_size
        } else if self_size < rhs_size {
            self_size
        } else {
            rhs_size
        };

        self.bit_flags.truncate(truncate_size);

        for index in 1..truncate_size {
            self.bit_flags[index] = self.bit_flags[index] & rhs.bit_flags[index];
        }
    }

    pub fn or_bit(&mut self, rhs: &LargeBitFlag) {
        let self_size: usize = self.bit_flags.len();
        let rhs_size: usize = rhs.bit_flags.len();

        let resize_size: usize = if self_size == rhs_size {
            self_size
        } else if self_size > rhs_size {
            self_size
        } else {
            rhs_size
        };

        self.bit_flags.resize(resize_size, 0);
        for index in 1..resize_size {
            self.bit_flags[index] = if index < self_size && index < rhs_size {
                self.bit_flags[index] | rhs.bit_flags[index]
            } else if index >= rhs_size {
                self.bit_flags[index]
            } else {
                rhs.bit_flags[index]
            };
        }
    }

    pub fn set_bit(&mut self, rhs: &LargeBitFlag) {
        return self.or_bit(rhs);
    }

    pub fn unset_bit(&mut self, rhs: &LargeBitFlag) {
        // Must treat all empty array indices of rhs as full of 1s
        // Which means that for any array indices of self that extend
        // past rhs, remain unchanged

        let self_size: usize = self.bit_flags.len();
        let rhs_size: usize = rhs.bit_flags.len();

        let min_size: usize = if self_size == rhs_size {
            self_size
        } else if self_size < rhs_size {
            self_size
        } else {
            rhs_size
        };

        for index in 1..min_size {
            self.bit_flags[index] &= !rhs.bit_flags[index];
        }
    }

    pub fn is_bit_set(&self, rhs: &LargeBitFlag) -> bool {        
        return &and_bits(self, rhs) != &LargeBitFlag::new();
    }

    pub fn invert(&mut self) {
        for index in 1..self.bit_flags.len() {
            self.bit_flags[index] = !self.bit_flags[index];
        }
    }
}

pub fn and_bits(lhs: &LargeBitFlag, rhs: &LargeBitFlag) -> LargeBitFlag {
    let mut result: LargeBitFlag = LargeBitFlag::new();

    let lhs_size: usize = lhs.bit_flags.len();
    let rhs_size: usize = rhs.bit_flags.len();

    let resize_size: usize = if lhs_size == rhs_size {
        lhs_size
    } else if lhs_size > rhs_size {
        rhs_size
    } else {
        lhs_size
    };

    result.bit_flags.resize(resize_size, 0);
    for index in 1..resize_size {
        result
            .bit_flags[index] = 
            lhs.bit_flags[index] & rhs.bit_flags[index];
    }

    return result;
}

pub fn or_bits(lhs: &LargeBitFlag, rhs: &LargeBitFlag) -> LargeBitFlag {
    let mut result: LargeBitFlag = LargeBitFlag::new();

    let lhs_size: usize = lhs.bit_flags.len();
    let rhs_size: usize = rhs.bit_flags.len();

    let resize_size: usize = if lhs_size == rhs_size {
        lhs_size
    } else if lhs_size > rhs_size {
        lhs_size
    } else {
        rhs_size
    };

    result.bit_flags.resize(resize_size, 0);
    for index in 1..resize_size {
        result
            .bit_flags[index] = if index < lhs_size && index < rhs_size {
                lhs.bit_flags[index] | rhs.bit_flags[index]
            } else if index >= rhs_size {
                lhs.bit_flags[index]
            } else {
                rhs.bit_flags[index]
            };
    }

    return result;
}

pub fn invert(bit: &LargeBitFlag) -> LargeBitFlag {
    let mut result: LargeBitFlag = LargeBitFlag::new();

    for index in 1..bit.bit_flags.len() {
        result.bit_flags.push(!bit.bit_flags[index]);
    }

    return result;
}

/*
let x: LargeBitFlag;
let y: LargeBitFlag;
if x == y
Both variables are passed by reference, not by value (move)
*/
impl cmp::PartialEq for LargeBitFlag {
    fn eq(&self, rhs: &Self) -> bool {
        let self_size: usize = self.bit_flags.len();
        let rhs_size: usize = rhs.bit_flags.len();

        let zero_flag: usize = 0;

        let max_size: usize = cmp::max(self_size, rhs_size);

        for index in 1..max_size {
            if self_size == max_size && rhs_size == max_size {
                if self.bit_flags[index] != rhs.bit_flags[index] {
                    return false;
                }
            } else if self_size == max_size && rhs_size < max_size {
                if rhs_size > index {
                    if self.bit_flags[index] != rhs.bit_flags[index] {
                        return false;
                    }
                } else if self.bit_flags[index] != zero_flag {
                    return false;
                }
            } else if self_size < max_size && rhs_size == max_size {
                if self_size > index {
                    if self.bit_flags[index] != rhs.bit_flags[index] {
                        return false;
                    }
                } else if rhs.bit_flags[index] != zero_flag {
                    return false;
                }
            }
        }

        return true;
    }

    fn ne(&self, rhs: &Self) -> bool {
        return !(self == rhs);
    }
}

/*
Not sure about this, but I think it calls PartialEq
*/
impl cmp::Eq for LargeBitFlag {}
