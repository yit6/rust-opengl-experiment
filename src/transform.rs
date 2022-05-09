use crate::quaternion;

#[derive(Clone,Copy,Debug)]
pub struct Transform {
    pub translation: [f32; 3],
    pub rotation: quaternion::Quaternion,
    pub scale: [f32; 3],
}

impl Transform {

    pub fn new() -> Self {
        Transform {
            translation: [0.0, 0.0, 0.0f32],
            rotation: quaternion::Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            scale: [1.0, 1.0, 1.0f32],
        }
    }

    pub fn to_matrix(self) -> [[f32; 4]; 4] {
        let sx = self.scale[0];
        let sy = self.scale[1];
        let sz = self.scale[2];
        let tx = self.translation[0];
        let ty = self.translation[1];
        let tz = self.translation[2];
        let rot = self.rotation.to_matrix();
        [
            [ rot[0][0]*sx, rot[0][1]*sy, rot[0][2]*sz, 0.0],
            [ rot[1][0]*sx, rot[1][1]*sy, rot[1][2]*sz, 0.0],
            [ rot[2][0]*sx, rot[2][1]*sy, rot[2][2]*sz, 0.0],
            [     tx,     ty,     tz, 1.0f32],
        ]
    }

    pub fn view_matrix(self) -> [[f32; 4]; 4] {
        let translation_mat = [
            [ 1.0, 0.0, 0.0, 0.0 ],
            [ 0.0, 1.0, 0.0, 0.0 ],
            [ 0.0, 0.0, 1.0, 0.0 ],
            [ -self.translation[0], -self.translation[1], -self.translation[2], 1.0 ],
        ];
            let rotation_mat = self.rotation.inv().to_matrix();
            mat_mul(translation_mat, rotation_mat)
    }

    pub fn scale(&mut self, s: f32) {
        self.scale[0] *= s;
        self.scale[1] *= s;
        self.scale[2] *= s;
    }

    pub fn translate(&mut self, t: [f32; 3]) {
        self.translation[0] += t[0];
        self.translation[1] += t[1];
        self.translation[2] += t[2];
    }

    pub fn local_translate(&mut self, t: [f32; 3]) {
        let local_t = self.rotation.inv() * quaternion::Quaternion{ x: t[0], y: t[1], z: t[2], w: 0.0 } * self.rotation;
        self.translation[0] += local_t.x;
        self.translation[1] += local_t.y;
        self.translation[2] += local_t.z;
    }

    pub fn set_translation(&mut self, t: [f32; 3]) {
        self.translation[0] = t[0];
        self.translation[1] = t[1];
        self.translation[2] = t[2];
    }

    pub fn rotate(&mut self, q: quaternion::Quaternion) {
        self.rotation = q * self.rotation.clone();
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform::new()
    }
}

fn mat_mul(a: [[f32; 4]; 4], b: [[f32; 4]; 4]) -> [[f32; 4]; 4] {

    let mut c = [
        [ 0.0, 0.0, 0.0, 0.0 ],
        [ 0.0, 0.0, 0.0, 0.0 ],
        [ 0.0, 0.0, 0.0, 0.0 ],
        [ 0.0, 0.0, 0.0, 0.0f32 ],
    ];

    for j in 0..4 {
        for i in 0..4 {
            for k in 0..4 {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}
