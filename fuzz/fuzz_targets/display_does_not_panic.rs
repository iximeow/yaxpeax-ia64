#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate yaxpeax_ia64;

use yaxpeax_arch::{Decoder, U8Reader};

fuzz_target!(|data: &[u8]| {
    let ia64_decoder = yaxpeax_ia64::InstDecoder::default();

    let mut reader = U8Reader::new(data);

    if let Ok(inst) = ia64_decoder.decode(&mut reader) {
        let mut out = String::new();
        let res = format!("{inst}");
    }
});
