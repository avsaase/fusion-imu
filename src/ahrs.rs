use core::mem::MaybeUninit;

use fusion_imu_sys as sys;

use crate::math::{Quaternion, Vector};
use crate::settings::Settings;
use crate::{Flags, InternalStates};

/// AHRS algorithm structure.
pub struct FusionAhrs {
    inner: sys::FusionAhrs,
}

impl FusionAhrs {
    /// Create a new `FusionAhrs` instance.
    pub fn new() -> Self {
        let mut ahrs = MaybeUninit::uninit();
        unsafe {
            sys::FusionAhrsInitialise(ahrs.as_mut_ptr());
            FusionAhrs {
                inner: ahrs.assume_init(),
            }
        }
    }

    /// Sets the AHRS algorithm settings.
    pub fn set_settings(&mut self, settings: Settings) {
        unsafe {
            sys::FusionAhrsSetSettings(
                &mut self.inner as *mut sys::FusionAhrs,
                &settings.inner as *const sys::FusionAhrsSettings,
            )
        }
    }

    /// Resets the AHRS algorithm. This is equivalent to reinitialising the
    /// algorithm while maintaining the current settings.
    pub fn reset(&mut self) {
        unsafe {
            sys::FusionAhrsReset(&mut self.inner as *mut sys::FusionAhrs);
        }
    }

    /// Updates the AHRS algorithm using the gyroscope, accelerometer, and
    /// magnetometer measurements.
    ///
    /// Arguments:
    /// - `gyroscope`: Gyroscope measurement in degrees per second.
    /// - `accelerometer`: Accelerometer measurement in g.
    /// - `magnetometer`: Magnetometer measurement in arbitrary units.
    /// - `delta_time`: Delta time in seconds.
    pub fn update(
        &mut self,
        gyroscope: Vector,
        accelerometer: Vector,
        magnetometer: Vector,
        delta_time: f32,
    ) {
        unsafe {
            sys::FusionAhrsUpdate(
                &mut self.inner as *mut sys::FusionAhrs,
                gyroscope.into(),
                accelerometer.into(),
                magnetometer.into(),
                delta_time,
            )
        }
    }

    /// Updates the AHRS algorithm using the gyroscope and accelerometer
    /// measurements only.
    ///
    /// Arguments:
    /// - `gyroscope`: Gyroscope measurement in degrees per second.
    /// - `accelerometer`: Accelerometer measurement in g.
    /// - `delta_time`: Delta time in seconds.
    pub fn update_no_magnetometer(
        &mut self,
        gyroscope: Vector,
        accelerometer: Vector,
        delta_time: f32,
    ) {
        unsafe {
            sys::FusionAhrsUpdateNoMagnetometer(
                &mut self.inner as *mut sys::FusionAhrs,
                gyroscope.into(),
                accelerometer.into(),
                delta_time,
            )
        }
    }

    /// Updates the AHRS algorithm using the gyroscope, accelerometer, and
    /// heading measurements.
    ///
    /// Arguments:
    /// - `gyroscope`: Gyroscope measurement in degrees per second.
    /// - `accelerometer`: Accelerometer measurement in g.
    /// - `heading`: Heading measurement in degrees.
    /// - `delta_time`: Delta time in seconds.
    pub fn update_external_heading(
        &mut self,
        gyroscope: Vector,
        accelerometer: Vector,
        heading: f32,
        delta_time: f32,
    ) {
        unsafe {
            sys::FusionAhrsUpdateExternalHeading(
                &mut self.inner,
                gyroscope.into(),
                accelerometer.into(),
                heading,
                delta_time,
            )
        }
    }

    /// Sets the heading of the orientation measurement provided by the AHRS
    /// algorithm.
    ///
    /// This function can be used to reset drift in heading when the AHRS
    /// algorithm is being used without a magnetometer.
    pub fn set_heading(&mut self, heading: f32) {
        unsafe {
            sys::FusionAhrsSetHeading(&mut self.inner as *mut sys::FusionAhrs, heading);
        }
    }

    /// Returns the quaternion describing the sensor relative to the Earth.
    pub fn get_quaternion(&self) -> Quaternion {
        unsafe { sys::FusionAhrsGetQuaternion(&self.inner as *const sys::FusionAhrs).into() }
    }

    /// Returns the linear acceleration measurement equal to the accelerometer
    /// measurement with the 1g of gravity removed.
    pub fn get_linear_acceleration(&self) -> Vector {
        unsafe {
            sys::FusionAhrsGetLinearAcceleration(&self.inner as *const sys::FusionAhrs).into()
        }
    }

    /// Returns the Earth acceleration measurement equal to the accelerometer
    /// measurement in the Earth coordinate frame with the 1g of gravity
    /// removed.
    pub fn get_earth_acceleration(&self) -> Vector {
        unsafe { sys::FusionAhrsGetEarthAcceleration(&self.inner as *const sys::FusionAhrs).into() }
    }

    /// Returns the AHRS algorithm internal states.
    pub fn get_internal_states(&self) -> InternalStates {
        unsafe {
            let states = sys::FusionAhrsGetInternalStates(&self.inner as *const sys::FusionAhrs);
            InternalStates { inner: states }
        }
    }

    /// Returns the AHRS algorithm flags.
    pub fn get_flags(&self) -> Flags {
        unsafe {
            let flags = sys::FusionAhrsGetFlags(&self.inner as *const sys::FusionAhrs);
            Flags { inner: flags }
        }
    }
}

impl Default for FusionAhrs {
    fn default() -> Self {
        Self::new()
    }
}
