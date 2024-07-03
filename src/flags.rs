use fusion_imu_sys as sys;

/// AHRS algorithm flags.
pub struct Flags {
    pub(crate) inner: sys::FusionAhrsFlags,
}

impl Flags {
    /// Initialising flag.
    ///
    /// See <https://github.com/xioTechnologies/Fusion> for details.
    pub fn initialising(&self) -> bool {
        self.inner.initialising
    }

    /// Angular rate recovery flag.
    ///
    /// See <https://github.com/xioTechnologies/Fusion> for details.
    pub fn angular_rate_recovery(&self) -> bool {
        self.inner.angularRateRecovery
    }

    /// Acceleration recovery flag.
    ///
    /// See <https://github.com/xioTechnologies/Fusion> for details.
    pub fn acceleration_recovery(&self) -> bool {
        self.inner.accelerationRecovery
    }

    /// Magnetic recovery flag.
    ///
    /// See <https://github.com/xioTechnologies/Fusion> for details.
    pub fn magnetic_recovery(&self) -> bool {
        self.inner.magneticRecovery
    }
}
