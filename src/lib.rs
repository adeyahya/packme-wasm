use entity::{Algo, AlgoInput};
pub mod entity;

#[no_mangle]
pub unsafe fn pack(input_ptr: *mut u8, input_len: usize) -> *mut u8 {
    let buf: Vec<u8> = Vec::from_raw_parts(input_ptr, input_len, input_len);
    let input_str = String::from_utf8(buf).unwrap();
    let (items, containers) = AlgoInput::parse(&input_str).unwrap();
    let mut algo = Algo { items, containers };
    let result = algo.pack();
    let result_str = String::new();
    let mut buf = result_str.into_bytes();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub fn alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub unsafe fn dealloc(ptr: *mut u8, size: usize) {
    let buf = Vec::from_raw_parts(ptr, size, size);
    std::mem::drop(buf);
}
