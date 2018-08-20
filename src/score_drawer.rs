use SCREENWIDTH;
// use SCREENHEIGHT;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use graphics::Context;
use assets::Sprite;
use assets::AssetManager;

pub struct ScoreDrawer<'a> {
	digit_sprites: [Sprite<'a>; 10],
}

impl<'a> ScoreDrawer<'a> {
	pub fn new(assets: &'a AssetManager) -> Self {
		let digit_sprites: [Sprite; 10] = [
			Sprite::new(assets.get_texture("score0")),
			Sprite::new(assets.get_texture("score1")),
			Sprite::new(assets.get_texture("score2")),
			Sprite::new(assets.get_texture("score3")),
			Sprite::new(assets.get_texture("score4")),
			Sprite::new(assets.get_texture("score5")),
			Sprite::new(assets.get_texture("score6")),
			Sprite::new(assets.get_texture("score7")),
			Sprite::new(assets.get_texture("score8")),
			Sprite::new(assets.get_texture("score9")),
		];

		ScoreDrawer {
			digit_sprites,
		}
	}


	pub fn draw(&self, score: i8, gl: &mut GlGraphics, context: &Context, _args: &RenderArgs) {
		let zero = &self.digit_sprites[0];
		zero.draw_at([100.0, 100.0], gl, context);
	}
}