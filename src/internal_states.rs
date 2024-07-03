use fusion_imu_sys as sys;

/// AHRS algorithm flags.
pub struct InternalStates {
    pub(crate) inner: sys::FusionAhrsInternalStates,
}

impl InternalStates {
    /// Acceleration error.
    ///
    /// See <https://github.com/xioTechnologies/Fusion> for details.
    pub fn acceleration_error(&self) -> f32 {
        self.inner.accelerationError
    }

    /// Accelerometer ignored.
    ///
    /// See <https://github.com/xioTechnologies/Fusion> for details.
    pub fn accelerometer_ignored(&self) -> bool {
        self.inner.accelerometerIgnored
    }

    /// Acceleration recovery trigger.
    ///
    /// See <https://github.com/xioTechnologies/Fusion> for details.
    pub fn acceleration_recovery_trigger(&self) -> f32 {
        self.inner.accelerationRecoveryTrigger
    }

    /// Angular error.
    ///
    /// See <https://github.com/xioTechnologies/Fusion> for details.
    pub fn magnetic_error(&self) -> f32 {
        self.inner.magneticError
    }

    /// Magnetometer ignored.
    ///
    /// See <https://github.com/xioTechnologies/Fusion> for details.
    pub fn magnetometer_ignored(&self) -> bool {
        self.inner.magnetometerIgnored
    }

    /// Magnetic recovery trigger.
    ///
    /// See <https://github.com/xioTechnologies/Fusion> for details.
    pub fn magnetic_recovery_trigger(&self) -> f32 {
        self.inner.magneticRecoveryTrigger
    }
}
