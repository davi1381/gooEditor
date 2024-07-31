#![doc = include_str!("../README.md")]

mod default;
mod encoded_layer;
mod file;
mod header_info;
mod layer_content;
pub mod misc;
mod preview_image;
pub mod serde;
pub mod slice_config;

pub use encoded_layer::{LayerDecoder, LayerEncoder};
pub use file::File as GooFile;
pub use header_info::HeaderInfo;
pub use misc::Run;
pub use preview_image::PreviewImage;

const ENDING_STRING: &[u8] = &[
    0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x44, 0x4C, 0x50, 0x00,
];
const MAGIC_TAG: &[u8] = &[0x07, 0x00, 0x00, 0x00, 0x44, 0x4C, 0x50, 0x00];
const DELIMITER: &[u8] = &[0xD, 0xA];
