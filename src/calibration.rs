use core::mem::transmute;

use fusion_imu_sys as sys;

use crate::{Matrix, Vector};

/// Gyroscope and accelerometer calibration model.
///
/// Arguments:
/// - `uncalibrated`: Uncalibrated measurement.
/// - `misalignment`: Misalignment matrix.
/// - `sensitivity`: Sensitivity vector.
/// - `offset`: Offset vector.
pub fn calibration_inertial(
    uncalibrated: Vector,
    misalignment: Matrix,
    sensitivity: Vector,
    offset: Vector,
) -> Vector {
    unsafe {
        transmute(sys::FusionCalibrationInertial(
            transmute(uncalibrated),
            transmute(misalignment),
            transmute(sensitivity),
            transmute(offset),
        ))
    }
}

/// Magnetometer calibration model.
///
/// Arguments:
/// - `uncalibrated`: Uncalibrated measurement.
/// - `soft_iron`: Soft-iron matrix.
/// - `hard_iron`: Hard-iron offset vector.
pub fn calibration_magnetic(uncalibrated: Vector, soft_iron: Matrix, hard_iron: Vector) -> Vector {
    unsafe {
        transmute(sys::FusionCalibrationMagnetic(
            transmute(uncalibrated),
            transmute(soft_iron),
            transmute(hard_iron),
        ))
    }
}
