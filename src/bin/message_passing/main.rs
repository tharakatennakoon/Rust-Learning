mod msg_queue;

use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

fn main() {
/*     let tx:Sender<String>;
    let rx:Receiver<String>;

    (tx, rx) = mpsc::channel();

    thread::spawn(move ||{
        let msg = String::from("Hi From Thread");
        tx.send(msg).unwrap();
    });

    let r_msg = rx.recv().unwrap();
    println!("msg is {r_msg}"); */

    let mq = msg_queue::MsgQueue::new();

    let tx1 = mq.get_tx_handle();
    let tx2 = mq.get_tx_handle();

    thread::spawn(move ||{
        let msg = msg_queue::Msg{msg_type: 1, msg_payload: String::from("How are you")};
        tx1.send(msg).unwrap();
    });

    thread::spawn(move ||{
        let msg = msg_queue::Msg{msg_type: 2, msg_payload: String::from("Are you OK")};
        tx2.send(msg).unwrap();
    });

    mq.execute();

}