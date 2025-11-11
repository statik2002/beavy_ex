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
    let ferris = load_texture("tex2.png").await.unwrap();

    loop {
        clear_background(BLACK);
        set_camera(&Camera3D{
            position: vec3(-20., 15., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

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

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, WHITE);
        if DEBUG {
            let fps = get_fps();
            draw_text(&format!("FPS: {}", fps), 10.0,50.0, 30.0, WHITE);
        }

        next_frame().await
    }
}