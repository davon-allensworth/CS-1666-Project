extern crate rogue_sdl;

use sdl2::render::Texture;
use sdl2::rect::Rect;

const TILE_SIZE: u32 = 64;
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const CENTER_W: i32 = (CAM_W / 2 - TILE_SIZE / 2) as i32;
const CENTER_H: i32 = (CAM_H / 2 - TILE_SIZE / 2) as i32;

pub struct Background<'a> {
	pub black: Texture<'a>,
	pub texture_0: Texture<'a>,
	pub texture_1: Texture<'a>,
	pub texture_2: Texture<'a>,
	pub texture_3: Texture<'a>,
	pub x_tiles: (i32,i32),
	pub y_tiles: (i32,i32),
	pub tiles: Vec<(bool,i32)>,
}

impl<'a> Background<'a> {
	pub fn new(black: Texture<'a>, texture_0: Texture<'a>, texture_1: Texture<'a>, texture_2: Texture<'a>, texture_3: Texture<'a>, x_tiles: (i32,i32), y_tiles: (i32,i32)) -> Background<'a> {
		let tiles: Vec<(bool,i32)> = vec![(true,0); ((x_tiles.1+2)*(y_tiles.1+1)) as usize]; // (draw?, texture)
		Background {
			black,
			texture_0, 
			texture_1, 
			texture_2, 
			texture_3, 
			x_tiles,
			y_tiles,
			tiles,
		}
	}

	pub fn get_tile_info(&self, num: i32, i: i32, j: i32, x: f64, y: f64) -> (&Texture<'a>, Rect, Rect) {
		let texture;
		match num {
			7 => { texture = &self.texture_3 } // pillar 
			6 => { texture = &self.texture_2 } // border tiles
			1 => { texture = &self.texture_1 } // slime on tile
			_ => { texture = &self.texture_0 } // regular tile
		}
		// double tile size 
		let src;
		let pos;
		if num==7 {
			src = Rect::new(0, 0, TILE_SIZE * 2, TILE_SIZE * 2);
			pos = Rect::new(i * TILE_SIZE as i32 + (CENTER_W - x as i32),
								j * TILE_SIZE as i32 + (CENTER_H - y as i32),
								TILE_SIZE * 2, TILE_SIZE * 2);
		} else {
			src = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
			pos = Rect::new(i * TILE_SIZE as i32 + (CENTER_W - x as i32),
								j * TILE_SIZE as i32 + (CENTER_H - y as i32),
								TILE_SIZE, TILE_SIZE);
		}
		return (texture, src, pos);
	}

	pub fn texture(&self) -> &Texture {
        &self.texture_0
    }
}