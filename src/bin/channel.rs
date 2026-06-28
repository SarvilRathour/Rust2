use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main(){
    let (tx,rx)=mpsc::channel();
    let tx0=tx.clone();
    thread::spawn(move ||{
        let val=vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            String::from("!"),
        ];
        val.into_iter().for_each(|x| {tx0.send(x).unwrap();
            thread::sleep(Duration::from_secs(1));
        });
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        vals.into_iter().for_each(|x| {tx.send(x).unwrap();
        });
    });
    for received in rx {
        println!("Received: {}", received);
    }
    
}