extern crate opengl_graphics;
extern crate piston_window;
extern crate rand;

use piston_window::*;
use opengl_graphics::GlGraphics;
use rand::Rng;

struct Game {
    pub folks: Vec<Folk>,
    pub player: Player
}

impl Game {
    pub fn new() -> Game {
        Game {
            folks: vec![],
            player: Player::new()
        }
    }
    pub fn new_folk (&mut self, param_ltr:bool, param_speed:f64) {
        self.folks.push(Folk::new(param_ltr, param_speed));
    }
}

struct Player {
    x: f64,
    y: f64
}

impl Player {
    pub fn new() -> Player {
        Player {
            x: 0.0,
            y: 0.0
        }
    }
    pub fn update(&mut self, x:f64, y:f64) {
        self.x += x;
        self.y += y;
    }
}

struct Folk {
    pub x: f64,
    pub y: f64,
    pub moving: bool,
    ltr: bool,
    speed: f64
}

fn decide_x(ltr:bool) -> f64 {
    if ltr {
        0.0
    } else {
        1000.0
    }
}

impl Folk {
    pub fn new(param_ltr:bool, param_speed:f64) -> Folk {
        Folk {
            x: decide_x(param_ltr),
            y: 500.0,
            moving: true,
            ltr: param_ltr,
            speed: param_speed
        }
    }
    pub fn update(&mut self) {
        if self.moving {
            if self.ltr {
                self.x += 1.0 * self.speed;
                if self.x > 1000.0 {
                    self.moving = false;
                }
            } else {
                self.x -= 1.0 * self.speed;
                if self.x < 0.0 {
                    self.moving = false;
                }
            }
        }
    }
}


fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(
        "MOVE IT", [1000, 600])
        .opengl(opengl).samples(8).exit_on_esc(true).build().unwrap();

    window.set_ups(60);
    window.set_max_fps(60);

    let mut gl = GlGraphics::new(opengl);

    let mut last_folk = 0.0;
    let folk_interval = 3.0;
    let mut timer = 0.0;
    let mut rng = rand::thread_rng();

    let mut game = Game::new();

    // The game loop
    while let Some(e) = window.next() {
        // Event handling
        match e {

            Input::Press(Button::Keyboard(key)) => {
                match key {
                    Key::W => {
                        game.player.update(0.0, -5.0);
                    }
                    Key::S => {
                        game.player.update(0.0, 5.0);
                    }
                    Key::A => {
                        game.player.update(-5.0, 0.0);
                    }
                    Key::D => {
                        game.player.update(5.0, 0.0);
                    }
                    _ => {}
                }

            }

            Input::Update(args) => {

                timer += args.dt;
                last_folk += args.dt;

                if last_folk > folk_interval {
                    println!("SPAWN");
                    last_folk = 0.0;

                    let speed = rng.gen::<f64>() + 1.0;
                    let dir = rng.gen();

                    println!("{:?} {:?}", speed, dir);
                    game.new_folk(dir, speed);
                }

                for f in game.folks.iter_mut() {
                    f.update();
                    if f.moving == false {
                        //println!("remove item");
                    }
                }
                // game.folks.remove(game.folks.iter_mut().position(|&x| x.moving == false).unwrap());

            }

            Input::Render(args) => {

                window.draw_2d(&e, |c, g| {
                    clear([1.0; 4], g);
                    let folk_square = rectangle::square(0.0,0.0, 40.0);
                    rectangle([0.0, 0.0, 0.0, 1.0], folk_square, c.transform.trans(
                        game.player.x, game.player.y), g);

                    for f in game.folks.iter() {
                        let folk_square = rectangle::square(0.0,0.0, 40.0);
                        rectangle([0.0, 1.0, 0.0, 1.0], folk_square, c.transform.trans(
                            f.x, f.y), g);
                    }

                });
                //gl.draw(args.viewport(), |c, g| view::render_game(c, g, &mut resources, &state));
            }

            _ => {}
        }
    }
}
