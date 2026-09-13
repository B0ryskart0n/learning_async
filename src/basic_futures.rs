use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct Ready;
impl Future for Ready {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

pub struct AlmostReady {
    ready: bool,
}
impl Future for AlmostReady {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.ready {
            true => Poll::Ready(()),
            false => {
                // Next time it's going to be ready.
                self.get_mut().ready = true;
                // Call to `wake()` only after making internal progress.
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}
