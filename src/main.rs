#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::*;
use std::time::Duration;

pub mod grid;
use grid::*;

pub mod import_modal;
use import_modal::*;

enum CurrentState {
    SETUP,
	PAUSED,
	RUNNING,
	IMPORT,
}

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;
const SCALE: i32 = 20;
const EDIT_FPS: i32 = 24;
const RUN_FPS: i32 = 10;
const COOLDOWN_TIMER: i32 = 8;

fn main() -> Result<(), String> {
	let mut FPS = EDIT_FPS;
	let mut stored_FPS: i32 = RUN_FPS;

	let mut cooldown_counter = 0;
	let mut on_cooldown = false;

	let sdl_context = sdl2::init()?;
	let video_subsystem = sdl_context.video()?;

	let window = video_subsystem.window("Game Of Life", WIDTH as u32, HEIGHT as u32)
		.position_centered()
		.fullscreen()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.present();
	let mut event_pump = sdl_context.event_pump().unwrap();
	let texture_creator = canvas.texture_creator();

	let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

	let fc = FontCreator::build("./assets/FiraCode-Retina.ttf".to_string(), ttf_context, texture_creator, SCALE);

	let mut grid: Grid = Grid::build(HEIGHT, WIDTH, SCALE);
	let mut state: CurrentState = CurrentState::SETUP;
	let mut modal: ImportModal = ImportModal::build(HEIGHT, WIDTH, SCALE, fc);

	'running: loop {
		match state {
			CurrentState::SETUP => {
				for event in event_pump.poll_iter() {
					match event {
						Event::Quit {..} |
						Event::KeyDown { keycode: Some(Keycode::Escape), ..}
							=> { break 'running },
						Event::KeyDown { keycode: Some(Keycode::R), ..}
							=> { grid = Grid::build(HEIGHT as i32, WIDTH as i32, SCALE) },
						Event::KeyDown { keycode: Some(Keycode::Q), ..}
						=> { 
							grid = Grid::build_empty(HEIGHT as i32, WIDTH as i32, SCALE);
							FPS = 30;
							state = CurrentState::SETUP;
						},
						Event::KeyDown { keycode: Some(Keycode::Return), ..}
							=> { 
								FPS = RUN_FPS; 
								state = CurrentState::RUNNING; 
							},
						Event::KeyDown { keycode: Some(Keycode::I), ..}
							=> {
								// change this if you want to import another file
								grid = Grid::build_from_file(HEIGHT as i32, WIDTH as i32, SCALE as i32, "gun_and_eater@20.txt");
								FPS = EDIT_FPS;
								state = CurrentState::SETUP;
							},
						Event::KeyDown { keycode: Some(Keycode::E), ..}
							=> {
								grid.export_to_file("export.txt");
							},
						_ => {}
					}
				}

				// bad hack to avoid registering presses multiple times. 
				// i hate this with all my heart but it seems it works so who am 
				// i to judge it.
				if !on_cooldown {
					if grid.update(&event_pump) {
						on_cooldown = true;
					}
				} else {
					cooldown_counter += 1;
					if cooldown_counter == COOLDOWN_TIMER {
						on_cooldown = false;
						cooldown_counter = 0;
					}
				}

			},
			CurrentState::PAUSED => {
				for event in event_pump.poll_iter() {
					match event {
						Event::Quit {..} |
						Event::KeyDown { keycode: Some(Keycode::Escape), ..}
							=> { break 'running },
						Event::KeyDown { keycode: Some(Keycode::R), ..}
							=> { grid = Grid::build(HEIGHT as i32, WIDTH as i32, SCALE) },
						Event::KeyDown { keycode: Some(Keycode::Q), ..}
							=> { 
								grid = Grid::build_empty(HEIGHT as i32, WIDTH as i32, SCALE);
								FPS = EDIT_FPS;
								state = CurrentState::SETUP;
							},
						Event::KeyDown { keycode: Some(Keycode::I), ..}
							=> {
								// change this if you want to import another file
								grid = Grid::build_from_file(HEIGHT as i32, WIDTH as i32, SCALE as i32, "gun_and_eater@20.txt");
								FPS = EDIT_FPS;
								state = CurrentState::SETUP;
							},
						Event::KeyDown { keycode: Some(Keycode::E), ..}
							=> {
								grid.export_to_file("export.txt");
							},
						Event::KeyDown { keycode: Some(Keycode::M), ..}
							=> {
								modal.toggle();
								state = CurrentState::IMPORT;
							}
						Event::KeyDown { keycode: Some(Keycode::Return), ..}
							=> { 
								FPS = stored_FPS;
								state = CurrentState::RUNNING; 
							},
						Event::KeyDown { keycode: Some(Keycode::Plus), .. }
							=> { FPS += 1 },
						Event::KeyDown { keycode: Some(Keycode::Minus), .. }
							=> { FPS = if FPS == 1 { 1 } else { FPS - 1 } }
						_ => {}
					}
				}

				if !on_cooldown {
					if grid.update(&event_pump) {
						on_cooldown = true;
					}
				} else {
					cooldown_counter += 1;
					if cooldown_counter == COOLDOWN_TIMER {
						on_cooldown = false;
						cooldown_counter = 0;
					}
				}
			},
			CurrentState::RUNNING => {
				for event in event_pump.poll_iter() {
					match event {
						Event::Quit {..} |
						Event::KeyDown { keycode: Some(Keycode::Escape), .. } 
							=> { break 'running },
						Event::KeyDown { keycode: Some(Keycode::Return), ..}
							=> { 
								stored_FPS = FPS;
								FPS = EDIT_FPS;
								state = CurrentState::PAUSED
							},
						Event::KeyDown { keycode: Some(Keycode::Plus), .. }
							=> { FPS += 1 },
						Event::KeyDown { keycode: Some(Keycode::Minus), .. }
							=> { FPS = if FPS == 1 { 1 } else { FPS - 1 } }
						_ => {}
					}
				}

				grid.tick();
			},
			CurrentState::IMPORT => {
				for event in event_pump.poll_iter() {
					match event {
						Event::Quit {..} |
						Event::KeyDown { keycode: Some(Keycode::Escape), .. } 
							=> { break 'running },
						Event::KeyDown { keycode: Some(Keycode::M), ..}
							=> {
								modal.toggle();
								state = CurrentState::PAUSED;
							}
						Event::KeyDown { keycode: Some(Keycode::Up), ..}
							=> { modal.up(); }
						Event::KeyDown { keycode: Some(Keycode::Down), ..}
							=> { modal.down() }
						Event::KeyDown { keycode: Some(Keycode::Return), ..} 
							=> {
								let chosen_file = modal.get_chosen_file();
								grid = Grid::build_from_file(HEIGHT as i32, WIDTH as i32, SCALE as i32, chosen_file);
								FPS = EDIT_FPS;
								modal.toggle();
								state = CurrentState::SETUP;
							}
						_ => { }
					}
				}
			}
		}

		canvas.set_draw_color(BACKGROUND);
		canvas.clear();

		grid.draw(&mut canvas);

		if modal.get_visible() {
			modal.draw(&mut canvas);
		}
		
		canvas.present();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / (FPS as u32)));
	}

	Ok(())
}
