mod game;
mod states;

use macroquad::prelude::*;

#[macroquad::main("Kapitan Dupa The Game - cvgore 2021")]
async fn main() {
    loop {
        clear_background(BLACK);

        let delta = get_frame_time();



        next_frame().await
    }
}