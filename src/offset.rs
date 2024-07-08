use core::mem::MaybeUninit;

use fusion_imu_sys as sys;

use crate::Vector;

/// Gyroscope offset algorithm structure.
pub struct FusionOffset {
    inner: sys::FusionOffset,
}

impl FusionOffset {
    /// Create a new `FusionOffset` instance.
    ///
    /// Sample rate in Hz.
    pub fn new(sample_rate: u32) -> Self {
        let mut offset = MaybeUninit::uninit();
        unsafe {
            sys::FusionOffsetInitialise(offset.as_mut_ptr(), sample_rate);
            FusionOffset {
                inner: offset.assume_init(),
            }
        }
    }

    /// Updates the gyroscope offset algorithm and returns the corrected
    /// gyroscope measurement. Values are in degrees per second.
    pub fn update(&mut self, gyroscope: Vector) -> Vector {
        unsafe {
            sys::FusionOffsetUpdate(&mut self.inner as *mut sys::FusionOffset, gyroscope.into())
                .into()
        }
    }
}
