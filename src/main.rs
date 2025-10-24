use heap_test::Heap;

fn main() {
    let mut heap = Heap::new();
    let hello = "こんばんは！";
    let sleepy = "眠いですねー。";
    let hello_ptr = unsafe { heap.alloc(hello.as_bytes()) };
    let sleepy_ptr = unsafe { heap.alloc(sleepy.as_bytes()) };

    unsafe {
        println!(
            "text:\n\t{}\n\t{}",
            heap.read_as_str(hello_ptr),
            heap.read_as_str(sleepy_ptr)
        );
    }

    let add_ptr = unsafe { heap.strcat(hello_ptr, sleepy_ptr) };

    unsafe {
        println!("strcat: \n\t{}", heap.read_as_str(add_ptr));
    }
}
