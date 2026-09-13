pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub fn or<A: Future, B: Future>(
    _f1: A,
    _f2: B,
) -> impl Future<Output = Either<A::Output, B::Output>> {
    async { todo!() }
}
pub fn and<A: Future, B: Future>(_f1: A, _f2: B) -> impl Future<Output = (A::Output, B::Output)> {
    async { todo!() }
}

pub fn wait_for<T: Future>(future: T) -> T::Output {
    todo!()
}
