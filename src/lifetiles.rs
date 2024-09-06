use ndarray::Array2;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use std::cmp::{min, max};

use crate::playertile::PlayerTile;

pub struct LifeTiles {
    pub grid: Array2<u8>,
    pub grid_cell_size: u32,
    


}


impl LifeTiles {
    pub fn new(grid: Array2<u8>, grid_cell_size: u32) -> Self {
        LifeTiles {
            grid,
            grid_cell_size,

        }
    }

    pub fn update_grid(&mut self) {
        // This is where the rules of the game of life are played
        const LIFE: u8 = 1;
        const NO_LIFE: u8 = 0;
        let mut adjacent_life_counter: u8;
        let mut new_grid = self.grid.clone();
        let (rows, columns) = new_grid.dim();

        for row in 0..rows {
            for column in 0..columns {
                adjacent_life_counter = self.count_live_neighbours(row, column);

                if self.grid[(row, column)] == LIFE {
                    if adjacent_life_counter < 2 || adjacent_life_counter > 3 {
                        new_grid[(row, column)] = NO_LIFE;
                    }
                 
                } else if self.grid[(row, column)] == NO_LIFE && adjacent_life_counter == 3  {
                        new_grid[(row, column)] = LIFE;
                }
            
            }          
        }
        self.grid = new_grid;
    }

    pub fn count_live_neighbours(&self, row: usize, column: usize) -> u8 {
        // Count the life in the neighbours of each square in the grid
        const LIFE: u8 = 1;
        let mut adjacent_life_counter: u8 = 0;

        let row_min: usize = max(0, row as isize - 1) as usize;
        let row_max: usize = min(row + 2, self.grid.nrows());
        let col_min: usize = max(0, column as isize - 1) as usize;
        let col_max: usize = min(column + 2, self.grid.ncols());

        for i in row_min..row_max {
            for j in col_min..col_max {
                if (i != row || j != column) && self.grid[(i, j)] == LIFE {
                    adjacent_life_counter += 1;
                }
            }
        }
        adjacent_life_counter
    }

    pub fn toggle_life(&mut self, player_tile: &mut PlayerTile) {
        // Turn life off, or on where the player currently is situated on the grid
        player_tile.get_grid_position();
        self.life_array_update(player_tile.current_grid_x, player_tile.current_grid_y)

    }

    pub fn draw_life_living(&mut self, canvas: &mut Canvas<Window>) {
        // Draw life as per the rules of the game of life
        const LIFE: u8 = 1;
        const NO_LIFE: u8 = 0;

        self.update_grid();
        let (rows, columns) = self.grid.dim();

        for row in 0..rows - 1{
            for column in 0..columns - 1 {
                if self.grid[(row, column)] == LIFE {
                    let (x_pos, y_pos) = self.find_position_from_grid_array((row as u32, column as u32));
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    let rect: Rect = Rect::new(x_pos as i32, y_pos as i32, self.grid_cell_size, self.grid_cell_size);
                    canvas.fill_rect(rect).unwrap();
                } else if self.grid[(row, column)] == NO_LIFE {
                    let (x_pos, y_pos) = self.find_position_from_grid_array((row as u32, column as u32));
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    let rect: Rect = Rect::new(x_pos as i32 + 1, y_pos as i32 + 1, self.grid_cell_size - 2, self.grid_cell_size - 2);
                    canvas.fill_rect(rect).unwrap();
                }
            }
        }
    }
    pub fn draw_paused_life(&mut self, canvas: &mut Canvas<Window>){
        // keep drawing life to the screen when game is paused
        const LIFE: u8 = 1;
        const NO_LIFE: u8 = 0;
        let (rows, columns) = self.grid.dim();

        for row in 0..rows {
            for column in 0..columns {
                if self.grid[(row, column)] == LIFE {
                    let (x_pos, y_pos): (u32, u32) = self.find_position_from_grid_array((row as u32, column as u32));
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    let rect: Rect = Rect::new(x_pos as i32, y_pos as i32, self.grid_cell_size, self.grid_cell_size);
                    canvas.fill_rect(rect).unwrap();
                } else if self.grid[(row, column)] == NO_LIFE {
                    let (x_pos, y_pos): (u32, u32) = self.find_position_from_grid_array((row as u32, column as u32));
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    let rect: Rect = Rect::new(x_pos as i32 + 1, y_pos as i32, self.grid_cell_size - 2, self.grid_cell_size - 2);
                    canvas.fill_rect(rect).unwrap();
                    
                }
            }
        }

    }

    fn find_position_from_grid_array(&mut self, (row, column): (u32, u32)) -> (u32, u32) {
        // Find where the player is so that the player and the life it spawns can be drawn in the correct spot
        const LINE_WIDTH: u32 = 1;
        let dead_space: u32 = self.grid_cell_size * 2;
        let x_pos: u32 = (dead_space + LINE_WIDTH) + (self.grid_cell_size * row);
        let y_pos: u32 = (dead_space + LINE_WIDTH) + (self.grid_cell_size * column);

        (x_pos, y_pos)
    }

    fn life_array_update(&mut self, grid_x_pos: u32, grid_y_pos: u32) {
        // Simple function to turn life off if it is currently on in a grid cell, and on if it is currently off in a grid cell
        const LIFE: u8 = 1;
        const NO_LIFE: u8 = 0;

        if self.grid[(grid_x_pos as usize, grid_y_pos as usize)] == NO_LIFE {
            self.grid[(grid_x_pos as usize, grid_y_pos as usize)] = LIFE
        } else if self.grid[(grid_x_pos as usize, grid_y_pos as usize)] == LIFE {
            self.grid[(grid_x_pos as usize, grid_y_pos as usize)] = NO_LIFE;
        }
    }



    



}