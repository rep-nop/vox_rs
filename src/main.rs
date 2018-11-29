// crates
extern crate three;
extern crate vox_utils;

// mods
mod gamestate;

// namespacing
use gamestate::GameState;
use vox_utils::{chunk::ChunkBuilder, Color};

const RED: Color = Color {
    a: 0x00,
    r: 0xFF,
    g: 0x00,
    b: 0x00,
};
const GREEN: Color = Color {
    a: 0x00,
    r: 0x00,
    g: 0xFF,
    b: 0x00,
};
const BLUE: Color = Color {
    a: 0x00,
    r: 0x00,
    g: 0x00,
    b: 0xFF,
};

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
