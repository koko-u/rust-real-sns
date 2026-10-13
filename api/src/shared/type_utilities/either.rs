use axum::response;
use axum::response::Response;

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }
    pub fn is_right(&self) -> bool {
        matches!(self, Either::Right(_))
    }

    pub fn left(self) -> Option<L> {
        match self {
            Either::Left(l) => Some(l),
            _ => None,
        }
    }

    pub fn right(self) -> Option<R> {
        match self {
            Either::Right(r) => Some(r),
            _ => None,
        }
    }

    pub fn as_ref(&self) -> Either<&L, &R> {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => Either::Right(r),
        }
    }

    pub fn as_mut(&mut self) -> Either<&mut L, &mut R> {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => Either::Right(r),
        }
    }

    pub fn map_left<M>(self, f: impl FnOnce(L) -> M) -> Either<M, R> {
        match self {
            Either::Left(l) => Either::Left(f(l)),
            Either::Right(r) => Either::Right(r),
        }
    }

    pub fn map_right<S>(self, f: impl FnOnce(R) -> S) -> Either<L, S> {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => Either::Right(f(r)),
        }
    }
}

impl<L, R> response::IntoResponse for Either<L, R>
where
    L: response::IntoResponse,
    R: response::IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            Either::Left(l) => l.into_response(),
            Either::Right(r) => r.into_response(),
        }
    }
}
