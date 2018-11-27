// crates
extern crate three;
extern crate vox_utils;

// mods
mod gamestate;

// namespacing
use vox_utils::{
    chunk::Chunk,
};
use gamestate::GameState;

fn main() {
    let mut gs = GameState::new();

    gs.chunks.push(Chunk::new(&mut gs.win.factory, (0,0,0)));
    gs.chunks[0].set_voxel(&mut gs.win.factory, (0,2,0), 0x00FF00);
    gs.chunks[0].set_voxel(&mut gs.win.factory, (2,0,0), 0xFF0000);
    gs.chunks[0].set_voxel(&mut gs.win.factory, (0,0,2), 0x0000FF);

    &gs.chunks[0].add_to_scene(&mut gs.win.scene);

    while gs.win.update() && !gs.win.input.hit(three::KEY_ESCAPE) {
        gs.handle_input();

        gs.update();
        
        gs.win.render(&gs.cam);
    }
}