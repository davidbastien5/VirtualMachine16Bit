pub fn create_memory(size: usize) -> Box<[u8]> {
    vec![0; size].into_boxed_slice()
}