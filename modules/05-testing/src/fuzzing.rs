pub fn fuzz_target(data: &[u8]) {
    if !data.is_empty() {
        let _ = std::str::from_utf8(data);
    }
}
