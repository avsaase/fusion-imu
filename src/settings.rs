use fusion_imu_sys as sys;

use crate::math::Convention;

/// AHRS algorithm settings.
#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub(crate) inner: sys::FusionAhrsSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            inner: sys::FusionAhrsSettings {
                convention: sys::FusionConvention_FusionConventionNwu,
                gain: 0.5,
                gyroscopeRange: 0.0,
                accelerationRejection: 90.0,
                magneticRejection: 90.0,
                recoveryTriggerPeriod: 0,
            },
        }
    }
}

impl Settings {
    /// Create a new `Settings` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the Earth axes convention.
    pub fn set_convention(&mut self, convention: Convention) {
        self.inner.convention = convention as u32;
    }

    /// Sets the AHRS algorithm gain.
    pub fn set_gain(&mut self, gain: f32) {
        self.inner.gain = gain;
    }

    /// Sets the gyroscope range.
    pub fn set_gyroscope_range(&mut self, range: f32) {
        self.inner.gyroscopeRange = range;
    }

    /// Sets the acceleration rejection.
    pub fn set_acceleration_rejection(&mut self, rejection: f32) {
        self.inner.accelerationRejection = rejection;
    }

    /// Sets the magnetic rejection.
    pub fn set_magnetic_rejection(&mut self, rejection: f32) {
        self.inner.magneticRejection = rejection;
    }

    /// Sets the recovery trigger period.
    pub fn set_recovery_trigger_period(&mut self, period: u32) {
        self.inner.recoveryTriggerPeriod = period;
    }
}
