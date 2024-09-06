use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

#[derive(Clone)]
pub struct PlayerTile {
    pub grid_cell_size: u32,
    pub player_width: u32,
    pub player_height: u32,
    pub player_x_pos: u32,
    pub player_y_pos: u32,
    pub window_width: u32,
    pub window_height: u32,
    pub current_grid_x: u32,
    pub current_grid_y: u32,

}

impl PlayerTile {
    // Draw the player tile to the grid
    pub fn draw_player(self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 0, 0)); 
        let rect: Rect = Rect::new(self.player_x_pos as i32, self.player_y_pos as i32, self.player_width, self.player_height);
        canvas.fill_rect(rect).unwrap();
    }
    // Move the player tile around the grid
    pub fn move_player(&mut self, dx: i64, dy: i64) {
        let dead_space: i64 = self.grid_cell_size as i64 * 2;
        let new_x_pos: i64 = self.player_x_pos as i64 + dx;
        let new_y_pos: i64 = self.player_y_pos as i64 + dy;

        // Make sure we don't move out of the grid
        if dead_space <= new_x_pos && new_x_pos <= self.window_width as i64 - dead_space {
            self.player_x_pos = new_x_pos as u32;
        }
        if dead_space <= new_y_pos && new_y_pos <= self.window_height as i64 - dead_space{
            self.player_y_pos = new_y_pos as u32;
        }
    }

    pub fn get_grid_position(&mut self) {
        // Find out where we are on the grid
        // This is used by the toggle_life function in the lifetiles module 
        let dead_space: u32 = self.grid_cell_size * 2;


        let player_x_pos = self.player_x_pos as f64;
        let player_y_pos = self.player_y_pos as f64;
        let dead_space = dead_space as f64;
        let grid_cell_size = self.grid_cell_size as f64;

        self.current_grid_x = ((player_x_pos - dead_space) / grid_cell_size).round() as u32;
        self.current_grid_y = ((player_y_pos - dead_space) / grid_cell_size).round() as u32;
    }
}