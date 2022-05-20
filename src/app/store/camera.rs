use nalgebra::{Isometry3, Perspective3, Point3, Vector3};
use std::f32::consts::PI;

pub struct Camera {
    projection: Perspective3<f32>,
    left_right_radians: f32,
    up_down_radians: f32,
    orbit_radius: f32,
    center: Vector3<f32>,
}

const FOVY: f32 = PI / 4.0;
const ZNEAR: f32 = 1.0;
const ZFAR: f32 = 1000.0;

impl Camera {
    pub fn new(aspect: f32) -> Camera {
        Camera {
            projection: Perspective3::new(aspect, FOVY, ZNEAR, ZFAR),
            left_right_radians: 250.0f32.to_radians(),
            up_down_radians: -8.0f32.to_radians(),
            orbit_radius: 5.,
            center: Vector3::new(9.0, -84.0, -2.6),
        }
    }

    pub fn update_aspect(&mut self, aspect: f32) {
        self.projection = Perspective3::new(aspect, FOVY, ZNEAR, ZFAR);
    }

    pub fn view(&self) -> [f32; 16] {
        let eye = self.get_eye_pos();

        let target = Point3::new(0.0, 0.0, 0.0);

        let mut z1 = Vector3::z();
        z1.neg_mut();
        let view = Isometry3::look_at_rh(&eye, &(target + &self.center), &z1);

        let view = view.to_homogeneous();

        let mut view_array = [0.; 16];
        view_array.copy_from_slice(view.as_slice());

        view_array
    }

    pub fn view_flipped_y(&self) -> [f32; 16] {
        let mut eye = self.get_eye_pos();
        eye.z = -1.0 * eye.z;

        let target = Point3::new(1.0, 1.0, 0.0);

        let mut z1 = Vector3::z();
        z1.neg_mut();
        let view = Isometry3::look_at_rh(&eye, &(target + &self.center), &z1);

        let view = view.to_homogeneous();

        let mut view_array = [0.; 16];
        view_array.copy_from_slice(view.as_slice());

        view_array
    }

    pub fn get_eye_pos(&self) -> Point3<f32> {
        let yaw = self.left_right_radians;
        let pitch = self.up_down_radians;

        let eye_x = self.orbit_radius * yaw.sin() * pitch.cos();
        let eye_z = self.orbit_radius * pitch.sin();
        let eye_y = self.orbit_radius * yaw.cos() * pitch.cos();

        Point3::new(eye_x, eye_y, eye_z) + &self.center
    }
    pub fn projection(&self) -> [f32; 16] {
        let mut perspective_array = [0.; 16];
        perspective_array.copy_from_slice(self.projection.as_matrix().as_slice());

        perspective_array
    }

    pub fn orbit_left_right(&mut self, delta: f32) {
        self.left_right_radians += delta;
    }

    pub fn orbit_up_down(&mut self, delta: f32) {
        self.up_down_radians += delta;

        // Make sure:
        // 0.1 <= radians <= PI / 2.1
        // in order to restrict the camera's up/down orbit motion

        if self.up_down_radians - (PI / 2.1) > 0. {
            self.up_down_radians = PI / 2.1;
        }

        if self.up_down_radians + PI/8.1 < 0. {
            self.up_down_radians = -PI/8.1;
        }
    }

    pub fn zoom(&mut self, zoom: f32) {
        // self.orbit_radius += zoom;
        //
        // if self.orbit_radius > 50. {
        //     self.orbit_radius = 50.;
        // } else if self.orbit_radius < 5. {
        //     self.orbit_radius = 5.;
        // }
    }
}
