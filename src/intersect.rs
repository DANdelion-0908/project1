use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub texture_x: f32,
}

pub fn cast_ray3d(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool,
) -> Intersect {
    let mut d = 0.0;

    framebuffer.set_current_color(0xFFDDDD);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;

        if maze[j][i] != ' ' {
            let hit_x = player.pos.x + cos;
            let hit_y = player.pos.y + sin;

            let horizontal_hit = (hit_y % block_size as f32) == 0.0;

            let texture_x = if horizontal_hit {
                (hit_x % block_size as f32) / block_size as f32
            } else {
                (hit_y % block_size as f32) / block_size as f32
            };

            return Intersect {
                distance: d,
                impact: maze[j][i],
                texture_x, // Retornar la coordenada de textura calculada
            };
        }

        if draw_line {
            framebuffer.point(x, y);
        }

        d += 10.0;
    }
}
