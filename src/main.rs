use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
const WALL_THICKNESS: f32 = 30.;
const MAIN_BALL_RADIUS: f32 = 50.;
const MAIN_BALL_MASS: f32 = 2.;
const MAIN_BALL_GRAVITY_MULT: f32 = 1.5;
const BALL_RADIUS: f32 = 20.;
const GRAVITY: f32 = 70.;

pub enum Current {
    None,
    Left,
    Right,
}
#[derive(Debug, Clone, Copy, Default)]
pub struct Ball {
    pub center: Vec2,
    pub radius: f32,
    pub color: Color,
    pub velocity: Vec2,
    pub mass: f32,
    pub in_bound: bool,
}
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub pos: Vec2,
    pub size: Vec2,
    pub color: Color,
}
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct SaveDate {
    score: u32,
}
fn window_conf() -> Conf {
    Conf {
        window_title: "Wall 2 Wall".to_owned(),
        fullscreen: false,
        window_height: 600,
        window_width: 800,
        window_resizable: false,
        ..Default::default()
    }
}
// to build wasm
// cargo build --target wasm32-unknown-unknown
// cargo install basic-http-server
// basic-http-server .
#[macroquad::main(window_conf)]
async fn main() {
    let mut score: u32 = 0;
    let mut best_score = load();
    let mut balls: Vec<Ball> = vec![];
    let mut right_rect: Rect = Rect {
        pos: vec2(screen_width() - WALL_THICKNESS, 0.),
        size: vec2(WALL_THICKNESS, screen_height()),
        color: GREEN,
    };
    let mut left_rect: Rect = Rect {
        pos: vec2(0., 0.),
        size: vec2(WALL_THICKNESS, screen_height()),
        color: GREEN,
    };
    let ceiling: Rect = Rect {
        pos: vec2(0., 0.),
        size: vec2(screen_width(), WALL_THICKNESS),
        color: Color::new(0.65, 0.8, 1., 1.),
    };
    let mut current_wall: Current = Current::None;
    let canon_pos = vec2(screen_width() / 2., screen_height());
    println!("{:?}", screen_width());
    let mut main_ball: Ball = spawn_main_ball();
    let mut show_message = true;
    let mut is_paused = false;
    let mut angle = canon_angle(canon_pos);

    loop {
        // check pause
        if is_mouse_button_pressed(MouseButton::Left) {
            let has_changed = handle_pause();
            is_paused = is_paused != has_changed;
            // We skip frame so that the canon won't shot when unpausing.
            if has_changed {
                draw(
                    score,
                    best_score,
                    main_ball,
                    &balls,
                    left_rect,
                    right_rect,
                    ceiling,
                    is_paused,
                    show_message,
                    canon_pos,
                    angle,
                );
                next_frame().await;
                continue;
            }
        }
        if is_paused {
            draw(
                score,
                best_score,
                main_ball,
                &balls,
                left_rect,
                right_rect,
                ceiling,
                is_paused,
                show_message,
                canon_pos,
                angle,
            );
            next_frame().await;
            continue;
        }
        angle = canon_angle(canon_pos);
        if is_mouse_button_pressed(MouseButton::Left) {
            let new_ball = Ball {
                center: canon_pos,
                radius: BALL_RADIUS,
                color: WHITE,
                velocity: vec2(-angle.sin(), angle.cos()) * 700.,
                mass: 0.7,
                in_bound: true,
            };
            balls.push(new_ball);
        }
        let mut gravity = GRAVITY;
        if balls.len() > 4 {
            gravity = (MAIN_BALL_GRAVITY_MULT).powi((balls.len() - 4) as i32) * GRAVITY;
        }
        main_ball.move_kinematic(gravity);

        // ceil
        if main_ball.center.y < main_ball.radius + WALL_THICKNESS {
            main_ball.velocity.y *= -1.;
            main_ball.center.y = main_ball.radius + WALL_THICKNESS;
        }

        // right
        if main_ball.center.x > screen_width() - main_ball.radius - WALL_THICKNESS {
            match current_wall {
                Current::None | Current::Right => {
                    show_message = false;
                    current_wall = Current::Left;
                    increment_score(&mut score, &mut best_score);
                    left_rect.color = GREEN;
                    right_rect.color = RED;
                }
                Current::Left => {}
            }
            main_ball.velocity.x *= -1.;
            main_ball.center.x = screen_width() - main_ball.radius - WALL_THICKNESS;
        }

        //left
        if main_ball.center.x < main_ball.radius + WALL_THICKNESS {
            match current_wall {
                Current::None | Current::Left => {
                    show_message = false;
                    current_wall = Current::Right;
                    right_rect.color = GREEN;
                    left_rect.color = RED;
                    increment_score(&mut score, &mut best_score);
                }
                Current::Right => {}
            }
            main_ball.velocity.x *= -1.;
            main_ball.center.x = main_ball.radius + WALL_THICKNESS;
        }

        // lose
        if main_ball.center.y > screen_height() + MAIN_BALL_RADIUS {
            main_ball = spawn_main_ball();
            current_wall = Current::None;
            left_rect.color = GREEN;
            right_rect.color = GREEN;
            score = 0;
        }

        for ball in &mut balls {
            ball.move_kinematic(GRAVITY);
            ball.bounce_walls();
            ball.bounce_balls(&mut main_ball);

            if ball.center.y > screen_height() + BALL_RADIUS {
                ball.in_bound = false;
            }
        }
        balls.retain(|x| x.in_bound);
        for i in 1..balls.len() {
            let (ball, others) = balls.split_at_mut(i);
            let ball = ball
                .last_mut()
                .expect("Failed to find the current object. Make sure that the index start at 1.");
            for other in others {
                ball.bounce_balls(other);
            }
        }
        draw(
            score,
            best_score,
            main_ball,
            &balls,
            left_rect,
            right_rect,
            ceiling,
            is_paused,
            show_message,
            canon_pos,
            angle,
        );
        next_frame().await
    }
}
fn handle_pause() -> bool {
    let pos = mouse_position();
    if 50. < pos.0 && pos.0 < 100. && 50. < pos.1 && pos.1 < 100. {
        return true;
    }
    return false;
}
fn draw_pause(is_paused: bool) {
    const PAUSE_BTN_COLOR: Color = Color::new(0., 0., 0., 0.4);
    // draw pause
    draw_rectangle(50., 50., 50., 50., Color::from_rgba(0, 0, 0, 50));
    if is_paused {
        draw_triangle(
            vec2(60., 55.),
            vec2(60., 95.),
            vec2(90., 75.),
            PAUSE_BTN_COLOR,
        );
        draw_rectangle(250., 208., 335., 128., Color::new(0., 0., 0., 0.4));
        draw_text("Paused", 250., 300., 128., Color::new(1., 0.6, 0.6, 1.));
        return;
    }
    draw_rectangle(60., 55., 10., 40., PAUSE_BTN_COLOR);
    draw_rectangle(80., 55., 10., 40., PAUSE_BTN_COLOR);
    return;
}
fn draw(
    score: u32,
    best_score: u32,
    main_ball: Ball,
    balls: &Vec<Ball>,
    left_rect: Rect,
    right_rect: Rect,
    ceiling: Rect,
    is_paused: bool,
    show_message: bool,
    canon_pos: Vec2,
    angle: f32,
) {
    clear_background(Color {
        r: 0.9,
        g: 0.9,
        b: 0.9,
        a: 1.,
    });
    if show_message {
        draw_text(
            "Have the red ball hit the green wall to earn points.",
            100.,
            250.,
            28.,
            Color::new(0., 0., 0., 0.4),
        );
    }
    draw_text(&score.to_string(), 200., 200., 64., BLACK);
    let text = "Best score: ".to_owned() + &best_score.to_string();
    draw_text(&text, 200., 300., 64., BLACK);
    left_rect.draw();
    right_rect.draw();
    ceiling.draw();

    main_ball.draw();

    for ball in balls {
        ball.draw();
    }

    draw_rectangle_ex(
        canon_pos.x,
        canon_pos.y,
        25.,
        128.,
        DrawRectangleParams {
            offset: vec2(0.5, 0.),
            color: WHITE,
            rotation: angle,
            ..Default::default()
        },
    );
    draw_pause(is_paused);
}
fn increment_score(score: &mut u32, best_score: &mut u32) {
    *score += 1;
    if score > best_score {
        save(*score);
        *best_score = *score;
    }
}
fn load() -> u32 {
    #[cfg(target_family = "wasm")]
    {
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        let Some(data) = storage.get("save") else {
            return 0;
        };

        let data: SaveDate =
            serde_json::from_str(data.as_str()).expect("Failed to deserialize save file.");

        return data.score;
    }
    #[cfg(not(target_family = "wasm"))]
    {
        let data = dirs::data_local_dir()
            .expect("Local data dir not found.")
            .join("Wall 2 Wall")
            .join("save.json");
        let data = std::fs::read(data);
        match data {
            Ok(data) => {
                let data: SaveDate =
                    serde_json::from_slice(&data).expect("Failed to deserialize save.");
                return data.score;
            }
            Err(_) => return 0,
        }
    }
    0
}
fn save(score: u32) {
    #[cfg(target_family = "wasm")]
    {
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        let data = SaveDate { score: score };
        storage.set(
            "save",
            serde_json::to_string(&data)
                .expect("Failed to serialize save.")
                .as_str(),
        );
    }
    #[cfg(not(target_family = "wasm"))]
    {
        let data = SaveDate { score: score };
        let path = dirs::data_local_dir()
            .expect("Local data dir not found.")
            .join("Wall 2 Wall");
        std::fs::create_dir_all(&path).expect("Failed to create dir.");
        let path = path.join("save.json");
        std::fs::write(
            path,
            serde_json::to_vec(&data).expect("Failed to serialize save file."),
        )
        .expect("Failed to save.");
    }
}
fn spawn_main_ball() -> Ball {
    Ball {
        center: vec2(screen_width() / 2., screen_height() / 2.),
        radius: MAIN_BALL_RADIUS,
        color: RED,
        velocity: vec2(0., -3.),
        mass: MAIN_BALL_MASS,
        in_bound: true,
    }
}
fn canon_angle(position: Vec2) -> f32 {
    let (x, y) = mouse_position();
    -vec2(x - position.x, y - position.y).angle_between(vec2(0., 1.))
}
impl Rect {
    fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.size.x, self.size.y, self.color);
    }
}
impl Ball {
    fn draw(&self) {
        draw_circle(self.center.x, self.center.y, self.radius, self.color);
    }
    fn move_kinematic(&mut self, gravity: f32) {
        let Vec2 { x, y } = self.velocity;
        let y = y + gravity * get_frame_time() / 2.;

        self.velocity = vec2(x, y);
        self.center += self.velocity * get_frame_time();
        let y = y + gravity * get_frame_time() / 2.;
        self.velocity = vec2(x, y);
    }
    fn bounce_walls(&mut self) {
        if self.center.y < self.radius + WALL_THICKNESS {
            self.velocity.y *= -1.;
            self.center.y = self.radius + WALL_THICKNESS;
        }

        if self.center.x > screen_width() - self.radius - WALL_THICKNESS {
            self.velocity.x *= -1.;
            self.center.x = screen_width() - self.radius - WALL_THICKNESS;
        }

        if self.center.x < self.radius + WALL_THICKNESS {
            self.velocity.x *= -1.;
            self.center.x = self.radius + WALL_THICKNESS;
        }
    }
    fn bounce_balls(&mut self, other: &mut Ball) {
        let radius_sum = self.radius + other.radius;
        if self.center.distance_squared(other.center) >= (radius_sum) * (radius_sum) {
            return;
        }
        let distance = self.center.distance(other.center);
        let strength = distance - radius_sum;

        let dir = self.center - other.center;
        let dir = dir.normalize();

        // self.center -= dir * strength / 2.;
        // other.center += dir * strength / 2.;

        let total_mass = self.mass + other.mass;

        self.velocity -= strength * dir * (total_mass / self.mass) * 2.;
        other.velocity += strength * dir * (total_mass / other.mass) * 2.;
    }
}
