fn main() {
    println!("Hello, world!");
    // stack_frame_oom();
    cause_overflow(0);
}

fn stack_frame_oom() {
    // 100MB array doesn't fit on 8MB stack frame
    let big_data: [u8; 100 * 1024 * 1024] = [1; 100 * 1024 * 1024];
    println!("first byte: {}", big_data[0]);
}

fn cause_overflow(i: i64) {
    let _waste_of_space = [0u8; 128 * 1024];

    if i % 10 == 0 {
        println!("Frame depth: {i}");
    }

    cause_overflow(i + 1);
}
