use std::thread;
use std::time::Duration;

fn main() {
    let v = vec!{1, 2, 3};

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("thread says {}, {:?}", i, v);
            thread::sleep(Duration::from_millis(1));
        }
    });


    handle.join().unwrap();

    // drop(v);

    for i in 1..5 {
        println!("main thread says {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}