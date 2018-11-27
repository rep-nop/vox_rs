// crates
extern crate three;
extern crate vox_utils;

// mods
mod gamestate;

// namespacing
use vox_utils::voxel::Voxel;
use gamestate::GameState;

fn main() {
    let mut gs = GameState::new();

    let vox = Voxel::new(&mut gs.win.factory, (0, 0, 0), 0xF8D790);
    gs.win.scene.add(&vox.group);

    while gs.win.update() && !gs.win.input.hit(three::KEY_ESCAPE) {
        gs.handle_input();

        gs.update();
        
        gs.win.render(&gs.cam);
    }
}