use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::{lifetiles::LifeTiles, playertile::PlayerTile};


pub struct EventHandler {
    pub is_paused: bool,
    pub is_quit: bool,
    pub grid_cell_size: u32,

}

impl EventHandler {
    pub fn handle_events(&mut self, event_pump: &mut sdl2::EventPump, player_tile: &mut PlayerTile, life_tiles: &mut LifeTiles) {
        // Handles events like keypresses and exiting the game
        self.is_quit = false;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape),  ..} => {
                    self.is_quit = true;
                    return 
                },
                Event::KeyUp { keycode: Some(Keycode::Left), ..} => {
                    if self.is_paused {
                        player_tile.move_player(-(self.grid_cell_size as i64), 0);}
                },
                Event::KeyUp { keycode: Some(Keycode::Right), ..} => {
                    if self.is_paused {
                        player_tile.move_player(self.grid_cell_size as i64, 0);
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::Up), ..} => {
                    if self.is_paused {
                        player_tile.move_player(0, -(self.grid_cell_size as i64));
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::Down), ..} => {
                    if self.is_paused {
                        player_tile.move_player(0, self.grid_cell_size as i64);
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::Space), ..} => {
                    if self.is_paused == false {
                        self.is_paused = true;
                    } else {
                        self.is_paused = false;
                    }                   
                },
                Event::KeyUp { keycode: Some(Keycode::LCtrl), ..} => {
                    if self.is_paused {
                        life_tiles.toggle_life(player_tile);
                    }
                },
                _ => {}
            }
        }
    }
}