

use macroquad::prelude::*;

const DEBUG: bool = true;

struct CircleBrick {
    x: f32,
    y: f32,
    radius: f32,
    color: Color
}

fn window_conf() -> Conf {
    Conf {
        window_title: "First steps".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

fn init() {
    
}

#[macroquad::main(window_conf)]
async fn main() {
    //block count
    const BLOCKS_W: i32 = 10;
    const BLOCKS_H: i32 = 5;
    const SCR_W: f32 = 20.0;
    const SCR_H: f32 = 20.0;

    let mut platform_x = 10.;
    let platform_width = 5.;
    let platform_height = 0.2;

    let mut ball_x = 0.;
    let mut ball_y = 0.;
    //let ferris = load_texture("tex2.png").await.unwrap();

    loop {
        clear_background(BLACK);

        let delta = get_frame_time();

        if is_key_down(KeyCode::Right) && platform_x < 17.5 - platform_width {
            platform_x += 3.0 * delta;
        }

        if is_key_down(KeyCode::Left) && platform_x > -2.5 {
            platform_x -= 3.0 * delta;
        }

        if is_key_down(KeyCode::Escape) {
            break;
        }

        
        set_camera(&Camera3D{
            position: vec3(-20., 15., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        /* 

        //draw_grid(20, 1., BLACK, GRAY);
        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);
        draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), DARKBLUE);
        draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), YELLOW);

        //draw_plane(vec3(-8., 0., -8.), vec2(5., 5.), Some(&ferris), WHITE);

        draw_cube(
            vec3(-5., 1., -2.),
            vec3(2., 2., 2.),
            Some(&ferris),
            WHITE,
        );
        draw_cube(vec3(2., 0., -2.), vec3(1.4, 1.4, 1.4), None, GREEN);

        draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);
        draw_sphere_wires(vec3(-4., 0., 0.), 1., None, WHITE);

        */

        draw_rectangle(
            platform_x - platform_width / 2.,
            SCR_H - platform_height,
            platform_width,
            platform_height,
            DARKPURPLE,
        );
        //draw blocks
        for j in 0..BLOCKS_H{
            for i in 0..BLOCKS_W {
                let block_width = 20.0 / BLOCKS_W as f32;
                let block_height = 10. / BLOCKS_H as f32;
                let block_x = j as f32 * block_width + 1.5;
                let block_y = i as f32 * block_height - 9.0;
                draw_cube_wires(vec3(block_x, 0., block_y), vec3(block_height, 0.5, block_width), WHITE);
            }
        }
        //draw_cube_wires(vec3(10., 0., -10.), vec3(1., 1., 1.), WHITE);

        //draw pad
        draw_cube_wires(
            vec3(-9.5, 0., platform_x - platform_width), 
            vec3(platform_height, 0.5, platform_width), 
            WHITE
        );

        // draw walls
        draw_cube_wires(vec3(0.5, 0., -10.5), vec3(21., 1., 0.5), WHITE);
        draw_cube_wires(vec3(0.5, 0., 10.5), vec3(21., 1., 0.5), WHITE);
        draw_cube_wires(vec3(10.5, 0., 0.), vec3(0.5, 1., 20.5), WHITE);

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, WHITE);
        if DEBUG {
            let fps = get_fps();
            draw_text(&format!("FPS: {}", fps), 10.0,50.0, 30.0, WHITE);
            draw_text(&format!("Platform X: {}", platform_x), 10.0,70.0, 30.0, WHITE);
        }

        draw_sphere_wires(vec3(0., 1., 0.), 10.0, None, WHITE);

        next_frame().await
    }
}