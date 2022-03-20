use actix_utils::future::{ready, Ready};
use derive_more::{AsRef, Deref, DerefMut, Display, From};
use serde::de;

use actix_web::{dev::Payload, error::Error, web, FromRequest, HttpRequest};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut, AsRef, Display, From)]
pub struct Path<T>(pub T);

impl<T> Path<T> {
    /// Unwrap into inner `T` value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> FromRequest for Path<T>
where
    T: de::DeserializeOwned,
{
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        ready(
            web::Path::from_request(req, payload)
                .into_inner()
                .map(|path| Self(web::Path::into_inner(path))),
        )
    }
}
