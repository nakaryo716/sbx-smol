use std::time;

fn main() {
    smol::block_on(async move {
        // async task
        // 出力までに3sかかる

        // キューイング
        let task1 = smol::spawn(task());
        let task2 = smol::spawn(task());

        let res1 = task1.await;
        let res2 = task2.await;
        println!("{}:{}", res1, res2);

        // blocking task
        // ここでは同期スリープしているが、CPUバウンドな重い計算など
        // 出力に6s程度かかる
        let task1 = smol::spawn(task_blocking());
        let task2 = smol::spawn(task_blocking());

        let res1 = task1.await;
        let res2 = task2.await;
        println!("{}:{}", res1, res2);

        // blocking task
        // 出力に3s程度
        // 別スレッドプールにオフロードする
        // tokio::task::spawn_blocking()に近い
        // クロージャを渡す
        let task1 = smol::unblock(sync_blocking);
        let task2 = smol::unblock(sync_blocking);

        let res1 = task1.await;
        let res2 = task2.await;
        println!("{}:{}", res1, res2);
    });
}

async fn task() -> String {
    smol::Timer::after(time::Duration::from_secs(3)).await;
    "Hello".to_string()
}

async fn task_blocking() -> String {
    std::thread::sleep(time::Duration::from_secs(3));
    "Hello".to_string()
}

fn sync_blocking() -> String {
    std::thread::sleep(time::Duration::from_secs(3));
    "Hello".to_string()
}
