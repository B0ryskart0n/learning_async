use std::time::Duration;

fn main() {
    let (tx, mut rx) = trpl::channel();
    // The `move` moves the `tx` ownership, so it gets dropped after finishing sending.
    // This makes the `rx` return `None` because senders are closed.
    let tx_fut = async move {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            trpl::sleep(Duration::from_millis(500)).await;
            tx.send(val).unwrap();
        }
    };

    let rx_fut = async {
        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    };

    trpl::block_on(trpl::join(tx_fut, rx_fut));
}
