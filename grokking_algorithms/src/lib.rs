use bbggez::{
    ggez::event::EventHandler,
    ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder},
    ggez::nalgebra::Point2,
    ggez::{graphics, timer, Context, GameResult},
    rand::prelude::*,
};

const LINES: u8 = 1;
const NODE_STROKE: f32 = 2.0;
const NODE_RADIUS: f32 = 5.0;

pub struct GameState {
    node_mesh: Option<Mesh>,
    rng: ThreadRng,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            node_mesh: None,
            rng: thread_rng(),
        }
    }

    fn init(&mut self, context: &mut Context) -> GameResult {
        let node_mesh = MeshBuilder::new()
            .circle(
                DrawMode::stroke(NODE_STROKE),
                Point2::new(0.0, 0.0),
                NODE_RADIUS,
                0.1,
                Color::new(0.0, 1.0, 0.0, 1.0),
            )
            .build(context)?;

        self.node_mesh = Some(node_mesh);
        Ok(())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        if timer::ticks(context) == 1 {
            self.init(context)?;
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::BLACK);

        graphics::present(context)
    }
}
