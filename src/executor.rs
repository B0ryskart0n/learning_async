use std::pin::pin;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;

/// Very naive waiter that loops and polls the given future.
pub fn wait_for<F: Future>(future: F) -> F::Output {
    let mut poll_counter = 0usize;

    let mut cx = Context::from_waker(Waker::noop());
    let mut pinned_future = pin!(future);

    loop {
        poll_counter += 1;
        match pinned_future.as_mut().poll(&mut cx) {
            Poll::Pending => continue,
            Poll::Ready(t) => {
                eprintln!("Future ready after {} polls.", poll_counter);
                return t;
            }
        }
    }
}
