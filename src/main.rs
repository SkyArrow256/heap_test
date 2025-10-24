use heap_test::Heap;

fn main() {
    let mut heap = Heap::new();
    let hello = "こんな感じの文章はどうかなー？";
    let sleepy = "おねむです...　　ねむーい！！！💥💥💥💥";
    let hello_ptr = unsafe { heap.alloc(hello.as_bytes()) };
    let sleepy_ptr = unsafe { heap.alloc(sleepy.as_bytes()) };

    unsafe {
        println!(
            "text: \t{}\n\t{}",
            heap.read_as_str(hello_ptr),
            heap.read_as_str(sleepy_ptr)
        );
    }
}
