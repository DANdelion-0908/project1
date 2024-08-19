mod framebuffer;
mod maze;
mod player;
mod caster;
mod intersect;

use gilrs::{Gilrs, Button, Event};
use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;
use image::RgbaImage;
use intersect::cast_ray3d;
use minifb::{ Window, WindowOptions, Key };
use nalgebra_glm::Vec2;
use player::process_events;
use std::f32::consts::PI;
use std::time::{ Duration, Instant };
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;
use crate::caster::cast_ray;

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(0xFFFFFF);

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.point(x, y);
        }
    }
}

fn load_texture(path: &str) -> RgbaImage {
    let img = image::open(path).expect("No se pudo cargar la textura");
    img.to_rgba8()
}

fn load_floor_texture(path: &str) -> RgbaImage {
    let img = image::open(path).expect("No se pudo cargar la textura");
    img.to_rgba8()
}

fn apply_texture(framebuffer: &mut Framebuffer, texture: &RgbaImage) {
    let texture_width = texture.width() as f32;
    let texture_height = texture.height() as f32;
    let framebuffer_width = framebuffer.width as f32;
    let framebuffer_height = framebuffer.height as f32;

    for x in 0..framebuffer_width as usize {
        for y in 0..framebuffer_height as usize {
            let tex_x = (x as f32 / framebuffer_width * texture_width) as u32;
            let tex_y = (y as f32 / framebuffer_height * texture_height) as u32;
            let pixel = texture.get_pixel(tex_x, tex_y);

            let color = ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);
            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
        }
    }
}

fn render_minimap(framebuffer: &mut Framebuffer, player: &Player, xo: usize, yo: usize, minimap_size: usize) {
    let maze = load_maze("./maze.txt");
    let block_size = minimap_size / maze.len(); // Ajusta el tamaño del bloque al tamaño del minimapa

    // Dibuja el fondo del minimapa en negro
    framebuffer.set_current_color(0x000000); // Color negro
    for x in xo..xo + minimap_size {
        for y in yo..yo + minimap_size {
            framebuffer.point(x, y);
        }
    }

    // Dibuja el laberinto en el minimapa
    framebuffer.set_current_color(0xFFFFFF); // Color blanco para las paredes del laberinto
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            let cell_x = xo + col * block_size;
            let cell_y = yo + row * block_size;
            draw_cell(framebuffer, cell_x, cell_y, block_size, maze[row][col]);
        }
    }

    // Dibuja al jugador más grande en el minimapa
    framebuffer.set_current_color(0xFF0000); // Color rojo para el jugador
    let player_x = xo + (player.pos.x as usize * minimap_size / framebuffer.width);
    let player_y = yo + (player.pos.y as usize * minimap_size / framebuffer.height);
    let player_size = (5.0 * minimap_size as f32 / framebuffer.width as f32) as usize; // Tamaño del jugador en el minimapa

    // Dibuja un rectángulo para representar al jugador
    for x in player_x..player_x + player_size {
        for y in player_y..player_y + player_size {
            framebuffer.point(x, y);
        }
    }
}


fn render(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 100;

    // draws maze
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col])
        }
    }

    // draw player
    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.point(player.pos.x as usize, player.pos.y as usize);

    // cast ray
    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        cast_ray(framebuffer, &maze, &player, a, block_size);
    }
}

fn render3d(framebuffer: &mut Framebuffer, player: &mut Player, texture: &RgbaImage) {
    let maze = load_maze("./maze.txt");
    let block_size = 100;
    let num_rays = framebuffer.width;

    let _hw = framebuffer.width as f32 / 2.0;
    let hh: f32 = framebuffer.height as f32 / 2.0;

    framebuffer.set_current_color(0xFFFFFF);

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray3d(framebuffer, &maze, player, a, block_size, false);

        let distance_to_wall = intersect.distance;

        let distance_to_projection_plane = (framebuffer.width as f32 / 2.0) / (player.fov / 2.0).tan();

        let stake_height = (block_size as f32 / distance_to_wall) * distance_to_projection_plane;
        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        let texture_x = (intersect.texture_x * (texture.width() as f32)) as usize;

        for y in stake_top..stake_bottom {
            let texture_y: usize = (((y as f32 - stake_top as f32) / (stake_bottom - stake_top) as f32) * texture.height() as f32) as usize;
            let pixel: &image::Rgba<u8> = texture.get_pixel(texture_x.try_into().unwrap(), texture_y.try_into().unwrap());
            let color = ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);

            framebuffer.set_current_color(color);
            framebuffer.point(i, y);
        }
    }
}


fn play_music(file_path: &str) {
    // Crea un nuevo stream de salida
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Abre el archivo de audio
    let file = File::open(file_path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    // Reproduce la música
    stream_handle.play_raw(source.convert_samples()).unwrap();

    // Mantén el programa corriendo mientras se reproduce la música
    std::thread::sleep(std::time::Duration::from_secs(5)); // Ajusta la duración según tu necesidad
}

fn main() {
    let window_width = 1300;
    let window_height = 900;
    let framebuffer_width = 1300;
    let framebuffer_height = 900;
    let frame_delay = Duration::from_millis(16);
    let texture = load_texture("src/test.png");
    let floor = load_texture("src/floor.png");

    let mut gilrs = Gilrs::new().unwrap();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let block_size = 100;

    let mut window = Window::new(
        "Maze Runner",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_background_color(0x000000);

    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0
    };
    
    let mut last_time = Instant::now();
    let mut fps = 0.0;

    let mut last_mouse_x = 0.0;

    if let Some(mouse_pos) = window.get_mouse_pos(minifb::MouseMode::Pass) {
        last_mouse_x = mouse_pos.0;
    }
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // play_music("src/Dialga's Fight to the Finish - 8bit.mp3");

        
        framebuffer.clear();
        
        apply_texture(&mut framebuffer, &floor);
        
        let get_maze = load_maze("./maze.txt");

        process_events(&window, &mut player, &get_maze, block_size, &mut gilrs, &mut last_mouse_x);
        
        render3d(&mut framebuffer, &mut player, &texture);

        render_minimap(&mut framebuffer, &player, 10, 10, 200);

        // Calcula los FPS
        let current_time = Instant::now();
        let duration = current_time.duration_since(last_time);
        last_time = current_time;
        fps = 1.0 / duration.as_secs_f32();

        // Actualiza el título de la ventana con los FPS
        let title = format!("Maze Runner - FPS: {:.2}", fps);
        window.set_title(&title);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}


