use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "MQ TEST!".to_string(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    next_frame().await;
    println!("{}", screen_width());

    let color = Color::from_rgba(255, 100, 150, 200);

    let mut tri = Triangle::new(
        Vec2 {
            x: (screen_width() / 2.) + 000.,
            y: 100.,
        },
        Vec2 {
            x: (screen_width() / 2.) + 400.,
            y: 300.,
        },
        Vec2 {
            x: (screen_width() / 2.) + 200.,
            y: 600.,
        },
        color,
    );
    let mut direction = 1.0;
    loop {
        if is_key_down(KeyCode::Q) && is_key_down(KeyCode::LeftControl) {
            break;
        }
        clear_background(LIGHTGRAY);

        if (is_key_down(KeyCode::Right) && direction < 0.0) || (is_key_down(KeyCode::Left) && direction > 0.0) {
            direction *= -1.0;
            tri.mirror_x();
        }

        if tri.b().x + tri.offset.0 > screen_width() || tri.b().x + tri.offset.0 < 0.0 {
            direction *= -1.0;
            tri.mirror_x();
        }

        let delta = get_frame_time();

        draw_line(0., 100., screen_width(), 100., 6., BLACK);
        draw_line(0., 300., screen_width(), 300., 6., BLACK);
        draw_line(0., 600., screen_width(), 600., 6., BLACK);

        tri.offset.0 += 250. * delta * direction;
        tri.draw();

        next_frame().await;
    }
}

struct Triangle {
    base: [Vec2; 3],
    offset: (f32, f32),
    color: Color,
}

impl Triangle {
    fn a(&self) -> &Vec2 {
        &self.base[0]
    }

    fn b(&self) -> &Vec2 {
        &self.base[1]
    }

    fn c(&self) -> &Vec2 {
        &self.base[2]
    }

    fn mirror_x(&mut self) {
        let minx = self.a().x.min(self.b().x.min(self.c().x));
        let maxx = self.a().x.max(self.b().x.max(self.c().x));

        let diff = maxx - minx;

        for v in self.base.iter_mut() {
            v.x = diff - v.x;
        }
    }

    fn new(a: Vec2, b: Vec2, c: Vec2, color: Color) -> Triangle {
        let mx = a.x.min(b.x).min(c.x);
        let my = a.y.min(b.y).min(c.y);

        let base = [
            Vec2 {
                x: a.x - mx,
                y: a.y - my,
            },
            Vec2 {
                x: b.x - mx,
                y: b.y - my,
            },
            Vec2 {
                x: c.x - mx,
                y: c.y - my,
            },
        ];
        let offset = (mx, my);

        Triangle { base, offset, color }
    }

    fn absolute(base: Vec2, offset: (f32, f32)) -> Vec2 {
        Vec2 {
            x: base.x + offset.0,
            y: base.y + offset.1,
        }
    }

    fn draw(&self) {
        let a = Self::absolute(self.base[0], self.offset);
        let b = Self::absolute(self.base[1], self.offset);
        let c = Self::absolute(self.base[2], self.offset);
        draw_triangle(a, b, c, self.color);
    }
}
