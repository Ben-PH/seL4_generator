## Generator for types and functions used by seL4 projects

The seL4 micro-kernel api has several types and assosciated functions that pack data into a single 8-byte word using bit-packing. These are generated at compile-time by parsing a simple specification, then generating the C code needed for struct creation, and field getters/setters.

For example:

```text
base 64

block seL4_MessageInfo {
    field label 52
    field capsUnwrapped 3
    field extraCaps 2
    field length 7
}
```
will generate the following C code:

```C
static inline seL4_MessageInfo_t CONST
seL4_MessageInfo_new(uint64_t label, uint64_t capsUnwrapped, uint64_t extraCaps, uint64_t length) {
    seL4_MessageInfo_t seL4_MessageInfo;

    /* fail if user has passed bits that we will override */  
    assert((label & ~0xfffffffffffffull) == ((0 && (label & (1ull << 63))) ? 0x0 : 0));  
    assert((capsUnwrapped & ~0x7ull) == ((0 && (capsUnwrapped & (1ull << 63))) ? 0x0 : 0));  
    assert((extraCaps & ~0x3ull) == ((0 && (extraCaps & (1ull << 63))) ? 0x0 : 0));  
    assert((length & ~0x7full) == ((0 && (length & (1ull << 63))) ? 0x0 : 0));

    seL4_MessageInfo.words[0] = 0
        | (label & 0xfffffffffffffull) << 12
        | (capsUnwrapped & 0x7ull) << 9
        | (extraCaps & 0x3ull) << 7
        | (length & 0x7full) << 0;

    return seL4_MessageInfo;
}

static inline uint64_t CONST
seL4_MessageInfo_get_capsUnwrapped(seL4_MessageInfo_t seL4_MessageInfo) {
    uint64_t ret;
    ret = (seL4_MessageInfo.words[0] & 0xe00ull) >> 9;
    /* Possibly sign extend */
    if (0 && (ret & (1ull << (63)))) {
        ret |= 0x0;
    }
    return ret;
}

static inline seL4_MessageInfo_t CONST
seL4_MessageInfo_set_capsUnwrapped(seL4_MessageInfo_t seL4_MessageInfo, uint64_t v64) {
    /* fail if user has passed bits that we will override */
    assert((((~0xe00 >> 9 ) | 0x0) & v64) == ((0 && (v64 & (1ull << (63)))) ? 0x0 : 0));
    seL4_MessageInfo.words[0] &= ~0xe00ull;
    seL4_MessageInfo.words[0] |= (v64 << 9) & 0xe00ull;
    return seL4_MessageInfo;
}

// Similarly there are getters and setters for the other fields

```

This project parses and generates the equivilant code for rust.

### TODOs

#### Input sanitisation

The generated C code uses run-time asserts to ensure that the values being set are constrained to 2^field_width. Replicating this with rust leaves behind some useful tools that rust gives us. The plan is to use the rust type system in the generated code. For example:

```rust
// current generated code
pub struct MessageInfo {
    inner: u64,
}
impl MessageInfo {
    pub fn new(label: u32, capsUnwrapped: u32, extraCaps: u32, length: u32) -> Self { /* ... */ }
}

// same code, but with type system at play

#[derive(BitConstrained)]
pub struct MessageInfo {
    inner: u64,
    label: PhantomData<U52>,
    capsUnwrapped: PhantomData<U3>,
    // ...
}

impl MessageInfo {
    pub fn new(label: U52, capsUnwrapped: U3, extraCaps: U2, length: U7) -> Self { /* ... */ }
}

// generated as part of deriving BitConstrained 
impl core::convert::TryFrom<u32> for U52 {
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let roof = 2u64::pow(52) - 1;
        match value > roof {
            true => Err(\*...*\),
            false => Ok(value),
        }
    }
}
```

The overall intention is that only a valid `Un` type can be passed in as an argument, where `n` is the bit-width, and all valid `Un` variables can be represented by an n-bit word.

#### Architecture awareness

Currently, it's assumed that the generation is for a 64-bit architecture. If the need is presented, 32-bit architecture will be catered to.

#### Ownership model

Currently, setters take-and-return the structure, and getters take immutable references. This might change in the future. The guiding principal, for the moment, is to seek a model of ownership that promotes a "correct by default" workflow in the context of developing on seL4.
