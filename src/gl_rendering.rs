use board::Board;
use columns::Jewel;
use graphics::Color;
use graphics::V2T2C4;
use graphics::Vector2;
use point2::Size2;
use rectangle::Rectangle;

pub fn push_quad_vertices(
    vertex_buffer: &mut Vec<V2T2C4>,
    position: Vector2,
    size: Vector2,
    color: Color,
) {
    let min_x = position[0];
    let min_y = position[1];
    let max_x = position[0] + size[0];
    let max_y = position[1] + size[1];
    vertex_buffer.push(V2T2C4([min_x, min_y], [0.0, 0.0], color));
    vertex_buffer.push(V2T2C4([max_x, max_y], [1.0, 1.0], color));
    vertex_buffer.push(V2T2C4([min_x, max_y], [0.0, 1.0], color));
    vertex_buffer.push(V2T2C4([min_x, min_y], [0.0, 0.0], color));
    vertex_buffer.push(V2T2C4([max_x, max_y], [1.0, 1.0], color));
    vertex_buffer.push(V2T2C4([max_x, min_y], [1.0, 0.0], color));
}

pub fn draw_board(
    mut vertex_buffer: &mut Vec<V2T2C4>,
    board: &Board,
    column: Option<::columns::Column>,
    target: Vector2,
    tile_size: Vector2,
    tile_padding: Vector2,
) {
    for x in 0..board.width() {
        for y in 0..board.height() {
            if let Some(jewel) = board[x][y] {
                let dest_x = (x as f32 * (tile_size[0] + tile_padding[0]) + tile_padding[0]) as f32
                    + target[0];
                let dest_y = (y as f32 * (tile_size[1] + tile_padding[1]) + tile_padding[1]) as f32
                    + target[1];
                draw_jewel(vertex_buffer, [dest_x, dest_y], tile_size, jewel);
            }
        }
    }
    if let Some(column) = column {
        let mut p = column.position;
        for i in 0..3 {
            let dest_x = (p.x as f32 * (tile_size[0] + tile_padding[0]) + tile_padding[0]) as f32
                + target[0];
            let dest_y = (p.y as f32 * (tile_size[1] + tile_padding[1]) + tile_padding[1]) as f32
                + target[1];
            draw_jewel(
                &mut vertex_buffer,
                [dest_x, dest_y],
                tile_size,
                column.jewels[i],
            );
            p.up();
        }
    }
}

use graphics;
use point2::P2;
use std;
pub fn draw_board_highlight_matches(
    vertex_buffer: &mut Vec<V2T2C4>,
    board: &Board,
    matches: &std::collections::HashSet<P2>,
    target: Vector2,
    tile_size: Vector2,
    tile_padding: Vector2,
) {
    for x in 0..board.width() {
        for y in 0..board.height() {
            if let Some(jewel) = board[x][y] {
                let dest_x = (x as f32 * (tile_size[0] + tile_padding[0]) + tile_padding[0]) as f32
                    + target[0];
                let dest_y = (y as f32 * (tile_size[1] + tile_padding[1]) + tile_padding[1]) as f32
                    + target[1];
                let p = P2::new(x, y);
                if matches.contains(&p) {
                    let color = graphics::WHITE;
                    push_quad_vertices(vertex_buffer, [dest_x, dest_y], tile_size, color)
                } else {
                    draw_jewel(vertex_buffer, [dest_x, dest_y], tile_size, jewel);
                }
            }
        }
    }
}

pub fn draw_board_fade_matches(
    vertex_buffer: &mut Vec<V2T2C4>,
    board: &Board,
    matches: &std::collections::HashSet<P2>,
    alpha: f32,
    target: Vector2,
    tile_size: Vector2,
    tile_padding: Vector2,
) {
    for x in 0..board.width() {
        for y in 0..board.height() {
            if let Some(jewel) = board[x][y] {
                let dest_x = (x as f32 * (tile_size[0] + tile_padding[0]) + tile_padding[0]) as f32
                    + target[0];
                let dest_y = (y as f32 * (tile_size[1] + tile_padding[1]) + tile_padding[1]) as f32
                    + target[1];
                let p = P2::new(x, y);
                if matches.contains(&p) {
                    let mut color = jewel.color_gl();
                    color[3] = alpha;
                    push_quad_vertices(vertex_buffer, [dest_x, dest_y], tile_size, color)
                } else {
                    draw_jewel(vertex_buffer, [dest_x, dest_y], tile_size, jewel);
                }
            }
        }
    }
}

pub fn draw_column(
    vertex_buffer: &mut Vec<V2T2C4>,
    column: ::columns::Column,
    target: Vector2,
    tile_size: Vector2,
    _tile_padding: Vector2,
    alpha: f32,
) {
    let mut p = column.position;
    for i in 0..3 {
        let x = target[0] + p.x as f32 * tile_size[0];
        let y = target[1] + p.y as f32 * tile_size[1];
        let mut color = column.jewels[i].color_gl();
        color[3] = alpha;
        let position = [x, y];
        push_quad_vertices(vertex_buffer, position, tile_size, color);
        p.up();
    }
}

fn draw_jewel(vertex_buffer: &mut Vec<V2T2C4>, target: Vector2, size: Vector2, jewel: Jewel) {
    let color = jewel.color_gl();
    push_quad_vertices(vertex_buffer, target, size, color)
}

pub fn get_scores_display_strings(
    score: u64,
    high_score: u64,
    window_rect: Rectangle,
    char_size: Vector2,
) -> Vec<(Vec<u8>, Vector2)> {
    vec![
        (
            format!("{:06}", high_score).into_bytes(),
            [
                window_rect.left() + char_size[0] * 13.,
                window_rect.top() - char_size[1] * 1.5,
            ],
        ),
        (
            format!("{:06}", score).into_bytes(),
            [
                window_rect.left() + char_size[0] * 3.,
                window_rect.top() - char_size[1] * 1.5,
            ],
        ),
    ]
}

pub fn draw_board_all_fading(
    vertex_buffer: &mut Vec<V2T2C4>,
    board: &Board,
    fading: &[(P2, f32)],
    target: Vector2,
    tile_size: Vector2,
    tile_padding: Vector2,
) {
    for x in 0..board.width() {
        for y in 0..board.height() {
            if let Some(jewel) = board[x][y] {
                let dest_x = (x as f32 * (tile_size[0] + tile_padding[0]) + tile_padding[0]) as f32
                    + target[0];
                let dest_y = (y as f32 * (tile_size[1] + tile_padding[1]) + tile_padding[1]) as f32
                    + target[1];
                let p = P2::new(x, y);
                let mut done = false;
                for fader in fading.iter() {
                    if p == fader.0 {
                        let color = jewel.color_gl();
                        let fade = {
                            if fader.1 < 0. {
                                0.
                            } else {
                                fader.1
                            }
                        };
                        let y = fade;
                        let x = 1. - fade;

                        let faded_color = [
                            0.8 * x + color[0] * y,
                            0.8 * x + color[1] * y,
                            0.8 * x + color[2] * y,
                            1.0,
                        ];
                        push_quad_vertices(vertex_buffer, [dest_x, dest_y], tile_size, faded_color);
                        done = true;
                        break;
                    }
                }

                if !done {
                    draw_jewel(vertex_buffer, [dest_x, dest_y], tile_size, jewel);
                }
            }
        }
    }
}

