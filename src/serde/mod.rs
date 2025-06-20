//! Encoders and decoders for arbitrary binary data.

mod deserializer;
mod serializer;
mod types;

pub use deserializer::Deserializer;
pub use serializer::{DynamicSerializer, Serializer, SizedSerializer};
pub use types::SizedString;
