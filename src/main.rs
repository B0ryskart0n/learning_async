use learning_async::basic_futures::Ready;
use learning_async::executor::wait_for;

fn main() {
    wait_for(Ready);
}
