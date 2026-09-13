pub fn or<A: Future, B: Future>(_f1: A, _f2: B) -> impl Future<Output = Or<A::Output, B::Output>> {
    async { todo!() }
}
pub fn and<A: Future, B: Future>(
    _f1: A,
    _f2: B,
) -> impl Future<Output = And<A::Output, B::Output>> {
    async { todo!() }
}

pub enum Or<A, B> {
    Left(A),
    Right(B),
}
pub struct And<A, B>(A, B);
