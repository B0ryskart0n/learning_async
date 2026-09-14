use std::{
    pin::Pin,
    task::{Context, Poll},
};

// TODO Could be leveraged to not have Pinned Boxed Futures, but rather the Futures be part of the struct.
pub struct Or<F1: Future, F2: Future> {
    f1: Pin<Box<F1>>,
    f2: Pin<Box<F2>>,
}
impl<F1: Future, F2: Future> Or<F1, F2> {
    pub fn new(f1: F1, f2: F2) -> Or<F1, F2> {
        Or {
            f1: Box::pin(f1),
            f2: Box::pin(f2),
        }
    }
}
impl<F1: Future, F2: Future> Future for Or<F1, F2> {
    type Output = Either<F1::Output, F2::Output>;

    // TODO Fuse (nothing prevents from calling poll after returning Ready, which can panic)
    // Biased towards F1
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // Moving out of the Pin is possible because Self is Unpin.
        // Otherwise moving out of the Pin violates the invariant of not moving.
        let this: &mut Self = self.get_mut();

        // Both statements are identical, the latter is desugared to the former with autoref
        let pinned_a: Pin<&mut F1> = Pin::as_mut(&mut this.f1);
        let pinned_b: Pin<&mut F2> = this.f2.as_mut();

        match pinned_a.poll(cx) {
            Poll::Ready(a) => Poll::Ready(Either::Left(a)),
            Poll::Pending => match pinned_b.poll(cx) {
                Poll::Ready(b) => Poll::Ready(Either::Right(b)),
                Poll::Pending => Poll::Pending,
            },
        }
    }
}

pub enum Either<A, B> {
    Left(A),
    Right(B),
}
pub struct Both<A, B>(A, B);
