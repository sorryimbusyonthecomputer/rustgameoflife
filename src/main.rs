use std::time::Duration;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::Sdl;
use sdl2;

mod eventhandler;
mod grid;
mod playertile;
mod lifetiles;


fn main() {
    let context: Sdl = sdl2::init().unwrap();
    let v_subsys: sdl2::VideoSubsystem = context.video().unwrap();

    let window: Window  = v_subsys.window("Welcome to the Rust grid", 800, 600).position_centered().build().unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    

    let mut grid: grid::Grid = grid::Grid::new(800, 600, 10);

   grid.player_tile = Some(playertile::PlayerTile {
        grid_cell_size: grid.grid_cell_size,
        player_width: grid.grid_cell_size - 1,
        player_height: grid.grid_cell_size - 1,
        player_x_pos: grid.grid_cell_size * 2 + 1,
        player_y_pos: grid.grid_cell_size * 2 + 1,
        window_width: grid.width,
        window_height: grid.height,
        current_grid_x: 0,
        current_grid_y: 0,
    });
    

    grid.precalculate_grid();
    let mut event_pump: sdl2::EventPump = context.event_pump().unwrap();

    let mut event_handler: eventhandler::EventHandler = eventhandler::EventHandler { is_paused: true, is_quit: false, grid_cell_size: grid.grid_cell_size};

    'game_window: loop {
        
        if let Some(player_tile) = grid.player_tile.as_mut() {
            event_handler.handle_events(&mut event_pump,  player_tile, grid.life_tiles.as_mut().unwrap());
        }
        if event_handler.is_quit {
            break 'game_window;
        }

        grid.draw(&mut canvas);

        if event_handler.is_paused {
            
            grid.life_tiles.as_mut().unwrap().draw_paused_life(&mut canvas);
            if let Some(playertile) = grid.player_tile.clone() {
                playertile.draw_player(&mut canvas);
            }
        } else if event_handler.is_paused == false {
            grid.life_tiles.as_mut().unwrap().draw_life_living(&mut canvas);
        }
        canvas.present();
        if event_handler.is_paused {
            std::thread::sleep(Duration::from_millis(10));
        } else if event_handler.is_paused ==  false {
            std::thread::sleep(Duration::from_millis(200));
        }
    }

}
