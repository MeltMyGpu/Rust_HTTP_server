use thiserror::Error;


#[derive(Debug,Error)]
pub enum HttpRequestError {
    #[error("There was an error when trying to wrap the HTTP request.")]
    RequestWrapError,
    #[error("There was an error when trying to read from the stream.")]
    ReadStreamError,

}
