use cgmath::*;
use winit::dpi::PhysicalPosition;
use winit::event::*;

use std::f32::consts::FRAC_PI_2;
// credit goes to sotrh on github for providing this code in theirs "learn wgpu" tutorial
#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

 
pub struct  FpsCamera {
  pub pos: Point3<f32>,
  yaw: Rad<f32>,
  pitch: Rad<f32>,
}

impl FpsCamera {
  pub fn new<V: Into<Point3<f32>>,Y: Into<Rad<f32>>,P: Into<Rad<f32>>,>
  (pos: V,yaw: Y,pitch: P,) -> Self {
      Self {
          pos: pos.into(),
          yaw: yaw.into(),
          pitch: pitch.into(),
      }
  }

  pub fn calc_matrix(&self) -> Matrix4<f32> {
      let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
      let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

      Matrix4::look_to_rh(
          self.pos,
          Vector3::new(
              cos_pitch * cos_yaw,
              sin_pitch,
              cos_pitch * sin_yaw
          ).normalize(),
          Vector3::unit_y(),
      )
  }
}

impl std::fmt::Display for FpsCamera {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();
        write!(f, "(x: {} , y: {} , z: {}) \n (pitch : sin {} | cos {} :  yaw : sin {} , cos {})",
         self.pos.x,self.pos.y,self.pos.z,sin_pitch , cos_pitch ,sin_yaw ,cos_yaw)
    }
}
pub struct Projection {
  aspect: f32,
  fov: Rad<f32>,
  z_near: f32,
  z_far: f32,
}

impl Projection {
  pub fn new<F: Into<Rad<f32>>>(
      width: u32,
      height: u32,
      fov: F,
      z_near: f32,
      z_far: f32,
  ) -> Self {
      Self {
          aspect: width as f32 / height as f32,
          fov: fov.into(),
          z_near,
          z_far,
      }
  }

  pub fn resize(&mut self, width: u32, height: u32) {
      self.aspect = width as f32 / height as f32;
  }

  pub fn calc_matrix(&self) -> Matrix4<f32> {
      OPENGL_TO_WGPU_MATRIX * perspective(self.fov, self.aspect, self.z_near, self.z_far)
  }
}


#[derive(Debug)]
pub struct FpsController {
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
}


impl FpsController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
        }
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool{
        let amount = if state == ElementState::Pressed { 1.0 } else { 0.0 };
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.amount_forward = amount;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.amount_backward = amount;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.amount_left = amount;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.amount_right = amount;
                true
            }
            VirtualKeyCode::Space => {
                self.amount_up = amount;
                true
            }
            VirtualKeyCode::LShift => {
                self.amount_down = amount;
                true
            }
            _ => false,
        }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = -match delta {
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 1.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition {
                y: scroll,
                ..
            }) => *scroll as f32,
        };
    }

    pub fn update_camera(&mut self, camera: &mut FpsCamera) {

        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
        let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.pos += forward * (self.amount_forward - self.amount_backward) * self.speed ;
        camera.pos += right * (self.amount_right - self.amount_left) * self.speed ;

        // Move in/out (aka. "zoom")
        // Note: this isn't an actual zoom. The camera's position
        // changes when zooming.
        let (pitch_sin, pitch_cos) = camera.pitch.0.sin_cos();
        let scrollward = Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        camera.pos += scrollward * self.scroll * self.speed * self.sensitivity ;
        self.scroll = 0.0;

        // Move up/down
        camera.pos.y += (self.amount_up - self.amount_down) * self.speed;

        // Rotate
        camera.yaw += Rad(self.rotate_horizontal)* self.sensitivity ;
        camera.pitch += Rad(-self.rotate_vertical)* self.sensitivity ;

        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        // Keep the camera's angle from going too high/low.
        if camera.pitch < -Rad(SAFE_FRAC_PI_2) {
            camera.pitch = -Rad(SAFE_FRAC_PI_2);
        } else if camera.pitch > Rad(SAFE_FRAC_PI_2) {
            camera.pitch = Rad(SAFE_FRAC_PI_2);
        }
        println!("postion : {:}",camera);
    }
    
    pub fn set_to_origin(&self, camera : &mut FpsCamera) {
        camera.pos = (0.0, 3.0, 0.0).into();
        camera.yaw = cgmath::Deg(-90.0).into();
        camera.pitch = cgmath::Deg(-90.0).into();
    }

}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
   pub  view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
  pub  fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

 pub   fn update_view_proj(&mut self, camera: &FpsCamera, projection: &Projection) {
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into();
    }
}

