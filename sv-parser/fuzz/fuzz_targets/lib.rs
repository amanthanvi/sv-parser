#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: &[u8]| {
    // fuzzing input for a function parse_sv<T: AsRef<Path>, U: AsRef<Path>, V: BuildHasher
    let _ = sv_parser::parse_sv::<&[u8], &[u8], _>(input, input, Default::default());
});
