use graphics::Context;
use graphics::Image;
use graphics::ImageSize;
use opengl_graphics::{Texture, TextureSettings, Filter, GlGraphics};
use std::collections::HashMap;
use std::path::Path;

pub struct AssetManager {
	textures: HashMap<&'static str, Texture>,
}

impl AssetManager {

	pub fn new() -> Self {
		AssetManager {
			textures: HashMap::new()
		}
	}

	pub fn add_texture(&mut self, name: &'static str, path: &str) {
		let new_texture = Texture::from_path(
			Path::new(path),
			&TextureSettings::new().filter(Filter::Linear)
		).expect(format!("Could not find path for texture: {}", path).as_str());

		self.textures.insert(name, new_texture);
	}

	pub fn get_texture(&self, name: &str) -> &Texture {
		self.textures.get(name)
			.expect(format!("No texture named {}", name).as_str())
	}
}

type Vector2 = [f64; 2];
type Rect = [f64; 4];

//=================================================================================================

/// a sprite is a combination of an image rectangle, and a reference to a texture.
pub struct Sprite<'a> {
	position: Vector2,
	size: Vector2,
	quad: Rect,
	image: Image,
	texture: &'a Texture,
}

impl<'a> Sprite<'a> {
	pub fn new_pos(texture: &'a Texture, position: Vector2) -> Self {
		let width = texture.get_width() as f64;
		let height = texture.get_height() as f64;
		let image_rect = [position[0], position[1], width, height];
		let quad_rect = [0.0, 0.0, width, height];
		let my_image = Image::new().rect(image_rect).src_rect(quad_rect);
		Sprite {
			position: position, // we do need copies of position, size, and quad
			size: [width, height], 
			quad: quad_rect,
			image: my_image,
			texture: texture,
		}
	}
	pub fn new(texture: &'a Texture) -> Self {
		Self::new_pos(texture, [0.0, 0.0])
	}

	pub fn set_position(&mut self, position: Vector2) {
		self.position = position;
		self.image.rectangle = Some(
			[position[0], position[1], self.size[0], self.size[1]]
		);
	}
	pub fn get_position(&self) -> Vector2 {
		self.position
	}
	pub fn set_size(&mut self, size: Vector2) {
		self.size = size;
		self.image.rectangle = Some(
			[self.position[0], self.position[1], size[0], size[1]]
		);
	}
	pub fn scale(&mut self, scale: Vector2) {
		if scale[0] == 0.0 || scale[1] == 0.0 {
			panic!("Cannot set scale to 0");
		}
		let new_size = [self.size[0] * scale[0], self.size[1] * scale[1]];
		self.set_size(new_size);
	}
	pub fn get_size(&self) -> Vector2 {
		self.size
	}
	pub fn set_quad(&mut self, quad: Rect) {
		self.quad = quad;
		self.image.source_rectangle = Some(quad);
		self.size = [quad[2], quad[3]];
		self.image.rectangle = Some(
			[self.position[0], self.position[1], self.size[0], self.size[1]]
		);
	}
	pub fn get_quad(&self) -> Rect {
		self.quad
	}

	pub fn draw(&self, gl: &mut GlGraphics, context: &Context) {
		self.image.draw(self.texture, &context.draw_state, context.transform, gl)
	}

	pub fn draw_at(&self, position: [f64; 2], gl: &mut GlGraphics, context: &Context) {
		let mut transform = context.transform;
		self.image.draw(self.texture, &context.draw_state, transform, gl);
	}
}
//=================================================================================================
