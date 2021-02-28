use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

struct GameState {
    paddle_texture: Texture,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let paddle_texture = Texture::new(ctx, "./resources/player1.png")?;
        Ok(GameState { paddle_texture })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        self.paddle_texture.draw(ctx, Vec2::new(16., 16.));
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(false)
        .build()?
        .run(GameState::new)
}
