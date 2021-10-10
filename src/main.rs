extern crate rogue_sdl;
mod enemy;
mod player;
mod credits;
use rand::Rng;

use std::time::Duration;
use std::collections::HashSet;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
//use sdl2::render::Texture;

use rogue_sdl::SDLCore;
use rogue_sdl::Game;
const TITLE: &str = "Roguelike";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const SPEED_LIMIT: i32 = 10;
const ACCEL_RATE: i32 = 1;

const TILE_SIZE: u32 = 64;


fn resist(vel: i32, deltav: i32) -> i32 {
	if deltav == 0 {
		if vel > 0 {
			-1
		}
		else if vel < 0 {
			1
		}
		else {
			deltav
		}
	}
	else {
		deltav
	}
}










pub struct ROGUELIKE {
	core: SDLCore,
}

impl Game for ROGUELIKE {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(ROGUELIKE{ core })
	}

	fn run(&mut self) -> Result<(), String> {
        let texture_creator = self.core.wincan.texture_creator();

		let w = 25;
		let mut x_pos = (CAM_W/2 - w/2) as i32;
		let mut y_pos = (CAM_H/2 - w/2) as i32;

		let mut x_vel = 0;
		let mut y_vel = 0;
		let mut e_x_vel = 0;
		let mut e_y_vel = 0;
		let mut rng = rand::thread_rng();
		let mut t = 0;// this is just a timer for the enemys choice of movement
		let mut e = enemy::Enemy::new(
	
	Rect::new(
		(CAM_W/2 - TILE_SIZE/2) as i32,
		(CAM_H/2 - TILE_SIZE/2) as i32,
		TILE_SIZE,
		TILE_SIZE,
	),
	texture_creator.load_texture("images/enemies/place_holder_enemy.png")?,
);
        let mut p = player::Player::new(
			Rect::new(
				(CAM_W/2 - TILE_SIZE/2) as i32,
				(CAM_H/2 - TILE_SIZE/2) as i32,
				TILE_SIZE,
				TILE_SIZE,
			),
            texture_creator.load_texture("images/player/blue_slime_l.png")?,
			texture_creator.load_texture("images/player/blue_slime_r.png")?,
		);
		let mut roll = rng.gen_range(1..4);

		'gameloop: loop {
			for event in self.core.event_pump.poll_iter() {
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
					_ => {},
				}
			}

			let keystate: HashSet<Keycode> = self.core.event_pump
				.keyboard_state()
				.pressed_scancodes()
				.filter_map(Keycode::from_scancode)
				.collect();

			let mut x_deltav = 0;
			let mut y_deltav = 0;
			if keystate.contains(&Keycode::W) {
				y_deltav -= ACCEL_RATE;
				//Move up
			}
			if keystate.contains(&Keycode::A) {
				x_deltav -= ACCEL_RATE;
                p.facing_left = true;
			}
			if keystate.contains(&Keycode::S) {
				y_deltav += ACCEL_RATE;
			}
			if keystate.contains(&Keycode::D) {
				x_deltav += ACCEL_RATE;
                p.facing_left = false;
			}

			// Slow down to 0 vel if no input and non-zero velocity
			x_deltav = resist(x_vel, x_deltav);
			y_deltav = resist(y_vel, y_deltav);

			// Don't exceed speed limit
			x_vel = (x_vel + x_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);
			y_vel = (y_vel + y_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);

			// Stay inside the viewing window
			x_pos = (x_pos + x_vel).clamp(0, (CAM_W - w) as i32);
			y_pos = (y_pos + y_vel).clamp(0, (CAM_H - w) as i32);

            p.update_pos((x_vel, y_vel), (0, (CAM_W - TILE_SIZE) as i32), (0, (CAM_H - TILE_SIZE) as i32));
			
			
			if(t >50){
			roll = rng.gen_range(1..5);
			t=0;
			}
			println!("{} ", roll);

			if(roll == 1){
				e.update_enemy_pos((e_x_vel+1, e_y_vel), (0, (CAM_W - TILE_SIZE) as i32), (0, (CAM_H - TILE_SIZE) as i32));
			}
			if(roll == 2){
				e.update_enemy_pos((e_x_vel, e_y_vel+1), (0, (CAM_W - TILE_SIZE) as i32), (0, (CAM_H - TILE_SIZE) as i32));
			}
			if(roll == 3){
				e.update_enemy_pos((e_x_vel, e_y_vel-1), (0, (CAM_W - TILE_SIZE) as i32), (0, (CAM_H - TILE_SIZE) as i32));
			}
			if(roll == 4){
				e.update_enemy_pos((e_x_vel-1, e_y_vel), (0, (CAM_W - TILE_SIZE) as i32), (0, (CAM_H - TILE_SIZE) as i32));
			}
			self.core.wincan.set_draw_color(Color::BLACK);
			self.core.wincan.clear();

            let background = texture_creator.load_texture("images/background/black_background.jpg")?;
            self.core.wincan.copy(&background, None, None)?;

            /* let cur_bg = Rect::new(
				((p.x() + ((p.width() / 2) as i32)) - ((CAM_W / 2) as i32)).clamp(0, (CAM_W * 2 - CAM_W) as i32),
				((p.y() + ((p.height() / 2) as i32)) - ((CAM_H / 2) as i32)).clamp(0, (CAM_H * 2 - CAM_H) as i32),
				CAM_W,
				CAM_H,
			); */

			self.core.wincan.copy(e.txtre(), e.src(), e.pos())?;

            if*(p.facing_left())
            {
                self.core.wincan.copy(p.texture_l(), p.src(), p.pos())?;
            } else {
                self.core.wincan.copy(p.texture_r(), p.src(), p.pos())?;
            }

			self.core.wincan.present();

			t +=1 ;

		}
		// Out of game loop, return Ok
		Ok(())
	}
}

pub fn main() -> Result<(), String> {
    rogue_sdl::runner(TITLE, ROGUELIKE::init);
     credits::run_credits();
    Ok(())
}
