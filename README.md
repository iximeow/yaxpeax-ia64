## yaxpeax-ia64

ia64 decoder implemented as part of the yaxpeax project.

`yaxpeax-ia64` implements traits provided by `yaxpeax-arch`, which are likely how you want to use this library from Rust.

implementation is heavily derived from the manual [`itanium-architecture-vol-1-2-3-4-reference-set-manual.pdf`](https://www.intel.com/content/dam/doc/manual/itanium-architecture-vol-1-2-3-4-reference-set-manual.pdf), as of 2019-09-07. `sha256: 705d2fc04ab378568eddb6bac4ee6974b6224b8efb5f73606f964f4a86e22955`.

bytes go in, instructions come out - from `test.rs`:
```rust
let decoder = yaxpeax_ia64::InstDecoder::default();
let expected = "[MMI] ld1 r17=[r17];; nop.m 0x0; dep r14=r18,r14,0x0,0x8";
let data = [0x0a, 0x88, 0x00, 0x22, 0x00, 0x10, 0x00, 0x00, 0x00, 0x02, 0x00, 0xc0, 0x21, 0x71, 0xdc, 0x4f];
let inst = decoder.decode(data[..].iter().cloned()).unwrap();
assert_eq!(format!("{}", inst), expected);
```

the `InstructionBundle` impl for `Display` is somewhat opinionated in output format, it will write instructions all in one line. for more customized display formats (some kind of cool multi-column layout perhaps?), you'll want to whip something more clever up by using `InstructionBundle::instructions()` and handling instructions independently.

### features

* probably works
* almost-`#[no_std]`

### probably works
the only decoding oracle i could find was the ia64 decoder in GNU `binutils`. i suspect it's correct, but between the size of the instruction set, details in immediate encoding, and user-mode-focused testing, there may be some misdecodes! a critical eye is warranted, though i expect `yaxpeax-ia64` to generally be correct or close to it.

### almost-`#[no_std]`
`yaxpeax-ia64` does not reference `std::`, and theoretically `#[no_std]` is as simple as putting a `#![no_std]` in `lib.rs` and moving on. i don't expect to build or use `yaxpeax-ia64` in this configuration, so it is not enabled out of avoiding extra test permutations.

if you would like to use `yaxpeax-ia64` in a `no-std` configuration:
* awesome! shouldn't be hard
* why?
* rust doesn't even target ia64 as a tier 3 platform, are you trying to get C bindings? that would be good to specify too

###
