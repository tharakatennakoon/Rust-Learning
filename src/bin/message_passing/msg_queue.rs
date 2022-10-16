use std::sync::mpsc;

pub struct Msg {
    pub msg_type:i8,
    pub msg_payload:String
}
pub struct MsgQueue {
    tx : mpsc::Sender<Msg>,
    rx : mpsc::Receiver<Msg>
}

impl MsgQueue {
    pub fn new() -> MsgQueue{
        let (tx , rx) = mpsc::channel();
        return MsgQueue{tx, rx};
    }

    pub fn get_tx_handle(&self) ->  mpsc::Sender<Msg>{
        return self.tx.clone();
    }

    pub fn execute(&self) {
        for received in &self.rx {
            println!("Type :{}, Msg :{}", received.msg_type, received.msg_payload);
        }
    }
}