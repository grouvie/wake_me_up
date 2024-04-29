use axum::{
    async_trait,
    extract::{FromRequest, Request},
};
use axum_extra::protobuf::Protobuf;
use bytes::Bytes;
use prost::Message;

use crate::error::MyError;

pub struct MyProtobuf<T>(pub Protobuf<T>);

#[async_trait]
impl<T, S> FromRequest<S> for MyProtobuf<T>
where
    T: Message + Default,
    S: Send + Sync,
{
    type Rejection = MyError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = match Bytes::from_request(req, state).await {
            Ok(bytes) => bytes,
            Err(e) => {
                return Err(MyError::BytesRejection {
                    error: e.to_string(),
                })
            }
        };

        if bytes.is_empty() {
            return Err(MyError::BytesRejection {
                error: "Empty request bytes".to_string(),
            });
        }

        match T::decode(&mut bytes.clone()) {
            Ok(value) => Ok(MyProtobuf(Protobuf(value))),
            Err(err) => Err(MyError::ProtobufDecodeError {
                error: err.to_string(),
            }),
        }
    }
}
