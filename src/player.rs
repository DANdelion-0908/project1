
use std::f32::consts::PI;

use minifb::Window;
use nalgebra_glm::Vec2;

pub struct Player {
    pub pos: Vec2,
    pub a: f32, // angulo de vista
    pub fov: f32, // campo de vista
}

pub fn process_events(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>, block_size: usize) {
    const MOVE_SPEED: f32 = 7.0;
    const ROTATION_SPEED: f32 = PI / 50.0;

    let mut new_x = player.pos.x;
    let mut new_y = player.pos.y;

    if window.is_key_down(minifb::Key::W) {
        new_x += player.a.cos() * MOVE_SPEED;
        new_y += player.a.sin() * MOVE_SPEED;
    }

    if window.is_key_down(minifb::Key::S) {
        new_x -= player.a.cos() * MOVE_SPEED;
        new_y -= player.a.sin() * MOVE_SPEED;
    }

    if window.is_key_down(minifb::Key::A) {
        player.a -= ROTATION_SPEED;
    }

    if window.is_key_down(minifb::Key::D) {
        player.a += ROTATION_SPEED;
    }

    let new_i = (new_x / block_size as f32) as usize;
    let new_j = (new_y / block_size as f32) as usize;

    if maze[new_j][new_i] == ' ' {
        player.pos.x = new_x;
        player.pos.y = new_y;
    }
}