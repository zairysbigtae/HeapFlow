use HeapFlow::allocator::Allocator;

#[test]
fn malloc_test() {
    let allocator = Allocator::new(1024 * 1024);
    let debug = true;

    for _ in 0..2 {
        let ptr = allocator.free_list_allocate(32, debug);
        assert!(!ptr.is_null());
        allocator.free(ptr, debug);
    }
}
