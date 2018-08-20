#![allow(dead_code)]
#![allow(unused_variables)]

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

use piston::window;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};
use piston::event_loop::{Events, EventSettings};

use piston::input::RenderEvent;
use piston::input::UpdateEvent;
use piston::input::ButtonEvent;
use piston::input::UpdateArgs;
use piston::input::RenderArgs;
use piston::input::ButtonArgs;

type Color = [f32; 4];

mod assets;
use assets::AssetManager;
use assets::Sprite;

mod bird;
use bird::Bird;

mod pipe;
use pipe::PipesManager;

mod score_drawer;
use score_drawer::ScoreDrawer;

const BACKGROUND: Color = [1.0; 4];

const SCREENHEIGHT: u32 = 512;
const SCREENWIDTH: u32 = 288;
const GROUND_Y: f64 = 512.0 - 112.0;
//===========================================================================================

fn main() {
	let opengl = OpenGL::V3_2;

    let windowsettings = window::WindowSettings::new("Rusty Bird", [SCREENWIDTH, SCREENHEIGHT])
    	.opengl(opengl)
    	.srgb(false)
    	.resizable(false)
    	.exit_on_esc(true);

    let mut glutinwindow: GlutinWindow = windowsettings.build()
    	.expect("Could not create window!");

    let mut events = Events::new(EventSettings::new());

    // the gl object stores shaders and buffers that the OpenGL
    // backend for Piston-Graphics needs to talk to the GPU
    let mut gl = GlGraphics::new(opengl);

    let mut assets = AssetManager::new();
    assets.add_texture("flappy", "assets/yellowbird-upflap.png");
    assets.add_texture("background", "assets/background-day.png");
    assets.add_texture("ground", "assets/base.png");
    assets.add_texture("title", "assets/message.png");
    assets.add_texture("pipe", "assets/pipe-green.png");
    assets.add_texture("gameover", "assets/gameover.png");
    assets.add_texture("score0", "assets/0.png");
    assets.add_texture("score1", "assets/1.png");
    assets.add_texture("score2", "assets/2.png");
    assets.add_texture("score3", "assets/3.png");
    assets.add_texture("score4", "assets/4.png");
    assets.add_texture("score5", "assets/5.png");
    assets.add_texture("score6", "assets/6.png");
    assets.add_texture("score7", "assets/7.png");
    assets.add_texture("score8", "assets/8.png");
    assets.add_texture("score9", "assets/9.png");

    let mut game = Game::new(&assets, rand::thread_rng());

    game.load();

    while let Some(e) = events.next(&mut glutinwindow) {
    	if let Some(args) = e.render_args() {
    		let context = gl.draw_begin(args.viewport());
    		graphics::clear(BACKGROUND, &mut gl);
    		game.draw(&mut gl, &context, &args);
    		gl.draw_end();
    	}
    	if let Some(args) = e.update_args() {
    		game.update(&args);
    	}
    	if let Some(args) = e.button_args() {
    		game.press(&args);
    	}
    }

    println!("Game exited succesfully");
}
//===========================================================================================

trait Drawable {
	fn draw(&self, gl: &mut GlGraphics, context: &graphics::Context, args: &RenderArgs);
}
//===========================================================================================

const PLAYER_X: f64 = 20.0;
const PLAYER_Y: f64 = 100.0;

enum GameState {
	Paused,
	Playing,
	Death,
}
//===========================================================================================

struct Game<'a> {
	state: GameState,
	player: Bird<'a>,
	pipes: PipesManager<'a>,
	thread_rng: rand::ThreadRng,
	assets: &'a AssetManager,
	score_drawer: ScoreDrawer<'a>,

	background_sprite: 	Sprite<'a>,
	ground_sprite: 		Sprite<'a>,
	title_sprite: 		Sprite<'a>,
	gameover_sprite: 	Sprite<'a>,
}
//===========================================================================================

impl<'a> Game<'a> {

	fn new(assets: &'a AssetManager, thread_rng: rand::ThreadRng) -> Self {
		let player_width = 34.0;
		let player_height = 24.0;
		Game {
			state: GameState::Paused,
			player: Bird::new(PLAYER_X, PLAYER_Y, player_width, player_height, assets),
			pipes: PipesManager::new(assets),
			thread_rng: thread_rng,
			assets: assets,
			score_drawer: ScoreDrawer::new(assets),

			background_sprite: Sprite::new(assets.get_texture("background")),
			ground_sprite: Sprite::new(assets.get_texture("ground")),
			title_sprite: Sprite::new(assets.get_texture("title")),
			gameover_sprite: Sprite::new(assets.get_texture("gameover")),
		}
	}

	fn load(&mut self) {
		self.ground_sprite.set_position([0.0, GROUND_Y]);

		let title_size = self.title_sprite.get_size();
		self.title_sprite.set_position([
			(SCREENWIDTH as f64 - title_size[0]) / 2.0,
			(SCREENHEIGHT as f64 - title_size[1]) / 2.0
		]);

		let gameover_size = self.gameover_sprite.get_size();
		self.gameover_sprite.set_position([
			(SCREENWIDTH as f64 - gameover_size[0]) / 2.0,
			(SCREENHEIGHT as f64 - gameover_size[1]) / 2.0
		]);

		self.pipes.reset();
	}

	fn update(&mut self, args: &UpdateArgs) {

		match self.state {
			GameState::Paused => {}
			GameState::Death => {}
			GameState::Playing => {
				self.player.update(args.dt);
				self.pipes.update(args.dt);
				if self.pipes.detect_collision_with(self.player.get_rect()) {
					self.game_over();
				}
				if self.player.signal_gameover {
					self.game_over();
				}
				if self.pipes.check_score_up() {
					self.player.score += 1;
					println!("score: {}", self.player.score);
				}
			}
		}
	}

	fn draw(&self, gl: &mut GlGraphics, context: &graphics::Context, args: &RenderArgs) {
		self.background_sprite.draw(gl, context);
		if let GameState::Playing = self.state {
			self.pipes.draw(gl, context, args);
		}
		if let GameState::Death = self.state {
			self.pipes.draw(gl, context, args);
		}
		self.ground_sprite.draw(gl, context);
		self.player.draw(gl, context, args);
		self.score_drawer.draw(self.player.score, gl, context, args);
		if let GameState::Paused = self.state {
			self.title_sprite.draw(gl, context);
		}
		if let GameState::Death = self.state {
			self.gameover_sprite.draw(gl, context);
		}

		
	}

	fn press(&mut self, args: &ButtonArgs) {
		use piston::input::ButtonState;
		if let ButtonState::Press = args.state {
			match self.state {
				GameState::Paused => {
					self.set_state(GameState::Playing);
					self.player.jump();
				},
				GameState::Playing => {
					self.player.jump()
				},
				GameState::Death => {
					self.pipes.reset();
					self.player.reset();
					self.set_state(GameState::Paused);
				}
			}
		}
	}

	fn set_state(&mut self, new_state: GameState) {
		self.state = new_state;
	}

	fn game_over(&mut self) {
		self.player.signal_gameover = false;
		self.set_state(GameState::Death);
	}
}

//===========================================================================================

fn check_collision_rect(rect_a: [f64; 4], rect_b: [f64; 4]) -> bool {
	let b_br_x = rect_b[0] + rect_b[2];
	if rect_a[0] > b_br_x {return false}

	let b_br_y = rect_b[1] + rect_b[3];
	if rect_a[1] > b_br_y {return false}

	let a_br_x = rect_a[0] + rect_a[2];
	if rect_b[0] > a_br_x {return false}

	let a_br_y = rect_a[1] + rect_a[3];
	if rect_b[1] > a_br_y {return false}

	true
}