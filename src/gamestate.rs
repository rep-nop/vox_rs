use three::{Key, Object};
use vox_utils::{chunk::Chunk, Vec3};

// consts
const CAMERA_MOVE: f32 = 0.5;

// holds camera info
pub struct GameState {
    pub win: three::Window,

    pub cam: three::camera::Camera,
    pub cam_pos: Vec3<f32>,

    pub light: three::light::Point,

    pub chunks: Vec<Chunk>,
}

impl GameState {
    pub fn new() -> Self {
        // init the window
        let mut win = three::Window::new("Voxel lads");
        win.scene.background = three::Background::Color(0x00C6_F0FF);

        let cam_pos: Vec3<f32> = (0.0, 0.0, 10.0).into();

        // init the cam
        let cam = win.factory.perspective_camera(60.0, 0.1..50.0);
        cam.set_position([cam_pos.x, cam_pos.y, cam_pos.z]);
        win.scene.add(&cam);

        // init the light
        let light = win.factory.point_light(0x00FF_FFFF, 0.5);
        light.set_position([0.0, 15.0, 5.0]);
        win.scene.add(&light);

        // the chunks
        let chunks: Vec<Chunk> = Vec::new();

        GameState {
            win,
            cam,
            cam_pos,
            light,
            chunks,
        }
    }

    pub fn handle_input(&mut self) {
        let input = self.win.input.keys_hit();
        for key_press in input {
            match key_press {
                Key::W => {
                    self.cam_pos.z -= CAMERA_MOVE;
                }
                Key::S => {
                    self.cam_pos.z += CAMERA_MOVE;
                }
                Key::D => {
                    self.cam_pos.x += CAMERA_MOVE;
                }
                Key::A => {
                    self.cam_pos.x -= CAMERA_MOVE;
                }

                Key::Space => {
                    self.cam_pos.y += CAMERA_MOVE;
                }
                Key::LControl => {
                    self.cam_pos.y -= CAMERA_MOVE;
                }

                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        let new = self.cam_pos;
        self.cam.set_position([new.x, new.y, new.z]);
    }
}
