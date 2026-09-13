use learning_async::basic_futures::AlmostReady;
use learning_async::executor::wait_for;

fn main() {
    wait_for(AlmostReady::default());
}
