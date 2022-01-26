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

enum CurrentState {
    SETUP,
	PAUSED,
	RUNNING,
}

fn main() -> Result<(), String> {
	let width = 1920;
	let height = 1080;
	let scale = 20;
	let mut FPS = 24;

	let cooldown_timer = 4;
	let mut cooldown_counter = 0;
	let mut on_cooldown = false;

	let sdl_context = sdl2::init()?;
	let video_subsystem = sdl_context.video()?;

	let window = video_subsystem.window("something something", width, height)
		.position_centered()
		.fullscreen()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.present();
	let mut event_pump = sdl_context.event_pump().unwrap();

	let mut grid: Grid = Grid::build(height as i32, width as i32, scale);
	let mut state: CurrentState = CurrentState::SETUP;

	'running: loop {
		match state {
			CurrentState::SETUP => {
				for event in event_pump.poll_iter() {
					match event {
						Event::Quit {..} |
						Event::KeyDown { keycode: Some(Keycode::Escape), ..}
							=> { break 'running },
						Event::KeyDown { keycode: Some(Keycode::R), ..}
							=> { grid = Grid::build(height as i32, width as i32, scale) },
						Event::KeyDown { keycode: Some(Keycode::Q), ..}
						=> { 
							grid = Grid::build_empty(height as i32, width as i32, scale);
							FPS = 24;
							state = CurrentState::SETUP;
						},
						Event::KeyDown { keycode: Some(Keycode::Return), ..}
							=> { 
								FPS = 8; 
								state = CurrentState::RUNNING; 
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
					if cooldown_counter == cooldown_timer {
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
							=> { grid = Grid::build(height as i32, width as i32, scale) },
						Event::KeyDown { keycode: Some(Keycode::Q), ..}
							=> { 
								grid = Grid::build_empty(height as i32, width as i32, scale);
								FPS = 24;
								state = CurrentState::SETUP;
							},
						Event::KeyDown { keycode: Some(Keycode::I), ..}
							=> { 
								grid = Grid::build_from_file(height as i32, width as i32, scale as i32, "test.txt");
								FPS = 24;
								state = CurrentState::SETUP;
							},
						Event::KeyDown { keycode: Some(Keycode::E), ..}
							=> {
								grid.export_to_file("export.txt");
							}
						Event::KeyDown { keycode: Some(Keycode::Return), ..}
							=> { state = CurrentState::RUNNING; },
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
					if cooldown_counter == cooldown_timer {
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
							=> { state = CurrentState::PAUSED },
						Event::KeyDown { keycode: Some(Keycode::Plus), .. }
							=> { FPS += 1 },
						Event::KeyDown { keycode: Some(Keycode::Minus), .. }
							=> { FPS = if FPS == 1 { 1 } else { FPS - 1 } }
						_ => {}
					}
				}
				grid.tick();
			}
		}

		canvas.set_draw_color(BACKGROUND);
		canvas.clear();

		grid.draw(&mut canvas);

		canvas.present();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
	}

	Ok(())
}
