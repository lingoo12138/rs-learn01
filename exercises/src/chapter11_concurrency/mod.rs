// 第11章：并发与异步基础
#![allow(dead_code)]
pub mod practice;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// 练习1：创建线程
pub fn spawn_thread() -> String {
    let handle = thread::spawn(|| {
        "hello from thread".to_string()
    });
    handle.join().unwrap()
}

/// 练习2：线程与 move 闭包
pub fn move_to_thread(x: i32) -> i32 {
    let handle = thread::spawn(move || {
        x * 2
    });
    handle.join().unwrap()
}

/// 练习3：mpsc 消息传递
pub fn channel_message() -> String {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("hello".to_string()).unwrap();
    });
    rx.recv().unwrap()
}

/// 练习4：多生产者消息
pub fn multi_send() -> Vec<String> {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || { tx.send("A".to_string()).unwrap(); });
    thread::spawn(move || { tx2.send("B".to_string()).unwrap(); });
    let mut results = vec![];
    for _ in 0..2 {
        if let Ok(msg) = rx.recv_timeout(Duration::from_millis(100)) {
            results.push(msg);
        }
    }
    results.sort();
    results
}

/// 练习5：Arc<Mutex<T>> 共享状态
pub fn shared_counter() -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        }));
    }
    for h in handles { h.join().unwrap(); }
    let result = *counter.lock().unwrap();
    result
}

/// 练习6：async/await 基础
async fn hello_async() -> String {
    "hello async".to_string()
}
pub fn run_async() -> String {
    futures_executor::block_on(hello_async())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_spawn() { assert_eq!(spawn_thread(), "hello from thread"); }
    #[test] fn test_move() { assert_eq!(move_to_thread(5), 10); }
    #[test] fn test_channel() { assert_eq!(channel_message(), "hello"); }
    #[test] fn test_multi_send() { let r = multi_send(); assert_eq!(r.len(), 2); assert!(r.contains(&"A".to_string())); }
    #[test] fn test_shared_counter() { assert_eq!(shared_counter(), 5); }
    #[test] fn test_async() { assert_eq!(run_async(), "hello async"); }
}
