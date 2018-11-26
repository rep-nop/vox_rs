use three::{
    Object,
    Key,
};

// holds camera info
pub struct GameState {
    pub win: three::Window,

    pub cam: three::camera::Camera,
    pub cam_pos: (f32, f32, f32),

    pub light: three::light::Point,
}

impl GameState {
    pub fn new() -> Self {
        // init the window
        let mut win = three::Window::new("Voxel lads");
        win.scene.background = three::Background::Color(0xC6F0FF);

        let cam_pos = (0.0, 0.0, 0.0);

        // init the cam
        let cam = win.factory.perspective_camera(60.0, 0.1 .. 100.0);
        cam.set_position([cam_pos.0, cam_pos.1, cam_pos.2]);
        win.scene.add(&cam);

        let light = win.factory.point_light(0xFFFFFF, 0.5);
        light.set_position([0.0, 15.0, 0.0]);
        win.scene.add(&light);

        GameState { win, cam, cam_pos, light }
    }

    pub fn handle_input(&mut self) {
        let input = self.win.input.keys_hit();
        for key_press in input {
            match key_press {
                Key::W => { self.cam_pos.2 += 0.1; },
                Key::S => { self.cam_pos.2 -= 0.1; },
                Key::D => { self.cam_pos.0 += 0.1; },
                Key::A => { self.cam_pos.0 -= 0.1; },

                Key::Space => { self.cam_pos.1 += 0.1; },
                Key::LControl => { self.cam_pos.1 -= 0.1; },

                _ => {},
            }
        }
    }

    pub fn update(&mut self) {
        
    }
}