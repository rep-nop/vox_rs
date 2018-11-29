// crates
extern crate three;
extern crate vox_utils;

// mods
mod gamestate;

// namespacing
use gamestate::GameState;
use vox_utils::{chunk::ChunkBuilder, Color};

const RED: u32 = Color::new(None, 0xFF, 0x00, 0x00);
const BLUE: u32 = Color::new(None, 0x00, 0xFF, 0x00);
const GREEN: u32 = Color::new(None, 0x00, 0x00, 0xFF);

fn main() {
    let mut gs = GameState::new();

    gs.chunks.push(
        ChunkBuilder::new(&mut gs.win.factory, (0, 0, 0))
            .with_voxel((0, 2, 0), BLUE)
            .with_voxel((2, 0, 0), RED)
            .with_voxel((0, 0, 2), GREEN)
            .finish(),
    );

    gs.chunks[0].add_to_scene(&mut gs.win.scene);

    while gs.win.update() && !gs.win.input.hit(three::KEY_ESCAPE) {
        gs.handle_input();

        gs.update();

        gs.win.render(&gs.cam);
    }
}
