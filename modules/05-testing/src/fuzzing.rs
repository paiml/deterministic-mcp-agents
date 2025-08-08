pub fn fuzz_target(data: &[u8]) {
    if data.len() > 0 {
        let _ = std::str::from_utf8(data);
    }
}