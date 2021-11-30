extern crate rogue_sdl;

use std::time::Instant;
use sdl2::rect::{Rect, Point};
use sdl2::render::{Texture};
use sdl2::image::LoadTexture;
use crate::projectile;
use crate::projectile::*;
use crate::gamedata::GameData;
use crate::gamedata::*;
use crate::power::*;
use crate::SDLCore;
use crate::player::Direction::{Down, Up, Left, Right};

#[derive(Copy, Clone)]
pub enum Direction{
	Up,
	Down,
	Left,
	Right,
	None,
}
#[derive(Copy, Clone)]
pub struct CollisionDecider{
	pub dir : Direction,
	pub dist : i32,
}

impl CollisionDecider{
	pub fn new(dir: Direction, dist: i32) -> CollisionDecider{
		let dir = dir;
		let dist = dist;
		CollisionDecider {
			dir,
			dist,
		}
	}
}

pub enum Weapon{
	Sword,
}

pub struct Player<'a> {
	// position values
	pos: (f64, f64),
	cam_pos: Rect,
	mass: f64,
	vel: (i32, i32),
	delta: (i32, i32),
	height: u32,
	width: u32,
	// display values
	src: Rect,
	attack_box: Rect,
	texture_all: Texture<'a>,
	// timers
	attack_timer: Instant,
	fire_timer: Instant,
	damage_timer: Instant,
	mana_timer: Instant,
	pickup_timer: Instant,
	shield_timer: Instant,
	dash_timer: Instant,
	// player attributes
	hp: u32,
	mana: i32,
	pub weapon: Weapon,
	power: PowerType,
	coins: u32,
	// check values
	max_mana: i32,
	max_hp: u32, 
	invincible: bool,
	shielded: bool,
	can_pickup: bool,
	can_pickup_shop: bool,
	shop_price: u32,
	pub facing_right: bool,
	is_attacking: bool,
	pub is_firing: bool,
	pub god_mode_timer: Instant,
	pub god_mode: bool,
}

impl<'a> Player<'a> {
	pub fn new(pos: (f64, f64), texture_all: Texture<'a>) -> Player<'a> {
		// position values
		let cam_pos = Rect::new(
			0,
			0,
			TILE_SIZE_CAM,
			TILE_SIZE_CAM,
		);
		let mass = 1.5;
		let vel = (0, 0);
		let delta = (0, 0);
		let height = TILE_SIZE;
		let width = TILE_SIZE;
		// display values
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
		let attack_box = Rect::new(0, 0, TILE_SIZE_CAM, TILE_SIZE_CAM);
		// timers
		let attack_timer = Instant::now();
		let fire_timer = Instant::now();
		let damage_timer = Instant::now();
		let mana_timer = Instant::now();
		let pickup_timer = Instant::now();
		let shield_timer = Instant::now();
		let dash_timer = Instant::now();
		// player attributes
		let hp = 30;
		let mana = 4;
		let weapon = Weapon::Sword;
		let power: PowerType;
		if DEBUG {power = PowerType::Shield; }
		else { power = PowerType::None; }
		let coins = 0;
		// check values
		let max_mana = 4;
		let max_hp = 30; 
		let invincible = true;
		let shielded = false; 
		let can_pickup = false;
		let can_pickup_shop = false;
		let shop_price = 0;
		let facing_right = false;
		let is_attacking = false;
		let is_firing = false;
		let god_mode_timer = Instant::now();
		let god_mode = false;

		Player {
			// position values
			pos,
			cam_pos,
			mass,
			vel,
			delta,
			height,
			width,
			// display values
			src,
			attack_box,
			texture_all,
			// timers
			attack_timer,
			fire_timer,
			damage_timer,
			mana_timer,
			pickup_timer,
			shield_timer,
			dash_timer,
			// player attributes
			hp,
			mana,
			weapon,
			power,
			coins,
			// check values
			max_mana,
			max_hp, 
			invincible,
			shielded,
			can_pickup,
			can_pickup_shop,
			shop_price,
			facing_right,
			is_attacking,
			is_firing,
			god_mode_timer,
			god_mode,
		}
	}

	// update player
	pub fn update_player(&mut self, game_data: &GameData, map: [[i32; MAP_SIZE_W]; MAP_SIZE_H], core: &mut SDLCore) -> Result<(), String>  {
		// debug stuff
		let tc = core.wincan.texture_creator();
		let hitbox =tc.load_texture("images/objects/crate.png")?;
		let src = Rect::new(0, 0, TILE_SIZE_CAM, TILE_SIZE_CAM);
		let speed_limit_adj = game_data.get_speed_limit();

		// Slow down to 0 vel if no input and non-zero velocity
		self.set_x_delta(resist(self.x_vel() as i32, self.x_delta() as i32));
		self.set_y_delta(resist(self.y_vel() as i32, self.y_delta() as i32));

		// Don't exceed speed limit
		match self.get_power() {
			PowerType::Dash => {
				if self.get_dash_timer() <= 1000 {
					self.set_x_vel((self.x_vel() + self.x_delta()).clamp((speed_limit_adj as f64 * -1.0 * 1.7) as i32,
																			(speed_limit_adj as f64 * 1.7) as i32));
					self.set_y_vel((self.y_vel() + self.y_delta()).clamp((speed_limit_adj as f64 * -1.0 * 1.7) as i32,
																			(speed_limit_adj as f64 * 1.7) as i32));
				} else {
					self.set_x_vel((self.x_vel() + self.x_delta()).clamp(speed_limit_adj as i32 * -1, speed_limit_adj as i32));
					self.set_y_vel((self.y_vel() + self.y_delta()).clamp(speed_limit_adj as i32 * -1, speed_limit_adj as i32));
				}
			},
			_ => {
				self.set_x_vel((self.x_vel() + self.x_delta()).clamp(speed_limit_adj as i32 * -1, speed_limit_adj as i32));
				self.set_y_vel((self.y_vel() + self.y_delta()).clamp(speed_limit_adj as i32 * -1, speed_limit_adj as i32));
			}
		}

		let h_bounds_offset = (self.y() / TILE_SIZE as f64) as i32;
		let w_bounds_offset = (self.x() / TILE_SIZE as f64) as i32;
		let mut collisions: Vec<CollisionDecider> = Vec::with_capacity(5);

		for h in 0..(CAM_H / TILE_SIZE) + 1 {
			for w in 0..(CAM_W / TILE_SIZE) + 1 {
				let w_pos = Rect::new((w as i32 + 0 as i32) * TILE_SIZE as i32 - (self.x() % TILE_SIZE as f64) as i32 - (CENTER_W - self.x() as i32),
									  (h as i32 + 0 as i32) * TILE_SIZE as i32 - (self.y() % TILE_SIZE as f64) as i32 - (CENTER_H - self.y() as i32),
									   TILE_SIZE, TILE_SIZE);

				let debug_pos = Rect::new((w as i32 + 0 as i32) * TILE_SIZE as i32 - (self.x() % TILE_SIZE as f64) as i32,
										  (h as i32 + 0 as i32) * TILE_SIZE as i32 - (self.y() % TILE_SIZE as f64) as i32,
										   TILE_SIZE, TILE_SIZE);

				if h as i32 + h_bounds_offset < 0 ||
				   w as i32 + w_bounds_offset < 0 ||
				   h as i32 + h_bounds_offset >= MAP_SIZE_H as i32 ||
				   w as i32 + w_bounds_offset >= MAP_SIZE_W as i32 ||
				   map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 0 {
					continue;
				} else if map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 2 || 
						  map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 5 {
					let p_pos = self.pos();

					if !DEBUG_NO_WALLS {
						if GameData::check_collision(&p_pos, &w_pos) {
							if DEBUG {
								core.wincan.copy(&hitbox, src, self.cam_pos)?;
								core.wincan.copy(&hitbox, src, debug_pos)?;
							}
							collisions.push(self.collect_col(p_pos, self.pos().center(), w_pos));
						}
					}
				}
			}
		}
		self.resolve_col(&collisions);

		for c in &game_data.crates{
			/* let crate_pos = c.pos();
			let p_pos =self.pos(); */
			if GameData::check_collision(&self.pos(), &c.pos()) {//I hate collisions
				//println!("welcome to hell");
				self.collect_col(self.pos(), self.pos().center(), c.pos());
			}
		}
		self.update_pos();/* game_data.rooms[0].xbounds, game_data.rooms[0].ybounds */
		// is the player currently attacking?
		if self.is_attacking { self.set_attack_box(self.x() as i32, self.y() as i32); }
		if self.get_attack_timer() > ATTK_COOLDOWN {
			self.is_attacking = false;
			// clear attack box
			self.attack_box = Rect::new(self.x() as i32, self.y() as i32, 0, 0);
		}
		// is the player currently firing?
		if self.fire_timer.elapsed().as_millis() > FIRE_COOLDOWN_P {
			self.is_firing =false;
		}
		// should the player be shielded
		if self.shield_timer.elapsed().as_millis() > SHIELD_TIME {
			self.shielded =false;
		}

		self.restore_mana();
		Ok(())
	}

	// player x values
	pub fn set_x(&mut self, x: f64){
		self.pos.0 = x;
	}
	pub fn x(&self) -> f64 {
		return self.pos.0;
	}
	pub fn set_x_vel(&mut self, x: i32){
		self.vel.0 = x;
	}
	pub fn x_vel(&self) -> i32 {
		return self.vel.0;
	}
	pub fn set_x_delta(&mut self, x: i32){
		self.delta.0 = x;
	}
	pub fn x_delta(&self) -> i32 {
		return self.delta.0;
	}
	pub fn width(&self) -> u32 {
		self.width
	}

	// player y values
	pub fn set_y(&mut self, y: f64){
		self.pos.1 = y;
	}
	pub fn y(&self) -> f64 {
		return self.pos.1;
	}
	pub fn set_y_vel(&mut self, y: i32){
		self.vel.1 = y;
	}
	pub fn y_vel(&self) -> i32 {
		return self.vel.1;
	}
	pub fn set_y_delta(&mut self, y: i32){
		self.delta.1 = y;
	}
	pub fn y_delta(&self) -> i32 {
		return self.delta.1;
	}
	pub fn height(&self) -> u32 {
		self.height
	}

	// update position
	#[allow(unused_variables)]
	pub fn update_pos(&mut self) {
		self.pos.0 = self.x() + self.x_vel() as f64 * 2.0 /* .clamp(x_bounds.0 as f64, x_bounds.1 as f64) */;
		self.pos.1 = self.y() + self.y_vel() as f64 * 2.0 /* .clamp(y_bounds.0 as f64, y_bounds.1 as f64) */;
	}

	pub fn set_src(&mut self, x: i32, y: i32) {
		self.src = Rect::new(x as i32, y as i32, TILE_SIZE_64, TILE_SIZE_64);
	}

	pub fn src(&self) -> Rect {
		self.src
	}

	pub fn pos(&self) -> Rect {
        return Rect::new(
			self.x() as i32,
			self.y() as i32,
			TILE_SIZE_PLAYER,
			TILE_SIZE_PLAYER,
		)
    }

	pub fn set_cam_pos(&mut self, x: i32, y: i32) {
		self.cam_pos = Rect::new(
			self.x() as i32 - x - (TILE_SIZE_CAM as i32 - TILE_SIZE_PLAYER as i32).abs()/2,
			self.y() as i32 - y - (TILE_SIZE_CAM as i32 - TILE_SIZE_PLAYER as i32).abs()/2,
			TILE_SIZE_CAM,
			TILE_SIZE_CAM,
		);
	}

	pub fn get_cam_pos(&self) -> Rect {
        self.cam_pos
    }

	pub fn get_mass(&self) -> f64 { self.mass }

	pub fn texture_all(&self) -> &Texture {
        &self.texture_all
    }

	pub fn get_frame_display(&mut self, gamedata: &mut GameData, fps_avg: f64) {
		let elapsed = gamedata.frame_counter.elapsed().as_millis() / (fps_avg as u128 * 2 as u128); // the bigger this divisor is, the faster the animation plays
		match elapsed % 12 as u128 {
			1 => { self.set_src(0 as i32, 0 as i32); }
			2 => { self.set_src(64 as i32, 0 as i32); }
			3 => { self.set_src(128 as i32, 0 as i32); }
			4 => { self.set_src(0 as i32, 64 as i32); }
			5 => { self.set_src(64 as i32, 64 as i32); }
			6 => { self.set_src(128 as i32, 64 as i32); }
			7 => { self.set_src(0 as i32, 128 as i32); }
			8 => { self.set_src(64 as i32, 128 as i32); }
			9 => { self.set_src(128 as i32, 128 as i32); }
			10 => { self.set_src(0 as i32, 192 as i32); }
			11 => { self.set_src(64 as i32, 192 as i32); }
			_ => { self.set_src(128 as i32, 192 as i32); }
		}
	}

	// attacking values
	pub fn get_attack_timer(&self) -> u128 {
		self.attack_timer.elapsed().as_millis()
	}

	pub fn get_attack_box(&self) -> Rect {
		self.attack_box
	}

	pub fn set_attack_box(&mut self, x: i32, y: i32) {
		if self.facing_right{
			self.attack_box = Rect::new(x + TILE_SIZE as i32, y as i32, ATTACK_LENGTH, TILE_SIZE);
		} else {
			self.attack_box = Rect::new(x - ATTACK_LENGTH as i32, y as i32, ATTACK_LENGTH, TILE_SIZE);
		}
	}

	pub fn attack(&mut self) {
		if self.get_attack_timer() < ATTK_COOLDOWN {
			return;
		}
		self.is_attacking = true;
		self.set_attack_box(self.x() as i32, self.y() as i32);
		self.attack_timer = Instant::now();
	}

	pub fn fire(&mut self, mouse_x: i32, mouse_y: i32, speed_limit: f64, p_type: ProjectileType, elapsed: u128) -> Projectile {
		self.is_firing = true;
		match p_type {
			ProjectileType::Shield => {
				self.use_mana(4);
			}
			ProjectileType::Bullet => {
				self.use_mana(2);
			}
			_ => {
				self.use_mana(1);
			}
		}
		self.fire_timer = Instant::now();

		let vec = vec![mouse_x as f64 - CENTER_W as f64 - (TILE_SIZE_HALF) as f64, mouse_y as f64 - CENTER_H as f64 - (TILE_SIZE_HALF) as f64];
		let angle = ((vec[0] / vec[1]).abs()).atan();
		let speed: f64 = 3.0 * speed_limit;
		let mut x = &speed * angle.sin();
		let mut y = &speed * angle.cos();
		if vec[0] < 0.0 {
			x *= -1.0;
		}
		if vec[1] < 0.0 {
			y *= -1.0;
		}

		let p_type = p_type;
		let bullet = projectile::Projectile::new(
			Rect::new(
				self.x() as i32,
				self.y() as i32,
				TILE_SIZE_PROJECTILE,
				TILE_SIZE_PROJECTILE,
			),
			false,
			vec![x, y],
			p_type,
			elapsed,
		);
		return bullet;
	}

	pub fn get_attacking(&self) -> bool {
		return self.is_attacking
	}

	//mana values
	pub fn get_mana(&self) -> i32 {
		return self.mana
	}

	pub fn get_mana_timer(&self) -> u128 {
		self.mana_timer.elapsed().as_millis()
	}

	pub fn use_mana(&mut self, x: i32) {
		self.mana -= x;
	}

	pub fn restore_mana(&mut self) {
		if self.get_mana_timer() < MANA_RESTORE_RATE || self.get_mana() >= self.max_mana {
			return;
		}

		self.mana += 1;
		self.mana_timer = Instant::now();
	}

	// power functions
	pub fn get_power(&self) -> &PowerType {
		&self.power
	}

	pub fn set_power(&mut self, power: PowerType) {
		self.power = power;
	}

	pub fn can_pickup(&self) -> bool {
		self.can_pickup
	}

	pub fn set_can_pickup(&mut self, can: bool) {
		self.can_pickup = can;
	}

	pub fn can_pickup_shop(&self) -> bool {
		self.can_pickup_shop
	}

	pub fn set_can_pickup_shop(&mut self, can: bool) {
		self.can_pickup_shop = can;
	}

	pub fn get_shop_price(&self) -> u32 {
		self.shop_price
	}

	pub fn set_shop_price(&mut self, price: u32) {
		self.shop_price = price;
	}

	pub fn get_pickup_timer(&self) -> u128 {
		self.pickup_timer.elapsed().as_millis()
	}

	pub fn reset_pickup_timer(&mut self) {
		self.pickup_timer = Instant::now();
	}

	pub fn set_shielded(&mut self, b: bool){
		self.shield_timer = Instant::now();
		self.mana -= 3; 
		self.shielded = b; 
	}

	pub fn get_shielded(&self) -> bool {
		self.shielded
	}

	pub fn get_dash_timer(&self) -> u128 {
		self.dash_timer.elapsed().as_millis()
	}

	pub fn set_dash_timer(&mut self) {
		self.dash_timer = Instant::now();
		self.mana -= 4;
	}

	// heatlh values
	pub fn get_hp(&self) -> u32 {
		return self.hp
	}

	pub fn is_dead(&self) -> bool {
		return self.hp <= 0;
	}

	pub fn minus_hp(&mut self, dmg: u32) {
		if self.set_get_invincible() || self.god_mode {
			return;
		}
		let adjusted_dmg: u32; 
		if self.get_shielded() { 
			adjusted_dmg = dmg-5; 
			self.set_shielded(false);
		}
		else { adjusted_dmg = dmg;  }
		self.damage_timer = Instant::now();
		self.hp -= adjusted_dmg;
	}

	pub fn plus_hp(&mut self, health: u32) {
		if self.hp+health >= self.max_hp {
			self.hp = self.max_hp; 
		}
		else { self.hp += health; }
	}

	pub fn upgrade_hp(&mut self, health: u32) {
		self.max_hp += health; 
	}

	pub fn set_get_invincible(&mut self) -> bool {
		if self.damage_timer.elapsed().as_millis() < DMG_COOLDOWN {
			self.invincible = true;
		} else {
			self.invincible = false;
		}
		return self.invincible; 
	}

	//coin values
	pub fn get_coins(&self) -> u32 {
		return self.coins
	}

	pub fn add_coins(&mut self, coins_to_add: u32)  {
		self.coins += coins_to_add;
	}

	pub fn sub_coins(&mut self, coins_to_add: u32)  {
		self.coins -= coins_to_add;
	}

	pub fn collect_col(&mut self, p_pos: Rect, p_center: Point, other_pos :Rect) -> CollisionDecider {
		let distance = ((p_center.x() as f64 - other_pos.center().x() as f64).powf(2.0) + (p_center.y() as f64 - other_pos.center().y() as f64).powf(2.0)).sqrt();

		// player above other
		if p_pos.bottom() >= other_pos.top() && p_center.y() < other_pos.top(){
			let resolution = CollisionDecider::new(Down, distance as i32);
			return resolution;
		}
		// player left of other
		if p_pos.right() >= other_pos.left() && p_center.x() < other_pos.left() {
			let resolution = CollisionDecider::new(Right, distance as i32);
			return resolution;
		}
		// player below other
		if p_pos.top() <= other_pos.bottom() && p_center.y() > other_pos.bottom(){
			let resolution = CollisionDecider::new(Up, distance as i32);
			return resolution;
		}
		// player right of other
		 else {
			 let resolution = CollisionDecider::new(Left, distance as i32);
			 return resolution;
		}
	}

	pub fn resolve_col(&mut self, collisions : &Vec<CollisionDecider>){
		// Sort vect of collisions by distance
		let mut sorted_collisions: Vec<CollisionDecider> = Vec::new();
		for c in collisions{
			let new_dir = &c.dir;
			sorted_collisions.push(CollisionDecider::new(*new_dir,c.dist) );
		}
		sorted_collisions.sort_by_key(|x| x.dist);

		// Handle collisions based on distance
		if sorted_collisions.len() > 0 {
			match sorted_collisions[0].dir{
				Direction::Up=>{
					self.set_y_vel(self.y_vel().clamp(0,100));
					if sorted_collisions.len() > 2 {
						match sorted_collisions[2].dir{
							Direction::Up=>{
								self.set_y_vel(self.y_vel().clamp(0,100));
							}
							Direction::Down=>{
								println!("I have no clue how this happened");
							}
							Direction::Left=>{
								self.set_x_vel(self.x_vel().clamp(0,100));

							}
							Direction::Right=>{
								self.set_x_vel(self.x_vel().clamp(-100,0));

							}
							Direction::None=>{
								println!("I have no clue how this happened");
							}
						}
					}
				}
				Direction::Down=>{
					self.set_y_vel(self.y_vel().clamp(-100,0));
					if sorted_collisions.len() > 2 {
						match sorted_collisions[2].dir{
							Direction::Up=>{
								println!("I have no clue how this happened");
							}
							Direction::Down=>{
								self.set_y_vel(self.y_vel().clamp(-100,0));
							}
							Direction::Left=>{
								self.set_x_vel(self.x_vel().clamp(0,100));
							}
							Direction::Right=>{
								self.set_x_vel(self.x_vel().clamp(-100,0));
							}
							Direction::None=>{
								println!("I have no clue how this happened");
							}
						}
					}
				}
				Direction::Right=>{
					self.set_x_vel(self.x_vel().clamp(-100,0));
					if sorted_collisions.len() > 2 {
						match sorted_collisions[2].dir{
							Direction::Up=>{
								self.set_y_vel(self.y_vel().clamp(0,100));
							}
							Direction::Down=>{
								self.set_y_vel(self.y_vel().clamp(-100,0));
							}
							Direction::Left=>{
								println!("I have no clue how this happened");
							}
							Direction::Right=>{
								self.set_x_vel(self.x_vel().clamp(-100,0));
							}
							Direction::None=>{
								println!("I have no clue how this happened");
							}
						}
					}
				}
				Direction::Left=>{
					self.set_x_vel(self.x_vel().clamp(0,100));
					if sorted_collisions.len() > 2 {
						match sorted_collisions[1].dir{
							Direction::Up=>{
								self.set_y_vel(self.y_vel().clamp(0,100));
							}
							Direction::Down=>{
								self.set_y_vel(self.y_vel().clamp(-100,0));
							}
							Direction::Left=>{
								self.set_x_vel(self.x_vel().clamp(0,100));
							}
							Direction::Right=>{
								println!("I have no clue how this happened");
							}
							Direction::None=>{
								println!("I have no clue how this happened");
							}
						}
					}
				}
				Direction::None=>{
					println!("I have no clue how this happened");
				}
			}
		}
	}

	pub fn get_god_mode_timer(&self) -> u128 {
		self.god_mode_timer.elapsed().as_millis()
	}

	pub fn set_god_mode_timer(&mut self) {
		self.god_mode_timer = Instant::now();
	}
}

// calculate velocity resistance
pub(crate) fn resist(vel: i32, delta: i32) -> i32 {
	if delta == 0 {
		if vel > 0 {-1}
		else if vel < 0 {1}
		else {delta}
	} else {delta}
}