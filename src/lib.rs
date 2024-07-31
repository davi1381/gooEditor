#![doc = include_str!("../README.md")]

mod default;
pub mod encoded_layer;
pub mod file;
pub mod header_info;
pub mod layer_content;
pub mod misc;
pub mod preview_image;
pub mod serde;
pub mod slice_config;

const ENDING_STRING: &[u8] = &[
    0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x44, 0x4C, 0x50, 0x00,
];
const MAGIC_TAG: &[u8] = &[0x07, 0x00, 0x00, 0x00, 0x44, 0x4C, 0x50, 0x00];
const DELIMITER: &[u8] = &[0xD, 0xA];
