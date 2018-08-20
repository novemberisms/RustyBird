use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use graphics;
use rand;
use rand::ThreadRng;
use rand::Rng;
use assets::{AssetManager, Sprite};
use Drawable;
use check_collision_rect;
use SCREENWIDTH;
use GROUND_Y;
//===========================================================================================

const PIPE_PERIOD: f64 = 3.0;

pub struct PipesManager<'a> {
	pipes: Vec<Pipe<'a>>,
	assets: &'a AssetManager,
	thread_rng: ThreadRng,
	new_pipe_timer: f64,
	signal_score_up: bool
}

impl<'a> PipesManager<'a> {
	pub fn spawn_pipe(&mut self) {
		self.pipes.push(Pipe::new(self.assets, &mut self.thread_rng));
	}

	pub fn new(assets: &'a AssetManager) -> Self {
		PipesManager {
			pipes: Vec::new(),
			assets: assets,
			thread_rng: rand::thread_rng(),
			new_pipe_timer: PIPE_PERIOD,
			signal_score_up: false,
		}
	}

	pub fn update(&mut self, dt: f64) {
		self.new_pipe_timer -= dt;
		if self.new_pipe_timer < 0.0 {
			self.new_pipe_timer = PIPE_PERIOD;
			self.spawn_pipe();
		}

		for pipe in &mut self.pipes {
			pipe.update(dt);
		}

		if self.pipes.len() > 0 {
			if self.pipes[0].to_be_deleted {
				self.pipes.remove(0);
				self.signal_score_up = true;
			}
		}
		

	}

	pub fn reset(&mut self) {
		self.pipes.clear();
		self.spawn_pipe();
		self.new_pipe_timer = PIPE_PERIOD;
	}

	pub fn detect_collision_with(&self, entity_rect: [f64; 4]) -> bool {
		for pipe in &self.pipes {
			if check_collision_rect(pipe.upper_rect, entity_rect) {
				return true
			}
			if check_collision_rect(pipe.lower_rect, entity_rect) {
				return true
			}
		}
		false
	}

	pub fn check_score_up(&mut self) -> bool { 
		if self.signal_score_up {
			self.signal_score_up = false;
			true
		} else {
			false
		}	
	}
}

impl<'a> Drawable for PipesManager<'a> {
	fn draw(&self, gl: &mut GlGraphics, context: &graphics::Context, args: &RenderArgs) {
		for pipe in &self.pipes {
			pipe.draw(gl, context, args);
		}
	}
}


//===========================================================================================
const PIPE_WIDTH: f64 = 52.0;
const Y_INCREMENT: f64 = 52.0;
const GAP_SIZE: f64 = 100.0;
const START_Y: f64 = 32.0;
const START_X: f64 = SCREENWIDTH as f64;
const SCROLL_SPEED: f64 = 150.0;

struct Pipe<'a> {
	upper_sprite: Sprite<'a>,
	lower_sprite: Sprite<'a>,
	upper_rect: [f64; 4],
	lower_rect: [f64; 4],
	pub to_be_deleted: bool,
} 

impl<'a> Pipe<'a> {
	pub fn new(assets: &'a AssetManager, thread_rng: &mut rand::ThreadRng) -> Self {
		let mut upper_sprite: Sprite = Sprite::new(assets.get_texture("pipe"));
		let mut lower_sprite: Sprite = Sprite::new(assets.get_texture("pipe"));

		let upper_height: f64 = START_Y + thread_rng.gen_range(0, 5) as f64 * Y_INCREMENT;
		let lower_height: f64 = GROUND_Y - upper_height - GAP_SIZE; 

		let lower_y = GROUND_Y - lower_height;

		upper_sprite.set_quad([0.0, 0.0, PIPE_WIDTH, upper_height]);
		lower_sprite.set_quad([0.0, 0.0, PIPE_WIDTH, lower_height]);

		upper_sprite.scale([1.0, -1.0]);

		Pipe {
			upper_sprite,
			lower_sprite,
			upper_rect: [SCREENWIDTH as f64, 0.0, 	  PIPE_WIDTH, upper_height],
			lower_rect: [SCREENWIDTH as f64, lower_y, PIPE_WIDTH, lower_height],
			to_be_deleted: false,
		}
	}

	pub fn update(&mut self, dt: f64) {
		self.upper_rect[0] -= SCROLL_SPEED * dt;
		self.lower_rect[0] -= SCROLL_SPEED * dt;

		self.lower_sprite.set_position([self.lower_rect[0], self.lower_rect[1]]);
		self.upper_sprite.set_position([self.upper_rect[0], self.upper_rect[3]]);

		if self.upper_rect[0] + self.upper_rect[2] < 0.0 {
			self.to_be_deleted = true;
		}
	}
}

impl<'a> Drawable for Pipe<'a> {
	fn draw(&self, gl: &mut GlGraphics, context: &graphics::Context, args: &RenderArgs) {
		self.lower_sprite.draw(gl, context);
		self.upper_sprite.draw(gl, context);
	}
}
