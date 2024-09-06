use ndarray::Array2;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Point;
use sdl2;

use crate::lifetiles;
use crate::playertile;

pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub grid_cell_size: u32,
    pub life_grid_array: Option<Array2<u8>>,
    pub life_tiles: Option<lifetiles::LifeTiles>,
    pub player_tile : Option<playertile::PlayerTile>,

}

impl Grid {
    // Associated function to construct a new grid
    pub fn new(width: u32, height: u32, grid_cell_size: u32) -> Self {
        Grid {
            width,
            height,
            grid_cell_size,
            life_grid_array: None,
            life_tiles: None, 
            player_tile: None,           
        }
    }

    pub fn precalculate_grid(&mut self) {
        // This creates an array of the grid for other structs to access
        // Also it spawns in the class that will use the array once it's ready
        let dead_space: u32 = self.grid_cell_size * 2;

        // Calculate the number of rows and columns based on grid spacing
        let row_count: u32 = ((self.width - dead_space * 2) / self.grid_cell_size) + 1;
        let column_count: u32 = ((self.height - dead_space * 2) / self.grid_cell_size) + 1;

        // Create an Array2 with the dimensions from previous step
        self.life_grid_array = Some(Array2::<u8>::zeros((row_count as usize, column_count as usize)));
        

        // Initialize the life_tiles with the new grid
        if let Some(ref life_grid_array) = self.life_grid_array {
            self.life_tiles = Some(lifetiles::LifeTiles::new(life_grid_array.clone(), self.grid_cell_size));
    }
    }

    // Method to draw grid on the canvas
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear(); 

        canvas.set_draw_color(Color::RGB(255, 255, 255));

        let dead_space: u32 = self.grid_cell_size * 2;

        // Draw the vertical lines
        for i in (dead_space..=self.width - dead_space).step_by(self.grid_cell_size as usize) {
            canvas.draw_line(Point::new(i as i32, 0 + dead_space as i32), Point::new(i as i32, self.height as i32 - dead_space as i32)).unwrap();
        }
        // Draw the horizontal lines
        for i in (dead_space..=self.height - dead_space).step_by(self.grid_cell_size as usize) {
            canvas.draw_line(Point::new(0  + dead_space as i32, i as i32), Point::new(self.width as i32 - dead_space as i32, i as i32)).unwrap();
        }        
    }
}