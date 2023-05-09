// Tetris

use crate::tetris::gamestate::GameState;
use crate::tetris::tetlib::*;
use crate::print;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

pub fn run() {
    print!("1");
    const MAX_LEVEL: usize = 20;
    const GRAV_TICK: usize = 40;
    const LEVEL_MULT: f64 = 0.85;
    let mut gs = GameState::new();

    print!("2");
    // main loop
    loop {
        let prev_display = gs.display.clone();

        // handle input
        let key = get_input();

        // quit
        if key == 'q' {
            break;
        }

        // gravity
        if gs.counter >= 100_000_000_000_000 {
            if gravity(&mut gs) {
                gs.is_game_over = true;
                break;
            }
        }

        // handle input
        handle_input(&mut gs, key);

        // hold piece
        if key == 'c' {
            hold(&mut gs);
        }

        // full line
        full_line(&mut gs);

        // ghost piece
        ghost_piece(&mut gs);

        // check if gs.display was changed
        let is_updated = gs.display != prev_display || gs.is_game_over;

        // render
        render(&mut gs, is_updated);
        // sleep(Duration::from_millis(args.gravity));
        gs.counter += 1;
    }

    // print!("{}", "\n".repeat(HEIGHT / 2 + 4));
}
