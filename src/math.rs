use core::mem::transmute;

use fusion_imu_sys as sys;

/// 3D vector.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(C)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    /// Create a new `Vector`.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// Quaternion.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(C)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    /// Converts a quaternion to ZYX Euler angles in degrees.
    pub fn to_euler(self) -> Euler {
        unsafe { transmute(sys::FusionQuaternionToEuler(transmute(self))) }
    }
}

/// 3x3 matrix in row-major order.
///
/// See <http://en.wikipedia.org/wiki/Row-major_order>
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(C)]
pub struct Matrix {
    pub xx: f32,
    pub xy: f32,
    pub xz: f32,
    pub yx: f32,
    pub yy: f32,
    pub yz: f32,
    pub zx: f32,
    pub zy: f32,
    pub zz: f32,
}

/// Euler angles.  
///
/// Roll, pitch, and yaw correspond to rotations around X, Y, and Z
/// respectively.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(C)]
pub struct Euler {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

/// Earth axes convention.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub enum Convention {
    #[default]
    NorthWestUp,
    EastNorthUp,
    NorthWestDown,
}
