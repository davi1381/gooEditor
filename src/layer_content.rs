use anyhow::{ensure, Result};

use crate::{
    encoded_layer::{LayerDecoder, LayerEncoder},
    misc::Run,
    serde::{Deserializer, Serializer},
    DELIMITER,
};

#[cfg(feature = "image")]
use image::GrayImage;

/// The header of each layer in a `.goo` file.
///
/// Check the [official format spec](https://github.com/elegooofficial/GOO) for more information.
pub struct LayerContent {
    pub pause_flag: u16,
    pub pause_position_z: f32,
    pub layer_position_z: f32,
    pub layer_exposure_time: f32,
    pub layer_off_time: f32,
    pub before_lift_time: f32,
    pub after_lift_time: f32,
    pub after_retract_time: f32,
    pub lift_distance: f32,
    pub lift_speed: f32,
    pub second_lift_distance: f32,
    pub second_lift_speed: f32,
    pub retract_distance: f32,
    pub retract_speed: f32,
    pub second_retract_distance: f32,
    pub second_retract_speed: f32,
    pub light_pwm: u16,
    pub data: Vec<u8>,
    pub checksum: u8,
}

impl LayerContent {
    pub fn serialize<T: Serializer>(&self, ser: &mut T) {
        ser.write_u16(self.pause_flag);
        ser.write_f32(self.pause_position_z);
        ser.write_f32(self.layer_position_z);
        ser.write_f32(self.layer_exposure_time);
        ser.write_f32(self.layer_off_time);
        ser.write_f32(self.before_lift_time);
        ser.write_f32(self.after_lift_time);
        ser.write_f32(self.after_retract_time);
        ser.write_f32(self.lift_distance);
        ser.write_f32(self.lift_speed);
        ser.write_f32(self.second_lift_distance);
        ser.write_f32(self.second_lift_speed);
        ser.write_f32(self.retract_distance);
        ser.write_f32(self.retract_speed);
        ser.write_f32(self.second_retract_distance);
        ser.write_f32(self.second_retract_speed);
        ser.write_u16(self.light_pwm);
        ser.write_bytes(DELIMITER);
        ser.write_u32(self.data.len() as u32 + 2);
        ser.write_bytes(&[0x55]);
        ser.write_bytes(&self.data);
        ser.write_u8(calculate_checksum(&self.data));
        ser.write_bytes(DELIMITER);
    }

    pub fn deserialize(des: &mut Deserializer) -> Result<Self> {
        let pause_flag = des.read_u16();
        let pause_position_z = des.read_f32();
        let layer_position_z = des.read_f32();
        let layer_exposure_time = des.read_f32();
        let layer_off_time = des.read_f32();
        let before_lift_time = des.read_f32();
        let after_lift_time = des.read_f32();
        let after_retract_time = des.read_f32();
        let lift_distance = des.read_f32();
        let lift_speed = des.read_f32();
        let second_lift_distance = des.read_f32();
        let second_lift_speed = des.read_f32();
        let retract_distance = des.read_f32();
        let retract_speed = des.read_f32();
        let second_retract_distance = des.read_f32();
        let second_retract_speed = des.read_f32();
        let light_pwm = des.read_u16();
        ensure!(des.read_bytes(2) == DELIMITER);
        let data_len = des.read_u32() as usize - 2;
        ensure!(des.read_u8() == 0x55);
        let data = des.read_bytes(data_len);
        let checksum = des.read_u8();
        ensure!(des.read_bytes(2) == DELIMITER);

        Ok(Self {
            pause_flag,
            pause_position_z,
            layer_position_z,
            layer_exposure_time,
            layer_off_time,
            before_lift_time,
            after_lift_time,
            after_retract_time,
            lift_distance,
            lift_speed,
            second_lift_distance,
            second_lift_speed,
            retract_distance,
            retract_speed,
            second_retract_distance,
            second_retract_speed,
            light_pwm,
            data: data.to_vec(),
            checksum,
        })
    }
}

pub fn calculate_checksum(data: &[u8]) -> u8 {
    let mut out = 0u8;
    for &byte in data {
        out = out.wrapping_add(byte);
    }
    !out
}

impl LayerContent {
    /// Decode the pixel data of this layer into a flat `Vec<u8>`.
    ///
    /// `width` and `height` should match the resolution of the file.
    pub fn decode_pixels(&self, width: u32, height: u32) -> Vec<u8> {
        let mut out = Vec::with_capacity((width * height) as usize);
        let decoder = LayerDecoder::new(&self.data);
        for Run { length, value } in decoder {
            out.extend(std::iter::repeat_n(value, length as usize));
        }
        debug_assert_eq!(out.len(), (width * height) as usize);
        out
    }

    /// Replace the pixel data of this layer from a flat slice of pixels.
    ///
    /// The slice length must match `width * height`.
    pub fn set_pixels(&mut self, width: u32, height: u32, pixels: &[u8]) {
        assert_eq!(pixels.len(), (width * height) as usize);

        if pixels.is_empty() {
            self.data.clear();
            self.checksum = 0;
            return;
        }

        let mut encoder = LayerEncoder::new();
        let mut run_value = pixels[0];
        let mut run_length: u64 = 1;

        for &value in &pixels[1..] {
            if value == run_value {
                run_length += 1;
            } else {
                encoder.add_run(run_length, run_value);
                run_value = value;
                run_length = 1;
            }
        }
        encoder.add_run(run_length, run_value);

        let (data, checksum) = encoder.finish();
        self.data = data;
        self.checksum = checksum;
    }

    #[cfg(feature = "image")]
    /// Convert this layer into an `image::GrayImage`.
    pub fn to_image(&self, width: u32, height: u32) -> GrayImage {
        let pixels = self.decode_pixels(width, height);
        GrayImage::from_vec(width, height, pixels).expect("pixel count matches")
    }

    #[cfg(feature = "image")]
    /// Replace this layer's pixel data with the contents of `image`.
    pub fn set_from_image(&mut self, image: &GrayImage) {
        let pixels = image.as_raw();
        self.set_pixels(image.width(), image.height(), pixels);
    }
}
