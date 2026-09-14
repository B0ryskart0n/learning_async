#![allow(unused_imports)]

use learning_async::basic_futures::AlmostReady;
use learning_async::basic_futures::Ready;
use learning_async::combinators::Or;
use learning_async::executor::wait_for;

fn main() {
    wait_for(Or::new(AlmostReady::default(), Ready));
}
