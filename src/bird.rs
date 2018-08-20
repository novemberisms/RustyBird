use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use graphics;
use Drawable;

use assets::AssetManager;
use assets::Sprite;

use GROUND_Y;
use PLAYER_X;
use PLAYER_Y;
//===========================================================================================

pub struct Bird<'a> {
	position: [f64; 2],
	size: [f64; 2],
	y_velocity: f64,
	gravity: f64,
	jump_speed: f64,

	pub score: i8,

	sprite: Sprite<'a>,
	pub signal_gameover: bool,
} 
//===========================================================================================

impl<'a> Bird<'a> {

	pub fn new(x: f64, y: f64, width: f64, height: f64, assets: &'a AssetManager) -> Self {
		let flappy_texture = assets.get_texture("flappy");
		let sprite = Sprite::new_pos(flappy_texture, [x, y]);
		Bird {
			position: [x, y],
			size: [width, height],
			y_velocity: 0.0,
			gravity: 1250.0,
			jump_speed: 375.0,
			sprite: sprite,
			score: 0,
			signal_gameover: false,
		}
	}


	pub fn update(&mut self, dt: f64) {
		self.y_velocity += self.gravity * dt;
		self.position[1] += self.y_velocity * dt;

		if self.position[1] + self.size[1] > GROUND_Y as f64 {
			self.y_velocity = 0.0;
			self.position[1] = GROUND_Y as f64 - self.size[1];
			self.signal_gameover = true;
		}

		if self.position[1] < 0.0 {
			self.position[1] = 0.0;
		}

		self.sprite.set_position(self.position);
	}

	pub fn jump(&mut self) {
		self.y_velocity = -self.jump_speed;
	}

	pub fn reset(&mut self) {
		self.position = [PLAYER_X, PLAYER_Y];
		self.score = 0;
		self.sprite.set_position(self.position);
	}

	pub fn get_rect(&self) -> [f64; 4] {
		[
			self.position[0],
			self.position[1],
			self.size[0],
			self.size[1]
		]
	}
}

impl<'a> Drawable for Bird<'a> {
	fn draw(&self, gl: &mut GlGraphics, context: &graphics::Context, _args: &RenderArgs) {
		self.sprite.draw(gl, context)
	}
}
