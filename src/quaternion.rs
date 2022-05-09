use std::ops::Mul;

#[derive(Clone,Copy,Debug)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn inv(self) -> Self {
        Quaternion {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w:  self.w,
        }
    }

    pub fn to_matrix(self) -> [[f32; 4]; 4] {
        let xx = self.x * self.x;
        let yy = self.y * self.y;
        let zz = self.z * self.z;
        let xy = self.x * self.y;
        let xz = self.x * self.z;
        let xw = self.x * self.w;
        let yz = self.y * self.z;
        let yw = self.y * self.w;
        let zw = self.z * self.w;
        [
            [ 1.0-2.0*yy-2.0*zz,     2.0*xy-2.0*zw,     2.0*xz+2.0*yw, 0.0],
            [     2.0*xy+2.0*zw, 1.0-2.0*xx-2.0*zz,     2.0*yz-2.0*xw, 0.0],
            [     2.0*xz-2.0*yw,     2.0*yz+2.0*xw, 1.0-2.0*xx-2.0*yy, 0.0],
            [               0.0,               0.0,               0.0, 1.0f32 ],
        ]
    }
}

impl Mul for Quaternion {

    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Quaternion {
            x: (self.w * rhs.x) + (self.x * rhs.w) + (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.w * rhs.y) - (self.x * rhs.z) + (self.y * rhs.w) + (self.z * rhs.x),
            z: (self.w * rhs.z) + (self.x * rhs.y) - (self.y * rhs.x) + (self.z * rhs.w),
            w: (self.w * rhs.w) - (self.x * rhs.x) - (self.y * rhs.y) - (self.z * rhs.z),
        } 
    }
}

pub fn from_axis_angle(axis: [f32; 3], angle: f32) -> Quaternion {
    Quaternion {
        x: axis[0] * (angle/2 as f32).sin(),
        y: axis[1] * (angle/2 as f32).sin(),
        z: axis[2] * (angle/2 as f32).sin(),
        w: (angle/2 as f32).cos(),
    }
}
