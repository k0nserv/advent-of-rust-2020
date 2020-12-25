use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn map_left<U, F>(self, f: F) -> Either<U, R>
    where
        F: FnOnce(L) -> U,
    {
        match self {
            Self::Left(l) => Either::Left(f(l)),
            Self::Right(r) => Either::Right(r),
        }
    }

    pub fn map_right<U, F>(self, f: F) -> Either<L, U>
    where
        F: FnOnce(R) -> U,
    {
        match self {
            Self::Left(l) => Either::Left(l),
            Self::Right(r) => Either::Right(f(r)),
        }
    }

    pub fn is_left(&self) -> bool {
        matches!(self, Self::Left(_))
    }

    pub fn is_right(&self) -> bool {
        matches!(self, Self::Right(_))
    }

    pub fn left(self) -> Option<L> {
        match self {
            Self::Left(l) => Some(l),
            Self::Right(_) => None,
        }
    }

    pub fn right(self) -> Option<R> {
        match self {
            Self::Left(_) => None,
            Self::Right(r) => Some(r),
        }
    }
}

impl<L: FromStr, R: FromStr> FromStr for Either<L, R> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(l) = L::from_str(s) {
            return Ok(Self::Left(l));
        }

        if let Ok(r) = R::from_str(s) {
            return Ok(Self::Right(r));
        }

        return Err(format!(
            "`{}` is not parsable as either `{}` or `{}`",
            s,
            std::any::type_name::<L>(),
            std::any::type_name::<R>()
        ));
    }
}
