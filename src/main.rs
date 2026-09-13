use std::thread::sleep;
use std::time::Duration;

use learning_async::wait_for;

fn main() {
    let trivial_future = async {
        println!("Trivial future finishing.");
    };
    let less_trivial_future = async {
        sleep(Duration::from_secs(1));
        println!("Less trivial future finishing.");
    };

    wait_for(trivial_future);
    wait_for(less_trivial_future);
}
