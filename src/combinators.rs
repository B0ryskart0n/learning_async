use std::{
    pin::Pin,
    task::{Context, Poll},
};

// TODO `Output`s being `Unpin` should not be a critical limitation, but still it's a limitation that should be fixed.
pub fn and<F1, F2>(f1: F1, f2: F2) -> impl Future<Output = Both<F1::Output, F2::Output>>
where
    F1: Future,
    F2: Future,
    F1::Output: Unpin,
    F2::Output: Unpin,
{
    And {
        f1: Box::pin(f1),
        f1_return: None,
        f2: Box::pin(f2),
        f2_return: None,
        already_returned: false,
    }
}
struct And<F1: Future, F2: Future> {
    f1: Pin<Box<F1>>,
    f1_return: Option<F1::Output>,
    f2: Pin<Box<F2>>,
    f2_return: Option<F2::Output>,
    already_returned: bool,
}
impl<F1: Future, F2: Future> Future for And<F1, F2>
where
    F1::Output: Unpin,
    F2::Output: Unpin,
{
    type Output = Both<F1::Output, F2::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Relies on fields being `Unpin`, which is true if `F1::Output` is `Unpin`.
        let this = self.get_mut();

        // Prevent the executor from polling the underlying futures again, which could panic.
        if this.already_returned {
            return Poll::Pending;
        }

        if this.f1_return.is_none() {
            let pinned_f1 = Pin::as_mut(&mut this.f1);
            if let Poll::Ready(a) = pinned_f1.poll(cx) {
                this.f1_return = Some(a);
            }
        }

        if this.f2_return.is_none() {
            let pinned_f2 = Pin::as_mut(&mut this.f2);
            if let Poll::Ready(b) = pinned_f2.poll(cx) {
                this.f2_return = Some(b);
            }
        }

        if this.f1_return.is_some() && this.f2_return.is_some() {
            this.already_returned = true;
            // It's safe to move the `Option`s out of the `And` because it's not going to be used again.
            // Unwrapping is safe because of the if condition we are in.
            Poll::Ready(Both(
                this.f1_return.take().unwrap(),
                this.f2_return.take().unwrap(),
            ))
        } else {
            Poll::Pending
        }
    }
}

pub fn or<F1: Future, F2: Future>(
    f1: F1,
    f2: F2,
) -> impl Future<Output = Either<F1::Output, F2::Output>> {
    Or {
        f1: Box::pin(f1),
        f2: Box::pin(f2),
    }
}
// TODO Could be leveraged to not have Pinned Boxed Futures, but rather the Futures be part of the struct.
struct Or<F1: Future, F2: Future> {
    f1: Pin<Box<F1>>,
    f2: Pin<Box<F2>>,
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
