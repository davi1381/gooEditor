//! Random types.

use crate::slice_config::SliceConfig;

/// The result of slicing a model.
/// Contains both a list of layers and a `SliceConfig`, which is a simple configuration that can be used to create a .goo file.
pub struct SliceResult<'a, Layer> {
    pub layers: Vec<Layer>,
    pub slice_config: &'a SliceConfig,
}

#[derive(Debug)]
pub struct Run {
    pub length: u64,
    pub value: u8,
}

/// Defines a layer that can be encoded.
/// This was created to allow for different implementations of encoding layers for deferent formats, but there is only one implementation in this crate.
pub trait EncodableLayer {
    type Output: Send;

    fn new() -> Self;
    fn add_run(&mut self, length: u64, value: u8);
    fn finish(self, layer: usize, config: &SliceConfig) -> Self::Output;
}
