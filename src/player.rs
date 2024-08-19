use gilrs::{Gilrs, Button, Event};
use minifb::Window;
use nalgebra_glm::Vec2;
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vec2,
    pub a: f32,  // ángulo de vista
    pub fov: f32, // campo de vista
}

pub fn process_events(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>, block_size: usize, gilrs: &mut Gilrs, last_mouse_x: &mut f32) {
    const MOVE_SPEED: f32 = 7.0;
    const ROTATION_SPEED: f32 = PI / 50.0;
    const MOUSE_SENSITIVITY: f32 = 0.005;

    let mut new_x = player.pos.x;
    let mut new_y = player.pos.y;

    // Manejo de teclado
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

    // Manejo del mouse
    if let Some(mouse_pos) = window.get_mouse_pos(minifb::MouseMode::Pass) {
        let (mouse_x, _) = mouse_pos;

        // Calcula la diferencia de la posición X del mouse
        let delta_x = mouse_x - *last_mouse_x;

        // Actualiza el ángulo de visión del jugador en función del movimiento del mouse
        player.a += delta_x * MOUSE_SENSITIVITY;

        // Actualiza la última posición del mouse
        *last_mouse_x = mouse_x;
    }

    // Manejo del controlador
    while let Some(Event { id: _, event, .. }) = gilrs.next_event() {
        match event {
            gilrs::EventType::ButtonPressed(Button::South, _) => {
                new_x += player.a.cos() * MOVE_SPEED;
                new_y += player.a.sin() * MOVE_SPEED;
            }
            gilrs::EventType::ButtonPressed(Button::East, _) => {
                new_x -= player.a.cos() * MOVE_SPEED;
                new_y -= player.a.sin() * MOVE_SPEED;
            }
            gilrs::EventType::AxisChanged(gilrs::Axis::LeftStickX, value, _) => {
                player.a += value * ROTATION_SPEED;
            }
            gilrs::EventType::AxisChanged(gilrs::Axis::LeftStickY, value, _) => {
                new_x += player.a.cos() * value * MOVE_SPEED;
                new_y += player.a.sin() * value * MOVE_SPEED;
            }
            _ => {}
        }
    }

    let new_i = (new_x / block_size as f32) as usize;
    let new_j = (new_y / block_size as f32) as usize;

    if maze[new_j][new_i] == ' ' {
        player.pos.x = new_x;
        player.pos.y = new_y;
    }
}
