
use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub fn cast_ray(framebuffer: &mut Framebuffer, maze: &Vec<Vec<char>>, player: &Player, a: f32, block_size: usize) {
    let mut d = 0.0;

    framebuffer.set_current_color(0xFFDDDD);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();

        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;

        // Verificar si los índices están dentro de los límites del laberinto
        if i >= maze[0].len() || j >= maze.len() {
            break;
        }

        // Si el rayo choca con una pared, detén el bucle
        if maze[j][i] != ' ' {
            return;
        }

        framebuffer.point(x, y);

        d += 10.0;
    }
}
