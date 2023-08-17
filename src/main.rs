use macroquad::{miniquad::window::set_window_size, prelude::*, window};

// position size.
#[derive(Debug, Clone, Copy, Default)]
pub struct Transform {
    pub pos: Vec2,
    pub size: Vec2,
}
#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Square,
    Circle,
}
pub struct World {
    pub objects: Vec<Object>,
}
pub struct Object {
    pub color: Color,
    pub velocity: Option<Vec2>,
    pub shape: Shape,
    pub transform: Transform,
}

#[macroquad::main("Wall2Wall")]
async fn main() {
    set_window_size(800, 600);
    let mut world = World {
        objects: vec![
            // Floor
            Object {
                color: BROWN,
                shape: Shape::Square,
                transform: Transform {
                    pos: vec2(50., 300.),
                    size: vec2(700., 60.),
                },
                velocity: None,
            },
            // Square
            Object {
                color: BLACK,
                shape: Shape::Square,
                transform: Transform {
                    pos: vec2(375., 100.),
                    size: vec2(50., 50.),
                },
                velocity: Some(vec2(1., -3.)),
            },
        ],
    };
    loop {
        clear_background(Color {
            r: 0.9,
            g: 0.9,
            b: 0.9,
            a: 1.,
        });
        // floor
        world.render();
        world.simulate();
        next_frame().await
    }
}
pub fn aabb_check(obj1: Transform, obj2: Transform) -> bool {
    let x1 = obj1.pos.x > obj2.pos.x && obj1.pos.x < obj2.pos.x + obj2.size.x;
    let x2 = obj2.pos.x > obj1.pos.x && obj2.pos.x < obj1.pos.x + obj1.size.x;

    let y1 = obj1.pos.y > obj2.pos.y && obj1.pos.y < obj2.pos.y + obj2.size.y;
    let y2 = obj2.pos.y > obj1.pos.y && obj2.pos.y < obj1.pos.y + obj1.size.y;

    return (x1 || x2) && (y1 || y2);
}
impl World {
    pub fn render(&self) {
        for obj in &self.objects {
            match &obj.shape {
                crate::Shape::Square => {
                    let Transform { pos, size } = obj.transform;
                    draw_rectangle(pos.x, pos.y, size.x, size.y, obj.color);
                }
                crate::Shape::Circle => todo!(),
            }
        }
    }
    pub fn simulate(&mut self) {
        const GRAVITY: f32 = 10.;

        for obj in &mut self.objects {
            let Some(mut vel) = obj.velocity else{
                continue;
            };

            vel.y += GRAVITY * get_frame_time();
            obj.transform.pos += vel;
            obj.velocity = Some(vel);
        }
        let mut i = 1;
        while i < self.objects.len() {
            let (obj, others) = self.objects.split_at_mut(i);
            let obj = obj
                .last_mut()
                .expect("Failed to find the current object. Make sure that the index start at 1.");

            for other in others {
                handle_collision(obj, other);
            }
            i += 1;
        }
    }
}

fn handle_collision(obj: &mut Object, other: &mut Object) {
    if !aabb_check(obj.transform, other.transform) {
        return;
    }
    match (obj.shape, other.shape) {
        (Shape::Square, Shape::Square) => {
            dbg!("Walls do not need to calculate collision with themselves.");
        }
        (Shape::Square, Shape::Circle) => handle_square_circle_collision(obj, other),
        (Shape::Circle, Shape::Square) => handle_square_circle_collision(other, obj),
        (Shape::Circle, Shape::Circle) => handle_two_circle_collision(obj, other),
    }
}
fn handle_square_circle_collision(square: &mut Object, circle: &mut Object) {}
fn handle_two_circle_collision(circle1: &mut Object, circle2: &mut Object) {}
