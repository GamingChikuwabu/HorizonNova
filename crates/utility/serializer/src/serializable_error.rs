
use error::Error;

#[derive(Debug,Error)]
pub enum SerializableError {
    SerializeError(String),
    DeserializeError(String),
    FormatterError(String),
}