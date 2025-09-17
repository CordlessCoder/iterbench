#[inline]
pub fn expand_iota_views(input: &[i32]) -> impl Iterator<Item = i32> + '_ {
    input
        .iter()
        .flat_map(|&n| 1..=n)
        .flat_map(|n| 1..=n)
        .flat_map(|n| 1..=n)
}

#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn rust_max_ptr(ptr: *const i32, len: usize) -> i32 {
    let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
    let result = expand_iota_views(slice);
    result.max().unwrap_or(0)
}

unsafe extern "C" {
    pub fn cpp_max_ptr(ptr: *const i32, len: usize) -> i32;

}
