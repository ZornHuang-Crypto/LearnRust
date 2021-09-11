/* 思考题一
use std::thread;

fn main() {
    let arr = vec![1];
    let handle = thread::spawn(move || {
        println!("arr: {:?}", arr);
    });

    handle.join().unwrap();
}
*/

use std::thread;
use std::sync::Arc;

fn main() {
    let str = Arc::new("Hello CR");
    let v = Arc::clone(&str);
    let handle = thread::spawn(move || { 
        println!("str: {:?}", v);
    });
    handle.join().unwrap();
    println!("str1: {:?}", str);
}