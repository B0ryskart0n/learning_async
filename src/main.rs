use learning_async::wait_for;

fn main() {
    let trivial_future = async {
        println!("I've been awaited!");
    };

    wait_for(trivial_future);
}
