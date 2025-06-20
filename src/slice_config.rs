//! Simplified configuration for slicing a model.

/// A simplified configuration for slicing a model.
#[derive(Clone, Debug)]
pub struct SliceConfig {
    pub platform_resolution: [u32; 2],
    pub platform_size: [f32; 3],
    pub slice_height: f32,

    pub exposure_config: ExposureConfig,
    pub first_exposure_config: ExposureConfig,
    pub first_layers: u32,
}

#[derive(Clone, Debug)]
pub struct ExposureConfig {
    pub exposure_time: f32,
    pub lift_distance: f32,
    pub lift_speed: f32,
    pub retract_distance: f32,
    pub retract_speed: f32,
}

impl Default for ExposureConfig {
    fn default() -> Self {
        Self {
            exposure_time: 3.0,
            lift_distance: 5.0,
            lift_speed: 65.0,
            retract_distance: 5.0,
            retract_speed: 150.0,
        }
    }
}
