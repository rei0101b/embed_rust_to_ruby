use std::thread;
fn main() {
    process();
}
#[no_mangle]
pub extern "C" fn process() {
    let handless: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(move || {
                let mut x = 0;
                for _ in 0..9_000_000 {
                    x += 1
                }
                x
            })
        })
        .collect();
    for h in handless {
        println!(
            "Thread finished with count={}",
            h.join().map_err(|_| "Could not join a thead").unwrap()
        );
    }
}
