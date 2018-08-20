use assets::Sprite;
use assets::AssetManager;

struct Ground<'a> {
	sprite: Sprite<'a>
}

impl <'a> Ground<'a> {
	pub fn new(assets: &AssetManager) -> Self {
		Ground {
			sprite: Sprite::new(assets.get_texture("ground"))
		}
	}
}