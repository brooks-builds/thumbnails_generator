use bbggez::{
    ggez::event::EventHandler,
    ggez::graphics::{Color, DrawMode, DrawParam, Mesh, MeshBuilder, BLACK},
    ggez::nalgebra::Point2,
    ggez::{graphics, Context, GameResult},
    rand::prelude::*,
};

#[derive(Debug)]
pub struct GameState {
    node_mesh: Option<Mesh>,
    node_locations: [[Point2<f32>; 4]; 4],
    node_radius: f32,
    line_size: f32,
    line_color: Color,
}

impl GameState {
    pub fn new(screen_size: (f32, f32)) -> GameState {
        let standard_space_across = screen_size.0 / 4.0;
        let standard_space_down = screen_size.1 / 4.0;
        let mut rng = thread_rng();
        let mut node_locations = [[Point2::new(0.0, 0.0); 4]; 4];
        let node_radius = 5.0;
        let line_color = Color::new(0.0, 1.0, 0.0, 1.0);

        for (y_index, row) in node_locations.iter_mut().enumerate() {
            row[0].y =
                (y_index as f32 * standard_space_down) + rng.gen_range(0.0, standard_space_down);
            row[0].x = -node_radius - 1.0;
            row[1].y =
                (y_index as f32 * standard_space_down) + rng.gen_range(0.0, standard_space_down);
            row[1].x = rng.gen_range(0.0, standard_space_across);
            row[2].y =
                (y_index as f32 * standard_space_down) + rng.gen_range(0.0, standard_space_down);
            row[2].x = (2.0 * standard_space_across) + rng.gen_range(0.0, standard_space_across);
            row[3].y =
                (y_index as f32 * standard_space_down) + rng.gen_range(0.0, standard_space_down);
            row[3].x = screen_size.0 + node_radius + 1.0;
        }
        GameState {
            node_mesh: None,
            node_locations,
            node_radius,
            line_size: 1.0,
            line_color,
        }
    }

    fn init(&mut self, context: &mut Context) -> GameResult {
        let node_mesh = MeshBuilder::new()
            .circle(
                DrawMode::fill(),
                Point2::new(0.0, 0.0),
                self.node_radius,
                0.1,
                BLACK,
            )
            .circle(
                DrawMode::stroke(self.line_size),
                Point2::new(0.0, 0.0),
                self.node_radius,
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
        if let None = self.node_mesh {
            self.init(context)?;
        }

        let (screen_width, screen_height) = graphics::drawable_size(context);

        for row in self.node_locations.iter_mut() {
            for node_location in row.iter_mut() {
                node_location.x -= 1.0;
                if node_location.x < 0.0 {
                    node_location.x = screen_width + self.node_radius + 1.0;
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::BLACK);

        if let Some(mesh) = &self.node_mesh {
            for row in &self.node_locations {
                let lines = MeshBuilder::new()
                    .line(row, self.line_size, self.line_color)?
                    .build(context)?;
                graphics::draw(context, &lines, DrawParam::new())?;
                for node in row {
                    graphics::draw(context, mesh, DrawParam::new().dest(*node))?;
                }
            }
        }

        graphics::present(context)
    }
}
