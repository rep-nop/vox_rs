extern crate three;
extern crate vox_utils;

use three::Object;

fn main() {
    vox_utils::henlo();
    
    let title = "We out here with voxels";
    let mut win = three::Window::new(title);
    
    win.scene.background = three::Background::Color(0xC6F0FF);

    let cam = win.factory.perspective_camera(60.0, 0.1 .. 1.0);
    cam.set_position([0.0, 5.0, -5.0]);

    let vox = vox_utils::voxel::Voxel::new(&mut win.factory, (0.0, 0.0, 0.0), 0xFFFFFF);
    vox.group.set_position([vox.loc.0, vox.loc.1, vox.loc.2]);
    win.scene.add(&vox.group);


    while win.update() && !win.input.hit(three::KEY_ESCAPE) {
        win.render(&cam);
    }
}