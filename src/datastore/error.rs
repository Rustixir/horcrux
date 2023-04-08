use crate::index::error::IndexError;




pub enum Error {
    InvalidObjectId,
    
    IndexError(IndexError)
}