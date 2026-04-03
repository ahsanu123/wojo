use crate::{MsgTypeAndVal, TX};

pub async fn send_msg_and_val(msg: MsgTypeAndVal) -> Result<(), String> {
    if let Some(tx) = TX.get() {
        return tx
            .send(msg)
            .await
            .map_err(|_| "Fail to send message".into());
    }

    Err("Fail to get TX".into())
}

pub fn try_send_msg_and_val(msg: MsgTypeAndVal) -> Result<(), String> {
    if let Some(tx) = TX.get() {
        return tx.try_send(msg).map_err(|_| "Fail to send message".into());
    }

    Err("Fail to get TX".into())
}
