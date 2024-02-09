# Create a Large Bit Flag

Create a `LargeBitFlag` which allows for a near-endless amount of flags. The `struct` is just a `vec<usize>`, 
in which the `vec` can have up to `isize::MAX` entries and each entry is a `usize`. `usize` is based on the
CPU Architecture (e.g. 8-bit, 16-bit, 32-bit, 64-bit).

This useful when you need more than `usize::BITS` flags, with an emphasis on performing Bit-Wise `AND` and `OR`
operations. Otherwise, an `enum` would be a better choice.

Due to the way `NOT` / `!` works, to be used in unsetting a bit, the overloaded Bit-Wise operations could
not be supported. Without knowing the number of array indices to the flag that needs a bit-unset,
when a `LargeBitFlag` is inverted, the array indices that do not exist, are not converted to existing
set to all `1`s (`111111...1`).

## Use of `Panic` over `Result`

I chose to have the functions to `Panic` when the passed in variables were incorrect, instead of
returning a `Result`. The reason is there isn't a `defualt` implementation the callee could do
or use, if the passed in variables were incorrect. The expectation is that these flags need to be
set and will be used in various forms of logic. The callee could test for a successfull `Result`,
but on failure, the callee would not be able to continue.

However, I do agree that it is on the callee to ultimaly determine the best course of action,
but since this is local code, I chose to keep the `Panic` calls. Feel free to change them to
`Result`s if desired.

## Use of `checked_*` Functions

This is more of a self-improvement for other future code, in-which I get into the habit
of ensuring integer over/under-flow does not happen. Feel free to remove these function
calls. 

## Provided functions

### NEW

#### `new`

Ceates a blank, no bit set, `LargeBitFlag`.

#### `new_set_single_bit`

A value of 0 to `usize::MAX` can be passed in. The correct bit in the correct array will be set
for the callee. In this sense, a loop can be used to set a number of flags easily.

I expect this function to be preferenced over any of the other `new` functions.

#### `new_set_array_and_single_bit`

Based on a starting 1-index array and bit, allow the callee to set which bit in which array.
The passed in bit can not be greater than `usize::BITS`.

#### `new_set_array_of_bits`

Pass in an array of `usize`. This will be copied into a `LargeBitFlag`. Primary used for setting
multiple bits per array.

### AND

All Bit-Wise `AND` operations take a reference to the variables. This is to prevent copies from being made,
in the event one or both of the `LargeBitFlag`s has a large array length.

The `LargeBitFlag`s can be `AND`ed together to create a new `LargeBitFlag`.

For `x &= y` type operations: `x.and_bit(y)`.

For `z = x & y` type operations: `z = and_bits(x, y)`.

### OR

All Bit-Wise `OR` operations take a reference to the variables. This is to prevent copies from being made,
in the event one or both of the `LargeBitFlag`s has a large array length.

The `LargeBitFlag`s can be `OR`ed together to create a new `LargeBitFlag`.

For `x |= y` type operations: `x.or_bit(y)`.

For `z = x | y` type operations: `z = or_bits(x, y)`.

### Invert / ! / NOT

All Bit-Wise `! / NOT` operations take a reference to the variables. This is to prevent copies from being made,
in the event one or both of the `LargeBitFlag`s has a large array length.

A `LargeBitFlag`s can be `! / NOT`ed to create a new `LargeBitFlag` 

For `!x` type operations: `x.invert()`. Note that this will change `self`.

For `z = !x.clone()` type operations: `z = invert(x)`.

### Set Bit

`x.set_bit(y)` calls `x.or_bit(y)` and is added as a convience and syntax-sugar.

### Unset Bit

For `x &= !y` type operations that unset the `y` bit from `x`: `x.unset(y)`. Note that `y` is
not inverted before being passed in. The function will handle that, making a clone.

### Comparison

`LargeBitFlag`s can be compared using: `==` or `!=`.

## Usage in other Libraries or Binaries

### Cargo.toml
```
...
[dependencies]
...
large_bit_flag = { path = "<Path/to/large_bit_flag>" }
...
```

### Source Code
```
...
use large_bit_flag::*;
...

fn some_fn() {
  ...
  
  let a: LargeBitFlag = LargeBitFlag::new();
  let b: LargeBitFlag = LargeBitFlag::new_set_single_bit(5);
  let c: LargeBitFlag = LargeBitFlag::new_set_array_and_single_bit(3, 5);
  let d: LargeBitFlag = LargeBitFlag::new_set_array_of_bits(&[1 << 0, 1 << 2, 1 << 3, (1 << 0 | 1 << 5)]);

  if (&a == &b) {
  	...
  }

  if (&c != &d) {
  	...
  }

  let e: LargeBitFlag = and_bits(&a, &b);
  let f: LargeBitFlag = or_bits(&c, &d);

  let mut g: LargeBitFlag = and_bits(&a, &b);
  g.and_bit(&a);
  g.or_bit(&b);

  ...
}
```

## Build
`cargo build` or `cargo build --release`

## Tests
`cargo test`

### Integration Tests
Found at "large_bit_flag/tests/integration_tests.rs". Tests the public methods of the library, that would be used by other libraries or binaries.


