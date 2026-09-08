use std::time::Duration;

fn main() {
    let (tx, mut rx) = trpl::channel();
    let tx2 = tx.clone();

    // The `move` moves the `tx` ownership, so it gets dropped after finishing sending.
    // This makes the `rx` return `None` because senders are closed.
    let tx_fut = async move {
        for val in vec!["I", "love", "you"].into_iter().map(String::from) {
            trpl::sleep(Duration::from_millis(500)).await;
            tx.send(val).unwrap();
        }
    };
    let tx2_fut = async move {
        for val in vec!["this", "and", "that"].into_iter().map(String::from) {
            trpl::sleep(Duration::from_millis(800)).await;
            tx2.send(val).unwrap();
        }
    };

    let rx_fut = async {
        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    };

    trpl::block_on(async { trpl::join!(tx_fut, rx_fut, tx2_fut) });
}
