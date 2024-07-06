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

impl From<sys::FusionVector> for Vector {
    fn from(value: sys::FusionVector) -> Self {
        let values: sys::FusionVector__bindgen_ty_1 = unsafe { value.axis };
        Self {
            x: values.x,
            y: values.y,
            z: values.z,
        }
    }
}

impl From<Vector> for sys::FusionVector {
    fn from(value: Vector) -> Self {
        sys::FusionVector {
            array: [value.x, value.y, value.z],
        }
    }
}

/// Quaternion.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(C)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Quaternion {
    /// Converts a quaternion to ZYX Euler angles in degrees.
    pub fn to_euler(self) -> Euler {
        unsafe { sys::FusionQuaternionToEuler(self.into()).into() }
    }
}

impl From<sys::FusionQuaternion> for Quaternion {
    fn from(value: sys::FusionQuaternion) -> Self {
        let values: sys::FusionQuaternion__bindgen_ty_1 = unsafe { value.element };
        Self {
            w: values.w,
            x: values.x,
            y: values.y,
            z: values.z,
        }
    }
}

impl From<Quaternion> for sys::FusionQuaternion {
    fn from(value: Quaternion) -> Self {
        sys::FusionQuaternion {
            array: [value.w, value.x, value.y, value.z],
        }
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

impl From<sys::FusionMatrix> for Matrix {
    fn from(value: sys::FusionMatrix) -> Self {
        let values: sys::FusionMatrix__bindgen_ty_1 = unsafe { value.element };
        Self {
            xx: values.xx,
            xy: values.xy,
            xz: values.xz,
            yx: values.yx,
            yy: values.yy,
            yz: values.yz,
            zx: values.zx,
            zy: values.zy,
            zz: values.zz,
        }
    }
}

impl From<Matrix> for sys::FusionMatrix {
    fn from(value: Matrix) -> Self {
        sys::FusionMatrix {
            array: [
                [value.xx, value.xy, value.xz],
                [value.yx, value.yy, value.yz],
                [value.zx, value.zy, value.zz],
            ],
        }
    }
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

impl From<sys::FusionEuler> for Euler {
    fn from(value: sys::FusionEuler) -> Self {
        let values: sys::FusionEuler__bindgen_ty_1 = unsafe { value.angle };
        Self {
            roll: values.roll,
            pitch: values.pitch,
            yaw: values.yaw,
        }
    }
}

impl From<Euler> for sys::FusionEuler {
    fn from(value: Euler) -> Self {
        sys::FusionEuler {
            array: [value.roll, value.pitch, value.yaw],
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_maps_from_sys_array() {
        let sys_vector = sys::FusionVector {
            array: [1.0, 2.0, 3.0],
        };

        // Act
        let vector = Vector::from(sys_vector);

        assert_eq!(
            vector,
            Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            }
        );
    }

    #[test]
    fn vector_maps_from_sys_axis() {
        let sys_vector = sys::FusionVector {
            axis: sys::FusionVector__bindgen_ty_1 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
        };

        // Act
        let vector = Vector::from(sys_vector);

        assert_eq!(
            vector,
            Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            }
        );
    }

    #[test]
    fn vector_maps_to_sys_array() {
        let vector = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        // Act
        let sys_vector = sys::FusionVector::from(vector);

        let values = unsafe { sys_vector.array };
        assert_eq!(values, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn vector_maps_to_sys_axis() {
        let vector = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        // Act
        let sys_vector = sys::FusionVector::from(vector);

        let values = unsafe { sys_vector.axis };
        assert_eq!(values.x, 1.0);
        assert_eq!(values.y, 2.0);
        assert_eq!(values.z, 3.0);
    }

    #[test]
    fn quaternion_maps_from_sys_array() {
        let sys_quaternion = sys::FusionQuaternion {
            array: [1.0, 2.0, 3.0, 4.0],
        };

        // Act
        let quaternion = Quaternion::from(sys_quaternion);

        assert_eq!(
            quaternion,
            Quaternion {
                w: 1.0,
                x: 2.0,
                y: 3.0,
                z: 4.0,
            }
        );
    }

    #[test]
    fn quaternion_maps_from_sys_element() {
        let sys_quaternion = sys::FusionQuaternion {
            element: sys::FusionQuaternion__bindgen_ty_1 {
                w: 1.0,
                x: 2.0,
                y: 3.0,
                z: 4.0,
            },
        };

        // Act
        let quaternion = Quaternion::from(sys_quaternion);

        assert_eq!(
            quaternion,
            Quaternion {
                w: 1.0,
                x: 2.0,
                y: 3.0,
                z: 4.0,
            }
        );
    }

    #[test]
    fn quaternion_maps_to_sys_array() {
        let quaternion = Quaternion {
            w: 1.0,
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };

        // Act
        let sys_quaternion = sys::FusionQuaternion::from(quaternion);

        let values = unsafe { sys_quaternion.array };
        assert_eq!(values, [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn quaternion_maps_to_sys_element() {
        let quaternion = Quaternion {
            w: 1.0,
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };

        // Act
        let sys_quaternion = sys::FusionQuaternion::from(quaternion);

        let values = unsafe { sys_quaternion.element };
        assert_eq!(values.w, 1.0);
        assert_eq!(values.x, 2.0);
        assert_eq!(values.y, 3.0);
        assert_eq!(values.z, 4.0);
    }

    #[test]
    fn matrix_maps_from_sys_array() {
        let sys_matrix = sys::FusionMatrix {
            array: [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]],
        };

        // Act
        let matrix = Matrix::from(sys_matrix);

        assert_eq!(
            matrix,
            Matrix {
                xx: 1.0,
                xy: 2.0,
                xz: 3.0,
                yx: 4.0,
                yy: 5.0,
                yz: 6.0,
                zx: 7.0,
                zy: 8.0,
                zz: 9.0,
            }
        );
    }

    #[test]
    fn matrix_maps_from_sys_element() {
        let sys_matrix = sys::FusionMatrix {
            element: sys::FusionMatrix__bindgen_ty_1 {
                xx: 1.0,
                xy: 2.0,
                xz: 3.0,
                yx: 4.0,
                yy: 5.0,
                yz: 6.0,
                zx: 7.0,
                zy: 8.0,
                zz: 9.0,
            },
        };

        // Act
        let matrix = Matrix::from(sys_matrix);

        assert_eq!(
            matrix,
            Matrix {
                xx: 1.0,
                xy: 2.0,
                xz: 3.0,
                yx: 4.0,
                yy: 5.0,
                yz: 6.0,
                zx: 7.0,
                zy: 8.0,
                zz: 9.0,
            }
        );
    }

    #[test]
    fn matrix_maps_to_sys_array() {
        let matrix = Matrix {
            xx: 1.0,
            xy: 2.0,
            xz: 3.0,
            yx: 4.0,
            yy: 5.0,
            yz: 6.0,
            zx: 7.0,
            zy: 8.0,
            zz: 9.0,
        };

        // Act
        let sys_matrix = sys::FusionMatrix::from(matrix);

        let values = unsafe { sys_matrix.array };
        assert_eq!(values, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
    }

    #[test]
    fn matrix_maps_to_sys_element() {
        let matrix = Matrix {
            xx: 1.0,
            xy: 2.0,
            xz: 3.0,
            yx: 4.0,
            yy: 5.0,
            yz: 6.0,
            zx: 7.0,
            zy: 8.0,
            zz: 9.0,
        };

        // Act
        let sys_matrix = sys::FusionMatrix::from(matrix);

        let values = unsafe { sys_matrix.element };
        assert_eq!(values.xx, 1.0,);
        assert_eq!(values.xy, 2.0);
        assert_eq!(values.xz, 3.0);
        assert_eq!(values.yx, 4.0);
        assert_eq!(values.yy, 5.0);
        assert_eq!(values.yz, 6.0);
        assert_eq!(values.zx, 7.0);
        assert_eq!(values.zy, 8.0);
        assert_eq!(values.zz, 9.0);
    }

    #[test]
    fn euler_maps_from_sys_array() {
        let sys_euler = sys::FusionEuler {
            array: [1.0, 2.0, 3.0],
        };

        // Act
        let euler = Euler::from(sys_euler);

        assert_eq!(
            euler,
            Euler {
                roll: 1.0,
                pitch: 2.0,
                yaw: 3.0
            }
        );
    }

    #[test]
    fn euler_maps_from_sys_element() {
        let sys_euler = sys::FusionEuler {
            angle: sys::FusionEuler__bindgen_ty_1 {
                roll: 1.0,
                pitch: 2.0,
                yaw: 3.0,
            },
        };

        // Act
        let euler = Euler::from(sys_euler);

        assert_eq!(
            euler,
            Euler {
                roll: 1.0,
                pitch: 2.0,
                yaw: 3.0,
            }
        );
    }

    #[test]
    fn euler_maps_to_sys_array() {
        let euler = Euler {
            roll: 1.0,
            pitch: 2.0,
            yaw: 3.0,
        };

        // Act
        let sys_euler = sys::FusionEuler::from(euler);

        let values = unsafe { sys_euler.array };
        assert_eq!(values, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn euler_maps_to_sys_element() {
        let euler = Euler {
            roll: 1.0,
            pitch: 2.0,
            yaw: 3.0,
        };

        // Act
        let sys_euler = sys::FusionEuler::from(euler);

        let values = unsafe { sys_euler.angle };
        assert_eq!(values.roll, 1.0);
        assert_eq!(values.pitch, 2.0);
        assert_eq!(values.yaw, 3.0);
    }
}
