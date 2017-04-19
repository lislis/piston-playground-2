extern crate opengl_graphics;
extern crate piston_window;
extern crate rand;

use piston_window::*;
use opengl_graphics::GlGraphics;

struct Game {
    pub folks: Vec<Folk>
}

impl Game {
    pub fn new() -> Game {
        Game {
            folks: vec![]
        }
    }
    pub fn new_folk (&mut self) {
        self.folks.push(Folk::new());
    }
}

struct Folk {
    pub x: f64,
    pub y: f64,
    moving: bool,
    ltr: bool,
    speed: f64
}

impl Folk {
    pub fn new() -> Folk {
        Folk {
            x: 0.0,
            y: 500.0,
            moving: true,
            ltr: true,
            speed: 1.0
        }
    }
    pub fn update(&mut self) {
        if self.moving {
            if self.ltr {
                self.x += 1.0;
            } else {
                self.x -= 1.0;
            }
        }
    }
}


fn main() {


    let opengl = OpenGL::V3_2;

    //let game_size = Size::new(1024.0, 600.0);

    let mut window: PistonWindow = WindowSettings::new(
        "Rocket!", [1000, 600])
        .opengl(opengl).samples(8).exit_on_esc(true).build().unwrap();

    window.set_ups(60);
    window.set_max_fps(60);

    let mut gl = GlGraphics::new(opengl);

    let mut last_folk = 0.0;
    let folk_interval = 2.0;
    let mut timer = 0.0;

    let mut game = Game::new();

    // The game loop
    while let Some(e) = window.next() {
        // Event handling
        match e {
            Input::Update(args) => {
                //time_controller.update_seconds(args.dt, input_controller.actions(), &mut state);
                //eCollisionsController::handle_collisions(&mut state);
                timer += args.dt;
                last_folk += args.dt;
                println!("{:?}", timer);

                if last_folk > folk_interval {
                    println!("SPAWN");
                    last_folk = 0.0;
                    game.new_folk();
                }

                for f in game.folks.iter_mut() {
                    f.update();
                }
            }

            Input::Render(args) => {

                window.draw_2d(&e, |c, g| {
                    clear([1.0; 4], g);
                    let folk_square = rectangle::square(0.0,0.0, 40.0);
                    rectangle([0.0, 0.0, 0.0, 1.0], folk_square, c.transform.trans(
                        200.0, 200.0), g);

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
