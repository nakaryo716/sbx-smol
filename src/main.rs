use std::{
    sync::{Arc, Mutex},
    time,
};

use smol::channel::{self, Receiver, Sender};

type StoreType = Arc<Mutex<Vec<String>>>;
fn main() {
    let (tx, rx) = channel::unbounded();

    smol::block_on(async move {
        // メッセージの作成
        let messages: Vec<String> = ["Hello", "smol", "😽"]
            .iter()
            .map(|v| v.to_string())
            .collect();

        let messages2: Vec<String> = ["Goodby", "async_std", "😢"]
            .iter()
            .map(|v| v.to_string())
            .collect();

        // メッセージを保存するためのスタック
        // ここではstd::sync::Mutexを使っている
        let msg_store: StoreType = Arc::default();

        // Sender Tasks
        let send_task1 = smol::spawn(send_task(1, messages, tx.clone()));
        let send_task2 = smol::spawn(send_task(2, messages2, tx));
        // Receiver Tasks
        let receive_task1 = smol::spawn(receive_task(1, rx.clone(), msg_store.clone()));
        let receive_task2 = smol::spawn(receive_task(2, rx.clone(), msg_store.clone()));

        // タスク完了の待機
        // PanicするとmainもPanicする
        send_task1.await;
        send_task2.await;
        receive_task1.await;
        receive_task2.await;

        // 保存されたメッセージの出力
        println!("----Result----");
        msg_store.lock().unwrap().iter().for_each(|msg| {
            println!("{}", msg);
        });
    });
}

// 渡されたメッセージを3s周期で送信する
async fn send_task(id: u32, messages: Vec<String>, tx: Sender<String>) {
    for msg in messages.iter() {
        smol::Timer::after(time::Duration::from_secs(3)).await;
        // メッセージの送信
        if let Err(e) = tx.send(msg.to_string()).await {
            println!("[Sender-{}] Send error {}", id, e);
        }
        println!("[Sender-{}] Sended message '{}'", id, msg);
    }
    println!("[Sender-{}] Sended all messages", id);
}

// 受け取ったメッセージを保存する
async fn receive_task(id: u32, rx: Receiver<String>, store: StoreType) {
    while let Ok(v) = rx.recv().await {
        // idによってdelayさせる
        if id % 2 == 1 {
            smol::Timer::after(time::Duration::from_micros(1)).await;
        }
        println!("[Receiver-{}] Got '{}'", id, v);
        // 保存
        store.lock().unwrap().push(v);
    }
}
