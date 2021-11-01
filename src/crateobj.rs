extern crate rogue_sdl;
use crate::gamedata::*;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use crate::player::*;
use sdl2::pixels;
use crate::SDLCore;

pub struct Crate{
	pos: Rect,
	src: Rect,
}

impl Crate {
    pub fn newc() -> Crate{// default constructor for testing
        let pos = Rect::new(100 as i32, 100 as i32, TILE_SIZE, TILE_SIZE);
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
        Crate{
            pos,
            src,
			
        }
    }
	pub fn new(pos: Rect) -> Crate {
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
		Crate{
			pos,
			src,
		}
	}

	pub fn src(&self) -> Rect {
		self.src
	}

	pub fn set_src(&mut self, new_src: Rect) {
		self.src = new_src;
	}

	pub fn pos(&self) -> Rect {
        self.pos
    }

	pub fn update_crates(&mut self,game_data: &mut GameData, core :&mut SDLCore, crate_textures: &Vec<Texture>) {
		for c in game_data.crates.iter_mut() {
		 core.wincan.copy(&crate_textures[0],c.src(),c.pos());
		}
	}
}