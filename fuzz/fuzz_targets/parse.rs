#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: String| {
    stones::parse(&data);
});
