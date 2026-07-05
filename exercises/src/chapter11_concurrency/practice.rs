// 第11章实践练习：并发与异步基础
// 将 todo!() 替换为你的代码，然后移除 #[ignore] 运行测试验证
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn practice1() -> i32 {
    let handle = thread::spawn(|| {
        todo!("返回 42")
    });
    handle.join().unwrap()
}

pub fn practice2() -> String {
    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    thread::spawn(move || {
        todo!("tx.send(\"hello\".to_string())")
    });
    todo!("rx.recv().unwrap()")
}

pub fn practice3() -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..3 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        }));
    }
    for h in handles { h.join().unwrap(); }
    todo!("返回 *counter.lock().unwrap()")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn test_practice1() { assert_eq!(practice1(), 42); }
    #[test]
    #[ignore]
    fn test_practice2() { assert_eq!(practice2(), "hello"); }
    #[test]
    #[ignore]
    fn test_practice3() { assert_eq!(practice3(), 3); }
}
